# 이벤트 시퀀스 — 입력이 이어질 때 어긋나는 위젯 상태

## 왜

위젯 매트릭스(`findings-widget-matrix.md`)는 위젯 하나를 한 번 그려 보고, 결함
주입(`findings-fault-injection.md`)은 이상한 이벤트 하나를 보낸다. 둘 다 *한 번*의
입력을 본다. 하지만 상태를 가진 위젯은 입력이 *이어질 때* 깨진다. 빈 목록에서 아래로
가기, 마지막 항목을 고른 뒤 항목 줄이기, 필터로 고른 행이 사라지기, 글 끝에서
Shift+→ 뒤에 Backspace, 접힌 패널, 커서가 글 끝을 넘어가기 같은 것들이다. 한 단계씩
보면 모두 멀쩡하고, 순서가 맞아떨어질 때만 상태가 어긋난다.

이벤트 시퀀스는 상태 있는 위젯마다 무작위 입력열을 보내고, **매 단계 뒤에** 세 가지를
확인한다.

1. **그 단계가 패닉하지 않는다.**
2. **지금 크기로 그려도 패닉하지 않고 영역 밖에 쓰지 않는다.** 위젯 매트릭스의 센티널
   검사(`tests/matrix/sentinel.rs`)를 그대로 쓴다.
3. **위젯의 불변식이 지켜진다.** 공개 getter로 값싸게 볼 수 있는 것만 본다. 선택
   인덱스가 항목 안에 있다(비면 0이나 `None`), 커서가 글 안에 있다, 스크롤 offset이
   최댓값 이하다, 슬라이더 값이 범위 안이다, 날짜가 실제 날짜다 등. 불변식은 카탈로그
   항목마다 적었고, getter가 없는 위젯은 1·2만 본다.

## 무엇을 덮나

카탈로그(`tests/sequences/catalog.rs`)는 **위젯 55개, 설정 60개**를 갖는다. 같은
위젯을 다른 설정으로 두 번 넣은 것이 있다(`List`와 `List(empty)`,
`Combobox`와 `Combobox(multi)`, `MaskedInput`과 `MaskedInput(pin)`,
`DateTimePicker`와 `DateTimePicker(range)`, `RangePicker`와 `RangePicker(order)`).
항목마다 다음을 적는다.

- **만들기:** 평범한 내용(한글·이모지 한 줌 포함)으로 위젯을 만든다.
- **키가 닿는 길:** 위젯 자신의 공개 핸들러다. `handle_key(&Key)`,
  `handle_key_event(&KeyEvent)`, `Interactive::handle_key`를 쓰고, `ScrollView`처럼
  뷰포트 높이를 받는 것에는 지금 높이를 넘긴다. 키 핸들러 없이 이동 메서드만 있는
  목록형 위젯(`Table`, `VirtualList`, `OptionList`, `JsonViewer`, `CsvViewer`,
  `Pagination`, `Stepper`)은 앱의 핸들러가 하듯 키를 그 메서드에 연결하고, 항목에
  그렇다고 적었다.
- **마우스가 닿는 길**(있으면): `DataGrid`, `SortableList`, `Select`, `Combobox`,
  `Switch`, `Button`, `ScrollView`, `Popover`(`handle_click`), `ToastQueue`.
- **데이터 변경**(setter가 있으면): 비우기, 항목 줄이기, 값·검색어·필터 설정,
  내용 바꾸기, 항목 지우기·더하기 등. 시퀀스 사이사이에 끼어든다.
- **불변식**(위 3).

**빠진 것:** `Link`는 Enter와 클릭이 시스템 브라우저를 연다. `Vim`(`VimState`)은
`View`가 아니라서 그리지 않고 패닉만 본다.

### 단계

시퀀스는 1~64단계이고, 한 단계는 넷 중 하나다.

| 단계 | 내용 |
|---|---|
| 키 | 55개 알파벳에서 하나. 화살표, Home/End, PageUp/Down, Tab/BackTab, Enter, Esc, Space, Backspace/Delete, `a x 1 0 - + / :`, `한`, `😀`, vim 명령 글자(`j k h l g G i v d n y p u w q`), Shift+화살표, Ctrl+`a z y d f`, Ctrl+←/→/Home/End, Alt+↑/↓ |
| 변경 | 위젯의 데이터 변경 하나(위) |
| 크기 | 그리는 크기를 바꿈: 40×10(처음), 0×0, 1×1, 2×1, 5×2, 12×3, 20×6, 80×24, 7×40 |
| 마우스 | 왼쪽 누름·뗌·끌기, 휠 4방향, 이동, 오른쪽 누름. 좌표는 축마다 영역 시작·가운데·마지막 칸·끝 다음 칸·시작 앞 칸, 0, `u16::MAX`, 시작+0..90 |

