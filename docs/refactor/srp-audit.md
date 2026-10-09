# 단일 책임 점검 (3.x)

파일 하나에는 바뀔 이유가 하나만 있어야 한다. 이 문서는 그 점검의 기준과 진행
상태다. 루프는 표에서 다음 `todo`를 골라 판단하고, 결과를 이 표에 적는다.

## 기준

**쪼갠다** — 한 파일에 서로 다른 이유로 바뀌는 코드가 섞여 있을 때.

- 위젯 한 파일에 타입·설정, 빌더, 상태 변경(편집·탐색·선택), 입력 처리(키·마우스),
  렌더가 함께 있고, 그중 둘 이상이 각각 수십 줄을 넘을 때.
- 서로 관계없는 공개 개념이 한 파일에 있을 때(예: 테마 정의와 내장 테마 데이터).
- 범용 도구와 특정 기능 전용 로직이 섞여 있을 때.

**그대로 둔다** — 크기가 하나의 책임에서 나올 때. 이유를 표에 적는다.

- 관련된 데이터 타입만 모은 `types.rs`.
- 알고리즘 하나(파서, 레이아웃 계산)가 길 뿐인 파일.
- 쪼개면 서로를 계속 넘나들어야 하는 코드.

줄 수와 공개 타입 수는 **후보를 고르는 신호**일 뿐 판단 기준이 아니다.

**나눌 때의 모양** — 가까운 위젯이 이미 쓰는 구성을 따른다. 큰 위젯은 디렉터리
하나에 `types.rs`, `core.rs`(구조체와 생성), `builders.rs`, `render.rs`(`View`),
입력(`handler.rs`/`key_handling.rs`, `mouse.rs`), 상태 변경(`navigation.rs`,
`editing.rs`, `selection.rs`)을 둔다(`code_editor`, `textarea`, `datagrid`, `input` 참고).

## 절차 (파일 하나마다)

1. 읽고 판단한다: **split / keep / defer**(설계 결정이 필요함).
2. split이면 **코드를 옮기기만 한다.** 동작은 바꾸지 않는다. 옮기다 버그를 찾으면
   고치지 않고 표의 비고에 적는다(따로 PR).
3. 공개 경로를 지킨다. 옮긴 타입은 원래 모듈에서 `pub use`한다.
4. 검사: `cargo fmt --check`, `typos`, clippy(all features, all targets, `-D warnings`),
   `--no-default-features` 빌드, `RUSTFLAGS="-D warnings" cargo test --all-features`,
   docsrs 문서 빌드(`--document-private-items` 포함; pre-push hook과 같은 플래그), 그리고 **`cargo semver-checks`**(crates.io의 최신 릴리스와
   비교)로 공개 API가 그대로인지 확인한다.
5. 커밋: `refactor(<scope>): split <file> by responsibility`. 파일 하나에 커밋 하나.
6. 이 표에 결과를 적는다. keep도 이유와 함께 기록한다.

PR은 아래 묶음마다 하나다(묶음 번호 순서로 진행). 파일마다 PR을 내면 CI만 수십 번 돈다.

## 후보

`src` 803개 파일 중 테스트를 뺀 줄이 600 이상이거나, 공개 타입이 5개 이상이거나,
400줄 이상에 공개 타입이 3개 이상인 95개. 테스트 파일은 뺐다.


### 1. runtime: style, render (8)

| 파일 | 줄 | 공개 타입 | 신호 | 판단 | 비고 |
|---|---:|---:|---|---|---|
| `runtime/render/batch.rs` | 511 | 3 |  | keep | 렌더 배치 하나: 연산 큐, 병합 최적화, 버퍼 적용, 통계가 모두 `RenderOp`를 중심으로 맞물림. 버그 의심: `optimize()`가 위치로 정렬해 `Clear`·커서 연산의 순서가 셀 쓰기와 바뀜 (→ 고침), `apply_to_buffer`의 Text는 `*x + offset`을 검사 없이 더함 (→ 이 PR에서 고침) |
| `runtime/render/image_protocol.rs` | 696 | 8 | 696줄, 공개 타입 8 | split | 감지·인코더·Sixel 알고리즘·터미널별 명령이 섞임 → image_protocol/{mod,protocol,encoder,sixel,kitty,iterm2}.rs |
| `runtime/style/error.rs` | 669 | 6 | 669줄, 공개 타입 6 | split | 오류 보고 타입과 "did you mean" 속성 목록·Levenshtein이 섞임 → error/{mod,suggest}.rs. 참고: `suggest_property`/`KNOWN_PROPERTIES`는 크레이트 안에서 쓰이지 않고 목록이 실제 지원 속성과 어긋남 |
| `runtime/style/parser/apply.rs` | 707 | 0 | 707줄 | split | 속성 적용, `var()` 치환, 애니메이션 선언→`@keyframes` 해석이 섞임 → parser/{apply,vars,animation}.rs. 참고: `resolve_animation`의 `found_shorthand`는 쓰이지 않음, `CubicBezier`는 ease_in_out으로 대체됨 |
| `runtime/style/parser/types.rs` | 103 | 5 | 타입 모음, 공개 타입 5 | keep | 파싱된 CSS 데이터 모델만 모음. `StyleSheet` 메서드는 얇은 조회·위임뿐 |
| `runtime/style/properties/types.rs` | 427 | 20 | 타입 모음, 공개 타입 20 | keep | CSS 속성 값 타입만 모음. impl은 생성자와 `CalcExpr` 계산 정도로 작음 |
| `runtime/style/theme.rs` | 747 | 8 | 747줄, 공개 타입 8 | split | 테마 정의, 색 세트, 내장 테마 데이터, 런타임 전환(매니저·리스너)이 섞임 → theme/{mod,palette,builtin,manager}.rs (`ThemeBuilder`는 테스트가 private 필드를 보므로 mod.rs에 둠). 참고: `Themes`(Rust)와 `themes::BuiltinTheme`(CSS) 두 내장 테마 체계가 따로 있음 |
| `runtime/style/transition.rs` | 504 | 5 | 공개 타입 5 | split | CSS 정의·파싱과 실행 중 전환 상태·매니저가 섞임 → transition/{mod,definition,manager}.rs (lerp는 mod.rs). 버그 의심: `Transition::parse("opacity 0.3s 0.1s")`는 첫 길이가 기본값 300ms와 같아 두 번째 값이 delay가 아닌 duration을 덮어씀 (→ 고침) |

### 2. runtime: event, layout, dom (13)

