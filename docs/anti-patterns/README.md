# Revue Anti-Pattern Catalog

Revue가 선택한 기술 스택에서 **구조적으로 발생할 수 있는 실패 모드**를 ID로 정리한 카탈로그다.

## 적용 범위

Revue는 단순한 TUI 라이브러리가 아니라 다음 요소가 결합된 retained-mode reactive UI framework다.

- Rust 기반 retained-mode UI
- `Signal / Computed / Effect` 반응형 상태
- CSS 파서 · selector · cascade · layout
- Flex/Grid 레이아웃
- 이벤트 · 포커스 · 입력 처리
- worker pool 및 비동기 작업
- hot reload
- proc macro
- CLI · template · extension/plugin
- 100개 이상의 widget
- profiler · inspector · snapshot test
- tree-sitter 기반 CSS tooling

따라서 이 카탈로그는 Ratatui 애플리케이션의 안티패턴보다 **브라우저 엔진, React/Vue, 게임 UI 엔진,
async runtime, 컴파일러 프론트엔드에서 나타나는 안티패턴**까지 포함한다.

## 왜 문법 체크리스트가 아닌가

Revue에서 가장 위험한 것은 개별 Rust 문법 안티패턴이 아니다. 핵심 위험은 네 subsystem의 곱이다.

```text
Reactive graph  ×  Retained widget tree  ×  CSS/layout engine  ×  Terminal event/render loop
```

이 사이의 **변경 전파 · ownership · invalidation · scheduling 경계가 불분명해지는 것**이
가장 큰 구조적 안티패턴이다. 그래서 안티패턴 검사가 이렇게 끝나면 안 된다.

```text
clone을 많이 쓰지 말 것
Arc<Mutex>를 남용하지 말 것
unwrap을 쓰지 말 것
```

