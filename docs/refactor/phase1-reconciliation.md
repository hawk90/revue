# Phase 1 — Reconciliation 활성화

3.0 리팩터 계획의 Phase 1. Phase 0(`phase0-baseline.md`, `phase0-invariants.md`) 위에 올라간다.

## 무엇이 바뀌었나

### 1. `WidgetKey` — 프레임을 가로지르는 정체성

`View::key() -> Option<WidgetKey>`가 생겼다. 기본 `None`이라 기존 코드는 영향이 없다.
`WidgetProps`를 쓰는 위젯은 `impl_props_builders!`가 만들어주는 `.keyed(...)` 빌더로 지정한다.

```rust
Text::new(&todo.title).keyed(todo.id)
```

> 빌더 이름이 `key`가 아니라 `keyed`인 이유: `StatusBar::key(key, description)`가 이미
> "키보드 단축키"를 뜻한다. 같은 이름의 inherent 메서드 둘은 공존할 수 없다.
> 트레이트 메서드 `View::key()`는 inherent 메서드와 충돌하지 않으므로 그대로 `key`다.

**매칭 우선순위**

```
key 일치  >  element id 일치  >  position + widget_type 일치  >  신규 생성
```

key가 없으면 정체성은 **위치**다. 고정 레이아웃에서는 맞지만 동적 컬렉션에서는 틀리다 —
리스트 맨 앞에 행을 하나 넣으면 그 아래 모든 행이 이웃의 노드에 매칭돼 focus·selection·
scroll이 한 칸씩 밀린다. 키는 데이터의 정체성으로 준다. **반복문 인덱스는 위치 정체성을
다르게 쓴 것일 뿐**이라 리스트가 재정렬되는 순간 같이 무너진다.

키를 가진 노드는 그 인덱스에 우연히 도달한 keyless 형제에게 넘어가지 않는다. 형제 둘이 같은
키를 주장하면 먼저 온 쪽이 기존 노드를 갖고 나중 쪽은 새 노드를 받는다.

### 2. 매 프레임 reconcile — `App::builder().incremental_dom(true)`

**기본 off.** 켜지 않으면 DOM은 첫 프레임에 한 번 지어지고 그 뒤로 뷰를 따라가지 않는다
(`src/core/app/mod.rs`의 `needs_dom_rebuild` 분기). 이후에 추가된 위젯은 CSS 매칭·레이아웃·
devtools 어디에도 보이지 않는다.

켜면 매 프레임 reconcile한다. 살아남은 노드는 `DomId`, 상태(focus/hover/selection), 캐시된
스타일을 유지한다. **레이아웃 트리는 DOM의 *모양*이 바뀐 프레임에만 다시 짓는다** —
`DomRenderer::structure_dirty`가 노드 추가·제거·재정렬을 기록하고, 무변경 프레임은
레이아웃 재구축을 건너뛴다. 이게 없으면 매 프레임 reconcile이 대체하려는 전체 재구축보다
더 비싸진다.

### 3. 켜자마자 드러난 기존 버그 3개

증분 경로는 `#[allow(dead_code)]` 뒤에서 사실상 죽어 있었다. 매 프레임 돌리자 세 가지가 나왔다.

| 증상 | 원인 |
|---|---|
| 클래스가 바뀐 노드를 `get_by_class`가 옛 클래스로 계속 반환 | `node.meta.classes`를 직접 써서 `class_index`가 갱신되지 않음 |
| element id가 바뀐 노드를 옛 id로 계속 조회 가능 | 같은 이유로 `id_map`이 갱신되지 않음 |
| 재정렬·삭제 후 `:first-child` / `:nth-child`가 이전 프레임 순서를 봄 | `parent.children`을 직접 대입해 형제 위치 상태(`child_index`, `sibling_count`, …)가 재계산되지 않음 |

`DomTree::apply_meta`와 `DomTree::set_children`을 추가해 인덱스와 구조 상태를 함께 갱신한다.
`tests/reconciliation.rs`의 해당 테스트 4개는 수정을 되돌리면 실제로 실패한다 — 확인했다.

## 성능

Phase 0 기준선(`p0`)과 같은 머신·같은 세션에서 비교했다. 커밋된 표
(`phase0-baseline.md`)는 다른 시점의 측정이므로, 판정은 이 대조로 한다.

```bash
git checkout <phase0>  && cargo bench --bench dom -- --save-baseline p0
git checkout <phase1>  && cargo bench --bench dom -- --baseline p0
```

### DOM