| 파일 | 줄 | 공개 타입 | 신호 | 판단 | 비고 |
|---|---:|---:|---|---|---|
| `runtime/dom/node/mod.rs` | 383 | 5 | 공개 타입 5 | keep | DOM 노드 데이터 모델 하나: `WidgetKey`·`WidgetMeta`·`NodeState`·`DomId`는 모두 `DomNode`의 필드이고 impl은 생성자·빌더·조회 정도로 작음 |
| `runtime/dom/pool/mod.rs` | 578 | 8 | 공개 타입 8 | split | 범용 객체·Vec 풀, 렌더 버퍼 전용 풀, 문자열 인터닝이 섞임 → pool/{mod,object,buffer,string,vec}.rs (`PoolStats`와 생성 함수는 mod.rs) |
| `runtime/dom/query.rs` | 683 | 3 | 683줄 | keep | 파일 이름과 달리 대부분 `DomTree` 하나: 노드 맵과 id·타입·클래스 색인을 추가·삭제·재조정하면서 함께 유지하고, `Query` 구현과 상태 갱신도 같은 맵·색인을 씀. 나누면 private 필드를 넘나들어야 함. 버그 의심: `remove()`는 루트를 지워도 `root`를 그대로 두고 (→ 고침), 남은 형제의 위치 상태(`first_child` 등)를 갱신하지 않음 (→ 고침). 참고: `query_one`은 HashMap 순회라 여러 개가 맞으면 문서 순서가 아닌 임의의 노드를 돌려줌 (→ 고침) |
| `runtime/dom/selector/types.rs` | 382 | 8 | 타입 모음, 공개 타입 8 | keep | 셀렉터 AST 타입만 모음. impl은 생성자·명시도·Display로 작음 |
| `runtime/event/custom/types.rs` | 212 | 6 | 타입 모음, 공개 타입 6 | keep | 사용자 이벤트의 트레이트·ID·우선순위·메타데이터·봉투 타입만 모음. impl은 생성자와 전파 플래그 정도 |
| `runtime/event/drag.rs` | 580 | 5 | 공개 타입 5 | split | 끌어 옮길 데이터, 상태·결과, 놓을 곳, 상태 기계, 전역 싱글턴이 섞임 → drag/{mod,data,state,target,context,global}.rs (`DragId`는 mod.rs). 버그 의심: `clear_targets()`는 `hovered_target`만 비우고 상태를 `OverTarget`으로 남김(`unregister_target`은 `Dragging`으로 되돌림) (→ 고침), `DragState::is_active` 문서는 Dragging·OverTarget이라 하지만 `Pending`도 포함 (→ 코드가 맞음: 테스트와 `is_dragging`·`end_drag`가 Pending을 활성으로 쓰므로 문서를 고침) |
| `runtime/event/focus.rs` | 534 | 4 |  | keep | 포커스 관리자 하나: Tab 순서, 2D 이동, 트랩(중첩 포함)이 모두 같은 위젯 목록·현재 인덱스·트랩 상태를 씀. `FocusTrap`은 그 위의 얇은 도우미(약 110줄). 참고: `FocusTrapConfig::loop_focus`는 저장만 되고 어디서도 읽지 않음(`next`/`prev`는 항상 순환) (→ 고침) |
| `runtime/event/gesture/recognizer.rs` | 607 | 1 | 607줄 | keep | 제스처 인식 상태 기계 하나: 설정, 핸들러 등록, 마우스 이벤트 처리, 발행이 같은 추적 상태와 핸들러 목록을 씀 |
| `runtime/event/gesture/types.rs` | 381 | 11 | 타입 모음, 공개 타입 11 | keep | 제스처 결과·방향·상태·설정 데이터 타입만 모음. impl은 delta 계산과 Default 정도. 참고: 이 타입들의 테스트는 gesture/mod.rs에 있음 |
| `runtime/event/ime.rs` | 654 | 8 | 654줄, 공개 타입 8 | split | 조합 데이터 타입·설정, 조합 상태 기계, 렌더용 preedit 조각이 섞임 → ime/{mod,types,state,preedit}.rs (길이 제한 상수는 state.rs). 참고: 빈 목록으로 `set_candidates`를 부르면 `Selecting` 상태가 그대로 남음 (→ 고침) |
| `runtime/event/mod.rs` | 349 | 5 | 공개 타입 5 | keep | 하위 모듈 선언·re-export와 기본 입력 이벤트 타입(`Event`, `KeyEvent`, `MouseEvent` 등)만 있음. impl은 생성자·판별 메서드로 작음. 참고: 모듈 문서의 `CustomEvent` 예제는 없는 `fn id(&self)`를 씀(실제는 `event_type()`, `ignore`라 컴파일되지 않음) |
| `runtime/layout/node.rs` | 215 | 8 | 공개 타입 8 | keep | 레이아웃 노드 데이터 모델만 모음(속성 묶음·간격·크기 제약·계산 결과). impl은 gap·여백 계산 정도로 작음 |
| `runtime/layout/responsive.rs` | 571 | 7 | 공개 타입 7 | split | 중단점·중단점별 값, 화면 기준 레이아웃·미디어 쿼리, 컨테이너 쿼리가 섞임 → responsive/{mod,breakpoint,viewport,container}.rs (도우미 `responsive::responsive` 모듈은 `ResponsiveLayout`의 private 필드를 읽으므로 viewport.rs 안에 두고 mod.rs에서 다시 내보냄). 참고: `Breakpoints::current`는 첫 중단점보다 좁은 폭에도 첫 중단점을 돌려줌(`simple()`이면 폭 10도 "sm") (→ 의도된 동작: 기존 테스트가 이 대체를 확인하고 반환형이 `&Breakpoint`라 문서에 적음) |

### 3. core, devtools, testing, query (13)

| 파일 | 줄 | 공개 타입 | 신호 | 판단 | 비고 |
|---|---:|---:|---|---|---|
| `core/app/hot_reload.rs` | 428 | 5 | 공개 타입 5 | split | 파일 감시·디바운스와 감시 경로 보안 검사(경로 탈출·null 바이트)가 섞임 → hot_reload/{mod,path}.rs. 버그 의심: 현재 디렉터리 검사가 문자열 접두어 비교라 `/proj`일 때 `/proj-evil`도 통과 (→ 이 PR에서 고침), `Error` 이벤트 키가 `now.elapsed()`(항상 0)라 오류끼리 디바운스됨, `poll`/`wait`/`wait_timeout`이 키 계산을 세 번 복사 (→ 고침) |
| `core/app/mod.rs` | 1038 | 1 | 1038줄 | split | `App` 상태·접근자, 이벤트 루프(실행·디스패치·hover/focus 추적·핫 리로드 확인), 그리기 파이프라인(DOM·레이아웃 트리·버퍼·터미널)이 섞임 → app/{mod,event_loop,draw}.rs. `handle_event`·`get_buffer_size`는 mod.rs 테스트가 부르므로 `pub(super)`. 테스트는 공통 픽스처(`create_test_app`) 때문에 mod.rs에 둠 |
| `core/app/profiler.rs` | 582 | 10 | 공개 타입 10 | keep | 프로파일링 하나: 샘플→통계→리포트·스냅샷 비교. 나누면 `Snapshot`이 `Profiler`의 private `metrics`를, 리포트가 `Stats::format_duration`을 넘나들어야 함. 참고: 프로파일러가 `core::app::profiler`, `devtools::profiler`, `utils::profiler` 셋(prelude의 `Profiler`·`Stats`는 utils 쪽). `FpsCounter::fps()`는 프레임 수를 창 길이로 나눠 창이 차기 전에는 낮게 나옴 |
| `core/app/router.rs` | 556 | 5 | 공개 타입 5 | keep | 경로 매칭과 history 탐색 하나. `Route`·`HistoryEntry`·`NavigationEvent`는 `Router`가 다루는 데이터, 질의 문자열 파싱은 작은 private 함수. 버그 의심: `*` 와일드카드는 문서("matches rest")와 달리 세그먼트 하나만 맞음(세그먼트 수가 같아야 함) (→ 고침), `go()`는 가드·리스너를 거치지 않음 (→ 고침: `back`·`forward`처럼 리스너에 알림. 가드는 `back`·`forward`도 거치지 않으므로 그대로). 참고: `declarative_router`, `patterns::NavigationState`와 라우팅 개념이 겹침 |
| `core/app/screen/types.rs` | 240 | 7 | 타입 모음, 공개 타입 7 | keep | 화면 데이터 타입(`ScreenId`·`Transition`·`ScreenMode`·`ScreenEvent`·`ScreenResult`·`ScreenConfig`)과 `Screen` 트레이트만 모음. 디렉터리가 이미 core/state/types로 나뉨 |
| `devtools/mod.rs` | 527 | 4 |  | split | 모듈 선언·재노출에 패널 설정 타입(위치·설정·탭), `DevTools` 상태·영역 계산, 패널 그리기가 함께 있음 → devtools/{mod,types,render}.rs (`DevTools`는 테스트가 private `config`를 보므로 mod.rs에 둠). 버그 의심: 패널 폭이 2 미만이거나 높이가 0이면(`size` 0, 아주 작은 화면) `render_panel`의 `area.width - 2`와 `draw_border`의 `area.height - 1`이 넘침 (→ 이 PR에서 고침), 높이 2인 패널은 탭 구분선을 자기 영역 밖에 그림(높이 3에서는 아래 테두리를 덮음), 오버레이 `panel_rect`의 `area.width * 2`가 폭 32767 초과에서 넘침 (→ 이 PR에서 고침) |
| `devtools/profiler/types.rs` | 233 | 5 | 타입 모음, 공개 타입 5 | keep | 렌더 프로파일러의 데이터 타입만 모음. impl은 라벨·생성자·누적 정도로 작음 |
| `devtools/time_travel/debugger.rs` | 615 | 1 | 615줄 | split | 기록·탐색·diff·내보내기 상태와 탭 그리기(타임라인·diff·액션·상태 뷰)가 섞임 → debugger/{mod,render}.rs (render는 자식 모듈이라 private 필드를 그대로 봄). 버그 의심: `export()`가 label·action 이름의 따옴표를 escape하지 않아 JSON이 깨질 수 있음 (→ 고침), `import()`는 "exported data"라 하지만 `export()` 문자열이 아닌 `Vec<StateSnapshot>`를 받음 |
| `devtools/time_travel/types.rs` | 302 | 6 | 타입 모음, 공개 타입 6 | keep | 스냅샷·값·diff·액션·뷰·설정 타입만 모음. 가장 큰 impl은 `StateSnapshot::diff` 하나 |
| `lib.rs` | 855 | 1 | 855줄 | split | 크레이트 루트(문서·모듈 선언·`Error`)와 prelude 재노출 목록(위젯·API가 늘 때마다 바뀜)이 섞임 → prelude 본문을 `src/prelude.rs`로. `revue::prelude` 경로와 문서는 그대로(문서는 lib.rs의 `pub mod prelude;` 선언에 남김; 생성된 rustdoc 페이지는 Source 링크만 다름). 버그 의심: "Error Handling Guidelines" `///` 블록이 아무 항목에도 붙지 않고 prelude 문서 앞에 이어 붙어, `revue::prelude` 문서가 오류 처리 지침으로 시작함 |
| `query/mod.rs` | 489 | 7 | 공개 타입 7 | keep | 질의 모델과 평가 하나(값 비교→필터→정렬·페이지), 파서는 이미 parser.rs. 나누면 `Filter`·`Query`·`QueryValue`가 서로를 계속 부름. 버그 의심: `Query::is_empty()`는 `offset`을 보지 않음 (→ 고침), 문자열 `Eq`·`Contains`는 대소문자를 무시하지만 `Gt`·`Lt`와 정렬은 구분함 |
| `testing/assertions.rs` | 261 | 7 | 공개 타입 7 | keep | 버퍼 단언 하나. 공개 타입 7 중 `AssertionResult`·`Assertion` 외 다섯은 `#[cfg(test)]` 전용 |
| `testing/visual/types.rs` | 273 | 6 | 타입 모음, 공개 타입 6 | keep | 시각 회귀 테스트의 설정·결과·캡처·diff 타입만 모음. impl은 빌더, 셀 비교, diff 요약 정도 |