대신 [리뷰에서 반복할 질문](#리뷰에서-반복할-질문)에 구조적으로 답할 수 있어야 한다.
그래야 Revue가 "기능이 많은 TUI 라이브러리"가 아니라 안정적인 UI framework가 된다.

## 이 디렉터리의 파일

| 파일 | 대상 | 내용 |
|---|---|---|
| [`catalog.yaml`](catalog.yaml) | LLM / 스크립트 | 150개 항목의 기계 판독용 원본. **single source of truth** |
| `README.md` (이 파일) | 사람 | 카테고리·우선순위·invariant 요약과 전체 ID 색인 |
| [`architecture-review.md`](architecture-review.md) | 사람 | "지금 아키텍처가 맞나?" 논의와 대안 스택 3안 |

> [!IMPORTANT]
> 이 카탈로그는 **"일어날 수 있는 실패 모드"의 목록이지, Revue 코드베이스 감사 결과가 아니다.**
> 각 항목이 실제 코드에 존재하는지는 확인되지 않았다. 리뷰 체크리스트로 쓰되,
> "REV-XXX-NNN 위반"이라고 주장하려면 해당 코드를 직접 확인해야 한다.
> 출처는 [ChatGPT 대화](https://chatgpt.com/share/6a843f6f-64f8-83e9-a269-bb1cb8a7ba44) (2026-08-18 정리).

## 사용법

### 사람: PR 리뷰

리뷰 코멘트를 태그 + ID로 시작한다.

```text
[REACTIVE] REV-REACT-003: 이 effect가 자신이 읽는 signal을 다시 set 한다 → 재진입 가능
[RENDER]   REV-RENDER-005: chars().count()를 폭으로 쓰면 CJK/emoji에서 어긋난다
```

### LLM: 코드 리뷰 프롬프트

`catalog.yaml`을 컨텍스트로 넣고 스코프를 좁혀서 물어본다. 전체 150개를 한 번에
적용하려 하면 정확도가 떨어지므로, 변경된 파일의 subsystem에 해당하는 카테고리만 필터링한다.

```bash
# reactive + render 카테고리만 뽑아서 리뷰 컨텍스트로 사용
yq '.entries[] | select(.category == "reactive" or .category == "render")' \
  docs/anti-patterns/catalog.yaml
```

프롬프트 예시:

```text
아래는 Revue의 anti-pattern 카탈로그 중 reactive/render 항목이다.
diff에서 실제로 해당하는 항목만 골라 id, 파일:줄, 근거를 제시하라.
해당 없으면 "없음"이라고 답하라. 추측으로 항목을 만들지 마라.
```

### 스키마

각 항목의 필드는 다음과 같다.

| 필드 | 의미 |
|---|---|
| `id` | `REV-<TAG>-<NNN>` 안정 식별자 |
| `title` | 패턴 이름 (영문) |
| `category` / `review_tag` | 소속 subsystem |
| `summary` | 이 패턴이 무엇인지 |
| `problem` | 그래서 무엇이 깨지는지 (증상 · 결과 · 징후) |
| `detect` | 어디를 보면 발견되는지 (예시 · 취약 대상 · 검증 입력) |
| `fix` | 방지 원칙과 개선 방향 |
| `snippets` | 원문 코드/도식 (등장 순서) |

최상위에는 `review_tags`, `invariants`, `review_questions`, `priorities`, `categories`, `entries`가 있다.

## 카테고리

| 태그 | 영역 | 원문 분류 | 항목 수 | ID 범위 |
|---|---|---|---:|---|
| `ARCH` | Framework identity | 프레임워크 정체성 안티패턴 | 5 | `REV-ARCH-001` – `REV-ARCH-005` |
| `TREE` | Retained UI tree | Retained UI Tree 안티패턴 | 6 | `REV-TREE-001` – `REV-TREE-006` |
| `REACTIVE` | Reactive state | Reactive State 안티패턴 | 10 | `REV-REACT-001` – `REV-REACT-010` |
| `RENDER` | Rendering | 렌더링 안티패턴 | 10 | `REV-RENDER-001` – `REV-RENDER-010` |
| `LAYOUT` | Layout | 레이아웃 안티패턴 | 7 | `REV-LAYOUT-001` – `REV-LAYOUT-007` |
| `STYLE` | CSS / style engine | CSS 및 스타일 엔진 안티패턴 | 9 | `REV-STYLE-001` – `REV-STYLE-009` |
| `EVENT` | Event system | 이벤트 시스템 안티패턴 | 8 | `REV-EVENT-001` – `REV-EVENT-008` |
| `WIDGET` | Widget design | Widget 설계 안티패턴 | 10 | `REV-WIDGET-001` – `REV-WIDGET-010` |
| `ASYNC` | Worker pool / async | Worker Pool 및 비동기 안티패턴 | 7 | `REV-ASYNC-001` – `REV-ASYNC-007` |
| `ANIM` | Animation | Animation 안티패턴 | 5 | `REV-ANIM-001` – `REV-ANIM-005` |
| `HOT` | Hot reload | Hot Reload 안티패턴 | 4 | `REV-HOT-001` – `REV-HOT-004` |
| `MACRO` | Proc macro | Proc Macro 안티패턴 | 5 | `REV-MACRO-001` – `REV-MACRO-005` |
| `PLUGIN` | Plugin / extension | Plugin 및 Extension 안티패턴 | 5 | `REV-PLUGIN-001` – `REV-PLUGIN-005` |
| `CLI` | CLI / templates | CLI 및 Template 안티패턴 | 4 | `REV-CLI-001` – `REV-CLI-004` |
| `PERF` | Performance measurement | 성능 측정 안티패턴 | 6 | `REV-PERF-001` – `REV-PERF-006` |
| `RUST` | Rust-specific | Rust 특화 안티패턴 | 10 | `REV-RUST-001` – `REV-RUST-010` |
| `MEM` | Memory / resources | 메모리 및 Resource 안티패턴 | 5 | `REV-MEM-001` – `REV-MEM-005` |
| `DEV` | DevTools | DevTools 안티패턴 | 4 | `REV-DEV-001` – `REV-DEV-004` |
| `TEST` | Testing | 테스트 안티패턴 | 6 | `REV-TEST-001` – `REV-TEST-006` |
| `API` | Public API / SemVer | API 및 SemVer 안티패턴 | 5 | `REV-API-001` – `REV-API-005` |
| `UX` | Accessibility / UX | 접근성 및 UX 안티패턴 | 7 | `REV-UX-001` – `REV-UX-007` |
| `SEC` | Security / robustness | 보안 및 안정성 안티패턴 | 5 | `REV-SEC-001` – `REV-SEC-005` |
| `DOC` | Documentation | 문서화 안티패턴 | 3 | `REV-DOC-001` – `REV-DOC-003` |
| `PROJ` | Project operation | 프로젝트 운영 안티패턴 | 4 | `REV-PROJ-001` – `REV-PROJ-004` |

## 우선순위

### P0 — 반드시 먼저 통제할 것

이 8개는 나중에 고치면 데이터/상태 손상이나 전면 리팩터링으로 이어진다.

1. **Reactive cycle과 effect scheduling** — `REV-REACT-003`, `REV-REACT-004`, `REV-REACT-010`
2. **Widget identity 및 lifecycle** — `REV-TREE-001`, `REV-TREE-003`, `REV-TREE-004`
3. **Focus ownership과 event propagation** — `REV-EVENT-003`, `REV-EVENT-004`, `REV-EVENT-005`, `REV-UX-004`
4. **Terminal panic recovery** — `REV-SEC-005`
5. **Unicode width와 wide-character 처리** — `REV-RENDER-005`, `REV-RENDER-006`
6. **Worker cancellation과 stale result** — `REV-ASYNC-002`, `REV-ASYNC-003`, `REV-ASYNC-004`
7. **CSS parser resource bound** — `REV-SEC-003`, `REV-SEC-004`, `REV-LAYOUT-004`
8. **Full-screen repaint와 uncontrolled FPS** — `REV-RENDER-001`, `REV-RENDER-009`, `REV-ANIM-001`

### P1 — 프레임워크가 커지기 전에 고정할 것

Core crate 분리(`REV-ARCH-004`) · Layout/paint separation(`REV-LAYOUT-001`, `REV-TREE-006`) ·
Style invalidation model(`REV-STYLE-007`, `REV-PERF-006`) · Plugin stability boundary(`REV-PLUGIN-001`, `REV-PLUGIN-002`) ·
Widget maturity tier(`REV-WIDGET-010`, `REV-API-002`) · Feature flag matrix(`REV-RUST-009`, `REV-RUST-010`) ·
Public API stability policy(`REV-ARCH-005`, `REV-API-001`, `REV-API-003`) · Collection virtualization(`REV-WIDGET-003`)

### P2 — 생태계 확장 전에 준비할 것

CLI-template compatibility · custom widget contract · devtools causality · accessibility convention ·
ADR · performance budgets · migration guides · fuzz/property testing.
(ID 매핑은 `catalog.yaml`의 `priorities` 참고)

## 설계 Invariant

리뷰와 CI에서 항상 확인할 것. 위반이 곧 버그다.

| ID | 규칙 |
|---|---|
| `INV-01` | 하나의 application에는 하나의 UI-thread mutation owner만 존재한다 |
| `INV-02` | 하나의 focus scope에는 최대 하나의 focused widget만 존재한다 |
| `INV-03` | 제거된 widget은 effect, task, timer, focus를 유지하지 않는다 |
| `INV-04` | 한 transaction 안에서 동일 effect는 불필요하게 중복 실행되지 않는다 |
| `INV-05` | paint는 layout bounds와 clipping bounds를 벗어나지 않는다 |
| `INV-06` | 변경되지 않은 subtree는 style/layout/paint되지 않는다 |
| `INV-07` | 외부 입력은 terminal control sequence로 직접 출력되지 않는다 |
| `INV-08` | worker result는 생성 당시 generation과 일치할 때만 적용된다 |
| `INV-09` | panic 또는 정상 종료 후 terminal state는 복구된다 |
| `INV-10` | experimental API는 stable namespace에 자동 편입되지 않는다 |

## 리뷰에서 반복할 질문

체크리스트보다 이 질문들이 먼저다.

- 이 상태 변경이 **왜** 이 subtree를 다시 그리는가?
- 이 widget의 identity와 lifetime은 **누가** 관리하는가?
- 이 CSS 변경이 왜 layout까지 무효화하는가?
- 이 task의 결과는 widget이 제거된 뒤 **어디로** 가는가?
- 이 event는 누가 소비하고 어디까지 전달되는가?
- 이 cache는 **어떤 dependency**가 바뀔 때 무효화되는가?

---

## 전체 색인

각 항목의 `problem` / `detect` / `fix` / 코드 예시는 [`catalog.yaml`](catalog.yaml)에 있다.

### 1. Framework identity — 프레임워크 정체성 안티패턴

- **`REV-ARCH-001`** Ratatui Wrapper Syndrome — 내부 구조는 Ratatui 호출을 포장한 수준인데 외부 API만 framework처럼 확장하는 패턴이다.
- **`REV-ARCH-002`** Web Framework Mimicry — Vue·React·CSS 개념을 터미널 환경에 의미 검토 없이 그대로 복제하는 패턴이다.
- **`REV-ARCH-003`** Feature Checklist Framework — 경쟁 프레임워크에 있는 기능을 계속 추가하면서 핵심 실행 모델을 안정화하지 않는 패턴이다.
- **`REV-ARCH-004`** One Framework Does Everything — core crate가 다음을 모두 책임지는 패턴이다.
- **`REV-ARCH-005`** Public API Mirrors Internal Representation — 내부 구현 편의를 위해 public API가 설계되는 패턴이다.

### 2. Retained UI tree — Retained UI Tree 안티패턴

- **`REV-TREE-001`** Rebuild Everything Every Frame — 작은 상태 변경에도 전체 widget tree를 다시 생성하는 패턴이다.
- **`REV-TREE-002`** Mutable Tree Everywhere — 모든 subsystem이 UI tree를 직접 수정할 수 있는 구조다.
- **`REV-TREE-003`** Widget Identity by Position — widget의 identity를 부모의 child index만으로 판단하는 패턴이다.
- **`REV-TREE-004`** Zombie Widget State — tree에서 제거된 widget의 signal, callback, subscription, worker가 살아남는 패턴이다.
- **`REV-TREE-005`** Parent Knows Every Child Type — container가 각 child widget의 구체 타입에 따라 분기하는 패턴이다.
- **`REV-TREE-006`** Tree and Render Buffer Coupling — UI node가 terminal buffer의 좌표와 cell을 직접 소유하거나 조작하는 구조다.

### 3. Reactive state — Reactive State 안티패턴

- **`REV-REACT-001`** Global Signal Graph — 모든 signal과 effect가 하나의 전역 graph에 등록되는 패턴이다.
- **`REV-REACT-002`** Hidden Dependency Tracking — `Signal::get()` 호출만으로 암묵적 의존성이 생기지만 사용자가 언제 tracking되는지 알기 어려운 패턴이다.
- **`REV-REACT-003`** Effect Writes Its Own Dependency — effect가 자신이 읽는 signal을 다시 수정하는 패턴이다.
- **`REV-REACT-004`** Diamond Dependency Explosion — A 변경 한 번에 D가 여러 번 실행되는 패턴이다.
- **`REV-REACT-005`** Eager Computed Everywhere — 변경 가능성이 있는 모든 computed value를 즉시 재계산한다.
- **`REV-REACT-006`** Reactive Primitive Leakage — widget API 곳곳에서 반드시 `Signal<T>`를 요구하는 패턴이다.
- **`REV-REACT-007`** Signal as Application Database — 도메인 상태 전체를 다수의 signal에 흩어놓는 패턴이다.
- **`REV-REACT-008`** Lock per Signal — 각 signal마다 독립된 `Arc<RwLock<T>>` 등을 배치하는 패턴이다.
- **`REV-REACT-009`** Effects as Business Logic — 검증·저장·네트워크·도메인 규칙을 effect 안에 모두 작성하는 패턴이다.
- **`REV-REACT-010`** No Batch Boundary — signal을 연속으로 변경할 때 변경마다 render와 effect가 실행되는 패턴이다.

### 4. Rendering — 렌더링 안티패턴

- **`REV-RENDER-001`** Full Screen Repaint — 변경된 cell과 관계없이 매 frame 전체 terminal buffer를 출력한다.
- **`REV-RENDER-002`** Allocation per Cell — 각 terminal cell마다 `String`, `Vec`, style object를 소유하는 패턴이다.
- **`REV-RENDER-003`** Formatting in Hot Render Path — 렌더링할 때마다 다음을 반복한다.
- **`REV-RENDER-004`** Clone to Satisfy the Borrow Checker — 렌더링 경로의 borrow 문제를 빠르게 피하려고 `clone()`을 확산시키는 패턴이다.
- **`REV-RENDER-005`** Unicode Width Equals Character Count — `str.
- **`REV-RENDER-006`** Partial Wide-Character Overwrite — 폭 2인 문자의 한쪽 cell만 덮어쓰는 패턴이다.
- **`REV-RENDER-007`** Paint Without Clipping — 모든 widget이 자신의 bounds 밖까지 그릴 수 있는 구조다.
- **`REV-RENDER-008`** Z-Order by Render Accident — 호출 순서가 곧 layer 순서가 되는 패턴이다.
- **`REV-RENDER-009`** Unbounded Frame Rate — 애니메이션이나 timer가 활성화되면 가능한 한 빠르게 render하는 패턴이다.
- **`REV-RENDER-010`** Render on Every Input Byte — escape sequence가 완전히 파싱되기 전에 입력 byte마다 render하는 패턴이다.

### 5. Layout — 레이아웃 안티패턴

- **`REV-LAYOUT-001`** Layout and Paint Interleaving — widget이 paint 중에 child 크기를 계산하는 구조다.
- **`REV-LAYOUT-002`** Recursive Relayout Cascade — leaf 하나의 크기 변경이 전체 root까지 전달되고 다시 모든 subtree를 계산하는 패턴이다.
- **`REV-LAYOUT-003`** O(N²) Flex Distribution — 남은 공간을 child 하나씩 반복 조정하면서 분배하는 구현이다.
- **`REV-LAYOUT-004`** Grid Track Explosion — 동적으로 생성된 grid track이나 implicit track을 제한 없이 허용한다.
- **`REV-LAYOUT-005`** Floating-Point Geometry in Cell UI — 최종 결과가 정수 cell인데 모든 layout을 `f32`로 처리하는 패턴이다.
- **`REV-LAYOUT-006`** Negative or Overflow Geometry — padding, border, percentage, shrink를 계산한 뒤 음수 크기나 integer overflow가 발생하는 패턴이다.
- **`REV-LAYOUT-007`** Hidden Widgets Still Participate — 보이지 않는 tab, collapsed accordion, closed modal이 layout 및 paint 비용을 계속 발생시키는 패턴이다.

### 6. CSS / style engine — CSS 및 스타일 엔진 안티패턴

- **`REV-STYLE-001`** Parse CSS Every Frame — 렌더링 경로에서 CSS 문자열을 반복 파싱하는 패턴이다.
- **`REV-STYLE-002`** Match Every Rule Against Every Widget — widget N개와 selector M개에 대해 매번 `N × M` 비교하는 패턴이다.
- **`REV-STYLE-003`** Stringly-Typed Properties — 모든 CSS property와 value를 문자열로 보관한다.
- **`REV-STYLE-004`** Silent Unsupported CSS — 지원하지 않는 property를 조용히 무시하는 패턴이다.
- **`REV-STYLE-005`** CSS Specificity Guesswork — CSS cascade와 specificity를 일부만 구현하면서 문서화하지 않는 패턴이다.
- **`REV-STYLE-006`** Runtime String Selector Traversal — selector match 때마다 문자열 비교와 parent traversal을 반복한다.
- **`REV-STYLE-007`** Global Style Invalidation — CSS 한 줄 변경 시 모든 widget의 style·layout·paint를 무조건 다시 수행한다.
- **`REV-STYLE-008`** Theme as Arbitrary String Map — theme variable을 자유 문자열 map으로만 관리하는 패턴이다.
- **`REV-STYLE-009`** Terminal Capability Blindness — true color, 256 color, italic, underline style, image protocol을 항상 지원한다고 가정한다.

### 7. Event system — 이벤트 시스템 안티패턴

- **`REV-EVENT-001`** Boolean Event Result — event handler 결과를 `true/false` 하나로만 표현한다.
- **`REV-EVENT-002`** Global Event Match — 애플리케이션 root에서 모든 key event를 거대한 `match`로 처리한다.
- **`REV-EVENT-003`** Focus by Mutable Boolean — 각 widget이 `focused: bool`을 따로 가지는 패턴이다.
- **`REV-EVENT-004`** Focus Order Equals Tree Order — 렌더 tree 순서가 곧 keyboard navigation 순서가 된다.
- **`REV-EVENT-005`** Shortcut Ownership Collision — widget, app, modal, command palette가 동일한 shortcut을 독립적으로 처리한다.
- **`REV-EVENT-006`** Escape-Key Ambiguity Ignored — 터미널에서 `Esc` 단독 입력과 escape sequence 시작을 구분하지 못하는 문제를 무시한다.
- **`REV-EVENT-007`** Mouse-First Interaction — hover, precise pointing, drag를 주요 interaction으로 설계한다.
- **`REV-EVENT-008`** Resize Storm Rendering — terminal resize 이벤트마다 즉시 full relayout과 repaint를 실행한다.

### 8. Widget design — Widget 설계 안티패턴

- **`REV-WIDGET-001`** God Widget Trait — 하나의 trait가 다음을 모두 담당한다.
- **`REV-WIDGET-002`** Widget Owns Domain Data — table이나 tree widget이 애플리케이션의 실제 domain collection을 직접 소유한다.
- **`REV-WIDGET-003`** Collection Widget Without Virtualization — table·list·tree·log viewer가 전체 데이터를 layout하고 render한다.
- **`REV-WIDGET-004`** Stateful and Stateless Variants Diverge — 같은 widget에 stateful 버전과 stateless 버전을 따로 구현해 동작이 달라진다.
- **`REV-WIDGET-005`** Builder API Combinatorial Explosion — 모든 옵션에 builder method를 추가한다.
- **`REV-WIDGET-006`** Widget Feature Envy — widget 하나가 style engine, runtime, terminal, worker pool, clipboard에 직접 접근한다.
- **`REV-WIDGET-007`** Inconsistent Interaction Semantics — widget마다 Enter, Space, Esc, 방향키 동작이 다르다.
- **`REV-WIDGET-008`** Every Widget Has Its Own Scroll Logic — table, textarea, markdown, tree, list가 각각 별도 scroll 구현을 가진다.
- **`REV-WIDGET-009`** Visual Variant as New Widget — 색상이나 border 차이만으로 새로운 widget 타입을 만든다.
- **`REV-WIDGET-010`** 100 Widgets, No Maturity Tier — 실험적인 widget과 안정적인 widget이 같은 수준으로 노출된다.

### 9. Worker pool / async — Worker Pool 및 비동기 안티패턴

- **`REV-ASYNC-001`** Polling `try_join()` Every Frame — 매 렌더 프레임마다 worker 완료 여부를 polling한다.
- **`REV-ASYNC-002`** Unbounded Work Queue — worker pool의 submit queue에 제한이 없다.
- **`REV-ASYNC-003`** Detached Task Without Cancellation — widget이 사라지거나 query가 변경되어도 task가 계속 실행된다.
- **`REV-ASYNC-004`** UI State Mutated from Worker Thread — worker가 signal이나 widget state를 직접 변경한다.
- **`REV-ASYNC-005`** Blocking I/O Disguised as Async — worker API가 있다는 이유로 모든 blocking 작업을 같은 pool에 넣는다.
- **`REV-ASYNC-006`** No Panic Boundary — worker closure panic이 runtime 전체를 손상시키거나 조용히 사라진다.
- **`REV-ASYNC-007`** Runtime Lock-In — Tokio 또는 특정 executor 타입이 public API 전반에 노출된다.

### 10. Animation — Animation 안티패턴

- **`REV-ANIM-001`** Animation Forces Continuous Global Render — animation 하나 때문에 전체 앱이 고정 FPS로 다시 그려진다.
- **`REV-ANIM-002`** Sub-Cell Animation — 터미널 cell로 표현할 수 없는 미세한 좌표 애니메이션을 계산한다.
- **`REV-ANIM-003`** Animation Changes Layout Every Frame — width, height, padding 같은 layout property를 매 frame 변경한다.
- **`REV-ANIM-004`** No Reduced-Motion Mode — animation을 끌 방법이 없다.
- **`REV-ANIM-005`** Time-Based Snapshot Instability — 현재 시간에 따라 snapshot 결과가 달라진다.

### 11. Hot reload — Hot Reload 안티패턴

- **`REV-HOT-001`** File Watch Event Equals Reload — filesystem watcher 이벤트 하나당 즉시 reload한다.
- **`REV-HOT-002`** Broken CSS Replaces Valid CSS — 편집 중 일시적으로 문법이 깨졌을 때 기존 stylesheet를 제거한다.
- **`REV-HOT-003`** Reload During Render — 렌더링 도중 style tree를 교체한다.
- **`REV-HOT-004`** Production Watcher Leakage — hot reload 관련 watcher, dependency, thread가 기본 production binary에 포함된다.

### 12. Proc macro — Proc Macro 안티패턴

- **`REV-MACRO-001`** Macro Hides Runtime Cost — 간결한 선언 뒤에서 allocation, clone, subscription, dynamic dispatch를 대량 생성한다.
- **`REV-MACRO-002`** Stringly-Typed DSL — macro input 안에서 widget 이름, property, event를 문자열로 처리한다.
- **`REV-MACRO-003`** Poor Span Diagnostics — macro 오류가 사용자 코드가 아닌 생성 코드나 macro 내부 위치를 가리킨다.
- **`REV-MACRO-004`** Macro and Manual API Semantic Drift — macro로 만든 widget과 builder API로 만든 widget의 동작이 다르다.
- **`REV-MACRO-005`** Compile-Time Explosion — 중첩된 UI 구조를 모두 거대한 token stream으로 펼친다.

### 13. Plugin / extension — Plugin 및 Extension 안티패턴

- **`REV-PLUGIN-001`** Plugin Without Stability Boundary — 내부 trait를 그대로 plugin API로 공개한다.
- **`REV-PLUGIN-002`** Unrestricted Plugin Capability — plugin이 terminal, filesystem, runtime, global tree를 직접 수정한다.
- **`REV-PLUGIN-003`** Plugin Panic Takes Down the App — plugin hook에 panic boundary가 없다.
- **`REV-PLUGIN-004`** Plugin Ordering by Registration Accident — plugin 동작 순서가 등록 순서에만 의존한다.
- **`REV-PLUGIN-005`** Extension Duplicates Core — extension에서 필요한 내부 capability가 없어 core 기능을 복제한다.

### 14. CLI / templates — CLI 및 Template 안티패턴

- **`REV-CLI-001`** Generated Project Immediately Outdated — template가 특정 API 버전을 복사하지만 업데이트·호환성 검증이 없다.
- **`REV-CLI-002`** CLI and Library Version Drift — CLI가 생성하는 `Cargo.
- **`REV-CLI-003`** Destructive Generation — 기존 파일을 확인 없이 덮어쓴다.
- **`REV-CLI-004`** Template as Copy-Paste Dump — template가 best practice를 보여주는 최소 예제가 아니라 모든 기능을 넣은 거대한 샘플이 된다.

### 15. Performance measurement — 성능 측정 안티패턴

- **`REV-PERF-001`** Benchmarking Toy Counters — counter나 작은 form만 benchmark하고 실제 병목을 놓친다.
- **`REV-PERF-002`** Average-Only Metrics — 평균 frame time만 측정한다.
- **`REV-PERF-003`** Profiler Observes the Wrong Boundary — 전체 render 시간만 측정하고 내부 단계를 구분하지 않는다.
- **`REV-PERF-004`** Debug Build Performance Claims — Rust debug build 결과로 성능을 판단하거나, 반대로 release build만 측정해 debug 개발 경험을 무시한다.
- **`REV-PERF-005`** No Performance Budget — 기능 추가 시 허용 가능한 성능 회귀 기준이 없다.
- **`REV-PERF-006`** Cache Without Invalidation Model — 성능 문제를 cache로 덮지만 cache가 언제 무효화되는지 정의하지 않는다.

### 16. Rust-specific — Rust 특화 안티패턴

- **`REV-RUST-001`** `Arc<Mutex<...>>` Everywhere — ownership 설계를 피하려고 모든 객체를 공유 가변 상태로 만드는 패턴이다.
- **`REV-RUST-002`** Interior Mutability as Architecture — `RefCell`, `Cell`, `Mutex`, `RwLock`이 국소 구현 수단이 아니라 framework 전반의 상태 전달 방식이 된다.
- **`REV-RUST-003`** Trait Object Everywhere — 모든 widget, style, event, callback을 `Box<dyn Trait>`로 처리한다.
- **`REV-RUST-004`** Generic Type Explosion — 반대로 모든 것을 generic으로 만들어 type과 compile time이 폭발한다.
- **`REV-RUST-005`** `'static` as Lifetime Escape Hatch — worker, callback, effect API가 모든 입력에 `'static`을 요구한다.
- **`REV-RUST-006`** Enum Variant Explosion — 모든 widget과 event를 하나의 거대한 enum에 추가한다.
- **`REV-RUST-007`** Panic for User Input Error — CSS, terminal capability, malformed event, template 입력 오류에 `unwrap()`이나 `expect()`를 사용한다.
- **`REV-RUST-008`** Error String Erasure — 모든 오류를 `anyhow::Error`나 문자열 하나로만 노출한다.
- **`REV-RUST-009`** Feature Flag Combinatorial Failure — feature가 많지만 일부 조합은 build되지 않는다.
- **`REV-RUST-010`** MSRV Declaration Without Enforcement — Rust 1.

### 17. Memory / resources — 메모리 및 Resource 안티패턴

- **`REV-MEM-001`** Arena Without Generation — node ID가 단순 index라서 제거 후 재사용된 slot을 오래된 handle이 참조한다.
- **`REV-MEM-002`** Permanent String Interning — 동적 class, ID, user text를 영구 intern하여 메모리가 회수되지 않는다.
- **`REV-MEM-003`** Unbounded Caches — 다음 cache에 상한이 없다.
- **`REV-MEM-004`** Buffer Reallocate on Every Resize — terminal resize마다 정확한 크기로 buffer를 다시 allocation한다.
- **`REV-MEM-005`** Closure Capture Retains Application — effect나 callback closure가 전체 application state를 `Arc`로 capture해 일부 widget 제거 후에도 유지한다.

### 18. DevTools — DevTools 안티패턴

- **`REV-DEV-001`** Inspector Changes Observed Behavior — inspector를 켰을 때 추가 render·layout·subscription으로 원래 성능 특성이 크게 바뀐다.
- **`REV-DEV-002`** Profiler Without Causality — “render 12ms”만 보여주고 왜 render됐는지 알려주지 않는다.
- **`REV-DEV-003`** Widget Tree Without Semantic State — tree 구조만 보여주고 다음을 볼 수 없다.
- **`REV-DEV-004`** DevTools Coupled to Private Struct Layout — devtools가 내부 필드에 직접 접근해 core 리팩터링을 막는다.

### 19. Testing — 테스트 안티패턴

- **`REV-TEST-001`** Snapshot-Only Testing — 모든 UI correctness를 snapshot으로만 검증한다.
- **`REV-TEST-002`** Snapshot Approval Fatigue — 대규모 변경마다 수백 개 snapshot이 바뀌고 검토 없이 일괄 승인한다.
- **`REV-TEST-003`** Host Terminal-Dependent Tests — 개발자의 locale, terminal width, color capability에 따라 결과가 달라진다.
- **`REV-TEST-004`** No Model-Based Event Testing — input·focus·modal·scroll은 상태 머신 성격이 강하다.
- **`REV-TEST-005`** No Fuzzing at Parser Boundary — CSS, escape sequence, markdown, CSV, JSON, tree-sitter 입력은 fuzzing 대상이다.
- **`REV-TEST-006`** Testing Public Examples Manually — README와 tutorial 예제가 CI에서 compile되지 않는다.

### 20. Public API / SemVer — API 및 SemVer 안티패턴

- **`REV-API-001`** Release Number Inflation — 아주 잦은 release가 실제 stability와 동일하다고 인식되는 패턴이다.
- **`REV-API-002`** Builder Method Is Forever — 실험 단계의 builder method도 즉시 stable public API가 된다.
- **`REV-API-003`** Breaking Change Hidden as Fix — 동작 의미 변경을 bug fix release로 배포한다.
- **`REV-API-004`** Prelude Pollution — 모든 trait, widget, extension을 `prelude::*`에 넣는다.
- **`REV-API-005`** Convenience API Bypasses Core Invariants — shortcut API가 정상 lifecycle·validation·invalidation 절차를 우회한다.

### 21. Accessibility / UX — 접근성 및 UX 안티패턴

- **`REV-UX-001`** Color-Only Meaning — 오류·경고·선택 상태를 색상만으로 표시한다.
- **`REV-UX-002`** No Keyboard Discoverability — 가능한 shortcut이 UI에 드러나지 않는다.
- **`REV-UX-003`** Focus Invisible — focused widget이 visual하게 구분되지 않는다.
- **`REV-UX-004`** Modal Focus Leak — modal이 열린 상태에서 Tab이나 shortcut이 배경 widget으로 전달된다.
- **`REV-UX-005`** No Small-Terminal Strategy — terminal 크기가 작아졌을 때 panic하거나 모든 정보가 겹친다.
- **`REV-UX-006`** Truncation Without Signal — 텍스트가 잘렸지만 ellipsis나 scroll 가능 여부가 표시되지 않는다.
- **`REV-UX-007`** Mouse Hover as Persistent State — mouse를 움직이지 않으면 hover UI가 계속 남거나 keyboard 사용 중에도 hover가 우선된다.

### 22. Security / robustness — 보안 및 안정성 안티패턴

- **`REV-SEC-001`** Terminal Escape Injection — 사용자 문자열을 sanitize하지 않고 terminal에 출력한다.
- **`REV-SEC-002`** OSC Sequence Trust — clipboard, hyperlink, title 변경 등 OSC sequence를 무제한 허용한다.
- **`REV-SEC-003`** Untrusted CSS Resource Exhaustion — 비정상적으로 깊은 selector, 거대한 grid, 많은 animation 등을 제한하지 않는다.
- **`REV-SEC-004`** Recursive Input Stack Overflow — 깊게 중첩된 markdown, JSON, tree, CSS expression을 재귀로 처리한다.
- **`REV-SEC-005`** Panic Leaves Terminal Corrupted — panic 발생 후 raw mode, alternate screen, cursor 상태가 복구되지 않는다.

### 23. Documentation — 문서화 안티패턴

- **`REV-DOC-001`** Feature Documentation Without Mental Model — API 예제는 많지만 다음 질문에 답하지 못한다.
- **`REV-DOC-002`** Happy-Path-Only Tutorial — counter와 todo만 설명하고 실제 문제를 다루지 않는다.
- **`REV-DOC-003`** Unsupported Behavior Left Ambiguous — CSS나 terminal 기능을 “비슷하게 지원”한다고만 쓰고 정확한 차이를 설명하지 않는다.

### 24. Project operation — 프로젝트 운영 안티패턴

- **`REV-PROJ-001`** Core and Showcase Develop at Same Pace — 실제 core 안정성보다 새로운 showcase widget 개발이 더 빠르다.
- **`REV-PROJ-002`** Number of Widgets as Primary KPI — widget 개수가 framework 품질로 취급된다.
- **`REV-PROJ-003`** No Architectural Decision Records — retained mode, signal ownership, CSS semantic, threading 모델 같은 결정이 코드에만 남는다.
- **`REV-PROJ-004`** AI-Generated Surface Expansion — AI로 widget과 API를 빠르게 추가하지만 중복 abstraction과 semantic inconsistency를 검토하지 않는다.