그리는 영역은 매트릭스와 같게 버퍼 안쪽(왼쪽·오른쪽 2칸, 위·아래 1줄 여백)에 둔다.
그래서 가장자리 밖에 쓰면 센티널 여백에 걸린다. 포커스를 켜고 그린다.

### 층

| 층 | 파일 | 대상 | 단계 비중(키:변경:크기:마우스) | 위젯당 시퀀스 |
|---|---|---|---|---|
| 2a 키 | `tests/sequences/keys.rs` | 설정 60개 전부 | 12 : 2 : 2 : 0 | 512 |
| 2b 마우스 | `tests/sequences/mouse.rs` | 마우스 핸들러가 있는 설정 9개 | 2 : 1 : 2 : 10 | 512 |

합계 약 35,000 시퀀스(단계로는 100만 개 남짓)이고, 디버그 빌드에서 타깃 전체가 약
5~8초에 끝난다(위젯마다 스레드 하나, 두 층이 병렬). 기본 기능과
`--no-default-features`에서 같은 카탈로그가 돈다(기능 플래그 뒤의 위젯이 없다).

- **결정적이다.** 고정된 ChaCha 시드(`TestRng::deterministic_rng`)와 명시적
  `Config`(`failure_persistence: None`)를 쓰므로 `PROPTEST_*` 변수도 회귀 파일도 무엇이
  도는지를 바꾸지 않는다. 같은 커밋이면 언제나 같은 시퀀스가 돌고 같은 최소 시퀀스로
  줄어든다.
- **줄이기(shrinking).** 위젯마다 첫 실패에서 멈추고 proptest가 시퀀스를 최소로
  줄인다(최대 2048번). 그래서 위젯 하나가 한 번에 실패 하나만 보여 준다. 그 실패를
  고치면 같은 위젯의 다음 실패가 드러날 수 있다(아래 "가려진 실패").
- **멈춤.** 위젯 하나가 120초 안에 끝나지 않으면 `hang`으로 보고하고 버린다.

## 돌리는 법

```bash
cargo test --test event_sequences                          # 기본 기능
cargo test --test event_sequences -- --nocapture           # 층별 요약 출력
cargo test --no-default-features --test event_sequences
```

CI의 `cargo nextest run --all-features --tests`에 그대로 포함된다.

더 깊이 찾고 싶으면 `keys.rs`/`mouse.rs`의 `CASES`를 잠시 올리고 릴리스로 돌린다
(`cargo test --release --test event_sequences`; 4096이면 약 5초, 8192면 약 11초).
래칫은 커밋된 `CASES`에 맞춰져 있으므로 올린 채로 커밋하지 않는다. 아래 표의 두
원인은 이렇게 찾았다.

## 래칫 규칙

알려진 실패는 `tests/sequences/known_failures.rs`의 `KNOWN` 한 곳에 있다. 항목은
(층, 위젯 설정, **줄인 최소 시퀀스**, 원인 한 줄)이다. 시퀀스는 보고서가 찍는 그대로의
문자열이다. 단계는 공백으로 잇고, 키는 `Down`·`'한'`·`Ctrl+End`, 변경은
`<clear>`, 크기는 `[5x2]`, 마우스는 `Down(Left)@(start,mid)`로 적는다.

- 목록에 **없는** (위젯, 시퀀스) 실패가 나오면 실패한다. 출력은 실패한 단계와 이유,
  그리고 붙여 넣을 수 있는 `Known { … }` 항목을 보여 준다.
- 목록에 **있는** 실패가 더 이상 나지 않으면 역시 실패한다. 위젯을 고쳤으면 그 항목을
  지운다. 그래서 목록은 줄어들기만 한다.
- 한 원인을 고쳤는데 같은 위젯에서 가려져 있던 다른 실패가 드러나면, 그 수정 커밋이
  새 실패를 목록에 올리고 다음 커밋이 고친다.

## 발견한 실패

처음 돌렸을 때 **18개 설정**이 실패했다(키 층 18, 마우스 층 3 — 마우스 층의 셋은 키
층과 같은 원인). 고치는 동안 가려져 있던 실패 2개가 드러났고, 깊은 탐색(4096
시퀀스)이 2개를 더 찾았다. 원인은 모두 **22개**이고, **21개를 원인별 커밋으로
고쳤다.** 각 커밋은 줄인 시퀀스를 재현하는 회귀 테스트를 코드 가까이(그 위젯의 기존
테스트 파일이나 소스 안 테스트)에 하나 더한다. 남은 하나는 설계 결정이 필요해서
목록에 남겼다.

