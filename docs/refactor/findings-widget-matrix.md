# 위젯 매트릭스 — 크기·내용·상태 조합에서 깨지는 위젯

## 왜

라인 커버리지는 91%지만, 줄 하나가 실행됐다고 그 줄의 경계 조건까지 확인된 건
아니다. `area.width - 2`는 80×24에서 한 번만 실행돼도 커버리지에 잡힌다. 1×1에서
underflow로 패닉하는지는 아무도 모른다. 매트릭스는 모든 공개 위젯을 경계 크기,
까다로운 문자열, 포커스, 두 렌더 경로의 조합으로 그려 보고 두 가지만 확인한다.

1. **패닉하지 않는다.**
2. **받은 영역 밖에 아무것도 쓰지 않는다.** 버퍼를 센티널 셀(`'\u{E000}'`)로
   채운 뒤 그리고, 영역 밖 셀이 전부 그대로인지 본다. 기호뿐 아니라 색·modifier도
   비교한다(직접 경로만).

## 무엇을 덮나

카탈로그(`tests/matrix/catalog.rs`)는 공개 위젯마다 이름과 팩토리 하나를 갖는다.
팩토리는 케이스의 내용을 위젯에 먹인다. 텍스트를 받는 위젯은 `content.text`를,
목록을 받는 위젯(옵션, 행, 데이터 포인트, 자식)은 `content.items`나 거기서 만든
숫자를 받는다. 그래서 같은 내용이 항목 0개 / 1개 / 많이를 함께 몬다. `focused`
빌더가 있는 위젯은 케이스의 포커스를 그대로 넘긴다. 직접 경로의 포커스 케이스는
`RenderContext::full`에 `NodeState { focused: true }`도 넘겨 `ctx.is_focused()`를
읽는 위젯까지 덮는다.

- **위젯 131개**(`--all-features`, 기본 기능은 125개). `tests/widget_clone.rs`의
  타입 전부에 더해 `Clone`이 아닌 `Stack`/`Border`/`Card`/`Modal`/`DataGrid`/
  `Tree`/`MenuBar`/`ContextMenu`/`Grid`/`Layers`/`ScreenStack`/`ErrorBoundary`/
  `DeclarativeRouter` 등, 라우터 `Link`, 그리고 `View`가 아닌 `DevTools`를 탭
  6개마다 어댑터로 감싸 넣었다. 기능 플래그 뒤의 위젯(`DiffViewer`, `Image`,
  `Markdown`, `MarkdownPresentation`, `QrCodeWidget`, `ProcessMonitor`)은 해당
  `cfg`로 감쌌다.
- **빠진 것:** `DockArea`/`DockManager`. `widget::layout`이 비공개이고 다시
  내보내지도 않아 크레이트 밖에서 만들 수 없다.

전체 곱은 너무 크고 실패의 원인을 짚기 어렵다. 그래서 층을 셋으로 나누고 층마다
파일 하나를 뒀다. 실패 메시지가 층 이름을 단다.

| 층 | 파일 | 축 | 위젯당 케이스 | 케이스(전체 기능) |
|---|---|---|---|---|
| 1a 크기 | `tests/matrix/sizes.rs` | 0×0, 1×1, 2×1, 1×2, 3×3, 80×24, 1000×1, 1×1000, x축 끝(65530,1), y축 끝(1,65530). 내용은 "Hello"+항목 3개, 포커스 없음, 직접 경로 | 10 | 1,310 |
| 1b 내용 | `tests/matrix/contents.rs` | "", 한글, 피부색·ZWJ 이모지, 결합 문자(e\u{301}), RTL, 폭 0 문자, 탭·개행·제어 문자, 약 1000자, 항목 0/1/200개. 크기는 40×10, 포커스 없음, 직접 경로 | 11 | 1,441 |
| 1c 쌍 | `tests/matrix/pairwise.rs` | {크기 7개} × {내용 4개} × {포커스 켬/끔} × {직접/앱 경로}의 모든 쌍 | 28 | 3,668 |

합계 6,419 케이스, 디버그 빌드에서 약 1.5초(기본 기능은 0.3초).

- **u16 끝의 영역.** `Buffer`는 한 변을 16384칸으로 제한하므로 65530에 놓인
  영역을 품는 버퍼를 만들 수 없다. 대신 작은 버퍼 *밖*에 영역을 둔다. 그러면 버퍼에
  무엇이 써지든 영역 밖이고, overflow로 좌표가 0 근처로 감기면 그대로 드러난다.
