# 결함 주입 — 고장 난 출력, 이상한 이벤트, 망가진 텍스트

## 왜

위젯 매트릭스(`findings-widget-matrix.md`)는 위젯이 *그리는* 쪽의 경계를 본다.
하지만 앱이 실제로 깨지는 길은 그 밖에도 있다. 터미널이 쓰기를 거부하거나 일부만
받을 수 있다. 터미널이 0×0이나 65535×65535 크기를 보고할 수도 있다. 사용자가 1 MB를
붙여 넣을 수도 있다. 스타일시트·마크다운·로그 같은 텍스트가 이상한 모양으로 들어올
수도 있다. 결함 주입은 이 세 입구에 일부러 고장을 넣고 다음을 확인한다.

1. **패닉하지 않는다.** `Result`를 돌려주는 API는 패닉 대신 `Err`를 돌려준다.
2. **멈추지 않는다.** 무한 루프도, 끝나지 않는 할당도 없다.
3. **층마다 몇 가지를 더 본다.** 에러를 삼키지 않는지, 터미널을 되돌리는지, 위젯 상태가
   범위 안에 있는지다.

## 층

층마다 파일 하나를 두어 실패 메시지가 층 이름을 단다. 세 층 모두 테스트 타깃
`tests/fault_injection.rs` 하나(`mod fault;`)에 들어 있다.

| 층 | 파일 | 케이스 | 디버그 빌드 시간 |
|---|---|---|---|
| 4a 출력 | `tests/fault/output.rs` | 2,504(PTY) + 2,394(TTY 없이) | 0.6초 + 0.2초 |
| 4b 이벤트 | `tests/fault/events.rs` | 98 | 1.1초 |
| 4c 파서 | `tests/fault/parsers.rs` | 대상 16개 × 4(기본 기능은 15개) | 1.9초 |

층들이 병렬로 돌기 때문에 타깃 전체가 약 2초에 끝난다(빌드 제외).

### 4a 출력 — 고장 난 writer

`Terminal`과 `CrosstermBackend`를 init → 여러 번 그리기 → restore까지 몬다. 그리는
내용은 매번 바뀌고, 넓은 문자·색·modifier·링크·커서 이동이 들어간다. 그 아래 writer에
고장을 넣는다. 고장의 개수는 고장 없는 실행의 `write`/`flush` 호출 수로 정한다.

| 고장 | 내용 |
|---|---|
| `fail-write#N` | N번째 `write`가 에러(N은 깨끗한 실행의 쓰기 수까지 전부) |
| `fail-flush#N` | N번째 `flush`가 에러 |
| `interrupted#N` | N번째 `write`가 한 번 `Interrupted` |
| `short` | 모든 `write`가 최대 1바이트만 받음 |
| `zero` | 모든 `write`가 `Ok(0)` |

각 고장은 두 모드로 돈다. `bail`은 첫 에러에서 멈추고 터미널을 drop한다(앱의 `?`와
같다). `continue`는 계속 호출한 뒤 명시적으로 `restore`한다. 불변식은 다음과 같다.

1. **패닉하지 않는다.**
2. **에러를 삼키지 않는다.** 주입한 에러는 그 에러가 난 호출이 돌려준다. `Ok`로
   삼키면 안 된다(`fail-*`, `zero`).
3. **다시 시도할 수 있는 고장은 보이지 않는다.** 모든 호출이 성공하고 바이트가 깨끗한
   실행과 같아야 한다(`interrupted`, `short`).
4. **폭주하지 않는다.** 시나리오 하나가 `write` 예산(100만 번) 안에서 끝난다.
5. **진짜 TTY에서는 터미널을 되돌린다.**
   - 시나리오가 끝나면 raw mode가 꺼져 있다.
   - 출력이 켠 터미널 모드는 그 뒤에 다시 끄려고 했어야 한다. 대상은 대체 화면, 마우스,
     SGR 마우스, bracketed paste, 포커스 이벤트, 숨긴 커서다.
   - 대체 화면에서는 많아야 한 번 나간다(restore-once).