### 원인별로 고친 실패

| 위젯 | 줄인 시퀀스 | 무엇이 깨졌나 | 원인 | 고친 방법 | 회귀 테스트 |
|---|---|---|---|---|---|
| `Tabs` | `'0'` | 패닉 | 숫자 키를 `c - '1'`로 탭 번호로 바꿔 `'0'`에서 underflow | `'1'..='9'`만 탭을 고른다 | `layout::tabs::tests::test_tabs_zero_key_is_ignored` |
| `MenuBar` | `[5x2] <open menu 1> Left Left` | 그리기 패닉 | 열린 메뉴 제목이 좁은 영역 오른쪽 끝을 넘어 시작하면 `area.width - menu_x`가 underflow | 그런 드롭다운은 그리지 않음, 제목 위치 누적은 포화 | `feedback::menu::tests::test_menu_bar_dropdown_past_the_right_edge` |
| `Popover` | `<anchor far>` | 그리기 패닉(키·마우스 층) | 기준점 `u16::MAX`에서 `anchor + 1` overflow | 기준점 오프셋을 포화(위치는 원래 영역 안으로 잘림) | `tests/widget/feedback/popover.rs::test_anchor_at_the_far_corner_does_not_overflow` |
| `RichTextEditor` | `<empty content> '한' Ctrl+Right Tab` | 패닉 | 커서 열은 문자 수인데 블록 끝을 `Block::len()`(바이트)로 쟀다. 한글 뒤에서 →가 글 끝을 넘고 다음 입력이 범위 밖을 자름 | 커서 이동·삭제·병합·서식 undo가 문자 수(`char_count`)를 씀. `Block::len()`은 뜻(바이트)을 그대로 두고 문서에 적음 | `tests/widget/form/rich_text_editor/cursor.rs::test_cursor_stops_at_the_end_of_non_ascii_text` |
| `RichTextEditor` | `[2x1] <link dialog>` *(가려져 있던 실패)* | 그리기 패닉 | 링크/이미지 대화상자 폭이 `width - 4`라 4칸 이하 영역에서 0이 되고 `width - 1` underflow | 테두리 두 칸도 없으면 그리지 않음 | `tests/widget/form/rich_text_editor/link.rs::test_dialog_in_a_tiny_area` |
| `Input` | `Shift+Right Backspace` | 선택이 글 끝을 넘음 | 글 끝에서 Shift+→가 빈 선택(커서 위 anchor)을 남기고, Backspace/Delete가 그 anchor를 지우지 않음. 다음 편집이 낡은 범위에 작용 | Backspace/Delete가 글자를 지울 때 빈 anchor를 지움 | `tests/widget/input/input_widgets/input/mod.rs::test_empty_selection_does_not_outlive_backspace_or_delete` |
| `JsonViewer` | `Ctrl+End 'x'` | 선택이 보이는 행 밖 | `collapse_all()`이 선택을 그대로 둠 | 선택을 보이는 행으로 자름 | `tests/widget/data/json_viewer/mod.rs::test_collapse_all_keeps_the_selection_visible` |
| `MultiSelect` | `<select all> Escape Left <clear selection>` | 태그 커서가 없는 태그를 가리킴 | `clear_selection`/`deselect_option`/`remove_last_tag`가 태그 커서를 맞추지 않음 | 남은 마지막 태그로 옮기거나 태그와 함께 없앰 | `tests/widget/multi_select/selection.rs::test_tag_cursor_follows_a_shrinking_selection` |
| `LogViewer` | `<search 'e'> 'n' <clear> <push>` | 현재 검색 결과가 결과 밖 | `clear()`가 결과만 비우고 인덱스는 둠. 항목 추가·잘라내기로 결과를 다시 셀 때도 인덱스를 확인하지 않음 | `clear()`가 인덱스를 0으로, 다시 셀 때 범위 밖이면 첫 결과로 | `log_viewer::view::search::tests::test_search_index_stays_within_the_matches` |
| `CsvViewer` | `Ctrl+End <parse empty>` | 선택이 새 데이터 밖 | `parse()`/`data()`가 옛 데이터의 선택·스크롤·검색 결과를 그대로 둠 | 첫 칸으로 되돌리고 새 행에서 검색어를 다시 찾음 | `tests/widget/data/csv_viewer.rs::test_parse_resets_the_selection_and_the_search` |
| `CsvViewer` | `<search '1'> 'n'` *(가려져 있던 실패)* | 없는 열을 선택 | 첫 행만큼의 열만 보여 주는데 검색은 더 긴 행의 남는 칸까지 찾음 | 보이는 열만 검색 | `tests/widget/data/csv_viewer.rs::test_search_skips_cells_past_the_shown_columns` |
| `DataGrid` | `Ctrl+End <drop rows>` | 선택이 마지막 행 밖(키·마우스 층) | `recompute_cache()`가 행이 줄어도 선택·스크롤을 둠. 필터에 걸린 행을 편집해 필터 밖으로 내보낼 때도 같음 | 선택·스크롤을 남은 행으로 자름 | `tests/widget/data/datagrid/mod.rs::test_recompute_cache_keeps_the_selection_on_a_row` |
| `Pagination` | `<total 0> End` | 0쪽 | 쪽은 1부터 세고 `new(0)`·`set_total(0)`은 1쪽을 지키는데 `last()`/`goto()`/`current()`는 0쪽으로 감 | 쪽이 없어도 1쪽 | `tests/pagination_tests.rs::test_no_pages_stays_on_page_one` |
| `Slider` | `<set NaN>` | 값이 범위 밖 | NaN을 clamp하면 NaN. 그 뒤 증감도 계속 NaN | NaN은 무시하고 현재 값 유지 | `tests/widget/input/input_widgets/slider/value.rs::test_set_value_nan_keeps_the_value` |
| `ScrollView` | `Ctrl+End [80x24]` | offset이 최댓값 밖(키·마우스 층) | offset을 마지막 스크롤 호출의 뷰포트로만 자름. 뷰포트가 커진 뒤 옛 offset을 그대로 그려 내용 아래가 비고, ↑/PageUp/휠을 여러 번 눌러야 움직임 | 그릴 때와 키·마우스 핸들러 시작에서 지금 뷰포트로 자름 | `layout::scroll::tests::test_scroll_view_offset_after_the_viewport_grows` |
| `TextArea` | `<set content> Enter PageUp <add cursor below> Delete` | 보조 커서가 글 밖 | 편집은 주 커서만 옮김. 줄을 합치거나 바꿔 글이 줄면 다른 커서와 선택 anchor가 남음 | 그런 편집 뒤 모든 커서를 글 안으로 자름 | `tests/widget/input/input_widgets/textarea/bounds.rs::test_secondary_cursors_stay_in_the_text_after_an_edit` |
| `DateTimePicker` | `'j'` (1월 31일에서) | 2월 31일 | 달 이동이 커서 날짜만 새 달에 맞추고 저장된 날짜(`get_date()`)의 일은 둠 | 저장된 일도 새 달에 맞춤 | `tests/widget/datetime_picker.rs::test_month_navigation_keeps_a_real_date` |
| `DateTimePicker(range)` | `Up` (10월 7일, 범위 10/1~10/20) | 범위 밖 날짜 | 이동은 저장된 날짜를 함께 옮기는데 `date_range`는 날짜를 고를 때만 확인 | `RangePicker`처럼 커서와 날짜를 범위로 자름 — 이동이 한계에서 멈춤 | `tests/widget/datetime_picker.rs::test_navigation_stops_at_the_date_range` |
| `RangePicker` | `'j'` (1월 31일 시작) | 2월 31일 | `DateTimePicker`와 같은 원인 | 저장된 일도 새 달에 맞춤 | `range_picker::navigation::tests::test_navigation_keeps_real_dates` |
| `CodeEditor` | `<find 'n'> Backspace 'n' <set content> Enter` *(깊은 탐색)* | 커서가 글 밖 | `set_content()`가 옛 글의 찾기 결과를 둠. 다음 찾기가 없는 줄·열로 커서를 옮김 | 새 글로 찾기 결과를 다시 계산 | `tests/code_editor/find.rs::test_set_content_refreshes_the_find_matches` |
| `SortableList` | `Down(Left)@(+0,start) Drag(Left)@(start,mid) <remove 0> Up(Left)@(start,start)` *(깊은 탐색, 마우스 층)* | 패닉 | 끌기가 옛 목록의 인덱스를 들고 있는데 `remove()`가 끌기 중에 항목을 지움. 놓을 때 범위 밖에 삽입 | `remove()`가 진행 중인 끌기를 취소, `end_drag()`는 항목에 맞지 않는 인덱스를 무시 | `tests/widget/sortable/core.rs::test_remove_during_a_drag_cancels_it` |