- **쌍 조합**은 테스트 안의 작은 결정적 탐욕 알고리즘으로 만든다(새 의존성 없음).
  아직 덮이지 않은 쌍을 가장 많이 덮는 전체 조합을 반복해서 고른다. 앱 경로는 끝
  영역을 놓을 수 없어서 그 쌍만 미리 뺐다. `the_pairwise_set_covers_every_pair`가
  모든 쌍이 실제로 덮이는지 확인한다.
- **앱 경로**는 `PipelineHarness`(3.0 기본값 `dom_from_render`·`css_layout` 켬)로
  같은 크기의 화면을 그린다. 영역 밖 검사는 직접 경로만 한다. 앱 경로에서는 화면
  전체가 위젯의 영역이다.
- **proptest는 두지 않았다.** 무작위 케이스는 알려진 실패 목록의 정확한 케이스
  키와 맞출 수 없어서, 이미 아는 0×0 패닉에 매번 다시 걸린다. 경계 크기는 크기층이
  이미 열거한다.

## 돌리는 법

```bash
cargo test --test widget_matrix                    # 기본 기능
cargo test --all-features --test widget_matrix     # CI와 같은 위젯 집합
cargo test --all-features --test widget_matrix -- --nocapture   # 층별 요약 출력
WIDGET_MATRIX_DUMP=/tmp/m cargo test --all-features --test widget_matrix  # 실패 전체를 층별 TSV로
```

CI의 `cargo nextest run --all-features --tests`에 그대로 포함된다.

## 래칫 규칙

알려진 실패는 `tests/matrix/known_failures.rs`의 `KNOWN` 한 곳에 있다. 항목은
(층, 위젯, 종류, 원인 한 줄, 케이스 키 목록)이고, 케이스 키는
`"<영역> <내용> <plain|focus> <direct|app>"`이다.

- 목록에 **없는** 실패가 나오면 실패한다. 출력은 표와 함께 붙여 넣을 수 있는
  `Known { … }` 항목을 제안한다.
- 목록에 **있는** 실패가 더 이상 나지 않으면 역시 실패한다. 위젯을 고쳤으면 그
  케이스를 지워야 한다. 그래서 목록은 줄어들기만 한다.
- 패닉 메시지에는 패닉 위치(`파일:줄`)가 붙는다. 원인 한 줄도 그 위치 기준으로
  나눴다.

## 발견한 실패

16개 위젯, 285 케이스. `[진행 중]`은 다른 브랜치에서 지금 고치고 있는 패닉과
겹치는 것이다.