raw mode를 켜려면 TTY가 필요하다. 그래서 unix에서는 테스트 바이너리를 자식
프로세스로 다시 실행하고, 그 stdin에 의사 터미널(PTY)을 연결한다. crossterm은 stdin이
TTY이면 그것을 raw mode로 바꾼다. 자식은 결과를 파일로 남기고, 부모가 그 파일을 읽어
래칫에 넘긴다.

Windows는 WinAPI로 콘솔을 다루기 때문에 바이트로 셀 것이 없다. 그래서 Windows에서는
같은 시나리오를 `init` 없이 프로세스 안에서 돌리는 `output-no-tty` 층만 의미가 있다.
restore는 raw mode 밖에서는 아무것도 하지 않으므로 이 층은 restore 검사를 건너뛴다.
이 층은 unix에서도 함께 돈다.

케이스 키는 `"<terminal|backend> <고장> <bail|continue>"`이다.

### 4b 이벤트 — 이상한 입력

이벤트는 두 곳에 함께 보낸다. 하나는 `PipelineHarness::send`, 곧 실제
`App::handle_event`다. 다른 하나는 상호작용 위젯 묶음으로, 앱의 핸들러가 하는 일을
흉내 낸다. 묶음에는 입력, 텍스트 영역, 목록, 표, 탭, 선택, 스크롤 뷰가 들어 있다.

- **크기 바꾸기:**
  - 작은 크기: 0×0, 1×1, 40×0, 0×12, 1×0.
  - 한계 크기: 16384×1, 1×16384.
  - 버퍼 한계를 넘는 크기: 16385×1, 16384×16384, 65535×1, 65535×65535.
  - 줄였다가 다시 키우기.
- **마우스:** 누름·오른쪽 버튼·뗌·끌기·이동·스크롤 4방향을 화면 모서리, 바로 바깥,
  멀리 바깥, `u16::MAX`에서 보낸다.
- **붙여넣기:**
  - 1 MB(한글·한자·이모지·탭 포함).
  - 제어 문자, ESC 시퀀스, RTL 덮어쓰기.
- **연타:**
  - 키 10,000개.
  - 이상한 키: F0/F24/F255, `Null`, `Unknown`, modifier만, NUL·ESC·ZWJ·결합 문자.
  - 연타 중에 포커스 잃음·얻음과 Tick.
  - 연타 중에 크기 바꾸기.

불변식은 다음과 같다.

1. **패닉하지 않고, 프로세스가 죽지 않는다.**
   - 버퍼 한계를 넘는 크기 케이스는 자식 프로세스에서 20초 제한으로 돈다. 감당하지
     못할 할당은 패닉이 아니라 abort나 끝없는 메모리 채우기로 나타나기 때문이다.
   - 이 케이스들은 그 크기로 그리지는 않는다. 1,000만 칸은 디버그 빌드 테스트에서
     그리기에 너무 크다.
2. **앱이 계속 그린다.** 케이스가 끝나면 40×12로 되돌리고 그린다. 묶음의 제목이
   화면에 나와야 한다.
3. **위젯 불변식이 지켜진다.**
   - 모든 선택 인덱스가 범위 안에 있다.
   - 입력 커서가 글자 수 안에 있다.
   - 텍스트 영역 커서가 줄과 열 안에 있다.
   - 스크롤 offset이 내용 안에 있다.

케이스 키의 예로 `"resize 0x0"`, `"mouse scroll-down at 65535,65535"`,
`"burst 10000 keys"`가 있다.

### 4c 파서 — 망가진 텍스트

텍스트를 받는 모든 입구에 세 가지 입력을 넣는다.

- **`unicode`:** 임의의 유니코드 문자열(proptest, 256자까지, 128 케이스).
- **`soup`:** 형식마다 그 형식의 토큰을 무작위로 이은 "토큰 수프"(64토큰까지, 128
  케이스). CSS 토큰, 마크다운 표식, `[`/`]` 마크업, 이스케이프 시퀀스 등이다. 무작위
  문자보다 파서 깊숙이 닿는다.