공개 API는 바뀌지 않았다(`cargo semver-checks`). 새로 생긴 메서드는 크레이트 안에서만
보이는 `Block::char_count`(`pub(crate)`)뿐이다.

### 남긴 실패 (설계 결정)

| 위젯 | 줄인 시퀀스 | 무엇이 깨지나 | 왜 남겼나 |
|---|---|---|---|
| `RangePicker(order)` | `BackTab <start after end> <start after end> BackTab Up` | `get_range()`의 시작이 끝보다 뒤 | 달력 이동이 저장된 시작/끝 날짜를 커서와 함께 옮기는 것이 의도된 설계다(`keep_cursor_in_limits` 주석). 순서는 날짜를 고를 때(`select_date`의 `swap_if_needed`)만 맞춘다. 그래서 끝 달력을 시작보다 앞으로 옮기면 다음 선택 전까지 범위가 뒤집혀 보인다. 고치려면 (a) 이동 중에도 바꿔 넣을지(포커스한 달력이 뒤바뀐다), (b) 보이는 달과 저장된 날짜를 나눌지 정해야 한다. 같은 겹침이 `DateTimePicker`에도 있다(이동이 `get_date()`를 바꾼다). 그쪽은 필드가 전부 `pub`이라 (b)는 깨지는 변경이다 |