### 4. state, utils, text, a11y (12)

| 파일 | 줄 | 공개 타입 | 신호 | 판단 | 비고 |
|---|---:|---:|---|---|---|
| `a11y/backend/platform.rs` | 307 | 6 | 공개 타입 6 | keep | `ScreenReader` 구현체 모음: 플랫폼별 백엔드 셋과 기록·무동작 백엔드가 각각 50줄 안팎이고 감지·전역·설정은 이미 backend/의 다른 파일에 있음. 참고: `LoggingBackend::announcements()`가 돌려주는 `LoggedAnnouncement`는 다시 내보내지 않아 크레이트 밖에서 이름을 쓸 수 없음, `MacOSBackend` 등은 `pub`이지만 닿을 경로가 없음 |
| `a11y/tree.rs` | 521 | 3 |  | keep | 접근성 트리 데이터 모델 하나: `TreeNode`, 그것을 담는 `AccessibilityTree`(노드 맵·루트·포커스), 그 위의 얇은 빌더. 버그 의심: `focus_next`/`focus_prev`는 HashMap 순회 순서를 쓰므로 Tab 순서가 문서 순서가 아닌 임의 순서 (→ 고침), `remove_node`로 루트를 지워도 `root`는 그대로 (→ 고침) |
| `state/reactive/context.rs` | 530 | 4 |  | keep | 컨텍스트 API 하나: `Context`/`Provider`와 provide·use·scope 함수가 모두 같은 thread-local 저장소(전역 맵·스코프 스택)를 씀. 참고: `Provider::new`는 값을 저장소에 등록하지 않아 `use_context`로 보이지 않음 (→ 고침), `ContextScope`는 drop 순서와 상관없이 맨 위 스코프를 꺼냄 (→ 고침) |
| `state/reactive/store/mod.rs` | 181 | 5 | 공개 타입 5 | keep | 테스트를 뺀 약 180줄에 스토어 ID·트레이트·확장 트레이트·레지스트리만 있음. 사용 도우미는 이미 usage.rs. 참고: `StoreExt::subscribe`는 자리표시자라 아무것도 구독하지 않음 |
| `state/worker/channel.rs` | 324 | 5 | 공개 타입 5 | keep | 양방향 채널 하나: 메시지·명령 타입과 채널, 송신·수신 반쪽이 모두 private `ChannelInner`를 나눠 씀. 참고: `WorkerReceiver::send_command`는 용량을 검사하지 않음(`WorkerChannel::send_command`는 검사) (→ 고침), `WorkerSender::send`는 넘칠 때 경고를 남기지 않음 |
| `text/bidi/types.rs` | 444 | 8 | 타입 모음, 공개 타입 8 | keep | BiDi 데이터 타입만 모음(방향, 문자 분류, run, 분석 결과, 설정, 정렬). 가장 큰 impl은 `BidiClass::of`의 문자 범위 표. 참고: `BidiInfo::new`는 run을 계산하지 않는 자리표시자라 `runs`가 늘 비어 `is_pure_rtl()`이 늘 true, `visual_text()`는 원문 그대로 (→ 고침) |
| `utils/border/mod.rs` | 568 | 5 | 공개 타입 5 | split | 테두리 문자·스타일·그리기와 테두리 제목(위치·변·제목 타입과 그리기)이 섞임 → border/{mod,title}.rs. 참고: 왼쪽·오른쪽 변 제목은 표시 폭이 아닌 `chars().count()`로 길이를 잼(위·아래 변은 표시 폭) (→ 고침), `offset` 적용은 `i16`으로 바꿔 더해 32767을 넘는 좌표에서 넘침 (→ 이 PR에서 고침) |
| `utils/clipboard.rs` | 503 | 6 | 공개 타입 6 | split | 오류·백엔드 트레이트·`Clipboard`와 플랫폼 명령을 찾아 실행하는 시스템 백엔드, 메모리 백엔드, 앱 안 복사 기록(`ClipboardHistory`)이 섞임 → clipboard/{mod,system,memory,history}.rs. 참고: `SystemClipboard::set`은 내용을 정리(ANSI·제어 문자 제거)하지만 `MemoryClipboard::set`은 그대로 저장함 (→ 고침) |
| `utils/diff.rs` | 456 | 3 |  | keep | LCS 기반 텍스트 비교 알고리즘 하나와 그 결과 타입·통계·unified 형식 출력. 버그 의심: 입력이 클 때 쓰는 `simplified_diff`는 b 쪽 위치가 거꾸로 가는 짝을 돌려줘 결과 diff가 틀릴 수 있음 (→ 고침) |
| `utils/i18n.rs` | 427 | 4 |  | keep | 번역 조회 하나: `Locale`(내장 로케일 생성자는 몇 줄짜리 복수형 규칙뿐), `Translation`, `I18n` 저장소. 참고: `t_plural`은 현재 로케일의 복수형 번호를 대체 로케일 번역에도 그대로 씀 (→ 고침) |
| `utils/keymap.rs` | 469 | 4 |  | split | 모드·키 묶음과 묶음 조회 상태(`KeymapConfig`), 키 문자열 파싱·표시, 내장 Vim·Emacs 프리셋 데이터가 섞임 → keymap/{mod,parse,presets}.rs. 버그 의심: `s-` 수식어를 먼저 떼므로 `parse_key`의 `"s-tab"`(BackTab) 별칭에 닿지 않음(`S-Tab`은 Shift+Tab) (→ 고침), 전역 묶음은 접두사 대기를 하지 않아 여러 키 전역 묶음은 맞을 수 없음 (→ 고침), `chord_timeout`은 저장만 되고 읽히지 않음 |
| `utils/profiler.rs` | 472 | 5 | 공개 타입 5 | keep | 프로파일러 하나: 타이밍·통계·RAII 가드·보고서가 모두 private `ProfilerInner`를 씀. `FlameNode`는 작은 독립 타입(약 60줄). 버그 의심: `report()`의 `&name[..27]`은 바이트로 잘라 30바이트 넘는 비ASCII 이름에서 panic (→ 이 PR에서 고침), `stack`은 아무도 push하지 않아 `Timing::parent`는 늘 None, `FlameNode`는 크레이트 안에서 만들지 않음 |