- **`corpus`:** 고정된 고약한 입력 45개.
  - 크기: 빈 문자열, 200,000자 한 줄, 20,000줄.
  - 중첩: 괄호 10,000겹(`[`·`(`·`{`·JSON·인용·마크업), `var()` 10,000단 사슬과
    순환.
  - 닫히지 않은 구문: 주석, 문자열, 코드 펜스, 마크업, 링크, CSI, OSC, `rgb(`, `var(`.
  - 특수 문자: NUL, 제어 문자, CRLF와 CR, 결합 문자만, ZWJ 이모지, RTL+LTR 섞임,
    넓은 문자, 탭 10,000개.
  - 무작위 바이트를 `String::from_utf8_lossy`로 디코드한 것 4개.

| 대상 | 입구 |
|---|---|
| `css` | `parse_css`, 규칙마다 `apply`/`animation`/`parse_selectors`, `keyframes_definition` |
| `selector` | `dom::parse_selector`, `parse_selectors` |
| `declaration` | 속성 15개에 `apply_declaration`(`var()` 치환, 자기 참조 변수 포함) |
| `color` | `color`/`background`/`background-color`/`border-color`의 `apply_declaration` |
| `transition` | `Easing::parse`, `Transition::parse`(시간 파싱), `Transitions::parse` |
| `markdown` | `Markdown::new` + 그리기 + `toc`, `parse_slides`(`markdown` 기능) |
| `richtext` | `RichText::markup` + 그리기 |
| `keymap` | `parse_key_binding`과 그 결과의 `format_key_binding` 되돌리기, `KeyChord::parse` |
| `mermaid` | `Diagram::parse` + `compute_layout` + 그리기 |
| `logviewer` | `LogViewer::load`/`push` + 그리기, `LogParser::parse`(JSON 켬) |
| `query` | `Query::parse` + `matches` |
| `terminal` | 터미널 위젯 `write`(ANSI 파서) + 그리기 |
| `ansi` | `parse_ansi`, `strip_ansi`, `ansi_len` |
| `json` | `JsonViewer::from_content` + 그리기 |
| `csv` | `CsvViewer::from_content` + 그리기 |
| `syntax` | `utils::highlight`(언어 6개) |

색 파싱 함수와 `parse_duration`은 크레이트 밖에 공개되지 않는다. 그래서 각각
`apply_declaration`과 `Transition::parse`를 거쳐 닿는다.

불변식은 패닉하지 않는 것과 멈추지 않는 것이다.

- 입력 하나가 3초를 넘으면 실패다.
- 대상 하나가 60초 안에 끝나지 않으면 `<대상> hang`으로 보고하고 버린다.
- 대상마다 메인 스레드와 같은 8 MB 스택의 스레드에서 돈다.

proptest는 고정된 ChaCha 시드와 명시적인 `Config`로 돈다. 그래서 `PROPTEST_*` 환경
변수나 regressions 파일이 실행을 바꾸지 않는다. 실패는 줄인(shrunk) 입력과 함께
보고된다.

케이스 키는 `"<대상> <unicode|soup|corpus|hang>"`이다.

## 돌리는 법

```bash
cargo test --test fault_injection                       # 기본 기능
cargo test --all-features --test fault_injection        # CI와 같은 대상 집합(markdown 포함)
cargo test --no-default-features --test fault_injection
cargo test --all-features --test fault_injection -- --nocapture   # 층별 요약 출력
```

CI의 `cargo nextest run --all-features --tests`에 그대로 포함된다. 자식 프로세스는
같은 바이너리를 다시 실행한다. PTY 자식은 `--exact fault::output::output_faults`로,
이벤트 자식은 `--exact fault::events::event_anomalies`로 실행된다. 그래서 cargo
test와 nextest 어느 쪽에서도 같게 동작한다.

