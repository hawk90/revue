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

## 왜 그냥 다 넣지 않았나

프렐류드는 glob export다. 74개를 넣으면:

- **이름 충돌.** `Chart`, `Form`, `Grid`, `Calendar`, `Table`, `Column`은 사용자 코드나 다른
  크레이트와 부딪히기 쉬운 이름이다. glob import 하나로 조용히 가려질 수 있다
- **되돌리기 어렵다.** 공개 표면을 늘리는 것은 semver상 추가지만, 뺄 때는 breaking이다.
  3.0 직전에 74개를 늘리는 판단은 가볍지 않다

## 선택지

| | |
|---|---|
| **A. 전부 넣는다** | 문서와 일치. 충돌 위험을 받아들인다 |
| **B. 문서가 쓰는 것만 넣는다** | 표면을 작게 유지. 어느 것이 "문서가 쓰는 것"인지 정해야 한다 |
| **C. 아무것도 안 넣고 문서를 고친다** | 표면 그대로. 예제마다 `use revue::widget::X;`를 명시 |

어느 쪽이든 **문서와 코드가 일치해야 한다**는 것만은 정해져 있다. 지금은 둘 다 아니다.

이 판단은 공개 API에 관한 것이라 사람이 한다.
