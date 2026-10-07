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
| `runtime/render/batch.rs` | 511 | 3 |  | todo | |
| `runtime/render/image_protocol.rs` | 696 | 8 | 696줄, 공개 타입 8 | split | 감지·인코더·Sixel 알고리즘·터미널별 명령이 섞임 → image_protocol/{mod,protocol,encoder,sixel,kitty,iterm2}.rs |
| `runtime/style/error.rs` | 669 | 6 | 669줄, 공개 타입 6 | todo | |
| `runtime/style/parser/apply.rs` | 707 | 0 | 707줄 | todo | |
| `runtime/style/parser/types.rs` | 103 | 5 | 타입 모음, 공개 타입 5 | todo | |
| `runtime/style/properties/types.rs` | 427 | 20 | 타입 모음, 공개 타입 20 | todo | |
| `runtime/style/theme.rs` | 747 | 8 | 747줄, 공개 타입 8 | todo | |
| `runtime/style/transition.rs` | 504 | 5 | 공개 타입 5 | todo | |

### 2. runtime: event, layout, dom (13)

| 파일 | 줄 | 공개 타입 | 신호 | 판단 | 비고 |
|---|---:|---:|---|---|---|
| `runtime/dom/node/mod.rs` | 383 | 5 | 공개 타입 5 | todo | |
| `runtime/dom/pool/mod.rs` | 578 | 8 | 공개 타입 8 | todo | |
| `runtime/dom/query.rs` | 683 | 3 | 683줄 | todo | |
| `runtime/dom/selector/types.rs` | 382 | 8 | 타입 모음, 공개 타입 8 | todo | |
| `runtime/event/custom/types.rs` | 212 | 6 | 타입 모음, 공개 타입 6 | todo | |
| `runtime/event/drag.rs` | 580 | 5 | 공개 타입 5 | todo | |
| `runtime/event/focus.rs` | 534 | 4 |  | todo | |
| `runtime/event/gesture/recognizer.rs` | 607 | 1 | 607줄 | todo | |
| `runtime/event/gesture/types.rs` | 381 | 11 | 타입 모음, 공개 타입 11 | todo | |
| `runtime/event/ime.rs` | 654 | 8 | 654줄, 공개 타입 8 | todo | |
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
