# 셀렉터 매처가 두 개고, 답이 다르다

형제 결합자 무효화(`findings-style-invalidation.md`의 남은 항목)를 고치다 발견.

## 이 크레이트에는 매처가 둘 있다

| | 위치 | 쓰는 곳 |
|---|---|---|
| A | `DomTree::matches_selector` (`dom/query.rs`) | `query()`, `query_all()`, devtools |
| B | `StyleResolver::matches` (`dom/cascade/resolver.rs`) | **캐스케이드 — 화면을 칠하는 쪽** |

그리고 서로 다른 답을 냈다.

```rust
h.query_ids(".mark ~ Text")   // ["b", "c", "d"]   — A
// 화면에서 빨간 것                 ["b"]            — B
```

## 원인 — 한 칸 밀린 인덱스

`Selector.parts`는 `Vec<(SelectorPart, Option<Combinator>)>`이고, **결합자는 왼쪽 파트에
붙어 오른쪽을 가리킨다.** 마지막 파트의 결합자는 `None`이다.

descendant(` `)와 general sibling(`~`)은 "아무거나"라는 뜻이므로, 한 번 실패해도 그 방향으로
계속 걸어가야 한다. B에는 그 재시도가 **있었고, 한 번도 실행되지 않았다.**

```rust
match selector.parts.get(part_idx + 1) {      // 오른쪽 파트의 결합자를 읽는다
    Some((_, Some(Combinator::Descendant))) => { /* 부모로 올라가서 재시도 */ }
    Some((_, Some(Combinator::GeneralSibling))) => { /* 이전 형제로 재시도 */ }
    _ => {}                                    // ← 2-파트 셀렉터는 항상 여기
}
```

2-파트 셀렉터(`.a .b`, `.a ~ .b`)에서 `parts[part_idx + 1]`은 **마지막 파트**이고 그 결합자는
`None`이다. 그래서 재시도 없이 실패로 떨어졌다.

`parts[part_idx]`가 맞다.

## 증상

| 셀렉터 | 전 | 후 |
|---|---|---|
| `.card .label` (직계 자식) | ✅ | ✅ |
| `.card .label` (손자 이하) | ❌ | ✅ |
| `.a ~ .b` (바로 다음 형제) | ✅ | ✅ |
| `.a ~ .b` (그 뒤 형제들) | ❌ | ✅ |
| `.a > .b` | ✅ | ✅ (영향 없음) |
| `.a + .b` | ✅ | ✅ (영향 없음) |

`>`와 `+`는 "정확히 한 칸"이라 재시도 분기에 들어가지 않는다.

**descendant 셀렉터가 한 단계를 넘어가면 동작하지 않았다.** 문서와 튜토리얼이 권하는 형태
(`.container .item`)가 중첩이 한 겹만 깊어져도 조용히 무시됐다는 뜻이다.

## 왜 아무도 못 봤나

`query()`가 올바르게 답한다. 셀렉터를 검증하는 테스트와 devtools는 A를 쓴다. B를 통과하는
경로는 화면뿐이고, 화면은 "왜 이 스타일이 안 먹지"로만 관찰된다.

**두 구현을 하나로 합치는 것이 옳다.** 이번에는 B를 A와 같은 답을 내도록 고치고,
`the_cascade_and_query_select_the_same_nodes`가 두 매처가 같은 노드 집합을 고르는지 확인한다.
통합은 별도 작업이다.

## 형제 무효화

매처가 고쳐져도, `+`/`~`가 **프레임 사이에 다시 평가되지 않는** 문제가 따로 있었다.
무효화는 위(정착한 조상 아래의 낡은 노드를 찾기 위해)와 아래(후손 재상속) 두 방향만 알았다.
형제 결합자는 **옆으로** 매칭한다 — `.a:hover + .b`는 `.b`를 `.a`의 상태로 칠하는데 `.b`는
아무것도 안 바뀌었으므로 정착 상태로 남는다.

바뀐 노드 **뒤의** 형제들을 무효화한다(`+`/`~`는 뒤만 본다). 상태 변경(focus/hover)과 meta
변경(규칙이 keying하는 클래스의 등장/소멸) 두 경로 모두에서.

**스타일시트에 그런 규칙이 실제로 있을 때만** 한다 — 파싱된 셀렉터와 함께 한 번 계산해둔다.
아니면 결과를 바꿀 수 없는 일에 모든 앱이 변경마다 비용을 낸다.

`tests/cascade_combinators.rs`와 `tests/sibling_invalidation.rs`가 고정한다.
