# 결함 주입 — 고장 난 출력, 이상한 이벤트, 망가진 텍스트, 외부 자원, 동시성

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

층마다 파일 하나를 두어 실패 메시지가 층 이름을 단다. 다섯 층 모두 테스트 타깃
`tests/fault_injection.rs` 하나(`mod fault;`)에 들어 있다. 4d와 4e는 아래
[4d 외부 자원](#4d-외부-자원--파일클립보드감시http)과 [4e 동시성](#4e-동시성--태스크워커잠금반응형플러그인)에서
따로 다룬다.

| 층 | 파일 | 케이스 | 디버그 빌드 시간 |
|---|---|---|---|
| 4a 출력 | `tests/fault/output.rs` | 2,504(PTY) + 2,394(TTY 없이) | 0.6초 + 0.2초 |
| 4b 이벤트 | `tests/fault/events.rs` | 98 | 1.1초 |
| 4c 파서 | `tests/fault/parsers.rs` | 대상 16개 × 4(기본 기능은 15개) | 1.9초 |
| 4d 외부 자원 | `tests/fault/resources.rs` | 65(모든 기능), 50(기본), 41(기능 없이) | 10초 |
| 4e 동시성 | `tests/fault/concurrency.rs` | 51 | 4초 |

층들이 병렬로 돌기 때문에 타깃 전체가 약 10초에 끝난다(빌드 제외). 가장 긴 것은
4d의 클립보드 멈춤 케이스로, 고친 뒤의 도구 제한 시간 10초를 그대로 기다린다.

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

**지금 `KNOWN`에는 4e의 두 항목만 남아 있다.** 둘 다 버그라기보다 설계 결정이
필요한 것이다. [4e의 남긴 것](#남긴-것--설계-결정이-필요하다)을 보라. 4a–4d는 비어 있다.

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

## 4d 외부 자원 — 파일·클립보드·감시·HTTP

앱 밖에서 오는 것은 언제든 고장 날 수 있다. 파일은 없거나, 읽을 수 없거나, 디렉터리이거나,
크기를 속인다. 클립보드 도구는 실패하거나 멈춘다. 감시하던 파일은 지워지거나 rename으로
바뀐다. HTTP 백엔드는 에러나 쓰레기를 보낸다.

### 주입한 것

- **파일:** 경로를 받는 로더 셋에 넣는다. `Image::from_file`(`image` 기능),
  `AppBuilder::style`(스타일시트 파일), `AppConfig::load_from`(`config` 기능)이다.
  - 없는 파일, 읽기 권한 없음(`chmod 000`, unix, root가 아닐 때만), 디렉터리.
  - 로더의 크기 한계보다 1바이트 큰 sparse 파일.
  - 반쯤 잘린 내용, 깨진 내용(PNG 서명 뒤 100,000×100,000을 주장하는 IHDR 등),
    UTF-8이 아닌 바이트.
  - 심볼릭 링크 고리(unix).
  - 한계보다 1 MiB 더 흘려보내는 FIFO(unix). 메타데이터로는 0바이트다.
- **디렉터리:** `FilePicker`에 넣는다. 없는 디렉터리, 만든 뒤 지운 디렉터리, 읽을 수 없는
  하위 디렉터리로 `navigate_to`, 심볼릭 링크 고리 항목, UTF-8이 아닌 이름(파일 시스템이
  허락할 때만, 곧 Linux), 항목 2,000개. 그리고 8 MB짜리 깨진 로그 파일을 앱처럼 읽어
  `LogViewer::load`에 넣는다.
- **파일 감시**(`hot-reload` 기능):
  - 없는 경로 감시.
  - 감시하던 파일 삭제 뒤 다시 만들기, 임시 파일을 rename으로 덮어쓰기.
  - 감시하던 디렉터리 삭제.
  - 디바운스 창 안의 두 번 저장.
  - 디렉터리가 없는 스타일시트에 `AppBuilder::hot_reload(true)`.
- **클립보드:**
  - 모든 메서드가 실패하는 `ClipboardBackend`.
  - 크기 한계와 제어 문자를 받는 `MemoryClipboard`, 8 스레드 동시 사용.
  - 시스템 백엔드(unix): 자식 프로세스에서 `PATH`를 가짜 `pbcopy`/`pbpaste`/`xclip`/
    `xsel`/`wl-copy`/`wl-paste`만 있는 디렉터리로 바꾼다. 가짜 도구는 실패하거나,
    멈추거나, 곧 끝나면서 출력을 쥔 자식을 남기거나, UTF-8이 아닌 것이나 20 MB를
    출력하거나, 입력을 읽지 않고 끝나거나, 아예 없다.
- **HTTP:** `HttpBackend` 구현이 돌려준 것을 앱처럼 `HttpClient`에 넣고, 보기 셋
  (본문·헤더·원본)을 위아래 끝까지 스크롤하며 그린다.
  - 백엔드 에러.
  - 이상한 상태 코드: 0, 100, 204(본문 있음), 302, 399, 600, 999, 65535.
  - 손실 변환한 바이너리 본문, 8 MB 본문, 100,000단 JSON 본문, 헤더 10,000개.

### 불변식

1. **패닉하지 않는다.**
2. **고장을 알린다.** `Err`, 에러 상태(`HttpClient::state()`가 `Error`이고 화면에
   메시지가 보인다), 또는 아무것도 읽어 들이지 않음(스타일시트 규칙 0개)이다. 반쯤 잘린
   파일만은 예외다. 반쪽도 올바른 내용일 수 있다.
3. **멈추지 않는다.** 케이스마다 제한 시간(대부분 10초)이 있다. 클립보드 케이스는 자식
   프로세스에서 35초 제한으로 돈다.

모든 케이스는 `CARGO_TARGET_TMPDIR` 아래의 임시 디렉터리에서 돈다. 핫 리로드 감시자가
작업 디렉터리 밖의 경로를 거부하기 때문이다.

케이스 키는 `"<대상> <고장>"`이다. 예: `"image denied"`, `"css stream"`,
`"clipboard get hangs"`, `"watch rapid-saves"`, `"http status-999"`.

### 발견하고 고친 것

처음 돌렸을 때 5 케이스가 실패했다. macOS에서는 만들 수 없는 케이스 하나가 Linux에서
더 실패했을 것이다. 원인은 5개였다. 클립보드 수정에는 Linux CI가 찾은 후속 수정이 하나
더 있다(표의 둘째 줄).

| 실패한 케이스 | 원인 | 고친 방법 | 회귀 테스트 |
|---|---|---|---|
| `clipboard get hangs`, `clipboard set hangs` | `SystemClipboard`가 클립보드 도구를 제한 없이 기다렸다. 멈춘 붙여넣기 도구, 그리고 입력을 읽지 않는 복사 도구(쓰기가 꽉 찬 파이프에서 막힌다)가 호출한 쪽을 영원히 붙잡았다 | 입력 쓰기와 출력 읽기를 각자의 스레드에서 하고, 10초가 지나면 도구를 죽여 `CommandFailed`를 돌려준다. 출력은 `MAX_CLIPBOARD_SIZE`+1바이트까지만 보관하고 나머지는 읽어 버린다 | `utils::clipboard::system::tests::a_tool_that_hangs_is_killed_at_the_timeout` 외 3개 |
| `clipboard get forked-hangs`(후속) | 제한 시간이 도구 프로세스만 덮었다. 도구가 끝나거나 죽은 뒤에는 파이프를 읽고 쓰는 스레드를 제한 없이 join했다. 도구가 시작한 프로세스가 파이프를 쥐고 있으면(명령을 fork하는 셸 — Linux CI의 dash가 그랬다 — 이나 데몬이 되는 도구) 그 프로세스가 끝날 때까지 호출이 붙잡혔다. 첫 수정의 단위 테스트가 Linux CI에서 30초 걸려 드러났다 | 도구가 끝나고 *파이프도 닫힐 때까지*를 모두 제한 시간 안에서 기다린다. 넘으면 (필요하면 도구를 죽이고) 스레드는 스스로 끝나게 둔다. 이 케이스를 4d에 더했다 | 같은 단위 테스트들. 이제 `sh -c 'sleep 30; :'`로 모든 플랫폼에서 셸이 fork한다 |
| `config stream` | 로더들이 메타데이터로 크기를 본 뒤 파일 전체를 읽었다. FIFO나 문자 장치, 쓰이는 중인 파일은 메타데이터 크기가 읽히는 양과 무관하다. FIFO로 흘린 2 MiB 설정이 1 MiB 한계를 넘어 읽혔고, `/dev/zero`라면 메모리가 바닥날 때까지 읽었을 것이다. CSS는 파서가 크기를 다시 봐서 케이스는 통과했지만 같은 길이었다 | `read_capped`: 한계+1바이트에서 읽기를 멈추고 그보다 길면 거부한다. 설정, `AppBuilder::style`, 핫 리로드, `Image::from_file`(한계까지 읽고 `from_png`가 거부)에 쓴다 | `utils::tests::read_capped_reads_up_to_the_limit_and_refuses_more`, `state::patterns::config::tests::a_stream_past_the_size_limit_is_refused` |
| `picker denied` | `FilePicker::navigate_to`가 읽을 수 없는 디렉터리로 들어가 빈 디렉터리처럼 보여 주었다 | `FilePickerError::IoError`를 돌려주고 제자리에 머문다 | `widget::filepicker::tests::navigate_to_an_unreadable_directory_is_an_error_and_stays_put` |
| `watch rapid-saves` | `HotReload::poll`이 디바운스 창 안에 온 이벤트를 버렸다. 빠른 두 번 저장에서 첫 이벤트의 리로드가 두 번째 저장 전에 파일을 읽으면, 마지막 변경은 파일이 다시 바뀔 때까지 반영되지 않았다 | 창 안의 반복은 키마다 가장 최근 것 하나를 붙잡아 두었다가 창이 지나면 `poll`이 돌려준다 | `core::app::hot_reload::tests::test_poll_defers_a_repeat_until_the_window_has_passed` |
| `picker non-utf8`(Linux에서만 돈다) | `PickerEntry::from_path`가 이름을 `to_str()?`로 받아, UTF-8이 아닌 이름의 파일을 목록에서 조용히 뺐다. APFS와 NTFS는 그런 이름을 거부하므로 처음 돌린 macOS에서는 케이스가 돌지 않았다 | 이름을 손실 변환해 보여 준다. `path`는 실제 이름을 지닌다 | `widget::filepicker::tests::an_entry_whose_name_is_not_utf8_is_listed`(파일 없이 모든 unix에서 돈다) |

### 견딘 것

- **이미지·CSS·설정 로더는 나머지 고장을 모두 견뎠다.** 없음, 권한 없음, 디렉터리, sparse
  파일, 깨진 내용, 심볼릭 링크 고리에서 모두 `Err`(또는 경고 로그와 규칙 0개)였다.
  100,000×100,000을 주장하는 PNG 헤더는 `image` 크레이트의 할당 한계(512 MiB)에 걸려
  바로 거부되었다.
- **`HttpClient`는 모든 응답을 견뎠다.** 8 MB 본문도, 100,000단 JSON도 세 보기 모두
  3초 안에 그렸다. 상태 코드 분류(2xx만 성공)도 맞았다.
- **감시자는 파일 삭제·rename·디렉터리 삭제 뒤에도 계속 동작했다.**

### 남은 틈

- **시스템 클립보드의 명령은 바꿀 수 없다.** 명령이 코드에 고정되어 있고 `OnceLock`에
  캐시된다. 그래서 새 API 없이 `PATH`를 바꾼 자식 프로세스로 가짜 도구를 끼웠다. unix에서만
  돈다. Windows(`clip`, PowerShell)는 덮지 못한다.
- **`HttpClient`는 `HttpBackend`를 부르지 않는다.** `send()`는 모의 응답을 만들 뿐이다.
  그래서 "백엔드 시간 초과"는 위젯에 해당하지 않는다. 트레이트는 동기이고 시간 제한이 없다.
  위젯은 앱이 넘긴 응답과 에러를 보여 줄 뿐이다. 이 경로(`set_response`/`set_error`)를
  시험했다. 본문은 `String`이라 UTF-8이 아닌 본문은 백엔드가 손실 변환해야 한다.
- **감시자의 `Error` 이벤트는 밖에서 주입할 수 없다.** 채널이 비공개다. 실제 파일 시스템
  상황만 주입했다.
- **`LogViewer`와 `FileTree`에는 파일 입출력이 없다.** 앱이 읽어서 넘긴다. `LogViewer`는
  읽은 텍스트를 받는 쪽을 시험했다.

## 4e 동시성 — 태스크·워커·잠금·반응형·플러그인

### 주입한 것

- **태스크**(`TaskRunner`, `PooledTaskRunner`):
  - 패닉하는 태스크와 에러를 돌려주는 태스크.
  - 끝나지 않는 태스크가 도는 동안 러너 drop.
  - 같은 id로 취소한 뒤 다시 spawn.
  - 꽉 찬 작업 큐(`MAX_TASK_QUEUE_SIZE`+10개).
  - spawn 500개.
- **워커**(`WorkerHandle`, `WorkerPool`, `WorkerChannel`):
  - 패닉하는 blocking 태스크와 future.
  - 끝나지 않는 future의 취소.
  - 태스크가 도는 동안 핸들과 풀 drop.
  - 꽉 찬 풀 큐, shutdown 뒤 submit.
  - 꽉 찬(그리고 용량 0인) 명령 큐로 취소, 명령을 읽은 뒤의 취소 상태.
  - 받는 쪽 drop, 보내는 쪽 4 스레드 동시 전송.
- **잠금:** 잠금을 쥔 채 패닉하는 `Signal::update`, `Computed`, `SignalVec` diff 구독자.
- **반응형:**
  - 반응하는 시그널을 스스로 바꾸는 구독자와 effect. 수렴하는 것과 끝없는 것.
  - 패닉하는 effect.
  - 알림 중에 구독 drop.
  - 자기 벡터에 push하는 `SignalVec` 구독자.
  - 서로 읽는 `Computed` 둘, 자기를 읽는 `Computed`.
  - flush 중에 다른 업데이트를 큐에 넣는 업데이트, 패닉하는 `batch`.
  - 겹치는 `use_async` 두 번 트리거, 패닉하는 `use_async` 태스크.
  - 다른 핸들러를 등록하는 커스텀 이벤트 핸들러, 재진입 dispatch.
- **플러그인:** init, mount, tick, unmount에서 에러를 내거나 패닉하는 플러그인.

### 불변식

1. **호출한 쪽으로 패닉이 새지 않는다.** 예외는 문서화된 패닉 하나다. 끝없는 업데이트
   루프의 "Maximum reactive update depth" 패닉이다.
2. **교착도 멈춤도 없다.** 케이스마다 제한 시간이 있다(멈춤이 예상되는 케이스는 4초).
   끝없는 업데이트 루프 케이스는 자식 프로세스에서 돈다. 스택 오버플로는 잡을 수 없는
   abort이기 때문이다.
3. **상태가 일관된다.**
   - 대기 수는 0으로 돌아온다.
   - 취소한 태스크의 결과는 전달되지 않는다.
   - 의존성 추적기는 깨끗하게 남는다(`is_tracking()`이 effect 밖에서 거짓).
   - mount된 플러그인은 모두 unmount된다.
4. **에러는 메시지를 지닌다.**

백그라운드 태스크는 `resume_unwind`로 패닉한다. 패닉 훅을 거치지 않아 출력이 조용하다.
"끝나지 않는" 태스크는 케이스가 끝날 때 여는 `Gate`에서 기다린다. 열리지 않아도 30초
뒤에는 놓아 주므로 스레드가 오래 남지 않는다. 케이스는 각자의 스레드에서 동시에 돈다.
그래서 스레드 지역 반응형 상태가 케이스마다 깨끗하다.

케이스 키는 `"<영역> <고장>"`이다. 예: `"runner cancel-then-respawn"`,
`"reactive computed-cycle"`, `"plugin mount-error"`. `WorkerHandle::spawn`의
케이스는 실행기 이름을 단다. `async` 기능이 켜지면 `tokio-panic`/`tokio-cancel`이고,
꺼지면 `polling-panic`/`polling-cancel`이다. 같은 케이스가 기능에 따라 다르게 실패하기
때문이다.

### 발견하고 고친 것

처음 돌렸을 때 32 케이스가 실패했다(모든 기능). 원인은 18개였다. 그중 16개를 원인마다
커밋 하나로 고쳤다.

| 실패한 케이스 | 원인 | 고친 방법 | 회귀 테스트 |
|---|---|---|---|
| `runner panic-message`, `pooled panic-message`, `pooled error-message` | 두 러너가 잡은 패닉 payload를 `{:?}`로 찍었다. `Box<dyn Any>`는 `Any { .. }`로 찍힌다. `PooledTaskRunner::spawn_result`는 에러를 패닉으로 실어 날라서 메시지를 같은 식으로 잃었고, 패닉 훅이 그것을 앱 화면 위에 찍었다 | payload의 `&str`/`String`을 메시지로 쓴다. 풀의 작업 항목이 `Result`를 돌려주게 해서 에러가 결과로 온다 | `tasks::runner::tests::a_panic_keeps_its_message`, `tasks::pooled_runner::tests::a_failure_keeps_its_message` |
| `runner cancelled-result`, `runner cancel-then-respawn` | `TaskRunner::cancel`은 id만 잊었다. 스레드의 결과는 나중에 그대로 `poll`로 나왔다. 취소 뒤 같은 id로 spawn하면 옛 실행의 결과가 새 실행의 결과로 전달되고, 새 실행이 아직 도는데 `is_running`이 거짓이 되었다 | 실행마다 번호를 매긴다. `poll`은 지금 대기 중인 실행이 아닌 결과를 버린다 | `tasks::runner::tests::a_cancelled_run_is_not_delivered_even_after_a_respawn` |
| `pooled full-queue` | `PooledTaskRunner::spawn`이 `try_send` 전에 id를 대기로 표시하고, 큐가 꽉 찬 것은 무시했다. 거부된 태스크는 영원히 대기로 남았다 | 큐가 받아들인 뒤에만 대기로 표시한다 | `tasks::pooled_runner::tests::a_task_the_full_queue_refuses_is_not_pending` |
| `handle tokio-panic`, `handle polling-panic` | `WorkerHandle::spawn`은 `spawn_blocking`과 달리 패닉을 잡지 않았다. 패닉한 future는 결과를 남기기 전에 워커 스레드를 풀어 버렸다. 상태가 영원히 `Running`이었다 | `block_on`과 매 poll 둘레에서 패닉을 잡아 `Failed` + `WorkerError::Panicked(메시지)`로 끝낸다. 런타임을 못 만든 경우도 `Completed`가 아니라 `Failed`가 된다 | `worker::handle::tests::a_panicking_future_fails_the_handle` |
| `handle tokio-cancel` | `async` 기능에서 `spawn`은 `block_on`만 하고 취소 플래그를 보지 않았다. `cancel()`, `join_timeout`의 취소, 핸들 drop이 도는 future에 아무 효과가 없었다 | future와 플래그 감시(10ms마다)를 `select!`로 경주시킨다. 플래그가 서면 `Cancelled`로 끝난다 | `worker::handle::tests::cancel_ends_a_future_that_never_finishes` |
| `pool task-panic` | `WorkerPool`의 워커가 태스크의 패닉을 잡지 않았다. 패닉한 태스크가 워커 스레드를 끝냈다. 마지막 워커가 죽으면 큐의 모든 태스크가 영원히 기다렸다. `active_workers()`는 죽은 워커도 셌다 | 태스크 둘레에서 패닉을 잡아 로그를 남기고 워커를 계속 돌린다 | `worker::pool::tests::a_panicking_task_does_not_kill_the_worker` |
| `channel cancel-full-queue`, `channel cancel-zero-capacity`, `channel cancel-consumed` | `WorkerChannel`의 취소는 큐에 넣는 명령 하나였다. 명령 큐가 꽉 차면(용량 0도) 버려졌다. `is_cancelled()`는 큐에서 `Cancel`을 찾았으므로, Pause/Resume을 처리하려고 명령을 읽는 워커는 취소를 읽어 없앴다 | 큐 옆에 취소 플래그를 둔다. `Cancel`은 큐에 들어가든 말든 플래그를 세우고, 플래그는 지워지지 않는다. 명령을 읽으면 취소가 풀린다고 기대하던 기존 테스트(`tests/worker/channel.rs::test_sender_is_cancelled`)는 취소가 남는다고 기대하도록 바꿨다 | `worker::channel::tests::cancel_gets_through_a_full_queue_and_stays` |
| `reactive vec-drop-subscription-in-callback`, `reactive vec-push-from-subscriber`, `lock signal-vec-subscriber-panics` | `SignalVec::notify_diff`가 구독자 목록의 mutex를 쥔 채 구독자를 불렀다. 구독을 drop하거나, 구독하거나, 같은 벡터를 바꾸는 구독자는 같은 스레드에서 그 mutex를 다시 잠가 교착했다. 패닉한 구독자는 mutex를 오염시켰고, 모든 사용처의 `if let Ok(..) = lock()`이 그 뒤의 구독·해지·알림을 조용히 건너뛰었다 | 콜백을 복사해 두고 잠금을 푼 뒤 부른다(`Signal::notify`와 같다). 오염된 잠금은 다른 반응형 코드처럼 복구한다 | `reactive::signal_vec::tests::a_subscriber_can_drop_its_subscription_while_notified` 외 2개 |
| `reactive effect-panics`, `reactive effect-loop` | `Effect`와 `Computed`는 `start_tracking`, 사용자 함수, `stop_tracking` 순으로 불렀다. 그 사이의 패닉(effect의 버그, 또는 업데이트 루프의 문서화된 깊이 패닉)이 `stop_tracking`을 건너뛰어 죽은 구독자가 추적 스택에 남았다. 그 뒤 그 스레드의 모든 시그널 읽기가 그것의 의존성이 되었고, 상관없는 시그널을 바꾸면 패닉한 effect가 다시 돌았다 | `run_tracked`가 drop 가드로 구독자를 꺼낸다. 풀림(unwind) 중에도 꺼낸다 | `reactive::effect::tests::a_panicking_effect_stops_tracking`, `reactive::computed::tests::a_panicking_compute_stops_tracking` |
| `reactive subscriber-loop` | 깊이 제한은 `notify_dependents`(effect·computed 알림)에만 있었다. `Signal::subscribe` 콜백과 `SignalVec` diff 구독자는 그 밖에서 불렸다. 자기 시그널을 매번 바꾸는 구독자는 스택이 넘칠 때까지 재귀했고, 잡을 수 없는 abort로 프로세스가 죽었다 | 가드를 `enter_notify`로 빼고 `Signal::notify`와 `SignalVec::notify_diff`에서도 들어간다. 그런 루프는 effect 사이의 순환과 같은 문서화된 패닉으로 끝난다 | `reactive::signal::tests::a_subscriber_loop_ends_in_the_depth_panic_not_a_stack_overflow`, `reactive::signal_vec::tests::a_diff_subscriber_loop_ends_in_the_depth_panic` |
| `reactive computed-cycle`, `reactive computed-self` | `Computed::get`은 recompute mutex를 쥔 채 다시 계산한다. 자기를 읽거나 서로 읽는 computed는 같은 스레드에서 `get()`으로 돌아와 이미 쥔 mutex를 다시 잠갔다. 스레드가 영원히 멈췄다 | 스레드가 지금 계산 중인 computed를 스레드 지역 목록에 둔다. 그중 하나에 `get()`하면 "Circular dependency"로 패닉한다(`# Panics`에 문서화) | `reactive::computed::tests::a_cycle_panics_instead_of_deadlocking` |
| `reactive batch-queue-in-flush` | `end_batch`가 `BATCH_DEPTH`의 `borrow_mut()`을 쥔 채 큐의 업데이트를 돌렸다. 다른 업데이트를 큐에 넣거나 batch를 시작하거나 `is_batching()`을 묻는 업데이트가 같은 `RefCell`을 빌리려다 "already mutably borrowed"로 패닉했다 | 깊이를 줄이고 빌림을 놓은 뒤 flush한다 | `reactive::batch::tests::a_flushed_update_can_queue_another` |
| `reactive batch-panics` | `batch()`는 클로저가 패닉하면 `end_batch`를 건너뛰었다. 스레드의 batch 깊이가 1에 영원히 남아, 그 뒤의 모든 `queue_update`가 오지 않을 flush로 미뤄졌다 | 클로저를 `BatchGuard` 아래에서 돌린다. 풀림 중에 drop되는 가드는 flush하지 않고 batch를 떠난다(flush한 업데이트가 또 패닉하면 프로세스가 abort한다). 끝나지 못한 가장 바깥 batch는 큐의 업데이트를 버린다 | `reactive::batch::tests::a_panicking_batch_still_ends` |
| `reactive use-async-stale` | `use_async`의 트리거마다 실행이 하나씩 돌아 끝나면 결과를 썼다. 두 번 트리거하면, 늦게 끝난 첫 실행이 더 새 결과를 옛 결과로 덮었다 | 실행에 번호를 매긴다. 여전히 최신인 실행만 결과를 쓴다. 확인과 쓰기는 시그널의 잠금 아래에서 한다(`use_async_poll`은 poll 상태의 잠금) | `reactive::async_state::tests::a_superseded_run_does_not_overwrite_the_newer_result`, `a_superseded_polled_run_is_not_reported` |
| `dispatch on-in-handler` | `EventDispatcher::dispatch`가 핸들러 표의 읽기 잠금을 쥔 채 핸들러를 불렀다. (표를 공유하는 복제본으로) 핸들러를 등록하거나 지우는 핸들러가 같은 스레드에서 쓰기 잠금을 청해 교착했다 | 그 이벤트 형식의 핸들러(이제 공유 `Arc`)를 복사해 두고 잠금을 푼 뒤 부른다. dispatch 중에 더하거나 지운 핸들러는 다음 dispatch부터 적용된다 | `tests/event/custom.rs::a_handler_can_register_and_remove_handlers_while_dispatched` |
| `plugin init-error`, `plugin mount-error` | `PluginRegistry::mount`는 `on_mount`가 실패한 첫 플러그인에서 돌아가며 `mounted`를 거짓으로 두었다. 그 앞에서 mount된 플러그인은 mount된 채였지만 `unmount()`는 `mounted == false`를 보고 바로 돌아갔다. 그래서 끝내 unmount되지 않았다. `App`은 mount 실패를 로그로 남기고 계속 도므로 이것이 평범한 길이다. 다시 부른 `init()`도 이미 초기화된 플러그인의 `on_init`을 또 불렀다 | init과 mount가 몇 번째 플러그인까지 갔는지 센다. 다시 부르면 실패한 플러그인부터 잇고, `unmount`는 mount된 것만 정확히 unmount한다. 실패한 플러그인이 컨텍스트의 현재 플러그인으로 남던 것도 지운다 | `plugin::registry::tests::a_failed_mount_leaves_the_mounted_plugins_to_unmount`, `a_retried_init_does_not_initialize_a_plugin_twice` |

### 남긴 것 — 설계 결정이 필요하다

두 원인은 고치지 않고 `KNOWN`에 남겼다. 어느 쪽이 맞는지는 API의 약속을 정하는
문제이기 때문이다.

| 케이스 | 지금 동작 | 정해야 할 것 |
|---|---|---|
| `runner drop-running`, `pool drop-running` | `TaskRunner`와 `WorkerPool`은 drop될 때 도는 태스크를 join한다(코드 주석에 "Wait for all threads to complete", "clean shutdown"이라고 의도가 적혀 있다). 끝나지 않는 태스크가 있으면 drop이 영원히 막힌다. 화면을 바꾸며 러너를 쥔 컴포넌트를 drop하면 UI 스레드가 네트워크 태스크가 끝날 때까지 멈춘다 | join을 지킬 것인가(drop 뒤에 태스크의 부수 효과가 남지 않는다), 아니면 형제들처럼 떼어 낼 것인가(`PooledTaskRunner`와 `WorkerHandle`은 drop에서 기다리지 않는다) |
| `plugin init-panic`, `plugin mount-panic`, `plugin tick-panic`, `plugin unmount-panic` | 플러그인 훅의 패닉은 `PluginRegistry`와 `App`을 지나 그대로 풀린다. 앱이 죽는다 | 플러그인 버그가 앱을 죽여야 하는가, 아니면 레지스트리가 패닉을 잡아 그 플러그인을 끄고 에러로 알려야 하는가 |

### 견딘 것

- `PooledTaskRunner`의 drop, `WorkerHandle`의 drop과 `join_timeout`은 도는 태스크를
  기다리지 않았다.
- `WorkerPool`은 꽉 찬 큐와 shutdown 뒤의 submit을 `false`로 알렸다.
- `Signal`은 `update` 중 패닉으로 오염된 값 잠금을 복구했고, 알림 중에 자기나 다른
  구독을 drop해도 교착하지 않았다. `Computed`도 패닉 뒤 다시 계산했다.
- `WorkerChannel`은 4 스레드가 동시에 보낸 8,000개를 모두 전달하고 수를 0으로 되돌렸다.
- 수렴하는 재진입(구독자·effect·`SignalVec` 구독자가 조건부로 자기 값을 바꾸기)은 모두
  제대로 끝났다.

### 남은 틈

- **`if let Ok(..) = lock()`으로 오염을 조용히 건너뛰는 잠금이 더 있다.** profiler,
  커스텀 이벤트 dispatcher·bus, `MockHttpBackend`, 드래그 컨텍스트다. 하지만 그 쓰기 잠금
  아래에서 사용자 코드가 돌지 않아 밖에서 오염시킬 수 없다. 그래서 바꾸지 않았다.
- **`WorkerChannel`에는 닫힘이 없다.** 받는 쪽이 drop되어도 보내기는 큐가 찰 때까지
  성공한다. 보내는 쪽이 drop되면 `recv`는 `None`이라 "아직 도는 중"과 구별되지 않는다.
  기능의 문제라 바꾸지 않았다.
- **`PluginRegistry::tick`은 mount되지 않은 플러그인도 돌린다.** mount가 실패한 뒤 `App`이
  계속 돌면 그렇다. 바꾸지 않았다.
- **알림 중에 drop한 구독의 콜백은 그 회차에 한 번 더 불릴 수 있다.** 콜백을 복사해 두고
  부르기 때문이다(`Signal`은 전부터, `SignalVec`과 `EventDispatcher`는 이번 수정부터).

## 동작이 바뀐 것

4d·4e 수정 가운데 공개 API의 동작이 눈에 띄게 바뀐 것은 다음과 같다. 시그니처는 바뀌지
않았다.

- `SystemClipboard`: 10초 안에 끝나지 않는 도구는 죽고 `CommandFailed`가 된다.
- 설정·CSS·이미지 로더: FIFO 같은 특수 파일도 한계를 넘으면 거부된다.
- `FilePicker::navigate_to`: 읽을 수 없는 디렉터리는 `Err(IoError)`이고 제자리에 머문다.
  UTF-8이 아닌 이름은 손실 변환되어 보인다.
- `HotReload::poll`: 디바운스 창 안의 반복이 창이 지난 뒤 나온다.
- `PooledTaskRunner::spawn_result`: 에러 메시지에 `Task error:` 접두어가 붙지 않는다
  (`TaskRunner::spawn_result`와 같다).
- `TaskRunner::cancel`: 취소한 태스크의 결과는 `poll`로 나오지 않는다.
- `WorkerHandle`: 런타임을 만들지 못하면 `Completed`가 아니라 `Failed`다.
- `WorkerSender::is_cancelled`: 한 번 참이면 계속 참이다.
- `Computed::get`: 순환 의존은 교착 대신 패닉한다.
- `BatchGuard`(그리고 `batch`): 패닉으로 풀리며 drop되면 flush하지 않는다.

## 빌드·설정 조합 — cargo 기능 조합과 런타임 스위치 조합

위의 층들은 한 가지 빌드, 한 가지 설정에서 입력을 흔든다. 이 층은 반대로 입력을 고정하고
**빌드와 설정**을 흔든다. 두 갈래다.

- **3a. cargo 기능 조합.** CI의 Feature Combinations 잡은 `--all-features`와
  `--no-default-features` 두 끝만 본다. 기능 하나만 켰을 때만 깨지는 경우는 보지 못한다.
  예를 들어 `cfg` 게이트가 빠졌거나, cfg 아래에서 import가 쓰이지 않거나, 어떤 기능이 다른
  기능을 말없이 필요로 하는 경우다.
- **3b. 런타임 스위치 조합.** `App`/`PipelineHarness`에는 프레임을 만드는 방식을 바꾸는
  스위치가 있다. `dom_from_render`, `css_layout`, `tab_navigation`, `incremental_dom`,
  `layout_engine`, 그리고 뷰 쪽의 `Stack::content_sized`다(3.0에서 모두 켜짐으로 바뀌었다).
  이 중 둘은 **출력을 바꾸면 안 된다.** `incremental_dom`은 정적인 뷰에서,
  `layout_engine`은 언제나 그렇다.

### 3a. cargo 기능 조합

`cargo-hack`으로 돌렸다. 기능은 19개(`default` 포함)이고, 그중 `full`, `std`, `gui`,
`all-gui`, `default`는 다른 기능을 묶은 프리셋이다.

| 명령 | 실행 수 | 로컬 시간(12코어) | 결과 |
|---|---|---|---|
| `cargo hack check --each-feature --no-dev-deps` | 21 | 2분 29초 | 경고·에러 0 |
| `cargo hack clippy --each-feature --no-dev-deps -- -D warnings` | 21 | 1분 47초 | 경고·에러 0 |
| `cargo hack check --feature-powerset --depth 2 --exclude-features full,std,gui,all-gui,default --no-dev-deps` | 106 | 7분 23초 | 경고·에러 0 |
| `cargo hack check --each-feature --all-targets` (테스트·예제 포함) | 21 | 4분 47초 | 경고·에러 0 |

**고칠 것이 없었다.** 기능 하나만 켜도, 두 개를 어떻게 짝지어도, 라이브러리와 테스트,
예제가 경고 없이 빌드된다. 이 상태가 깨지지 않게 하는 것이 남은 일이다.

`.github/workflows/features.yml`:

- **Each Feature** 잡: `cargo hack check --each-feature`와 `cargo hack clippy --each-feature`
  (`RUSTFLAGS=-Dwarnings`, `--keep-going`으로 실패한 기능을 모두 보고한다). 다음 경우에 돈다.
  - 매주 월요일
  - `workflow_dispatch`
  - `Cargo.toml`이나 이 워크플로를 건드린 PR
- **Feature Pairs** 잡: 깊이 2의 기능 멱집합(106번 빌드). 매주와 수동 실행에서만 돈다.
- `taiki-e/install-action@cargo-hack`은 저장소의 다른 `install-action` 사용
  (`@cargo-nextest`, `@cargo-audit`)과 같은 방식으로 고정했다.

**필수 CI Gate에 넣지 않았다.** `--each-feature`는 크레이트를 21번 빌드한다. 로컬
12코어에서 2분 반이었으니, 2코어 러너의 콜드 캐시에서는 5분을 넘는다. 그래서 기존
Feature Combinations 잡에 붙이지 않고 따로 두었다. 기능을 바꾸는 PR은 `Cargo.toml`을
건드리므로 그 PR에서는 여전히 돈다. 기능 게이트만 바뀌는 `src/` 변경은 주간 실행이 잡는다.

### 3b. 런타임 스위치 조합 — `tests/config_matrix.rs`

| 요인 | 값 |
|---|---|
| 화면 | 11개: 폼, 대시보드(카드 행 + 표), 스크롤 뷰 속 목록, CSS 여백이 있는 중첩 스택, `flex-wrap` 태그 행, `display: none`/`visibility: hidden`, 화면보다 큰 CSS 크기, 넓은 문자(한글·이모지·한자), 그리드 + 탭, 겹침(`Layers` + `Positioned`), 입력 컨트롤 |
| 크기 | 1×1, 7×3, 40×12, 120×40 |
| `dom_from_render` | 켬 / 끔 |
| `css_layout` | 켬 / 끔 |
| `content_sized` | 켬 / 끔 (화면이 만드는 모든 스택에 적용) |
| `tab_navigation` | 켬 / 끔 |
| 스타일시트 | 없음 / 화면의 것 / 화면의 것 + 프레임에 `overflow: hidden` |

**전체 곱을 돈다.** 2,112 케이스이고 디버그 빌드에서 약 6초 걸린다. 위젯 매트릭스의 all-pairs
생성기를 먼저 써 봤다. 44 케이스로 줄었지만, 전체 곱이 찾은 넘침 52건 중 3건만 잡았다. 이
크기에서는 전체 곱을 돌릴 만큼 싸다.

각 케이스는 두 프레임을 그리고, Tab을 두 번 보내고, 세 번째 프레임을 그린다. 불변식은
다음과 같다.

1. **패닉하지 않는다.**
2. **화면 밖에 쓰지 않는다.** 화면은 센티널 칸으로 된 테두리 한 칸 안쪽에 그린다. 테두리는
   모든 프레임에서 그대로 남아야 한다. 이 검사는 스타일시트가 없을 때와 `overflow: hidden`일
   때만 한다. 이유는 아래 "설계대로인 것"에 있다.
3. **바뀌지 않은 뷰를 다시 그리면 같은 화면이 나온다**(프레임 2 = 프레임 1).
4. **출력을 바꾸면 안 되는 스위치는 출력을 바꾸지 않는다.** 같은 케이스를
   `incremental_dom(true)`로 돌린 것, `layout_engine(false)`로 돌린 것 모두 세 프레임이
   칸 단위로(글자·색·modifier) 같아야 한다.

래칫은 다른 층과 같고, 목록(`tests/config_matrix.rs`의 `KNOWN`)은 지금 비어 있다.

```bash
cargo test --test config_matrix                        # 기본 기능
cargo test --no-default-features --test config_matrix  # 기능 없이
```

#### 발견하고 고친 것

| 원인 | 증상 | 고친 것 |
|---|---|---|
| `RenderContext`의 `draw_text*`/`draw_char*`가 `overflow: hidden` 클립을 보지 않았다(`set`과 `put_str`은 봤다). | 1×1 화면에서 equal-share 스택이 `Button`을 화면 아래 칸으로 밀어냈다. 버튼 배경(`set`)은 잘렸지만 라벨 `P`(`draw_text_bg`)는 `overflow: hidden` 상자 밖에 그려졌다(2 케이스). | `put_text_char`와 `draw_char*`가 클립을 확인한다. 칸 하나라도 클립 밖인 글리프는 그리지 않아서, 클립 경계에 걸린 넓은 문자가 반쪽만 남지 않는다. 단위 테스트 `render_context::tests::test_clip_text_primitives`. |

#### 설계대로인 것 — CSS 크기는 슬롯을 넘을 수 있다

스타일시트를 그대로 쓰면 52 케이스가 화면 밖에 쓴다. `dom+ layout+`에서
`.card { width: 20 }`, `.nav { width: 14 }`, `.tall { height: 3 }`,
`.huge { width: 500 }` 같은 크기가 컨테이너가 준 슬롯보다 크면, `box_model::apply`가 상자를
슬롯 밖으로 키운다. 그러면 위젯이 이웃이나 화면 가장자리 위에 그린다.

이것은 CSS의 기본값 `overflow: visible`이다. `tests/css_card_and_overflow.rs`의
`a_child_can_paint_outside_its_container`가 이 동작을 못박아 두었다. 이 동작이 없으면
`overflow: hidden`도 관찰할 수 없다. 슬롯으로 잘라내는 수정을 시험해 봤지만 그 테스트를
깨서 되돌렸다. 대신 매트릭스는 **넘친 것이 모두 `overflow: hidden` 상자 안에 갇히는지**를
본다. 그래서 세 번째 스타일시트 값이 있다. 위의 버그 하나를 고친 뒤 그 검사는 전부 통과한다.

남는 질문은 이것이다. 터미널에서는 넘친 칸이 갈 곳 없이 이웃을 덮어쓴다. 그런데도 기본값이
`visible`이어야 하는가? 바꾸면 동작이 바뀌는 일이라 여기서는 결정하지 않는다.

#### 견딘 것

- 2,112 케이스 모두에서 패닉이 없었다.
- `incremental_dom`과 `layout_engine`을 바꿔도 출력은 한 칸도 달라지지 않았다.
- 다시 그린 프레임은 언제나 같았다.
- `Positioned`를 화면 밖 좌표(118, 39)에 두어도, 크기가 1×1이어도, 화면 밖에 쓰지 않았다.

#### 남은 틈

- `RenderContext::get_mut`은 클립을 보지 않고 칸을 돌려준다. 이것으로 칸을 고치는 위젯은
  클립을 넘을 수 있다. 매트릭스의 화면에서는 나타나지 않았다.
- `ctx.buffer`에 직접 쓰는 위젯(예: `DevTools`)은 클립과 무관하다.
- 뷰가 프레임마다 바뀌는 경우는 보지 않는다. `incremental_dom`은 그때 출력을 바꾸는 것이
  목적이므로, 같음을 검사할 수 없다.
