# Phase 0 성능 기준선

Revue **2.74.0** 기준. 3.0 리팩터 계획의 Phase 0-4 산출물.

criterion baseline은 `target/criterion/*/phase0/`에 저장되지만 `target/`은 git 무시 대상이라
`cargo clean` 한 번에 사라진다. 이 표는 그 수치를 커밋된 형태로 남긴 것이다.

## 재현

```bash
cargo bench --bench dom --bench layout --bench render -- --save-baseline phase0
```

## 비교

```bash
cargo bench --bench dom --bench layout --bench render -- --baseline phase0
```

baseline이 없으면 (clean 후 등) 위 재현 명령으로 다시 만들되, **반드시 리팩터 커밋 이전
리비전에서** 만들어야 한다. 현재 코드에서 다시 뜨면 회귀를 기준선으로 삼게 된다.

## 벤치 자체의 수정 (Phase 0)

기준선을 뜨는 과정에서 `dom_incremental` 그룹이 **비교 불가능한 상태**임을 발견해 고쳤다.

`incremental_same`은 `b.iter()` 안에서 뷰 트리를 매번 새로 만들고 `fresh_build`는 루프
밖에서 한 번만 만들었다. 증분 쪽에만 뷰 생성 비용이 얹혀 두 수치를 비교할 수 없었다.
세 arm 모두 미리 만든 뷰를 재사용하도록 바꿨다.

| | 수정 전 | 수정 후 |
|---|---:|---:|
| `fresh_build` | 1.20 µs | 1.265 µs |
| `incremental_same` | 1.13 µs | **832 ns** |

증분 경로의 실제 이득은 6%가 아니라 약 34%다. Phase 1의 통과 게이트가 이 벤치이므로
고치지 않았다면 판정 자체가 불가능했다.

## 왜 이 벤치들인가

Phase 1이 reconciliation을 매 프레임 구동하므로 `dom_incremental`이 핵심 관측 지점이다.
`incremental_same`이 "변경 없는 프레임"의 비용이다.

## 기준선

| 그룹 | 벤치 | mean |
|---|---|---:|
| `buffer_ops` | `clear_80x24` | 1.88 µs |
| `buffer_ops` | `create_80x24` | 2.00 µs |
| `buffer_ops` | `put_str_long` | 250.9 ns |
| `buffer_ops` | `put_str_short` | 42.5 ns |
| `buffer_ops` | `resize` | 8.81 µs |
| `dom_build` | `nested_5_levels` | 1.71 µs |
| `dom_build` | `simple` | 272.7 ns |
| `dom_children` | `fresh/10` | 5.01 µs |
| `dom_children` | `fresh/50` | 34.0 µs |
| `dom_children` | `fresh/100` | 95.6 µs |
| `dom_children` | `incremental/10` | 4.14 µs |
| `dom_children` | `incremental/50` | 20.0 µs |
| `dom_children` | `incremental/100` | 38.0 µs |
| `dom_incremental` | `fresh_build` | 1.27 µs |
| `dom_incremental` | `incremental_same` | 845.1 ns |
| `dom_incremental` | `incremental_text_change` | 834.9 ns |
| `dom_invalidate` | `incremental` | 583.6 ns |
| `dom_invalidate` | `invalidate_rebuild` | 740.1 ns |
| `layout_engine` | `create` | 3.8 ns |
| `layout_engine` | `create_single_node` | 155.9 ns |
| `list_render` | `10` | 2.38 µs |
| `list_render` | `100` | 2.92 µs |
| `list_render` | `500` | 2.91 µs |
| `nested_layout` | `1` | 1.95 µs |
| `nested_layout` | `10` | 2.41 µs |
| `nested_layout` | `2` | 584.6 ns |
| `nested_layout` | `3` | 2.26 µs |
| `nested_layout` | `5` | 2.38 µs |
| `rect_ops` | `contains` | 0.3 ns |
| `rect_ops` | `intersection` | 0.8 ns |
| `rect_ops` | `intersects` | 0.3 ns |
| `rect_ops` | `union` | 0.3 ns |
| `table_render` | `10` | 3.62 µs |
| `table_render` | `100` | 6.10 µs |
| `table_render` | `50` | 5.04 µs |
| `text_render` | `10` | 1.90 µs |
| `text_render` | `100` | 2.16 µs |
| `text_render` | `1000` | 4.34 µs |
## 수용 기준

Phase 1에서 `incremental_dom`을 기본 on으로 전환하기 전에 확인한다.

- `dom_incremental/incremental_same` < `dom_incremental/fresh_build` — 같은 워크로드끼리
  비교할 것. `dom_build/*`는 트리 구성이 달라 비교 대상이 아니다
- 그 외 그룹은 기준선 대비 회귀가 없어야 한다
- **수치 판단은 사람이 한다.** 자동화된 루프는 회귀를 보고만 하고 임의로 수용하지 않는다

## 이 표의 한계

criterion은 **같은 머신·같은 세션**에서 뜬 baseline과 비교할 때만 신뢰할 수 있다. 위 수치는
기록용이고, 실제 판정은 매번 Phase 0 리비전에서 `--save-baseline`을 다시 떠서 대조해야 한다.
Phase 1이 그렇게 했다 — `phase1-reconciliation.md` 참고.

`dom_children` 그룹은 처음 이 표를 쓸 때 빠져 있었다. Phase 1 대조에서 발견해 채웠다.