## 래칫 규칙

알려진 실패는 `tests/fault/known_failures.rs`의 `KNOWN` 한 곳에 있다. 항목은 층,
원인 한 줄, 케이스 키 목록으로 이루어진다. 규칙은 위젯 매트릭스와 같다.

- 목록에 **없는** 실패가 나오면 실패한다. 출력은 붙여 넣을 수 있는 `Known { … }`
  항목을 제안한다.
- 목록에 **있는** 실패가 더 이상 나지 않아도 실패한다. 고쳤으면 그 항목을 지워야
  한다. 그래서 목록은 줄어들기만 한다.
- 이번 구성에서 돌지 않은 케이스(꺼진 기능 뒤의 대상)는 어느 쪽으로도 세지 않는다.

**지금 `KNOWN`은 비어 있다.**

## 발견하고 고친 것

처음 돌렸을 때 세 층에서 18 케이스가 실패했다(출력 9, 이벤트 1, 파서 8). 원인은
7개였고, 원인마다 커밋 하나로 고쳤다. 각 커밋은 실패하던 층 케이스를 목록에서 지우고,
코드 가까운 곳에 회귀 테스트를 더했다.

| 층 | 실패한 케이스 | 원인 | 고친 방법 | 회귀 테스트 |
|---|---|---|---|---|
| 출력 | `backend fail-write#3..6/#47`, `fail-flush#1/#7` (bail·continue, 9개) | `CrosstermBackend`는 활성화 `execute!` 전체가 성공한 *뒤에야* 마우스 캡처를 켰다고 기록했다. `?1000h`가 이미 나간 뒤 쓰기나 flush가 실패하면 restore가 `DisableMouseCapture` 없는 갈래로 갔다. 그러면 셸에 마우스 보고가 계속 들어왔다 | 모드를 쓰기 *전에* 기록한다. 같은 파일의 `init_with_mouse`와 `enable_mouse`가 같은 패턴이었다. 켜지지 않은 모드를 끄는 것은 무해하다 | `backend::crossterm::tests::a_failed_enable_mouse_is_still_undone_by_restore` |
| 이벤트 | `resize 65535x65535`(`16384x16384`도 같은 원인) | `Buffer::new`는 한 변 16384칸, 전체 1,000만 칸을 넘는 크기를 거부한다. 하지만 `App`이 Resize마다 부르는 `Buffer::resize`는 요청받은 대로 할당했다. 43억 칸을 채우다가 죽었다 | 한 변을 16384로 자르고, 1,000만 칸을 넘으면 행을 줄인다. Resize는 터미널이 보내므로 거부할 수 없다 | `buffer::tests::resize_is_clamped_to_the_buffer_limits` |
| 파서 | `declaration hang` | `var()` 치환은 깊이(16단)만 제한하고 크기는 제한하지 않았다. 변수 하나를 여러 번 참조하는 값은 지수적으로 커진다(4번씩 16단이면 4^16) | 값 하나의 치환에 작업 예산을 둔다. 참조마다 원래 값 길이+1, 치환한 바이트마다 1이 들고, 합계는 1 MiB다. 예산이 바닥나면 순환처럼 값을 그대로 둔다. 바이트뿐 아니라 참조도 세므로 빈 값으로 끝나는 확장도 막힌다 | `style::parser::vars::tests::an_exponential_expansion_*` |
| 파서 | `terminal hang` | 터미널 위젯은 탭을 "커서가 다음 8칸 정지점보다 앞인 동안" 공백으로 채웠다. 공백이 마지막 열을 채우면 커서가 0열로 감겨 다시 정지점 앞이 되므로 루프가 끝나지 않았다. 대기 중인 이스케이프 시퀀스가 삼킨 공백은 커서를 움직이지도 않았다 | 채울 공백 수를 미리 센다. 다음 정지점과 오른쪽 끝 중 가까운 쪽까지다 | `terminal::core::tests::a_tab_at_the_right_edge_ends`, `a_tab_inside_an_escape_sequence_ends` |
| 파서 | `syntax unicode/soup/corpus`, `markdown soup` | `SyntaxHighlighter`는 줄을 문자 인덱스로 걸으면서 주석 검사에는 `line[i..]`(바이트 인덱스)를 썼다. 주석·문자열·줄 끝 앞에 멀티바이트 문자가 있으면 패닉했다. 마크다운 코드 블록도 이 경로를 탄다 | 문자마다 바이트 offset을 두고 그것으로 자른다. `find`가 준 바이트 위치는 문자 인덱스로 되돌린다. 줄을 자르는 네 곳(블록 주석 이어짐, 블록 주석 시작, 줄 주석, `#` 주석)이 모두 같은 버그였다 | `tests/utils_syntax_tests.rs::multi_byte_text_before_comments_and_strings` |
| 파서 | `mermaid soup` | `Diagram::parse`는 `A[Label]`을 문자열 전체의 첫 `[`와 첫 `]`로 잘랐다. `]`이 먼저 오면(`A][`) 슬라이스가 거꾸로 되어 패닉했다 | 닫는 괄호를 여는 괄호 *뒤에서* 찾는다. `{}`와 `()`(`rfind`)도 같은 패턴이었다 | `tests/widget/mermaid::closing_bracket_before_the_opening_one_does_not_panic` |
| 파서 | `transition soup` | `parse_duration`은 파싱한 `f64`를 그대로 `Duration::from_secs_f64`에 넘겼다. 그래서 음수·NaN·무한·너무 큰 값에서 패닉했다(`opacity -1s`). `-1ms`는 `u64`로 파싱되지 않아 이미 거부되고 있었다 | `Duration::try_from_secs_f64`를 쓰고, 표현할 수 없는 값은 `-1ms`처럼 시간이 아닌 것으로 본다 | `tests/style/transition.rs::test_transition_parse_rejects_unrepresentable_seconds` |

