# Phase 0 invariant 커버리지

3.0 리팩터의 안전망은 스냅샷이 아니라 **invariant 테스트**다. 출력이 바뀌는 것이 목적인
리팩터에서는 리팩터 전후로 똑같이 참인 것만 그물이 될 수 있다.

대상 목록은 `docs/anti-patterns/catalog.yaml`의 `invariants:` 절이고, 구현은
`tests/invariants.rs`(통합)와 각 모듈의 단위 테스트다.

## 커버리지

| ID | 규칙 | 상태 | 위치 |
|---|---|---|---|
| INV-01 | 하나의 application에는 하나의 UI-thread mutation owner만 존재 | 제외 | 런타임 어서션으로 표현 불가. Phase 2의 `NodeCtx`가 소유권을 타입으로 강제하면 그때 재검토 |
| INV-02 | 한 focus scope에 focused widget 최대 1개 | ✅ | `tests/invariants.rs` — `inv02_*` 2개 |
| INV-03 | 제거된 widget은 effect·task·timer·focus를 유지하지 않음 | ⚠️ 부분 | `tests/invariants.rs` — `inv03_*` 2개. **노드 상태와 focus만** 검증. effect/task/timer는 미검증 |
| INV-04 | 한 transaction 안에서 동일 effect 중복 실행 없음 | 보류 | Phase 3의 `Invalidation` 도입과 함께. 지금은 검증할 transaction 경계가 없음 |
| INV-05 | paint가 layout·clipping bounds를 벗어나지 않음 | ✅ | `tests/invariants.rs` — `inv05_*` 2개 |
| INV-06 | 변경 없는 subtree는 style/layout/paint되지 않음 | ✅ | `tests/invariants.rs` — `inv06_*` 2개 |
| INV-07 | 외부 입력이 terminal control sequence로 출력되지 않음 | ✅ | `tests/invariants.rs` — `inv07_*` 2개 |
| INV-08 | worker result는 생성 당시 generation과 일치할 때만 적용 | 제외 | generation 개념이 아직 코드에 없다. 만들 때 테스트도 같이 만든다 |
| INV-09 | panic 또는 정상 종료 후 terminal state 복구 | ✅ | `tests/invariants.rs` — `inv09_*` 2개, `src/runtime/render/terminal/panic_hook.rs` 단위 테스트 4개 |
| INV-10 | experimental API가 stable namespace에 자동 편입되지 않음 | 제외 | 런타임 속성이 아니라 릴리스 프로세스. Phase 4의 tier 도입 시 문서/CI로 처리 |

추가로 Phase 1의 계약을 고정하는 테스트 4개가 있다 — `node_id_survives_*`,
`focus_survives_sibling_insertion`, `keyless_children_are_identified_by_position_today`.
마지막 것은 **현재의 위치 기반 identity를 의도적으로 못 박은 것**이고, Phase 1에서 key를
도입하면 이 테스트가 깨지는 것이 정상이다. 그게 Phase 1의 완료 조건이다.

## INV-09는 왜 Phase 0에서 코드까지 고쳤는가

나머지 invariant는 전부 "지금도 참"이라 테스트만 얹으면 됐지만, INV-09는 **release 빌드에서
성립하지 않고 있었다.**

`Terminal`과 `CrosstermBackend`에 `Drop`이 있어 정상 종료는 복구된다. 그러나 `Cargo.toml`의
release 프로파일이 `panic = "abort"`이므로 panic 시 unwinding이 일어나지 않고, **소멸자가
실행되지 않는다.** 사용자는 raw mode에 alternate screen이 켜진 채 커서가 사라진 터미널을
받는다. 카탈로그의 `REV-SEC-005`가 정확히 이것이다.

panic hook은 unwind와 abort 양쪽에서 실행되므로, 이 보장을 지탱하는 것은 `Drop`이 아니라
hook이다. `src/runtime/render/terminal/panic_hook.rs`를 추가하고 두 백엔드의
`init_with_mouse`에서 설치한다.

**순서상 수정이 테스트보다 먼저다.** 실패하는 테스트를 `#[ignore]`로 넣어두는 선택지도
있었지만, Phase 0에서 이미 `#[ignore]`가 거짓을 감춘 전례가 있어서 하지 않았다. 수정과
테스트를 같은 커밋에 넣되, 안전망(테스트)과 동작 변경(hook)이 섞이지 않도록 커밋은
`fix(terminal)`로 분리한다 — 안전 원칙 3(한 커밋 revert로 원복).

## 실행

```bash
cargo test --test invariants
cargo test --lib panic_hook
```
