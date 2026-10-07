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
   docsrs 문서 빌드, 그리고 **`cargo semver-checks`**(crates.io의 최신 릴리스와
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
| `runtime/render/batch.rs` | 511 | 3 |  | keep | 렌더 배치 하나: 연산 큐, 병합 최적화, 버퍼 적용, 통계가 모두 `RenderOp`를 중심으로 맞물림. 버그 의심: `optimize()`가 위치로 정렬해 `Clear`·커서 연산의 순서가 셀 쓰기와 바뀜, `apply_to_buffer`의 Text는 `*x + offset`을 검사 없이 더함 |
| `runtime/render/image_protocol.rs` | 696 | 8 | 696줄, 공개 타입 8 | split | 감지·인코더·Sixel 알고리즘·터미널별 명령이 섞임 → image_protocol/{mod,protocol,encoder,sixel,kitty,iterm2}.rs |
| `runtime/style/error.rs` | 669 | 6 | 669줄, 공개 타입 6 | split | 오류 보고 타입과 "did you mean" 속성 목록·Levenshtein이 섞임 → error/{mod,suggest}.rs. 참고: `suggest_property`/`KNOWN_PROPERTIES`는 크레이트 안에서 쓰이지 않고 목록이 실제 지원 속성과 어긋남 |
| `runtime/style/parser/apply.rs` | 707 | 0 | 707줄 | split | 속성 적용, `var()` 치환, 애니메이션 선언→`@keyframes` 해석이 섞임 → parser/{apply,vars,animation}.rs. 참고: `resolve_animation`의 `found_shorthand`는 쓰이지 않음, `CubicBezier`는 ease_in_out으로 대체됨 |
| `runtime/style/parser/types.rs` | 103 | 5 | 타입 모음, 공개 타입 5 | keep | 파싱된 CSS 데이터 모델만 모음. `StyleSheet` 메서드는 얇은 조회·위임뿐 |
| `runtime/style/properties/types.rs` | 427 | 20 | 타입 모음, 공개 타입 20 | keep | CSS 속성 값 타입만 모음. impl은 생성자와 `CalcExpr` 계산 정도로 작음 |
| `runtime/style/theme.rs` | 747 | 8 | 747줄, 공개 타입 8 | split | 테마 정의, 색 세트, 내장 테마 데이터, 런타임 전환(매니저·리스너)이 섞임 → theme/{mod,palette,builtin,manager}.rs (`ThemeBuilder`는 테스트가 private 필드를 보므로 mod.rs에 둠). 참고: `Themes`(Rust)와 `themes::BuiltinTheme`(CSS) 두 내장 테마 체계가 따로 있음 |
| `runtime/style/transition.rs` | 504 | 5 | 공개 타입 5 | split | CSS 정의·파싱과 실행 중 전환 상태·매니저가 섞임 → transition/{mod,definition,manager}.rs (lerp는 mod.rs). 버그 의심: `Transition::parse("opacity 0.3s 0.1s")`는 첫 길이가 기본값 300ms와 같아 두 번째 값이 delay가 아닌 duration을 덮어씀 |

### 2. runtime: event, layout, dom (13)

| 파일 | 줄 | 공개 타입 | 신호 | 판단 | 비고 |
|---|---:|---:|---|---|---|
| `runtime/dom/node/mod.rs` | 383 | 5 | 공개 타입 5 | todo | |
| `runtime/dom/pool/mod.rs` | 578 | 8 | 공개 타입 8 | split | 범용 객체·Vec 풀, 렌더 버퍼 전용 풀, 문자열 인터닝이 섞임 → pool/{mod,object,buffer,string,vec}.rs (`PoolStats`와 생성 함수는 mod.rs) |
| `runtime/dom/query.rs` | 683 | 3 | 683줄 | todo | |
| `runtime/dom/selector/types.rs` | 382 | 8 | 타입 모음, 공개 타입 8 | todo | |
| `runtime/event/custom/types.rs` | 212 | 6 | 타입 모음, 공개 타입 6 | todo | |
| `runtime/event/drag.rs` | 580 | 5 | 공개 타입 5 | split | 끌어 옮길 데이터, 상태·결과, 놓을 곳, 상태 기계, 전역 싱글턴이 섞임 → drag/{mod,data,state,target,context,global}.rs (`DragId`는 mod.rs). 버그 의심: `clear_targets()`는 `hovered_target`만 비우고 상태를 `OverTarget`으로 남김(`unregister_target`은 `Dragging`으로 되돌림), `DragState::is_active` 문서는 Dragging·OverTarget이라 하지만 `Pending`도 포함 |
| `runtime/event/focus.rs` | 534 | 4 |  | todo | |
| `runtime/event/gesture/recognizer.rs` | 607 | 1 | 607줄 | todo | |
| `runtime/event/gesture/types.rs` | 381 | 11 | 타입 모음, 공개 타입 11 | todo | |
| `runtime/event/ime.rs` | 654 | 8 | 654줄, 공개 타입 8 | split | 조합 데이터 타입·설정, 조합 상태 기계, 렌더용 preedit 조각이 섞임 → ime/{mod,types,state,preedit}.rs (길이 제한 상수는 state.rs). 참고: 빈 목록으로 `set_candidates`를 부르면 `Selecting` 상태가 그대로 남음 |
| `runtime/event/mod.rs` | 349 | 5 | 공개 타입 5 | todo | |
| `runtime/layout/node.rs` | 215 | 8 | 공개 타입 8 | todo | |
| `runtime/layout/responsive.rs` | 571 | 7 | 공개 타입 7 | todo | |

### 3. core, devtools, testing, query (13)

| 파일 | 줄 | 공개 타입 | 신호 | 판단 | 비고 |
|---|---:|---:|---|---|---|
| `core/app/hot_reload.rs` | 428 | 5 | 공개 타입 5 | todo | |
| `core/app/mod.rs` | 1038 | 1 | 1038줄 | todo | |
| `core/app/profiler.rs` | 582 | 10 | 공개 타입 10 | todo | |
| `core/app/router.rs` | 556 | 5 | 공개 타입 5 | todo | |
| `core/app/screen/types.rs` | 240 | 7 | 타입 모음, 공개 타입 7 | todo | |
| `devtools/mod.rs` | 527 | 4 |  | todo | |
| `devtools/profiler/types.rs` | 233 | 5 | 타입 모음, 공개 타입 5 | todo | |
| `devtools/time_travel/debugger.rs` | 615 | 1 | 615줄 | todo | |
| `devtools/time_travel/types.rs` | 302 | 6 | 타입 모음, 공개 타입 6 | todo | |
| `lib.rs` | 855 | 1 | 855줄 | todo | |
| `query/mod.rs` | 489 | 7 | 공개 타입 7 | todo | |
| `testing/assertions.rs` | 261 | 7 | 공개 타입 7 | todo | |
| `testing/visual/types.rs` | 273 | 6 | 타입 모음, 공개 타입 6 | todo | |

### 4. state, utils, text, a11y (12)

| 파일 | 줄 | 공개 타입 | 신호 | 판단 | 비고 |
|---|---:|---:|---|---|---|
| `a11y/backend/platform.rs` | 307 | 6 | 공개 타입 6 | todo | |
| `a11y/tree.rs` | 521 | 3 |  | todo | |
| `state/reactive/context.rs` | 530 | 4 |  | todo | |
| `state/reactive/store/mod.rs` | 181 | 5 | 공개 타입 5 | todo | |
| `state/worker/channel.rs` | 324 | 5 | 공개 타입 5 | todo | |
| `text/bidi/types.rs` | 444 | 8 | 타입 모음, 공개 타입 8 | todo | |
| `utils/border/mod.rs` | 568 | 5 | 공개 타입 5 | todo | |
| `utils/clipboard.rs` | 503 | 6 | 공개 타입 6 | todo | |
| `utils/diff.rs` | 456 | 3 |  | todo | |
| `utils/i18n.rs` | 427 | 4 |  | todo | |
| `utils/keymap.rs` | 469 | 4 |  | todo | |
| `utils/profiler.rs` | 472 | 5 | 공개 타입 5 | todo | |

### 5. widget: data (11)

| 파일 | 줄 | 공개 타입 | 신호 | 판단 | 비고 |
|---|---:|---:|---|---|---|
| `widget/data/chart/candlechart.rs` | 630 | 3 | 630줄, 위젯 한 파일 | todo | |
| `widget/data/chart/helper.rs` | 898 | 1 | 898줄, 위젯 한 파일 | todo | |
| `widget/data/chart/histogram/mod.rs` | 623 | 1 | 623줄, 위젯 한 파일 | todo | |
| `widget/data/chart/piechart.rs` | 442 | 4 | 위젯 한 파일 | todo | |
| `widget/data/chart/timeseries/types.rs` | 197 | 7 | 타입 모음, 공개 타입 7 | todo | |
| `widget/data/chart/waveline.rs` | 544 | 3 | 위젯 한 파일 | todo | |
| `widget/data/datagrid/render.rs` | 603 | 0 | 603줄, 위젯 한 파일 | todo | |
| `widget/data/json_viewer/view.rs` | 656 | 1 | 656줄, 위젯 한 파일 | todo | |
| `widget/data/log_viewer/view.rs` | 908 | 1 | 908줄, 위젯 한 파일 | todo | |
| `widget/data/timeline.rs` | 556 | 5 | 공개 타입 5, 위젯 한 파일 | todo | |
| `widget/data/timer/mod.rs` | 692 | 4 | 692줄, 위젯 한 파일 | todo | |

### 6. widget: developer (7)

| 파일 | 줄 | 공개 타입 | 신호 | 판단 | 비고 |
|---|---:|---:|---|---|---|
| `widget/developer/aistream.rs` | 481 | 4 | 위젯 한 파일 | todo | |
| `widget/developer/code_editor/types.rs` | 108 | 5 | 타입 모음, 공개 타입 5 | todo | |
| `widget/developer/diff.rs` | 567 | 5 | 공개 타입 5, 위젯 한 파일 | todo | |
| `widget/developer/httpclient/types.rs` | 139 | 5 | 타입 모음, 공개 타입 5 | todo | |
| `widget/developer/presentation.rs` | 682 | 4 | 682줄, 위젯 한 파일 | todo | |
| `widget/developer/procmon.rs` | 606 | 5 | 606줄, 공개 타입 5, 위젯 한 파일 | todo | |
| `widget/developer/vim.rs` | 646 | 5 | 646줄, 공개 타입 5 | todo | |

### 7. widget: display, feedback (11)

| 파일 | 줄 | 공개 타입 | 신호 | 판단 | 비고 |
|---|---:|---:|---|---|---|
| `widget/display/avatar.rs` | 543 | 3 | 위젯 한 파일 | todo | |
| `widget/display/empty_state.rs` | 469 | 3 | 위젯 한 파일 | todo | |
| `widget/display/gauge.rs` | 684 | 3 | 684줄, 위젯 한 파일 | todo | |
| `widget/display/richlog.rs` | 623 | 4 | 623줄, 위젯 한 파일 | todo | |
| `widget/display/richtext.rs` | 540 | 3 | 위젯 한 파일 | todo | |
| `widget/display/status_indicator.rs` | 518 | 4 | 위젯 한 파일 | todo | |
| `widget/feedback/alert.rs` | 570 | 3 | 위젯 한 파일 | todo | |
| `widget/feedback/modal/mod.rs` | 618 | 3 | 618줄, 위젯 한 파일 | todo | |
| `widget/feedback/statusbar.rs` | 570 | 5 | 공개 타입 5, 위젯 한 파일 | todo | |
| `widget/feedback/toast_queue.rs` | 607 | 4 | 607줄, 위젯 한 파일 | todo | |
| `widget/feedback/tooltip.rs` | 644 | 4 | 644줄, 위젯 한 파일 | todo | |

### 8. widget: input, form, layout, traits, datetime_picker (12)

| 파일 | 줄 | 공개 타입 | 신호 | 판단 | 비고 |
|---|---:|---:|---|---|---|
| `widget/datetime_picker/types.rs` | 129 | 5 | 타입 모음, 공개 타입 5 | todo | |
| `widget/form/form.rs` | 655 | 4 | 655줄, 위젯 한 파일 | todo | |
| `widget/form/masked_input.rs` | 787 | 3 | 787줄, 위젯 한 파일 | todo | |
| `widget/input/input_widgets/radio.rs` | 402 | 3 | 위젯 한 파일 | todo | |
| `widget/input/input_widgets/selection_list.rs` | 563 | 3 | 위젯 한 파일 | todo | |
| `widget/input/input_widgets/slider.rs` | 617 | 3 | 617줄, 위젯 한 파일 | todo | |
| `widget/input/input_widgets/stepper.rs` | 559 | 5 | 공개 타입 5, 위젯 한 파일 | todo | |
| `widget/layout/card/core.rs` | 727 | 1 | 727줄, 위젯 한 파일 | todo | |
| `widget/layout/dock.rs` | 543 | 4 | 위젯 한 파일 | todo | |
| `widget/layout/splitter.rs` | 587 | 6 | 공개 타입 6, 위젯 한 파일 | todo | |
| `widget/layout/stack.rs` | 1051 | 2 | 1051줄, 위젯 한 파일 | todo | |
| `widget/traits/view.rs` | 743 | 6 | 743줄, 공개 타입 6, 위젯 한 파일 | todo | |

### 9. widget: 나머지 (canvas, markdown, mermaid, debug_overlay, 단일 파일 위젯) (8)

| 파일 | 줄 | 공개 타입 | 신호 | 판단 | 비고 |
|---|---:|---:|---|---|---|
| `widget/canvas/braille/shapes.rs` | 515 | 10 | 공개 타입 10 | todo | |
| `widget/debug_overlay/mod.rs` | 495 | 3 | 위젯 한 파일 | todo | |
| `widget/image.rs` | 476 | 4 | 위젯 한 파일 | todo | |
| `widget/markdown/mod.rs` | 898 | 2 | 898줄, 위젯 한 파일 | todo | |
| `widget/markdown/types.rs` | 163 | 5 | 타입 모음, 공개 타입 5 | todo | |
| `widget/mermaid/types.rs` | 175 | 7 | 타입 모음, 공개 타입 7 | todo | |
| `widget/option_list.rs` | 724 | 4 | 724줄, 위젯 한 파일 | todo | |
| `widget/qrcode.rs` | 438 | 3 | 위젯 한 파일 | todo | |