### 5. widget: data (11)

| 파일 | 줄 | 공개 타입 | 신호 | 판단 | 비고 |
|---|---:|---:|---|---|---|
| `widget/data/chart/candlechart.rs` | 630 | 3 | 630줄, 위젯 한 파일 | split | `Candle`·`ChartStyle` 데이터 타입, 빌더, 가격 축척·캔들 열 그리기·Heikin-Ashi 변환과 `View`가 섞임 → candlechart/{mod,types,render}.rs (테스트는 각 코드와 함께 옮김). 버그 의심: `scroll(offset)`이 데이터 길이보다 크면 `visible_candles`(와 Heikin-Ashi 경로)의 `data[start..end]`가 `start > end`로 패닉함 (→ 이 PR에서 고침). 참고: Heikin-Ashi 모드도 축 범위는 원본 캔들의 고저가로 잡음(변환된 캔들은 잘릴 수 있음) (→ 고침) |
| `widget/data/chart/helper.rs` | 898 | 1 | 898줄, 위젯 한 파일 | split | 파일 이름과 달리 `Chart` 위젯 전체: 빌더, 범위·축 라벨·선 그리기와 `View`, 범용 선분 클리핑(Liang-Barsky)·래스터화(Bresenham)가 섞임 → helper/{mod,render,geometry}.rs. geometry의 타입 별칭·함수 6개는 render.rs가 쓰므로 `pub(super)`(helper 안으로만). `LineSegment`의 `pub(super)`는 render.rs로 옮겨 helper 안으로 좁아짐(밖에서 쓰는 곳 없음) |
| `widget/data/chart/histogram/mod.rs` | 623 | 1 | 623줄, 위젯 한 파일 | split | 빌더·통계 접근자와 막대·통계선·축 그리기(`View`)가 섞임 → histogram/{mod,render}.rs (boxplot과 같은 모양). `max_value`·`bin_value`는 그리기에서만 쓰여 render.rs로. 렌더 테스트도 render.rs로 옮기며 `BinConfig`·`ChartGrid` import를 테스트 모듈에 추가 |
| `widget/data/chart/piechart.rs` | 442 | 4 | 위젯 한 파일 | split | 경계선: 442줄이지만 슬라이스·스타일 타입(약 55줄), 빌더(약 115줄), 원 그리기·라벨·범례(`View`, 약 210줄)가 각각 수십 줄을 넘음 → piechart/{mod,types,render}.rs (heatmap과 같은 모양) |
| `widget/data/chart/timeseries/types.rs` | 197 | 7 | 타입 모음, 공개 타입 7 | keep | 시계열 데이터 타입만 모음(점·계열·선 스타일·시간 형식·범위·마커·마커 스타일). impl은 생성자와 빌더 몇 줄뿐이고 위젯 본체·그리기는 이미 mod.rs·view.rs |
| `widget/data/chart/waveline.rs` | 544 | 3 | 위젯 한 파일 | split | 스타일·보간 타입, 빌더, 색 그라데이션·보간과 `View`(약 265줄), 위젯과 무관한 데모용 파형 데이터 생성 함수가 섞임 → waveline/{mod,types,render,generators}.rs (사전 설정 생성 함수 `audio_waveform` 등은 mod.rs) |
| `widget/data/datagrid/render.rs` | 603 | 0 | 603줄, 위젯 한 파일 | split | 이미 나뉜 위젯의 render.rs지만 셀 그리기와 열 배치 계산(표시 순서·행 번호 여백·고정/가로 스크롤 열 슬롯, 약 140줄)이 섞임. 열 배치는 mouse.rs·reorder.rs·width.rs도 씀 → datagrid/layout.rs로 옮김(이미 `pub(super)`라 가시성 그대로) |
| `widget/data/json_viewer/view.rs` | 656 | 1 | 656줄, 위젯 한 파일 | split | `JsonViewer` 구조체·빌더·조회, 선택 이동·펼침/접기, `Search` 구현, 그리기(`View`, 약 225줄)가 섞임 → json_viewer/view/{mod,navigation,search_impl,render}.rs. 새 파일은 view의 자식이라 private 필드·`get_visible_nodes`를 그대로 봄(가시성 변경 없음). `parse`가 부르는 `clear_search` 때문에 mod.rs에 `Search` import 유지. 참고: `ensure_visible`은 빈 함수("Handled during render") |
| `widget/data/log_viewer/view.rs` | 908 | 1 | 908줄, 위젯 한 파일 | split | `LogViewer` 구조체·적재·빌더·조회·내보내기, 검색, 북마크·점프·스크롤·선택, 키 처리, 그리기(`View`, 약 220줄)가 섞임 → log_viewer/view/{mod,search,navigation,handler,render}.rs (자식 모듈이라 private 필드를 그대로 봄; 공용 `filtered_entries`·`ensure_visible`은 mod.rs). `update_search`는 mod.rs의 `load`/`push`도 부르므로 `pub(super)`(view 안으로만). 버그 의심: `update_search`가 찾은 위치 다음 바이트(`actual_start + 1`)부터 다시 잘라 `msg_lower[start..]`가 멀티바이트 문자(예: 한글) 경계가 아니면 패닉함, 일치 끝을 원래 질의의 바이트 길이로 잡아 소문자 변환으로 길이가 바뀌면 범위가 어긋남 (→ 이 PR에서 고침) |
| `widget/data/timeline.rs` | 556 | 5 | 공개 타입 5, 위젯 한 파일 | split | 이벤트·이벤트 종류·방향·스타일 타입(약 145줄), 빌더·선택 상태, 세로·가로 그리기(`View`, 약 220줄)가 섞임 → timeline/{mod,types,render}.rs. 렌더 테스트는 render.rs로(테스트 모듈에 `TimelineEvent` import 추가), `test_clear`와 모듈 밖 `#[test] test_timeline_render_private`는 mod.rs에 둠(mod.rs 테스트의 쓰지 않게 된 `Rect`·`Buffer` import는 뺌). 참고: `test_timeline_render_private`는 `#[cfg(test)]` 모듈 밖에 있고 아무것도 검사하지 않음 |
| `widget/data/timer/mod.rs` | 692 | 4 | 692줄, 위젯 한 파일 | split | 서로 다른 위젯 두 개(`Timer` 약 330줄, `Stopwatch` 약 230줄)가 한 파일에 있음 → timer/{mod,countdown,stopwatch}.rs (`timer/timer.rs`는 clippy `module_inception`이라 countdown). 공용 `TimerState`·`TimerFormat`·`format_ms`·`render_large_time`과 생성 함수는 mod.rs(자식이 private 함수를 그대로 씀), 테스트는 위젯별로 옮김, `pub use`로 경로 유지. docs/guides/constructor-patterns.md의 경로 갱신. 나누기 전에: `widgets_read_css_ratchet`이 파일당 첫 `impl_view_meta!`만 봐서 가려져 있던 "Stopwatch가 CSS를 읽지 않음"을 고침 — 스캐너가 파일의 모든 위젯을 보게 하고(한 파일에 여럿이면 그 위젯 자신의 `impl` 블록만 셈) Stopwatch의 시간 표시가 `color`를 읽게 함. 비고: `Timer::format_remaining`이 `format_ms`와 출력이 다르고 Precise에서 분을 빠뜨리던 것(65.5s → "05.500")은 고침 — Compact(카운트다운은 "1h 23m"으로 더 거침) 외에는 `format_ms`를 그대로 써서 Stopwatch와 같은 글자("01:05.500")를 냄. 남은 것: `format_ms`의 Precise는 시를 버린다(1h 1m 5.5s → "01:05.500"). |

### 6. widget: developer (7)

