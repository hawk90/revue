# 프렐류드에 없는 위젯 74개

`feedback/`로 CSS 스윕을 넓히다 `Alert`와 `Tooltip`이 `use revue::prelude::*` 아래에서
컴파일되지 않아 발견.

## 사실

DOM 노드를 보고하는(= `impl_view_meta!`를 쓰는) 위젯 중 **74개가 프렐류드에 없다.**

```
Accordion, Alert, Autocomplete, Breadcrumb, Calendar, Callout, Chart,
CodeEditor, Collapsible, ColorPicker, Combobox, ContextMenu, CsvViewer,
DataGrid, DateTimePicker, Diagram, DiffViewer, DropZone, ErrorBoundary,
FilePicker, FileTree, Form, GradientBox, Grid, HeatMap, Histogram,
HttpClient, ... (74)
```

전부 `revue::widget::X`로는 닿는다. **도달 불가가 아니라 프렐류드 불완전이다.**

그런데 `docs/FEATURES.md`는 이들을 프렐류드 전제로 보여준다 — `card()`, `alert()` 등의
예제에 import가 없다. 즉 **문서와 코드가 어긋나 있다.**

## 실측 (2026-08-24)

위 74개는 `impl_view_meta!`를 쓰는 위젯만 센 수치였다. `widget::`이 내보내는 것 전체로
넓히면 더 크다.

| | |
|---:|---|
| `widget::`이 내보내는 타입 | **414** |
| 그중 프렐류드에 있는 것 | **115** |
| **없는 것** | **299** |

### 충돌은 실제로 얼마나 되나

"이름 충돌"은 이 문서가 A안을 미뤄둔 이유였는데, **측정한 적이 없었다.** 측정했다.

프렐류드가 이미 내보내는 것과 **이름이 겹치는 위젯 타입은 9개**다:

```
Animation, BreadcrumbItem, EventType, Form, FormField,
Screen, Search, Timer, ValidationError
```

`std` 프렐류드의 흔한 이름을 가리는 것은 **3개**다:

```
Arc        (도형. std::sync::Arc가 아니다)
Element
Transform
```

원래 걱정했던 `Chart`, `Grid`, `Calendar`, `Table`, `Column`은 **프렐류드 안에서 부딪히지
않는다.** 사용자 코드와 부딪힐 수는 있지만, 그건 Rust에서 명시 `use`가 glob을 이기므로
사용자가 해결할 수 있는 종류다.

### 실험: 그냥 glob을 넣어봤다

`pub use crate::widget::*;`를 프렐류드에 넣고:

| | |
|---|---|
| `cargo build` | 경고 0 |
| `cargo build --examples --all-features` (예제 전부) | 통과 |
| `cargo test --all-features --no-run` (테스트 전부 컴파일) | 통과 |

그리고 프렐류드에 없던 이름이 실제로 닿는지 확인했다 — `Slider`, `Accordion`, `DataGrid`,
`ColorPicker`, `MaskedInput`를 `use revue::prelude::*` 하나로 쓰는 테스트는 glob이
있으면 통과하고, 없으면 `cannot find` 5개로 실패한다.

**즉 A안의 기술적 장애물은 없다.** glob import는 이름을 *실제로 쓸 때*까지 모호성 오류를
내지 않으므로 컴파일이 통과했다는 것만으로는 부족한데, 소비자(예제 46개 + 테스트 전체)까지
통과한 것이 근거다.

## 남은 것은 기술이 아니라 정책이다

- **되돌리기 어렵다.** 공개 표면을 늘리는 것은 semver상 추가지만 뺄 때는 breaking이다.
  3.0 직전에 299개를 늘리는 것은 가볍지 않다.
- **`Arc`가 진짜 함정이다.** 컴파일 오류가 아니라 **조용한 오해**다. `use revue::prelude::*`
  아래에서 `Arc::new(x)`를 쓰면 `std::sync::Arc`가 아니라 도형 위젯이 잡힌다. 명시
  `use std::sync::Arc;`가 있으면 그쪽이 이기지만, 없으면 오류 메시지가 엉뚱한 곳을 가리킨다.
  A안을 택한다면 **`Arc`만은 프렐류드에서 빼거나 이름을 바꾸는 것**을 같이 결정해야 한다.

## 선택지

| | | 대가 |
|---|---|---|
| **A. glob으로 전부 넣는다** | `pub use crate::widget::*;` | 표면 +299. `Arc` 문제를 따로 처리해야 한다 |
| **A′. glob에서 몇 개만 뺀다** | glob + `Arc`·`Element`·`Transform` 제외 | A와 같되 함정 제거. 제외 목록의 근거를 적어둬야 한다 |
| **B. 문서가 쓰는 것만 넣는다** | 표면을 작게 유지 | 어느 것이 "문서가 쓰는 것"인지 매번 정해야 한다 |
| **C. 아무것도 안 넣고 문서를 고친다** | 표면 그대로 | 예제마다 `use revue::widget::X;` |

권고는 **A′**. 기술적 장애물이 없다는 것이 측정됐고, 유일한 실질 함정(`Arc`)은 이름 3개를
빼는 것으로 사라진다.

어느 쪽이든 **문서와 코드가 일치해야 한다**는 것만은 정해져 있다. 지금은 둘 다 아니다.

**표면을 299개 늘리는 판단 자체는 공개 API에 관한 것이라 사람이 한다.**
