# 빌더가 말한 색이 스타일시트를 이겨야 한다 — 남은 것

## 문제

`RenderContext::color_or(builder, initial)`는 둘이 같으면 "빌더가 아무 말도 하지
않았다"로 읽는다. `initial`이 아무도 일부러 지정하지 않을 sentinel일 때는 옳다 —
`Color::default()`는 완전 투명이다. 그런데 위젯 대부분의 기본값은 `Color::WHITE`나
`DARK_GRAY` 같은 **실제 색**이라, `.color(DARK_GRAY)`를 명시한 빌더가 침묵과
구별되지 않았고 **말한 빌더가 스타일시트에 졌다.**

빌더 > 스타일시트 > 위젯 기본값이라는 확립된 우선순위의 역전이다.

판별 기준은 하나다: **기본값이 사용자가 일부러 지정할 만한 값인가.**

| 값 | 판정 |
|---|---|
| `Color::default()` (완전 투명) | 안전한 sentinel |
| `Color::WHITE`, `DARK_GRAY`, `Color::GREEN` … | 실제 값 — 함정 |
| `gap: 0`, `BorderStyle::None`, `Alignment::Left` | 실제 값 — 함정 |

## 해결한 것

필드를 `Option<Color>`로 바꾸고 `self.field.unwrap_or_else(|| ctx.css_color(D))`로
옮겼다. #656, #658, #660, #661, #662와 이 PR에서 21개 위젯.

## 부수 효과가 본래 목적만큼 컸다

필드를 `Option`으로 바꾸면 컴파일러가 **"해결된 색을 무시하고 필드를 직접 읽는"
누락을 전부 잡는다.** 일곱 위젯에서 나왔고 — `Slider`(#635), `Gauge`, `MenuBar`,
`Switch`, `Autocomplete`, `QrCode`, `StatusBar` — 전부 CSS가 그 부분에 도달하지
못하던 곳이다.

그리고 `Gauge`를 파다가 **`sub_ctx`가 계산된 스타일을 통째로 버리는 것**(#657)이
드러났다. 소스 레벨 ratchet으로는 볼 수 없는 종류였다.

## 공개 필드 — 2.x에서 못 바꿔 3.0에서 해결

아래는 같은 함정이 남아 있었지만 **필드가 `pub`이라 타입 변경이 breaking이었다.**
사용자가 구조체를 직접 만들거나 필드에 대입할 수 있다.

| 위치 | 필드 | 형태 | 상태 |
|---|---|---|---|
| `mermaid/types.rs` | `DiagramColors.node_fg` | `pub` 필드를 가진 `pub struct`, `.colors(DiagramColors)` | 3.0에서 해결 |
| `developer/procmon.rs` | `ProcColors.name` | 같음 | 3.0에서 해결 |
| `developer/presentation.rs` | `Slide.content_color` | `pub` 필드 | 3.0에서 해결 |
| `datetime_picker/mod.rs` | `DateTimePicker.field_fg` | `pub` 필드 (빌더 없음) | 3.0에서 해결 |
| `data/calendar/` | `day_fg` | 빌더가 없고 `CalendarView` params 구조체 경유 | 남김 (아래) |

**3.0에 넣었다** (2026-10-05 결정). 네 필드 모두 `Option<Color>`이 되었고 렌더는
`self.field.unwrap_or_else(|| ctx.css_color(D))`로 해결한다 — `Diagram`은 노드
자체의 `color`가 그보다 앞선다. 남은 `color_or` 호출은 `Calendar` 하나다.
계획서 Phase 4는 "제거만 한다"였지만 범위를 명시적으로 넓혔다 — 3.0은 어차피
breaking 릴리스이고, 미루면 4.0까지 이 네 곳만 우선순위가 뒤집힌 채 남았을 것이다. `Calendar`는 아래 이유로 뺀다. 사용자 쪽 안내는
[`docs/migration/v3.0.0.md`](../migration/v3.0.0.md).

`Calendar`는 다르다: 빌더가 아예 없어 스타일시트와 다툴 것이 없으므로 **결함이
아니다.** `ContextMenu.fg`, `Stepper.pending_color`도 같다. 일관성만 문제인 것들은
`Option`으로 옮겨두었고, 변환 비용이 큰 `Calendar`만 두었다.

## 테스트

`tests/builder_outranks_stylesheet.rs`. 위젯마다 **양방향**이다 (3.0의 네 필드는
기본 렌더가 그대로인지까지 세 방향):

1. 이름을 말한 빌더가 CSS를 이긴다
2. **침묵한 빌더는 여전히 CSS에 양보한다**

2번이 없으면 "항상 빌더 값"으로 구현해도 통과한다. 그리고 양방향이 *함께* 실패하면
배선이 아니라 fixture 문제라는 신호다 — `SearchBar`·`Autocomplete`·`Splitter`에서
실제로 그렇게 쓰였다.