| 파일 | 줄 | 공개 타입 | 신호 | 판단 | 비고 |
|---|---:|---:|---|---|---|
| `widget/developer/aistream.rs` | 481 | 4 | 위젯 한 파일 | split | 설정 타입, 구조체·빌더, 스트리밍 상태 변경(추가·완료·일시정지·타이핑 애니메이션 `tick`), 렌더가 한 파일에 섞임 → aistream/{mod,types,stream,render}.rs (구조체·빌더·생성 함수는 mod.rs). 참고: `markdown()`이 켜는 `render_markdown`은 어디서도 읽지 않아, 모듈 문서의 "markdown rendering, code block syntax highlighting"이 구현되지 않음 |
| `widget/developer/code_editor/types.rs` | 108 | 5 | 타입 모음, 공개 타입 5 | keep | 코드 편집기의 데이터 타입(괄호 쌍·매치, 들여쓰기, 설정, 되돌리기 연산)만 모음. impl은 `EditorConfig::default` 하나. 참고: `EditorConfig::show_whitespace`·`word_wrap`은 어디서도 읽지 않음 |
| `widget/developer/diff.rs` | 567 | 5 | 공개 타입 5, 위젯 한 파일 | split | 모드·줄·변경 종류·색 타입, 구조체·빌더·diff 계산, 분할·통합 뷰 그리기(약 240줄)가 섞임 → diff/{mod,types,render}.rs (구조체·빌더·`compute_diff`·생성 함수는 mod.rs, private `LineLayout`은 render.rs). 참고: `context()`가 정하는 `context_lines`는 어디서도 읽지 않음, `compute_diff`는 `ChangeType::Modified`를 만들지 않음, `DiffMode::Inline`은 통합 뷰와 같음(문자 단위 diff 없음), 모듈 문서의 "syntax highlighting"은 없음, 끝 주석의 테스트 경로(`tests/widget/developer/diff.rs`)는 실제로 `tests/widget/diff.rs` |
| `widget/developer/httpclient/types.rs` | 139 | 5 | 타입 모음, 공개 타입 5 | keep | HTTP 클라이언트의 데이터 타입(메서드·요청 상태·내용 형식·응답 보기·색)만 모음. impl은 메서드 이름·색, Content-Type 판별, 기본 색 정도로 작음. 디렉터리가 이미 backend·builder·client·render 등으로 나뉨 |
| `widget/developer/presentation.rs` | 682 | 4 | 682줄, 위젯 한 파일 | split | 설정 타입, 공개 `Slide`와 그 빌더, 구조체·빌더, 슬라이드 이동·조회·`tick`, 그리기(제목·본문 슬라이드, 전환 효과 `SlideFx`, 아래줄)가 한 파일에 섞임 → presentation/{mod,types,slide,navigation,render}.rs (`on_title_slide`는 이동과 그리기가 함께 쓰므로 mod.rs, `SlideFx`·`slide_fx`·`SLIDE_BG`/`SLIDE_FG`와 렌더 smoke 테스트는 render.rs). 참고: 제목 슬라이드에서도 아래줄은 "1/N"과 1/N만큼 찬 진행 막대를 그림 (→ 고침), `Slide::content` 문서의 "supports basic markdown"은 구현되지 않음 |
| `widget/developer/procmon.rs` | 606 | 5 | 606줄, 공개 타입 5, 위젯 한 파일 | split | 데이터·색 타입, 구조체·빌더, 시스템에서 프로세스 읽기·거르기·정렬, 선택 이동, 그리기(통계 줄·열 머리·행, 약 230줄)가 섞임 → procmon/{mod,types,refresh,navigation,render}.rs (`PROC_FG`·`format_bytes`는 render.rs, `ProcessInfo` 테스트는 types.rs, `refresh` 테스트는 refresh.rs). 버그 의심: `filter()`·`toggle_sort()`·`clear_filter()`는 다음 `refresh()`까지 목록에 반영되지 않고, `clear_filter()`는 선택을 되돌리지 않음; 전체 메모리가 0이면 MEM%·통계 줄이 NaN·inf. 참고: `view()`(`ProcessView`의 User·Tree)와 `show_cmd()`는 저장만 되고 어디서도 읽지 않음 |
| `widget/developer/vim.rs` | 646 | 5 | 646줄, 공개 타입 5 | split | 렌더 없는 상태 기계. 모드·이동·동작·명령 결과 타입(약 180줄)과 모드별 키 처리(약 320줄)가 상태·명령 해석 옆에 섞임 → vim/{mod,types,key_handling}.rs (구조체·접근자·`map`·`execute_command`·생성 함수는 mod.rs, 모드별 처리기와 `handle_key`, `handle_key`를 부르는 테스트는 key_handling.rs). 버그 의심: Normal 모드에서 `0`은 count 숫자로 먼저 잡혀 `LineStart`가 되지 않음 (→ 고침). 참고: `set_mode`는 Normal로 갈 때만 operator를 지워, `d` 다음 `i`로 Insert에 들어가면 operator가 남음(테스트 `test_set_mode_from_insert_clears_operator`는 이름과 달리 아무것도 확인하지 않음), `map()`의 `mappings`는 `handle_key`가 보지 않고 `last_action`은 기록만 됨(`.`은 `Repeat`만 돌려줌), `Replace`·`VisualBlock` 모드와 `FindChar`·`TillChar` 이동은 어떤 키로도 들어가지 않음 |

### 7. widget: display, feedback (11)