| 벤치 | Phase 1 | 변화 |
|---|---:|---:|
| `dom_build/simple` | 256.6 ns | **−6.3%** |
| `dom_build/nested_5_levels` | 1.682 µs | **−3.3%** |
| `dom_incremental/fresh_build` | 1.229 µs | +2.7% |
| `dom_incremental/incremental_same` | **686.8 ns** | **−15.9%** |
| `dom_incremental/incremental_text_change` | 688.7 ns | **−15.8%** |
| `dom_children/fresh/10` | 5.123 µs | +2.3% |
| `dom_children/incremental/10` | 3.305 µs | **−20.2%** |
| `dom_children/fresh/50` | 34.00 µs | 변화 없음 |
| `dom_children/incremental/50` | 16.58 µs | **−17.2%** |
| `dom_children/fresh/100` | 95.81 µs | 변화 없음 |
| `dom_children/incremental/100` | 32.45 µs | **−14.6%** |
| `dom_invalidate/incremental` | 524.9 ns | **−14.4%** |
| `dom_invalidate/invalidate_rebuild` | 738.5 ns | 변화 없음 |

**Phase 1 통과 게이트:** `incremental_same`(687 ns) < `fresh_build`(1.23 µs). 통과한다 —
Phase 0의 845 ns 대 1.27 µs보다 격차가 더 벌어졌다.

### layout / render

의미 있는 회귀 없음. `nested_layout/5` −44%, `layout_children/children/50` −4.8%,
`rect_ops/*` −3~4%, `text_render/*` −2%. 회귀는 `layout_engine/create` +2.7%(3.87 ns —
피코초 단위 벤치의 잡음)와 `table_render/10` +4.5% 둘뿐이다.

### 남은 회귀에 대한 판단

`dom_incremental/fresh_build` +2.7%, `dom_children/fresh/10` +2.3%. 둘 다 **fresh build**
경로이고 노드 수가 적을 때만 보인다 (`fresh/50`, `fresh/100`은 변화 없음). `WidgetMeta`에
`Option<WidgetKey>` 필드가 붙어 구조체가 커진 것이 원인으로 보인다.

**한 번만 도는 경로에서 2~3%를 내주고, 이제 매 프레임 도는 경로에서 15~20%를 얻는 교환이다.**
수용 여부는 사람이 판단한다 — Phase 0의 수용 기준에 그렇게 적어두었다.

### 첫 구현은 실제로 회귀했다

기록해둘 가치가 있다. 처음 쓴 매칭 코드는 `dom_invalidate/incremental`을 583 ns → **1.35 µs**로
2.3배 느리게 만들었다. 원인은 매 프레임·매 부모마다:

- 형제들의 element id를 `String`으로, `widget_type`을 `String`으로 전부 복제
- `HashMap` 2개 + `HashSet` 2개 할당
- `set_children`에서 자식 목록을 한 번 더 복제

지금 코드는 **2-pass**다. 1패스(`plan_matches`)는 트리를 불변으로 빌려 매칭만 결정하므로
lookup 테이블이 기존 노드의 id와 key를 **빌려 쓸 수 있다** — 복제가 사라진다. 결과는 `DomId`가
아니라 `old_children`의 **인덱스**라서, 이미 점유된 슬롯 추적을 `HashSet`이 아니라
`Vec<bool>`로 한다. 키를 쓰는 자식이 하나도 없으면 key 맵은 아예 만들지 않는다.

## 테스트

`tests/reconciliation.rs` 14개.

- 매칭 우선순위: prepend / reorder / 중간 삭제에서 키가 정체성을 지키는가
- 중복 키가 한 노드로 합쳐지지 않는가
- keyless 형제가 keyed 노드의 정체성을 훔치지 않는가
- focus가 재정렬을 따라가고, 노드가 사라지면 함께 사라지는가
- `class_index` / `id_map` / 형제 구조 상태가 reconcile 후에도 정확한가
- 플래그 off일 때 DOM이 뷰를 따라가지 **않는가** (기본 동작 문서화)
- 같은 뷰를 여러 번 그려도 정체성이 요동치지 않는가

`tests/invariants.rs`의 `keyless_children_are_identified_by_position_today`는 그대로 통과한다.
키는 opt-in이므로 keyless 폴백은 바뀌지 않았다 — 그 테스트가 음(陰)이고
`keyed_children_survive_a_prepend`가 양(陽)이다.

## 다음

플래그를 기본 on으로 뒤집기 전에:

- `examples/`를 실제로 띄워 focus 이동·스크롤·모달을 손으로 확인 (계획서 「검증」)
- 위 회귀 수치에 대한 사람의 판단
- 상태를 가진 위젯들에 `.keyed(...)` 적용 — 다만 상태 소유권이 아직 위젯에 있으므로
  (Phase 2) 효과가 절반이다