| 위젯 | 종류 | 층 | 예시 케이스 | 원인 추정 |
|---|---|---|---|---|
| `StatusBar` [진행 중] | 패닉 ×25 | 크기·내용·쌍 | `contents: 40x10 long plain direct` | `statusbar/render.rs:66` `area.width - right_width`가 오른쪽 섹션이 영역보다 넓으면 underflow |
| `StatusBar` [진행 중] | 패닉 ×3 | 쌍 | `pairwise: 2x1 empty focus app` | `statusbar/render.rs:80` `area.width - right_width - 2`가 키 힌트 자리가 없으면 underflow |
| `Modal` [진행 중] | 패닉 ×21 | 크기·쌍 | `pairwise: 0x0 hangul focus app` | `modal/render.rs:59` `y + modal_height - 2`가 영역이 모달 테두리보다 작으면 underflow |
| `LogViewer` [진행 중과 관련] | 패닉 ×8 | 내용·쌍 | `contents: 40x10 hangul plain direct` | `log_viewer/parser.rs:292` 타임스탬프를 찾으며 `&s[..8]`을 바이트로 자름. 8바이트째가 멀티바이트 문자 안이면 패닉. 그리기 전 `load`에서 난다 |
| `LogViewer` | 패닉 ×4 | 크기·쌍 | `pairwise: 0x0 empty plain direct` | `log_viewer/view/render.rs:203` 스크롤바 `area.height - 1`이 높이 0에서 underflow |
| `RichLog` | 패닉 ×4 | 크기·쌍 | `sizes: 0x0 hello plain direct` | `richlog/render.rs:141` 스크롤바 `area.height as usize - 1`이 높이 0에서 underflow |
| `CommandPalette` | 패닉 ×1 | 크기 | `sizes: 1x1000 hello plain direct` | `command_palette/view.rs:72` 제목 자르기 `x + width - 2`가 폭 2 미만에서 underflow |
| `Splitter` | 패닉 ×5 | 크기·쌍 | `pairwise: 5x2@65530,1 empty plain direct` | `splitter/mod.rs:151` `pane_areas`의 `area.x + offset`이 u16 끝에서 overflow |
| `Button`, `Layers`, `ScreenStack` | 패닉 ×2씩 | 쌍 | `pairwise: 5x2@65530,1 hangul focus direct` | `render_context/text.rs:33` **공용 `draw_text`**: 넓은 문자의 이어지는 칸 `cx + i`가 u16 끝에서 overflow. 위젯 문제가 아니라 그 자리에 넓은 문자를 그리는 모든 위젯에 해당 |
| `Inspector`(`core::app`) | 패닉 ×22 | 크기·쌍 | `pairwise: 0x0 empty plain direct` | `inspector.rs:278` `panel_width - 3`이 패널 폭 3 미만에서 underflow |
| `Inspector` | 패닉 ×5 | 크기·쌍 | `pairwise: 5x2@65530,1 empty plain direct` | `inspector.rs:268` 제목 `title_x + i`가 u16 끝에서 overflow |
| `Inspector` | 영역 밖 ×20 | 크기·내용·쌍 | `contents: 40x10 empty plain direct` | 살펴보는 위젯의 절대 좌표(0,0,10,3)에 강조를 칠한다. 오버레이라 의도된 것일 수 있지만 자기 영역으로 잘리지 않는다 |
| `DevTools*` 6개 탭 [진행 중] | 패닉 ×16씩 | 크기·쌍 | `pairwise: 0x0 empty plain direct` | `devtools/render.rs:31` 패널 테두리 계산 `area.width - 2`가 패널이 테두리보다 좁으면 underflow |
| `DevTools*` 6개 탭 [진행 중] | 패닉 ×5씩 | 크기·쌍 | `pairwise: 5x2@65530,1 empty plain direct` | `devtools/render.rs:91` 탭 막대 `x += 1`이 u16 끝에서 overflow |
| `DevToolsEvents`·`Styles`·`TimeTravel` [진행 중] | 패닉 ×1씩 | 크기 | `sizes: 2x5@1,65530 hello plain direct` | `events/core.rs:289`, `style/view.rs:39`, `time_travel/debugger/render.rs:31` 행 커서 `y += n`이 y축 끝에서 overflow |
| `DevToolsEvents` [진행 중] | 패닉 ×1 | 쌍 | `pairwise: 80x24 emoji plain direct` | `events/core.rs:324` 이벤트 설명을 `&details[..n]` 바이트로 잘라 멀티바이트 문자 안에서 패닉 |
| `DevTools*` 6개 탭 [진행 중] | 영역 밖 ×1–8 | 크기·내용·쌍 | `sizes: 3x3 hello plain direct` | 탭 막대와 내용 행을 패널에 맞춰 자르지 않는다. 낮은 패널에서는 아래로, 긴 텍스트는 오른쪽 테두리 밖으로 나간다 |

### 눈여겨볼 것

- **앱 경로에서만 나는 실패는 없었다.** 앱 경로의 실패(`Modal`, `StatusBar`)는
  직접 경로에서도 같은 위치에서 난다. 파이프라인이 위젯에 넘기는 영역이 직접
  경로와 다른 새 경계를 만들지는 않았다.
- **한 위치가 여러 위젯을 깨뜨린다.** `draw_text`의 넓은 문자 처리 하나가
  `Button`·`Layers`·`ScreenStack`을 한꺼번에 깨뜨린다. 고치면 세 항목이 함께
  사라진다.
- **매트릭스로 재현되지 않은 진행 중 수정.** `CandleChart` 스크롤, `OptionList`
  underflow, `RenderBatch`/테두리 overflow, 프로파일러 문자열 자르기,
  `hot_reload` 경로 검사는 이 매트릭스에서 나타나지 않았다. 모두 특정 상태(스크롤
  위치, 검색어, 배치 입력)나 렌더 밖의 경로가 필요해서 크기·내용·포커스 축으로는
  닿지 않는다.
- 그 밖의 위젯 115개(기본 기능 109개)는 6,419 케이스 전부에서 두 불변식을
  지켰다.