| 파일 | 줄 | 공개 타입 | 신호 | 판단 | 비고 |
|---|---:|---:|---|---|---|
| `widget/display/avatar.rs` | 543 | 3 | 위젯 한 파일 | split | 크기·모양 타입, 구조체·생성자·빌더(약 150줄), 크기·모양별 그리기(약 250줄)가 섞임 → avatar/{mod,types,render}.rs (이니셜·이름 기반 배경색을 구하는 `get_initials`·`get_bg_color`는 그리기만 쓰므로 render.rs). 버그 의심: `get_bg_color`의 `(hash % 360) as u8`이 256 이상인 색상값을 잘라 hue가 0..=255에 머물러 자홍 계열(h≥5)은 나오지 않음 (→ 고침). 참고: Large인데 높이가 3 미만이면 주석("Fall back to medium")과 달리 굵게 하지 않은 한 글자만 그림 |
| `widget/display/empty_state.rs` | 469 | 3 | 위젯 한 파일 | split | 상황·변형 타입(상황별 아이콘·강조색), 구조체·생성자·빌더와 높이 계산, 변형별 그리기(약 215줄)가 섞임 → empty_state/{mod,types,render}.rs (`height()`는 `measure`와 `render_full`이 함께 쓰므로 mod.rs, 그리기만 쓰는 `get_icon`은 render.rs). 참고: `EmptyStateVariant::Full` 문서는 "with border"라 하지만 테두리를 그리지 않음, Full의 아이콘은 `area.width / 2`에 놓여 두 칸짜리 이모지는 가운데보다 한 칸 오른쪽에 섬 |
| `widget/display/gauge.rs` | 684 | 3 | 684줄, 위젯 한 파일 | split | 스타일·라벨 위치 타입, 구조체·빌더·값 설정, 8가지 스타일 그리기(약 350줄)가 섞임 → gauge/{mod,types,render}.rs (그리기만 쓰는 `current_color`·`get_label`과 렌더 smoke 테스트는 render.rs, 빌더 테스트는 mod.rs). 버그 의심: `battery()`는 `thresholds(0.5, 0.2)`를 주지만 `thresholds`가 둘을 바꿔 warning 0.2·critical 0.5로 만들고 값이 "이상"일 때 색을 바꾸므로, 가득 찬 배터리(80%)가 빨강이고 낮은 배터리는 경고되지 않음 (→ 고침); `Arc`는 `progress <= value`라 값이 0이어도 첫 칸이 채워짐 (→ 고침). 참고: `label_position`은 `Inside`만 쓰여 Left·Right·Above·Below는 아무것도 그리지 않음, `border()`가 정하는 `border_color`는 어디서도 읽지 않음 |
| `widget/display/richlog.rs` | 623 | 4 | 623줄, 위젯 한 파일 | split | 레벨·항목·형식 타입(약 170줄), 구조체·빌더·기록, 스크롤·선택·키 처리, 그리기가 섞임 → richlog/{mod,types,navigation,render}.rs (`visible_entries`는 선택과 그리기가 함께 쓰므로 mod.rs, `LogEntry` 테스트는 types.rs). 버그 의심: `scroll`은 첫 줄로 보일 항목 번호인데 `scroll_to_bottom`(자동 스크롤 포함)이 `len - 1`로 두어, 자동 스크롤 중에는 마지막 항목 한 줄만 보임 (→ 고침); 선택 번호는 `visible_entries` 기준이지만 `toggle_selected`는 `entries`에서 찾음 (→ 고침). 참고: `format()`·`wrap()`은 저장만 되고 읽히지 않음, `show_labels`를 켤 빌더가 없음, `LogEntry`의 `details`·`expanded`는 그리지 않음, 테스트 주석의 `tests/widget/display/richlog.rs`는 실제로 `tests/widget/richlog.rs` |
| `widget/display/richtext.rs` | 540 | 3 | 위젯 한 파일 | split | 스팬 스타일·스팬 타입(약 195줄), 구조체·빌더, 마크업 파서(약 85줄), 그리기가 섞임 → richtext/{mod,types,parse,render}.rs (`RichText::markup`은 private `parse_markup`과 함께 parse.rs, 그리기만 쓰는 `Style::to_modifier`는 render.rs, 생성 함수 넷은 mod.rs). 버그 의심: `[`를 escape할 방법이 없어 글자 그대로의 `[`는 태그로 먹히고, 닫히지 않은 `[`는 남은 글 전체를 삼킴 (→ 고침: 닫히지 않은 `[`만, escape 문법은 설계 결정이라 남김). 참고: `markup` 문서의 태그 목록에 `reverse`·`black`·`on_*` 배경색이 빠짐, 모르는 태그는 조용히 버려짐 |
| `widget/display/status_indicator.rs` | 518 | 4 | 위젯 한 파일 | split | 상태·크기·표시 방식 타입(약 105줄), 구조체·생성자·빌더·펄스 프레임, 표시 방식별 그리기(약 155줄)가 섞임 → status_indicator/{mod,types,render}.rs (`get_label`은 `width()`와 그리기가 함께 쓰므로 mod.rs, 그리기만 쓰는 `is_visible`은 render.rs). 참고: Large `Dot`은 펄스로 점이 숨는 프레임에도 둘째 칸의 상태색 배경을 그대로 그림, `Badge`는 크기와 상관없이 늘 `●`을 씀 |
| `widget/feedback/alert.rs` | 570 | 3 | 위젯 한 파일 | split | 레벨·변형 타입(레벨별 아이콘·색), 구조체·생성자·빌더·닫기 상태·키 처리, 변형별 그리기(약 280줄)가 섞임 → alert/{mod,types,render}.rs (`height`·`handle_key`는 닫기 상태와 함께 mod.rs, 그리기만 쓰는 `get_icon`은 render.rs). 버그 의심: 닫을 수 있는 alert는 제목·메시지를 `×` 자리 앞에서 자르지 않아 긴 글이 `×`에 덮이거나 `×`가 글자를 덮음 (→ 고침). 참고: `render_outlined`는 받은 `border_color`(`_border_color`)를 쓰지 않음 |
| `widget/feedback/modal/mod.rs` | 618 | 3 | 618줄, 위젯 한 파일 | split | 버튼 설정 타입, 구조체·빌더·보이기·포커스 트랩·프리셋, 버튼 선택·키 처리(약 80줄), 그리기(약 165줄)와 테스트 약 640줄이 한 파일에 섞임 → modal/{mod,types,key_handling,render}.rs (`required_height`와 테스트용 getter는 mod.rs; 테스트는 대상 코드를 따라 types·key_handling·render로 나누고 빌더·보이기·포커스 트랩 테스트는 mod.rs). 버그 의심: 내용 줄이 있고 영역 높이가 2 이하면 `modal_height`가 0이 되어 `y + modal_height - 2`가 넘침(debug에서 panic; 기존 작은 영역 테스트는 내용이 없어 닿지 않음) (→ 이 PR에서 고침), 버튼 폭을 `label.len()`(바이트)로 재서 비ASCII 이름의 버튼 줄이 가운데에서 어긋남 (→ 고침). 참고: `buttons()`로 버튼을 바꿔도 `selected_button`을 범위 안으로 되돌리지 않음 |
| `widget/feedback/statusbar.rs` | 570 | 5 | 공개 타입 5, 위젯 한 파일 | split | 위치·정렬·구역·키 힌트 타입(약 105줄), 구조체·빌더·구역 갱신·테스트용 getter(약 230줄), 구역·키 힌트 그리기(약 195줄)가 섞임 → statusbar/{mod,types,render}.rs (`render_y`는 그리기와 `get_render_y`가 함께 쓰므로 mod.rs). 버그 의심: 오른쪽 구역이 영역보다 넓으면 `area.width - right_width`가, 한 줄 키 힌트에서는 `area.width - right_width - 2`가 넘침(debug에서 panic) (→ 이 PR에서 고침). 참고: `separator()`의 문자는 그리지 않고 한 칸 띄우기만 함, `SectionAlign`과 `StatusSection::priority`는 어디서도 읽지 않음 |
| `widget/feedback/toast_queue.rs` | 607 | 4 | 607줄, 위젯 한 파일 | split | 쌓는 방향·우선순위·항목 타입, 구조체·빌더, 큐 동작(넣기·중복 제거·tick·일시정지·마우스·닫기, 약 160줄), 배치·그리기(약 165줄)가 섞임 → toast_queue/{mod,types,queue,render}.rs (`tick`만 쓰는 `ToastEntry::is_expired`는 queue.rs, 그리기만 쓰는 `toast_height`·`calculate_base_position`은 render.rs). 버그 의심: `StackDirection::Up`은 기준 y에서 빼므로 위쪽 위치에서는 모든 토스트가 y=0에 겹치고, 아래쪽 위치에서는 전체 높이를 비워 둔 자리 위로 올라가 화면 밖으로 밀림 (→ 고침). 참고: `ToastPriority::Critical`의 "cannot be dismissed"와 `High`의 "shows immediately"는 구현되지 않음(닫기는 `dismissible`만 봄, 우선순위는 큐 순서만 바꿈), 모듈 문서 예제의 `QueuePosition`은 없는 타입(실제는 `ToastPosition`) |
| `widget/feedback/tooltip.rs` | 644 | 4 | 644줄, 위젯 한 파일 | split | 위치·화살표·스타일 타입, 구조체·빌더·보이기·지연·테스트용 getter, 크기·위치 계산(줄바꿈·치수·anchor 배치, 약 150줄), 오버레이 그리기(약 150줄)가 섞임 → tooltip/{mod,types,render}.rs. 크기·위치 계산(`wrap_text`·`calculate_dimensions`·`calculate_position`)은 그리기도 쓰므로 따로 떼지 않고 그 테스트와 함께 mod.rs에 둠(따로 두려면 `pub(super)`가 필요), 같은 이유로 `TooltipStyle`의 `colors`·`border_chars`도 mod.rs; 그리기만 쓰는 `TooltipArrow::chars`는 render.rs. 참고: `wrap_text`는 `max_width`보다 긴 한 낱말을 자르지 않아 상자가 `max_width`보다 넓어질 수 있음 |

### 8. widget: input, form, layout, traits, datetime_picker (12)