### 눈여겨볼 것

- **`Terminal`은 모든 출력 고장을 견뎠다.**
  - 주입한 에러는 모두 그 에러가 난 호출에서 돌아왔다.
  - 짧은 쓰기와 `Interrupted`는 crossterm의 `write_all`이 흡수했다. 바이트도 깨끗한
    실행과 같았다.
  - restore-once도 모든 경우에 지켜졌다.
  - 깨진 것은 상태 플래그를 늦게 기록한 `CrosstermBackend`뿐이다.
- **멈춤 둘은 패닉보다 고약했다.** `var()` 확장과 터미널 탭은 둘 다 패닉 없이 CPU를
  붙잡고 끝나지 않았다. 대상마다 스레드와 제한 시간을 둔 이유가 이것이다. 고치기
  전에는 층 하나가 60초 걸렸고, 지금은 2초 안에 끝난다.
- **Resize가 할당 크기를 정한다.** 65535×65535는 패닉도 abort도 아니었다. macOS는
  메모리를 넉넉히 약속하므로, 프로세스는 20초 제한에 걸릴 때까지 메모리를 채웠다.
  자식 프로세스로 격리하지 않았다면 테스트 러너도 함께 죽었을 것이다.
- **나머지 파서는 견고했다.** `parse_css`, 선택자, 색, 키맵, 쿼리, JSON, CSV, 리치
  텍스트, 로그 뷰어, ANSI 유틸은 세 입력 모두에서 패닉하지 않았다. 케이스 수를 2,048로
  올려 한 번 더 돌렸을 때도 새 실패는 없었다.
- **고치지 않고 줄인 것이 하나 있다.** 처음에 `syntax corpus`가 들여쓰기 목록
  2,000단(약 400만 자) 입력에서 3초 제한을 넘었다. 하이라이터는 공백 하나마다 토큰을
  하나 만드는데, 400만 자를 언어 6개로 넣었으니 버그가 아니라 크기 문제다. 그래서
  코퍼스를 500단으로 줄였다.
