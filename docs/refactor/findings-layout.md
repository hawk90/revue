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
| **L-5** | CSS 레이아웃 속성이 화면에 아무 영향이 없다 | ✅ 고침 (플래그) |

L-1이 풀리기 전까지 L-2\~L-4는 관측조차 불가능했다. 루트 말고는 전부 0×0이었기 때문이다.

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

## L-5 — 아무도 읽지 않는다 → 해결(다른 방식으로)

여기서 갈림길이 있었다.

**(A) 엔진을 권위로.** 계산된 rect가 컨테이너의 산술을 대체한다. 브라우저 모델. 하지만
L-2가 먼저 풀려야 하고 — 즉 `vstack()`의 방향·gap·자식 크기가 전부 인라인 스타일로
DOM에 실려야 하고 — 그러고 나서도 엔진이 내재 콘텐츠 크기를 못 재는 한 `auto`가 제대로
동작하지 않는다. 컨테이너 전부를 다시 쓰는 것과 사실상 같다.

**(B) CSS를 오버라이드로.** 컨테이너가 흐름을 결정하고, 스타일시트는 그 위에서 박스를
조정한다. Flutter/SwiftUI가 하는 것에 가깝고, revue가 실제로 빌더 우선 라이브러리라는
사실과도 맞는다.

**(B)를 택했다.** `.css_layout(true)` 뒤에서 켜진다.

- 적용: `display: none`, `width`, `height`, `margin`, `min-*`/`max-*`
- 적용 안 함: `padding` — 위젯의 *콘텐츠*를 들여쓰는 것이라 테두리를 직접 그리는
  위젯(`Border`, `Card`)의 테두리가 안쪽으로 밀린다
- `gap` / `column-gap` / `row-gap` — 흐름 속성이라 **컨테이너가 읽는다**.
  `ctx.gap_or(self.gap)` 한 줄이다. 처음에는 `gap`이 평범한 `u16`이라 `gap: 0`이
  미지정으로 읽혀 빌더 값이 남았는데 — **스타일시트가 gap을 닫을 방법이 없었다** —
  `column_gap`/`row_gap`처럼 `Option<u16>`으로 바꿔 해결했다.
  `Stack`과 `Grid`에 배선했다
- 적용 안 함: `flex-*`, `grid-template-*` — 컨테이너가 직접 계산한다

"지정됐는가"는 값 자체에서 읽는다. 캐스케이드가 기본값으로 채워진 `Style`에 병합하므로
`Size::Auto`가 "미지정"이고 0 마진이 "미지정"이다. 캐스케이드가 규칙 간 우선순위를 판단할
때 이미 쓰는 것과 같은 판별이다.

### 페인트 커서 재동기화

수집 패스와 페인트 패스는 카운터로 정렬된다. `display: none` 노드를 건너뛰려면 그
**서브트리 전체만큼** 커서를 밀어야 한다. 더 일반적으로는, 페인트 패스가 수집 때와 다른
area로 자식을 그리므로 자식이 다른 개수의 노드를 낼 수 있다. 그래서 자식 렌더가 끝날 때마다
커서를 `idx + subtree_len`으로 되돌린다 — 어긋남이 형제로 번지지 않는다.

`CollectSink::subtree_lens()`가 전위 순회의 연속성을 이용해 한 번의 역방향 패스로 계산한다.

### 부수적으로 고친 것

`margin-left` 같은 longhand가 파서에 없었다. 축약형만 있었고, longhand는 조용히 무시됐다.
`padding-*` / `margin-*` 8개를 추가했다.

## 비용

레이아웃 트리가 실제로 간선을 갖게 되면서 `compute`가 매 프레임 트리 전체를 내려간다.
`cargo bench --bench frame`, 120x40, 같은 실행 안에서:

| | 10 rows | 50 rows | 200 rows |
|---|---:|---:|---:|
| `changed` (기본 경로) | −1.4% | −2.2% | −1.7% |
| `dom_from_render` | −0.2% | +6.2% | +15.3% |
| `css_layout` 추가분 | 0 | 0 | 0 |

기본 경로는 변화 없고, `dom_from_render`에서 노드 수에 비례해 오른다. 200 위젯에서
179.7 µs — 60fps 예산의 1% 남짓이다.

**지금은 그 계산을 아무도 읽지 않는다.** L-1은 정확성 수정이고(틀린 레이아웃 트리보다는
맞는 쪽이 낫다), devtools·rect 기반 히트 테스트·향후 CSS 레이아웃의 전제이기도 하다.
소비자가 없을 때 레이아웃 패스를 통째로 건너뛰는 것은 별도로 한다.

## 남은 것

| | |
|---|---|
| **L-2** | 위젯 배치 의도가 DOM 스타일에 없다 — (A)를 언젠가 하려면 필요 |
| **L-3** | auto 교차축 1셀 붕괴 — 엔진 내부. (B)에서는 노출되지 않는다 |
| **L-4** | `dom_from_render`에서 레이아웃 한 프레임 지연 — 엔진 출력을 devtools가 읽기 시작하면 문제가 된다 |

`tests/css_layout.rs`가 (B)를, `tests/layout_tree.rs`가 L-1과 나머지 미해결 항목을 고정한다.