| 파일 | 줄 | 공개 타입 | 신호 | 판단 | 비고 |
|---|---:|---:|---|---|---|
| `widget/datetime_picker/types.rs` | 129 | 5 | 타입 모음, 공개 타입 5 | keep | DateTime 선택기의 데이터 타입(`Time`·`DateTime`·표시 형식·편집 모드·시간 필드)만 모음. impl은 생성자와 `Time`의 HH:MM 형식 정도로 작고, 위젯은 이미 mod·input·navigation·render·helpers로 나뉨. 참고: `Time::now()`는 문서대로 자리표시자라 늘 12:00:00을 돌려줌 |
| `widget/form/form.rs` | 655 | 4 | 655줄, 위젯 한 파일 | split | `Form`·`FormFieldWidget` 두 위젯의 타입·구조체·빌더·제출과 그리기(`Form`의 테두리·제목·필드 행·상태 줄 약 190줄, `FormFieldWidget` 약 160줄)가 섞임 → form/{mod,render}.rs (두 위젯의 타입·구조체·빌더·`Default`·생성 함수는 mod.rs, 그리기만 쓰는 `put_text`와 두 `View`는 render.rs; `Form`의 `View`를 먼저 두어 CSS ratchet이 보는 위젯은 그대로). 옮기며 render.rs의 `impl FormFieldWidget`에 원래 블록의 `#[allow(dead_code)]`를 한 줄 더 붙임. 버그 의심: `FormFieldWidget`은 `FormState`를 갖지 않아 값·오류를 그리지 않고(이름·placeholder·도움말만), 그것을 그리는 `render_label`·`render_value`·`render_helper_text`·`render_errors`는 `#[allow(dead_code)]`에 가려진 채 아무도 부르지 않음. 참고: `FormField`의 `View`는 CSS를 읽지 않았지만 ratchet은 파일의 첫 `impl_view_meta!`(`Form`)만 보아 놓침(이후 스캐너가 모든 위젯을 보게 되었고 `FormField`의 라벨이 `color`를 읽음); 모듈 문서 예제의 `Form::child`·`FormField::new(..).placeholder`와 form/mod.rs 문서의 `ErrorDisplayStyle::Tooltip`·`InputType::Tel`·`form().label(..)`은 없음, 제목은 늘 "Form" |
| `widget/form/masked_input.rs` | 787 | 3 | 787줄, 위젯 한 파일 | split | 마스크·검증 상태 타입, 구조체·빌더·접근자(약 260줄), 값 편집·커서 이동·peek 카운트다운, 비밀번호 강도·길이 검증, 마스킹과 그리기(약 190줄)가 섞임 → masked_input/{mod,types,editing,validation,render}.rs (편집·검증·마스킹이 함께 쓰는 `char_count`·`byte_at`과 CSS 클래스 도우미는 mod.rs, 공개 `masked_display`는 그리기와 함께 render.rs; `#![allow(clippy::iter_skip_next)]`는 원래 자리인 mod.rs에 두어 render.rs까지 덮음). 버그 의심: 키 처리(`handle_key`)가 없어 앱이 `insert_char` 등을 직접 불러야 함 (→ 고침), `set_value`는 `max_length`를 지키지 않음 (→ 고침), 포커스된 빈 입력은 placeholder를 회색이 아닌 글자색으로 그 위에 커서를 그림 (→ 고침). 참고: form/mod.rs 문서 예제는 없는 API를 씀(`password_input()`·`masked_input("(___) ___-____")`의 인자, `.mask(..)`, `pin_input().length(..)`) |
| `widget/input/input_widgets/radio.rs` | 402 | 3 | 위젯 한 파일 | split | 스타일·배치 타입, 구조체·빌더·선택 이동·키 처리(약 170줄), 옵션 그리기·크기 재기(약 165줄)가 섞임 → radio/{mod,render}.rs (`RadioStyle`의 private `chars`·`brackets`·`has_brackets`를 그리기가 쓰므로 타입은 types.rs로 떼지 않고 mod.rs에 둠, 그리기만 쓰는 `render_option`·`option_width`는 render.rs). 참고: 숫자 키 선택은 1–9만 되어 10번째 옵션부터는 숫자로 고를 수 없음, 선택 표시 색은 `selected_fg`가 없으면 CSS가 아닌 CYAN으로 고정 |
| `widget/input/input_widgets/selection_list.rs` | 563 | 3 | 위젯 한 파일 | split | 항목·표시 방식 타입, 구조체·빌더·조회, 선택 변경·강조 이동(약 110줄), 접두어·접미어와 그리기(약 145줄)가 섞임 → selection_list/{mod,types,selection,render}.rs (강조 이동만 쓰는 `ensure_visible`은 selection.rs, 그리기만 쓰는 `item_prefix`·`item_suffix`는 render.rs). 버그 의심: 키 처리가 없는데 포커스되면 "↑↓: Navigate | Space: Toggle | a: All | n: None" 도움말을 그림 (→ 고침), `selected()` 빌더는 범위·정렬·`max_selections`를 검사하지 않음 (→ 고침), `deselect_all`은 `min_selections`가 있으면 번호가 작은 것부터 남김 |
| `widget/input/input_widgets/slider.rs` | 617 | 3 | 617줄, 위젯 한 파일 | split | 방향·스타일 타입, 구조체·빌더, 값 조정·키 처리(약 90줄), 가로·세로 그리기와 크기 재기(약 290줄)가 섞임 → slider/{mod,types,render}.rs (값 계산과 그리기가 함께 쓰는 `clamp_value`·`normalized`는 mod.rs, 그리기와 `measure`만 쓰는 `format_value`는 render.rs). 참고: `handle_key`는 포커스가 없으면 키를 받지 않음(같은 묶음의 `RadioGroup`은 받음) |
| `widget/input/input_widgets/stepper.rs` | 559 | 5 | 공개 타입 5, 위젯 한 파일 | split | 공개 `Step`과 그 빌더·상태·방향·스타일 타입(약 110줄), 구조체·빌더, 단계 이동·상태 표시·진행률(약 90줄), 가로·세로 그리기(약 175줄)가 섞임 → stepper/{mod,types,navigation,render}.rs (`current()` 빌더도 부르는 `update_statuses`는 mod.rs, 그리기만 쓰는 `StepStatus::icon`·`Step::display_icon`·`step_color`는 render.rs). 버그 의심: 가로 그리기는 `StepperStyle::Numbered`로, 세로는 `show_numbers`(기본 켜짐)로 번호를 정해 같은 설정이 방향마다 다르게 보임(세로는 기본으로 상태 아이콘 대신 번호) (→ 고침) |
| `widget/layout/card/core.rs` | 727 | 1 | 727줄, 위젯 한 파일 | split | 디렉터리는 이미 core·types·helper로 나뉘었지만 core.rs에 구조체·빌더·접기 상태·키 처리(약 300줄)와 색·테두리 결정, 크기 재기, 그리기(약 400줄)가 함께 있음 → card/core.rs + card/core/render.rs (`Card`의 필드가 private이라 형제 파일 render.rs로는 가시성을 넓혀야 해서, core.rs의 자식 모듈로 둠; 그리기만 쓰는 `collapse_icon`·`footer_height`·`effective_colors*`·`border_type_with_css`·`TextDraw`는 render.rs). 참고: card/mod.rs의 테스트 다수는 아무것도 확인하지 않는 자리표시자(일부는 몸체가 주석뿐), `effective_colors_with_css` 문서는 요약 줄이 둘("Get effective colors…"가 남음) |
| `widget/layout/dock.rs` | 543 | 4 | 위젯 한 파일 | keep | 도킹 영역(`DockArea`)과 그 배치(`DockManager`)는 한 개념: `calculate_layout`이 `DockArea`의 private `ratio`·`min_size`·`collapsed`를 직접 읽어, 나누면 가시성을 넓혀야 함. 그리기는 각각 40줄 안팎. `DockManager`·`DockArea`는 크레이트 밖에서 닿지 않음(docs/refactor/findings-widget-clone.md) — 가시성·재노출은 그대로 둠. 버그 의심: `DockManager::render`는 영역을 `clone()`해 그리는데 `DockArea::clone`은 탭(위젯)을 비우므로 아무 내용도 그려지지 않음 (→ 고침); `active_tab`을 바꿀 방법이 없음. 참고: `min_width`·`min_height`·`max_width`·`max_height`(와 `min_dimensions`·`constrain` 등)는 저장만 되고 읽히지 않음, `to_pane`은 쓰이지 않음(`#![allow(dead_code)]`), 모듈 문서 예제의 `Panel`·`.tab(label, widget)`은 없음(`tab`은 라벨만, 위젯은 `tab_with`) | **(2026-10-09, #836: 다시 지음 - `SplitView`·`TabView` 위의 `Dock`/`DockState`. 위 버그는 새 코드에 없음.)**
| `widget/layout/splitter.rs` | 587 | 6 | 공개 타입 6, 위젯 한 파일 | split | 방향·구분선 스타일·`Pane` 타입, `Splitter`의 구조체·빌더·영역 계산·포커스·크기 조절·키 처리, 구분선 그리기, 그리고 따로 쓰는 두 칸 분할 위젯 `HSplit`·`VSplit`이 한 파일에 섞임 → splitter/{mod,types,render,two_pane}.rs (`pane_areas`는 공개 계산이라 mod.rs, 그리기만 쓰는 `SplitterStyle::char`와 그 테스트는 render.rs; 테스트는 대상을 따라 나눔). 버그 의심: `start_resize`·`resize`의 구분선 번호는 숨긴(collapsed) pane까지 세지만 그리기의 강조는 보이는 pane 사이 번호를 써서, 숨긴 pane이 있으면 강조되는 구분선과 실제로 조절되는 비율이 어긋남 (→ 고침). 참고: `HSplit`·`VSplit`은 구분선만 그리고 자식을 갖지 않으며 DOM 노드(`impl_view_meta!`)·props가 없음, `VSplit`에는 `HSplit`의 `min_widths`·`hide_splitter`에 해당하는 빌더가 없음, `Pane::min_size` 문서의 "percentage or absolute"와 달리 늘 칸 수 |
| `widget/layout/stack.rs` | 1051 | 2 | 1051줄, 위젯 한 파일 | split | 구조체·크기 지정 타입·빌더(긴 `content_sized` 문서 포함, 약 230줄)와 자식 배치(`View`의 render·measure·fills와 크기 계산 알고리즘, 약 800줄)가 섞임 → stack/{mod,sizing}.rs. 크기 계산(`measure_with`·`effective_sizes`·`content_size`·줄바꿈·`calculate_sizes`·`fit_content`·`AxisBox` 등)은 손대지 않고 sizing.rs 한 파일에 그대로 옮김. render가 private 크기 계산 함수를 부르므로 `View` impl도 sizing.rs에 둠(따로 두려면 `pub(super)`가 필요); `apply_constraints`는 mod.rs. 테스트는 빌더·제약 테스트는 mod.rs, `calculate_sizes`·렌더 테스트는 sizing.rs. 참고: features.yaml의 CSS-015 notes가 적은 대로 render의 "`gap: 0` reads as not specified" 주석은 동작과 다름 |
| `widget/traits/view.rs` | 743 | 6 | 743줄, 공개 타입 6, 위젯 한 파일 | split | `View` 트레이트(기본 메서드·`Box<dyn View>` 위임)와 `Fill` 옆에, 서로 관계없는 선택 트레이트 넷이 함께 있음: 입력 처리(`Interactive`), 켜고 끄는 위젯의 공통 동작(`ToggleWidget`), 끌어 놓기(`Draggable`), 실행 중 id·class 바꾸기(`StyledView`) → traits/{view,interactive,draggable,styled_view}.rs (`View`·`Fill`은 view.rs에 그대로; 공개 경로는 traits/mod.rs의 재노출로 그대로). view.rs의 모듈 문서 한 줄을 남은 내용에 맞게 고침. 참고: `ToggleWidget`만 `View`를 상위 트레이트로 두지 않음, `Draggable` 문서 예제의 `accepted_types(&self) -> &[&str]`은 실제 시그니처(`&[&'static str]`)와 다름 |

### 9. widget: 나머지 (canvas, markdown, mermaid, debug_overlay, 단일 파일 위젯) (8)

| 파일 | 줄 | 공개 타입 | 신호 | 판단 | 비고 |
|---|---:|---:|---|---|---|
| `widget/canvas/braille/shapes.rs` | 515 | 10 | 공개 타입 10 | keep | `Shape` 트레이트 하나와 그것을 구현하는 도형 타입들(선·원·호·다각형·사각형·점)만 모음. 각 타입은 생성자와 `draw` 정도(채운 다각형만 내부 판정 도우미가 하나 더)이고 서로 독립적이라 같은 이유로 바뀜 |
| `widget/debug_overlay/mod.rs` | 495 | 3 | 위젯 한 파일 | split | 패널 위치·설정 타입, 오버레이 구조체·빌더, 약 250줄의 패널 그리기(위치 계산, 지표·위젯 트리·이벤트, 테두리)가 섞임 → debug_overlay/{mod,types,render}.rs (전역 디버그 플래그는 25줄이라 mod.rs). 참고: `DebugConfig::show_styles`(스타일 검사기)와 `opacity`는 저장만 되고 읽히지 않음, 전역 `is_debug_enabled()`는 크레이트 안 어디서도 읽지 않음 |
| `widget/image.rs` | 476 | 4 | 위젯 한 파일 | split | 오류·크기 모드·픽셀 형식 타입, PNG·파일 디코딩과 원시 픽셀 생성(크기 제한 검사 포함, 약 180줄), Kitty 이스케이프 인코딩, 크기 계산·렌더가 섞임 → image/{mod,types,load,kitty}.rs (`rand_id`는 mod.rs). 버그 의심: `from_png`는 `with_guessed_format`으로 JPEG 등도 받지만 형식을 `Png`로 적어 Kitty에 `f=100`으로 보냄 (→ 고침: PNG가 아니면 RGBA 픽셀로 디코딩해 `Rgba`로 기록), 너비나 높이가 0이면(`from_rgb`로 가능) Fit/Fill 계산이 0으로 나눠 NaN·inf가 됨 (→ 고침). 참고: `ImageFormat`은 `pub`이고 `get_format()`이 돌려주지만 밖으로 다시 내보내지 않음 |
| `widget/markdown/mod.rs` | 898 | 2 | 898줄, 위젯 한 파일 | split | 설정·구조체·생성·빌더 옆에 pulldown-cmark 이벤트 처리(이벤트 루프, 태그 시작·끝, 텍스트·코드·HTML·각주 참조, 약 380줄)와 통째로 짜는 블록 배치(목차, FIGlet 제목, 코드 블록, 표, 약 200줄)가 섞임 → markdown/{mod,events,blocks}.rs (`extract_toc`는 `new`만 부르므로 mod.rs). mod.rs가 부르는 `parse_with_options`와 events.rs가 부르는 `render_toc`·`render_figlet_heading`·`render_code_block`·`render_table`은 `pub(super)`(실제 보이는 범위는 전과 같음). 참고: `Tag::Strikethrough` 시작은 아무것도 켜지 않는데 끝에서 `CROSSED_OUT`을 끔 (→ 고침) |
| `widget/markdown/types.rs` | 163 | 5 | 타입 모음, 공개 타입 5 | keep | 렌더된 줄 데이터 타입만 모음(`StyledText`·`Line`·`TocEntry`·`FootnoteDefinition`)과 콜아웃 종류 `AdmonitionType`. impl은 생성자·표식 해석·아이콘/색/이름 표 정도로 작음. 참고: `AdmonitionType` 테스트는 markdown/mod.rs에 있음 |
| `widget/mermaid/types.rs` | 175 | 7 | 타입 모음, 공개 타입 7 | keep | 다이어그램 데이터 모델만 모음(종류·방향·노드 모양·화살표·노드·간선·색). impl은 생성자·빌더·Default뿐 |
| `widget/option_list.rs` | 724 | 4 | 724줄, 위젯 한 파일 | split | 항목·옵션·구분선 타입(약 85줄), 구조체·빌더, 조회·선택·하이라이트 이동(약 155줄), 약 125줄의 렌더, 테스트용 getter가 한 파일에 섞임 → option_list/{mod,types,navigation,render}.rs (`separator_char`는 렌더와 테스트 getter가 함께 쓰므로 mod.rs). 버그 의심: 옵션이 없을 때 `highlight_first`는 `option_count() - 1`이 넘침(디버그 빌드는 패닉) (→ 이 PR에서 고침), 스크롤은 옵션만 건너뛰어 앞쪽 구분선·그룹 제목은 계속 그려짐 (→ 고침), 힌트 정렬이 `len()`(바이트)이라 한글·아이콘이 있으면 어긋남 (→ 고침). 참고: 모듈 문서는 키보드 탐색을 말하지만 키 처리기가 없고, 예제가 없는 `Option`을 import함 |
| `widget/qrcode.rs` | 438 | 3 | 위젯 한 파일 | split | 표시 방식·오류 정정 타입, 구조체·빌더·getter·QR 행렬 생성 옆에 네 가지 방식(반 블록·전체 블록·ASCII·점자)으로 그리는 약 200줄의 렌더가 섞임 → qrcode/{mod,types,render}.rs (`ErrorCorrection::to_ec_level`은 `get_matrix`만 쓰므로 mod.rs). 버그 의심: `inverted`는 `get_matrix`에서 모듈을 뒤집는데 반 블록·전체 블록 렌더는 전경·배경색도 맞바꿔 두 번 뒤집혀 효과가 사라짐(ASCII·점자는 한 번만 뒤집힘) (→ 고침: 모든 방식에서 행렬만 한 번, quiet zone까지 뒤집음). 참고: 구조체 문서 예제가 `QrCodeWidget` 대신 `QrCode::new`를 씀 |
