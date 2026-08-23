# 두 번째 프레임부터 CSS가 갱신되지 않는다

`dom_from_render` 위에서 선언 상태(`disabled`)를 DOM에 싣는 작업 중에 발견. 원인은 그
작업과 무관한 **기존 결함**이었고, 훨씬 넓은 것을 망가뜨리고 있었다.

## 증상

```rust
// 프레임 1
Text::new("Save").element_id("save")
// 프레임 2 — 클래스를 붙인다
Text::new("Save").element_id("save").class("hot")
```

```css
.hot { color: red; }
```

**아무 일도 일어나지 않는다.** DOM 노드에는 클래스가 실제로 붙는다(`query(".hot")`이 찾는다).
계산된 스타일만 옛날 것 그대로다. 첫 프레임 이후의 모든 동적 스타일링이 이랬다 —
클래스 토글, `:nth-child` 재정렬, 새로 나타난 노드 전부.

## 원인 1 — 깨끗한 조상이 더러운 후손을 가린다

`compute_subtree_styles`는 루트에서 내려가며 "정착한" 노드에서 하강을 멈춘다. 변화 없는
프레임이 캐스케이드 전체를 다시 돌리지 않게 하는 최적화다.

```rust
if !node.state.dirty && self.styles.contains_key(&node_id) {
    return;   // 자식도 최신이라고 가정한다
}
```

그 가정이 틀렸다. 무효화는 **바뀐 노드 자신**만 dirty로 찍는다. 루트는 첫 프레임 이후 항상
깨끗하므로, 걷기는 **언제나 루트에서 곧바로 되돌아온다.** 그 아래 무엇이 바뀌었든.

`NodeState.subtree_dirty`를 추가했다 — "나는 안 바뀌었지만 내 아래에 바뀐 게 있다". 무효화
시 조상 체인에 세우고, 걷기가 지나가며 지운다. 이미 세워진 조상을 만나면 멈춘다(그 위는
누가 이미 세웠다).

## 원인 2 — 상속이 내려가지 않는다

원인 1을 고치자 두 번째가 드러났다. 컨테이너에 `color`가 새로 붙으면 그 자식들은 **여전히**
정착 상태라 걷기가 거기서 멈춘다. 상속되는 속성이 옛 값에 얼어붙는다.

걷기가 "네가 상속하는 것이 바뀌었다"를 아래로 전달하도록 했다. 실제로 **다시 계산된** 노드만
전달한다 — `subtree_dirty`만 세워진 조상은 통과만 하고 자식에게 아무것도 알리지 않는다. 그래서
잎 하나가 더러워졌다고 트리 전체가 다시 계산되지 않는다.

부수적으로 hover/focus 무효화가 하던 **손수 서브트리 순회가 필요 없어졌다.** `:hover .child`
규칙과 상속 속성이 같은 경로로 처리된다.

## 원인 3 — 캐시가 dirty 플래그보다 먼저 답한다

`style_for_with_inheritance`는 캐시를 먼저 보고 dirty 플래그는 보지 않는다. 그래서 dirty로
찍는 것만으로는 아무것도 다시 계산되지 않는다 — **캐시 항목을 지워야** 한다. 걷기가 다시
계산하기로 결정한 노드에서 직접 지우도록 했다.

이건 hover에서 이미 한 번 물렸던 것과 같은 함정이다
([`phase2-hit-test.md`](phase2-hit-test.md) H-3).

## 무효화 지점

스타일을 무효화하는 모든 곳이 조상에 표시를 남겨야 한다.

| | |
|---|---|
| `apply_meta` (id·class·key 변경) | `update_node_meta_matched` |
| `add_child` (새 노드는 계산된 스타일이 없다) | `DomTree::add_child` |
| `set_children` (자리가 바뀐 자식은 `:nth-child`가 달라진다) | `DomTree::set_children` |
| focus / hover 이동 | `DomRenderer::invalidate_state_styles` |
| 루트 자신의 meta 변경 | `reconcile_collected` — 반환값을 버리고 있었다 |

## 남은 것

- **형제 결합자.** `.a:hover + .b`는 여전히 낡는다. 무효화가 조상과 후손 방향만 알기 때문이고,
  형제 방향을 알려면 부모가 자식 하나의 변경에 대해 형제 전체를 다시 계산해야 한다

`tests/style_invalidation.rs`가 위를 고정한다.
