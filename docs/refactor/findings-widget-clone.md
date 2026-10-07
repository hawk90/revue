# 위젯 `Clone` — 지금 한 것과 나중에 정할 것

## 문제

`Clone` 구현 여부가 위젯마다 제각각이었다. `Input`, `Select`는 되는데 `Badge`,
`Progress`, `Slider`, `Calendar`는 안 됐다. 대부분은 이유가 없었다. 필드가 전부
`Clone`인데 `#[derive(Clone)]`만 빠져 있었다.

## 지금 한 것

`View`를 구현하는 공개 타입 131개 중 `Clone`이 없던 86개에 derive를 붙여 보고,
컴파일러가 거부한 것만 되돌렸다.

- **68개에 `#[derive(Clone)]` 추가.** 필드가 전부 이미 `Clone`이었다.
- **`Splitter`도.** 막고 있던 건 단순 데이터인 `Pane`의 빠진 derive였다.
- `tests/widget_clone.rs`가 공개 위젯 102개의 `Clone`을 컴파일 시점에 확인한다.
  derive가 빠지거나 `Clone`이 아닌 필드가 들어오면 이 테스트가 깨진다. 기능
  플래그 뒤의 위젯(`Markdown`, `Image`, `QrCodeWidget`, `DiffViewer`,
  `MarkdownPresentation`)은 해당 `cfg`로 감쌌다.

복제가 상태를 공유하는지도 필드를 끝까지 따라가며 확인했다.

- `WidgetProps`의 `WidgetKey::Str(Arc<str>)`는 바뀌지 않는 키라 공유해도 무해하다.
  기존 `Clone` 위젯도 전부 같은 필드를 갖고 있다.
- **`Form`만 상태를 공유한다.** `FormState`의 필드가 `Signal`이라 복제본은 같은
  폼에 대한 두 번째 뷰다(제출 콜백도 `Arc`라 공유). `FormState` 자체가 원래 그렇게
  설계된 핸들이라 그대로 두고, rustdoc에 적고 테스트로 고정했다
  (`a_cloned_form_shares_its_form_state`).

## 나중에 정할 것

### 1. `Clone`이 안 되는 17개

| 원인 | 위젯 |
|---|---|
| 자식 뷰 `Box<dyn View>` | `Border`, `Card`, `ErrorBoundary`, `Modal`, `Positioned`, `Stack`, `ZenMode`, `Grid`(`GridItem.widget`), `Layers`(`LayerChild.child`) |
| 박스 콜백 `Box<dyn Fn…>` | `DataGrid`(`on_column_resize` 등), `SortableList`, `Tree`, `ContextMenu`·`MenuBar`(`MenuItem.action: MenuAction`), `ScreenStack`(`ScreenRenderer`), `DeclarativeRouter`(`Router.listeners`) |
| 외부 타입 | `ProcessMonitor`(`sysinfo::System`) |

선택지:

- **자식 뷰.** `dyn View`는 `Clone`일 수 없다. (a) `dyn-clone` 방식으로 `View`에
  `box_clone`을 넣는다. 사용자 정의 위젯 전부에 `Clone`을 요구하게 되므로 사실상
  breaking이다. (b) 자식을 `Rc<dyn View>`로 바꿔 복제가 자식을 공유하게 한다. 뷰가
  불변이면 안전하지만 `&mut` 접근이 막힌다. (c) 컨테이너는 `Clone`을 안 한다.
  지금의 상태다.
- **콜백.** `Form`은 이미 `Arc<dyn Fn>`을 쓴다. 콜백을 `Arc`로 통일하면
  `DataGrid`, `SortableList`, `Tree`, `ContextMenu`, `MenuBar`, `ScreenStack`,
  `DeclarativeRouter`도 `Clone`이 된다. `FnMut` 콜백은 `Arc<Mutex<…>>`
  나 `Fn` + 내부 가변성이 필요해 API가 바뀐다.
- **`ProcessMonitor`.** `sysinfo::System`을 복제하지 않고 새로 만들게 직접 `Clone`을
  구현할 수 있다.

### 2. 앞으로 위젯에 콜백을 넣을 때의 규칙

`Clone`을 공개 API로 약속했으니 `Clone` 위젯에 `Box<dyn Fn>`을 넣으면 그 약속이
깨지고 `tests/widget_clone.rs`가 실패한다. 콜백은 `Arc<dyn Fn>`으로 넣는다는 규칙을
정할지는 1번과 함께 결정한다.

### 3. `DockManager`/`DockArea`가 외부에서 보이지 않는다

`widget/layout/mod.rs`는 `dock::{DockArea, DockManager, …}`를 `pub use`하지만
`widget/mod.rs`가 그것들을 다시 내보내지 않는다. `layout` 모듈도 비공개라 외부에서
이름을 부를 방법이 없다. 내보낼지 지울지 정해야 한다. 이 PR에서 둘에게도 derive를
붙였지만 테스트 목록에서는 뺐다.