### 가려진 실패

위젯마다 첫 실패에서 멈추므로, 한 원인이 같은 위젯의 다른 실패를 가린다.

- `RichTextEditor`는 바이트/문자 패닉을 고치자 같은 위젯의 작은 영역 대화상자
  패닉이 드러났다.
- `CsvViewer`는 새 데이터의 선택을 고치자 들쭉날쭉한 행의 검색 실패가 드러났다.
- 피커 둘은 한 위젯에 불변식이 둘(실제 날짜, 범위·순서)이라 설정을 둘로 나눠 하나가
  다른 하나를 가리지 않게 했다.

## 눈여겨볼 것

- **패닉보다 어긋난 상태가 많다.** 22개 원인 중 패닉은 6개(Tabs, MenuBar, Popover,
  RichTextEditor 둘, SortableList)이고, 나머지 16개는 패닉 없이 상태가 어긋난 것이다.
  선택이 항목 밖, 커서가 글 밖, 날짜가 2월 31일 같은 것이다. 한 번의 입력만 보는
  매트릭스와 결함 주입으로는 닿지 않는다.
- **"줄어드는 쪽"이 늘 빠진다.** 항목을 지우거나, 데이터를 바꾸거나, 접거나, 선택을
  비울 때 그 데이터를 가리키는 인덱스(선택, 태그 커서, 검색 결과, 끌기, 보조 커서)를
  맞추지 않는 것이 가장 흔한 원인이었다. DataGrid·JsonViewer·MultiSelect·LogViewer·
  CsvViewer·TextArea·SortableList·CodeEditor가 모두 이 꼴이다.
- **`ScrollView`의 불변식은 위젯이 아는 만큼으로 정했다.** 위젯은 크기가 바뀐 것을
  다음 핸들러 호출 때 비로소 안다. 그래서 검사는 "마지막 키·마우스 단계가 넘긴
  뷰포트 기준 offset ≤ 최댓값"이고, 카탈로그가 내용을 실제로 그려 그려지는 offset도
  센티널 검사를 거친다.
- **TextArea의 여러 커서는 편집하지 않는다.** 편집은 주 커서에만 일어나고 보조
  커서는 자리만 지킨다. 이번 수정은 보조 커서가 글 밖에 남지 않게 했을 뿐, 여러
  커서 편집을 만들지는 않았다.
- **보았지만 이 층이 잡지 않아 고치지 않은 것.** `TextArea` undo가 커서 열을
  `col + text.len()`(바이트)로 되돌린다(한글이면 커서가 엉뚱한 곳에 선다. 범위 밖으로는
  잘린다). `Slider::range(min, max)`에 `min > max`를 주면 `f64::clamp`가 패닉한다. 둘
  다 이번 카탈로그의 변경 단계가 만들지 않는 입력이다.
- **깊은 탐색.** 커밋된 512 시퀀스로 못 찾은 원인 둘(`CodeEditor`, `SortableList`)을
  4096 시퀀스가 찾았다. 둘을 고친 뒤 8192 시퀀스(릴리스, 약 11초)에서는 남긴 하나
  말고 새 실패가 없었다.
