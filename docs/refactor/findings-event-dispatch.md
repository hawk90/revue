# Phase 3 선행 조사 — 위젯 이벤트 디스패치가 없다

계획서 Phase 3은 `Interactive::handle_key(&mut self)`를 `on_key(&self, &mut NodeCtx)`로
옮기는 것이고, 방법으로 "런타임은 `on_key`를 먼저 시도하고, 구현이 없으면 `handle_key`로
fallback한다"를 적어뒀다.

**그 런타임 시도가 존재하지 않는다.**

## 확인

`src/core`와 `src/runtime` 어디에서도 `Interactive::handle_key`를 부르지 않는다.
저장소 전체에서 `.handle_key(` 469곳을 훑으면 전부 셋 중 하나다:

| 종류 | 예 |
|---|---|
| 문서 예제 | `src/lib.rs:28`, `src/core/app/mod.rs:452` |
| 위젯 자체 테스트 | `src/widget/link.rs:351` 등 |
| 동명의 **inherent** 메서드 | `src/core/app/inspector.rs` — trait이 아니다 |

`App::handle_event`가 뷰 쪽으로 넘기는 것은 **사용자 핸들러 하나뿐**이다:

```rust
let mut should_draw = handler(&event, view, self);
```

즉 키를 위젯에 라우팅하는 것은 프레임워크가 아니라 애플리케이션이다. `Interactive`는
런타임이 호출하는 인터페이스가 아니라, 사용자가 자기 핸들러 안에서 직접 부르는
헬퍼 trait으로 존재한다.

## 규모

`impl Interactive`는 **9개 위젯**뿐이다 — `Button`, `Checkbox`, `Switch`, `Select`,
`TextArea`, `Link`, `ThemePicker`, `MultiSelect`, `SortableList`.

계획서가 Phase 3의 대상으로 적은 "`handle_key(&mut self)` 61곳"은 trait 구현이 아니라
**위젯이 각자 들고 있는 동명의 inherent 메서드**를 함께 센 수치다. 그쪽은 trait을 바꿔도
따라오지 않는다.

## 이것이 Phase 3에 의미하는 것

계획서의 3-1(default 구현으로 새 메서드 추가)과 3-2(`EventResult`에 `Invalidation` 확장)는
**둘 다 디스패처를 전제한다.**

- 3-1의 "런타임이 `on_key`를 먼저 시도" — 시도할 런타임이 없다.
- 3-2의 "dirty 정보가 `collect_dirty_regions`까지 내려간다" — `EventResult`가 런타임에
  도달하지 않으므로 내려갈 경로가 없다. 지금 넣으면 죽은 코드가 된다.

그런데 계획서의 **"제외"** 절은 "프레임워크 이벤트 라우팅 → 3.x"라고 명시한다.

**Phase 3은 계획서가 범위 밖으로 밀어둔 것에 의존한다.** 이건 순서 문제이지 설계 문제가
아니다. 둘 중 하나를 골라야 한다:

| 선택지 | 내용 | 대가 |
|---|---|---|
| **A. 디스패처를 먼저 짓는다** | `App`이 focus된 노드를 찾아 그 위젯에 키를 넘긴다. `NodeState.focused`가 이미 권위이고 Tab·클릭이 그것을 움직이므로(#609, #648) 재료는 다 있다. | 범위가 늘어난다. `App::run(view, handler)` 시그니처는 지킬 수 있지만, 프레임워크가 키를 먼저 보게 되므로 기존 앱과 충돌 가능 — `tab_navigation`처럼 플래그가 필요하다. |
| **B. Phase 3을 3.x로 미룬다** | 3.0은 Phase 4(deprecated 제거)만 한다. | Phase 4가 지울 대상 중 `Interactive::handle_key`는 대체재가 없는 채로 남는다. 즉 3.0에서 지울 수 없다. |

**B를 택하면 3.0의 내용이 줄어든다** — `WidgetState` 삭제와 위젯 미러 필드 제거는 그대로
할 수 있지만 `Interactive` 관련 항목은 빠진다.

## 권고

**A**, 단 최소 범위로. `tab_navigation`이 방금 보여준 모양을 그대로 쓴다:

1. `App::builder().key_dispatch(bool)`, 기본 off.
2. 켜면 `handle_event`가 focus된 노드에 대응하는 위젯을 찾아 `Interactive::on_key`를 부른다.
3. 사용자 핸들러는 지금처럼 **먼저** 돈다. 프레임워크는 그 다음이다.
4. `EventResult`가 그제서야 런타임에 도달하므로 3-2의 `Invalidation`이 의미를 갖는다.

막히는 지점이 하나 있다: **DomId에서 위젯으로 가는 길이 없다.** DOM은 `WidgetMeta`만
들고 있고 위젯 자체를 들고 있지 않다. 렌더 순회로 위젯을 찾아가는 방법(paint 패스가 이미
노드와 위젯을 짝지어 지나간다)이 가장 가까워 보이지만, 그때 위젯은 `&self`다 —
`handle_key(&mut self)`를 부를 수 없다.

**이것이 Phase 3이 `&self` 전환을 요구하는 진짜 이유다.** 계획서는 `&self` 전환을
"상태 소유권 이전의 결과"로 적었는데, 실제로는 **디스패처를 지을 수 있게 하는 전제**다.
순서가 반대로 적혀 있다.

## 다음

디스패처를 짓기로 한다면 `on_key(&self, &mut NodeCtx)`를 **먼저** 넣어야 한다
(3-1 → 디스패처 → 3-2). `handle_key(&mut self)`로는 디스패처를 지을 수 없다.

`NodeCtx`(계획서 2-1)는 아직 없다. `RenderContext.state`가 읽기 통로로 열려 있을 뿐,
변경을 **요청**하는 쪽은 지어지지 않았다. 그것이 실질적인 첫 단계다.
