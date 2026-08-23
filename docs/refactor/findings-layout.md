# 레이아웃 엔진은 왜 아무 효과가 없는가

`docs/refactor/findings-render-pipeline.md`의 **F-6** — CSS `width`, `padding`,
`gap`, `display: none`이 화면에 아무 영향을 주지 않는다 — 을 파고든 결과.

애초의 진단은 "위젯들이 자기 자식 배치를 스스로 계산하므로 엔진 출력을 아무도 읽지
않는다"였다. 그건 사실이지만 **마지막 단계**였다. 그 앞에 세 개가 더 있었고, 그중 하나는
엔진이 애초에 아무것도 계산하지 못하게 만들고 있었다.

## 요약

| | 증상 | 상태 |
|---|---|---|
| **L-1** | 레이아웃 트리에 부모→자식 간선이 하나도 없다 | ✅ 고침 |
| **L-2** | 위젯 자신의 배치 의도(`vstack` 방향, `gap`, 자식 크기)가 노드 스타일에 안 실린다 | ❌ |
| **L-3** | auto 교차축 크기가 1셀로 붕괴한다 | ❌ |
| **L-4** | `dom_from_render`에서 레이아웃이 한 프레임 뒤처진다 | ❌ |
| **L-5** | 계산된 rect를 렌더 경로에서 아무도 읽지 않는다 | ❌ |

L-1이 풀리기 전까지 L-2\~L-5는 관측조차 불가능했다. 루트 말고는 전부 0×0이었기 때문이다.

## L-1 — 레이아웃 트리가 평평했다

`App::build_layout_tree`는 **부모를 먼저 만들고 자식을 재귀했다.**

```rust
self.layout.create_node_with_children(dom_id, &style, &children);
for child in children { self.build_layout_tree(child); }
```

그런데 `LayoutEngine::create_node_with_children`는 **이미 만들어진** 자식만 연결한다.

```rust
let child_ids: Vec<u64> = children
    .iter()
    .filter_map(|id| self.nodes.get(id).copied())   // 아직 아무도 없다
    .collect();
```

호출 시점에 자식이 하나도 존재하지 않으므로 `child_ids`는 **항상 비어 있었다.** 결과적으로
레이아웃 트리는 노드만 있고 간선이 없는 상태였고, `compute`는 루트에 `(0, 0, w, h)`를
써넣은 뒤 아무 데도 내려가지 않았다. 루트를 제외한 모든 노드의 computed layout은 기본값
`0×0`이었다.

**고침:** 후위 순회로 바꿔 자식을 먼저 만든다. `tests/layout_tree.rs`가 고정한다 —
되돌리면 9개 테스트가 전부 깨진다(확인함).

이것이 "엔진 출력을 아무도 읽지 않는다"의 진짜 이유이기도 하다. 읽을 것이 없었다.

## L-2 — 위젯의 배치 의도가 DOM에 없다

`vstack()`은 Rust 빌더다. 방향도, `gap`도, `child_size`도 `Stack` 구조체 필드에만 있고
`WidgetMeta`에는 실리지 않는다. 그래서 DOM 노드의 스타일은 기본값이고, 엔진은 기본
`flex-direction: row`로 계산한다.

```
화면:  A   (열)          엔진:  A B   (행)
       B
```

HTML이 `<td align=right>` 같은 표현 속성을 presentational CSS로 매핑하는 것과 같은 통로가
필요하다 — `View::meta`가 인라인 스타일을 실어 보내고, 캐스케이드가 그것을 작성자 스타일보다
낮은 우선순위로 병합하는 것.

**이것이 풀리기 전에는 계산된 rect를 권위로 삼을 수 없다.** 아무도 요청하지 않은 배치를
서술하고 있기 때문이다.

## L-3 — auto 교차축이 1셀로 붕괴

`AlignItems`의 기본값이 `Start`이고, 내재 콘텐츠 크기를 재는 수단이 엔진에 없다. 그래서
교차축 크기가 `Size::Auto`인 flex 아이템은 `1`을 받는다(`flex.rs`).

CSS의 초깃값은 `normal`이고 flex 아이템에서는 `stretch`처럼 동작한다. 이 크레이트의 모든
컨테이너도 자식에게 교차축 전체를 준다. 즉 기본값이 CSS와도, 자기 자신과도 어긋난다.

`AlignItems::default()`를 바꾸는 것으로는 못 고친다 — 캐스케이드가
`!= AlignItems::default()`로 "명시적으로 지정됐는지"를 판별하고 있어서
(`dom/cascade/merge.rs`, `style/properties/style.rs`) 기본값을 바꾸면
`align-items: start`가 병합되지 않는다. flex 쪽에서 "auto 교차축 = 채움"으로 다루는 것이
맞다. 정렬은 아이템이 컨테이너보다 작을 때만 의미가 있다.

## L-4 — 한 프레임 뒤처짐

`dom_from_render`에서는 DOM이 렌더 패스 **안에서** 만들어진다. 그런데 `App::draw`는 렌더
이전에 레이아웃을 돌린다. 그래서 직전 프레임이 남긴 트리를 계산하고, 첫 프레임에는 계산할
것이 아예 없다.

올바른 순서는 수집 패스와 페인트 패스 사이다.

```
collect → reconcile → compute styles → compute layout → paint
```

`LayoutEngine`이 `App`에 있고 `DomRenderer::render`가 그걸 못 보기 때문에 지금 구조로는
불가능하다. 엔진을 `DomRenderer`로 옮기는 것이 자연스럽다 — 레이아웃 트리는 DOM 트리를
그대로 미러링하므로 소유자가 같아야 한다.

## L-5 — 아무도 읽지 않는다

`render_child`가 이미 모든 자식 렌더의 단일 통로이므로, 여기서 컨테이너가 건네준 area 대신
노드의 계산된 rect를 쓰면 된다. 컨테이너를 재작성할 필요는 없다 — 산술이 무시될 뿐이고,
기본값이 뒤집힌 뒤에 지우면 된다.

계산된 rect는 **부모의 콘텐츠 박스 기준 상대 좌표**다. `render_child`는 부모의 절대 area를
알고 있으므로 더하면 된다.

`display: none` 노드를 건너뛸 때는 페인트 패스의 커서를 그 **서브트리 전체만큼** 밀어야
한다. 두 순회는 카운터로 정렬되어 있어서, 한쪽만 건너뛰면 그 뒤가 전부 어긋난다.

## 순서

L-2 → L-3 → L-4 → L-5. L-5는 플래그(`layout_from_dom`, 기본 off) 뒤에서 켠다.
