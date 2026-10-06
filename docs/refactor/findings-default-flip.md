# 기본값을 뒤집기 전에 — 예제 전수 대조

3.0에서 `dom_from_render`·`css_layout`을 기본 on으로 뒤집는다. 뒤집는 것은 **breaking**이라
3.0 릴리스 때 한 번에 넣고, 그 전까지 main에는 breaking 없는 준비만 쌓는다(2026-10-05 결정).
이 문서는 그 준비 중 "뒤집으면 실제로 무엇이 달라지는가"를 잰 결과다.

## 방법

`examples/` 전부를 실제 PTY에서 두 번씩 띄워 화면을 비교했다 — 기본값 그대로, 그리고
두 플래그를 켠 채로. 플래그는 `AppBuilder::new`를 환경 변수로 뒤집는 **커밋하지 않은**
패치로 켰다. 예제 코드는 건드리지 않았다.

- 100×30, 첫 화면(2초) + 키 입력 뒤 화면(Tab, ↓, ↓, →, Space, Tab)
- `pyte`로 터미널을 에뮬레이트해 칸 단위로 **글자**와 **속성**(fg, bg, bold, reverse)을 비교
- PTY 주의점은 [`findings-render-pipeline.md`](findings-render-pipeline.md)의 "재현" 절과 같다

TUI가 아닌 `version`, `store`, `test_helper_functions`와 벤치용 `benchmark_rendering`은 뺐다.

## 결과 — 31개 중 28개가 칸 하나 다르지 않다

달라진 셋은 전부 **예제가 이미 적어 둔 CSS가 이제야 적용된 것**이다.

| 예제 | 달라진 것 | 원인 |
|---|---|---|
| `css_features` | 제목과 안내문이 가운데로, 색 93칸 | 자기 CSS의 `.title { text-align: center }`, `.center`, 색 규칙 |
| `plugins` | 색 94칸 | 자기 CSS의 `.plugin-value`, `.plugin-label` |
| `dashboard` | 배경 | 자기 CSS의 `* { background: #1e1e2e }` |

`showcase`(시계), `animations`·`theme_switcher`(키 입력 타이밍)는 **같은 모드 안에서도**
실행마다 달라서 뺐다. 반복 실행으로 확인했다.

## 발견 — `background`가 박스를 채우지 않았다

`dashboard`의 배경은 처음 대조에서 **누더기**였다. 테두리 선과 글자 칸에만 칠해지고
박스 안쪽 빈 칸과 루트는 터미널 배경 그대로였다.

```
....................................................   ← 루트: 아무것도 안 칠해짐
##################################################.##   ← 테두리
##.#################################.############....   ← 글자가 있는 줄
```

위젯은 **글리프를 칠한다.** `Border`는 테두리만, `Text`는 자기 글자만 쓴다. CSS
`background`는 위젯이 쓴 칸 중에서도 그 위젯이 CSS 배경을 읽어 칠한 칸에만 닿았다.

**고침:** 페인트 패스가 노드를 그린 **뒤에**, 그 노드의 박스 안에서 배경이 비어 있는
칸에 노드의 `background`를 채운다(`fill_background_under`).

- **뒤에** 채우는 이유: `Buffer::set`은 칸 전체를 교체하므로, 먼저 채우면 배경 없이 쓰인
  글리프마다 구멍이 난다
- 비어 있는 칸만 채우므로 위젯이 스스로 칠한 배경(빌더 `.bg(...)`, 자식 자신의 규칙)은
  유지된다 — CSS에서 투명한 자식 뒤로 부모 배경이 보이는 것과 같다
- `background`는 상속되지 않으므로 규칙 없는 자식은 아무것도 채우지 않고 부모의 것이 보인다

고친 뒤 `dashboard`는 위젯이 다른 색을 직접 지정한 칸을 빼고 전부 채워진다. 다른 예제는
바뀌지 않았고 전경색 변화도 없다. 계약은 `tests/css_background.rs`.

CSS 배경 채우기는 `dom_from_render`에서만 동작한다 — 루트 아래 노드에 스타일이 닿는
경로가 그것뿐이다.

### 같은 구멍이 위젯 자신의 배경에도 있었다

채우기를 넣자 `builder_outranks_stylesheet`의 세 테스트(`ZenMode`, `Presentation`,
`MarkdownPresentation`)가 깨졌다. 빌더로 배경을 지정했는데 스타일시트 빨강이 새어 나왔다.

원인은 같은 `Buffer::set`이다. 이 위젯들은 박스 전체를 자기 배경으로 칠한 뒤 그 위에
글자를 쓰는데, 글자가 배경 없이 쓰여 **자기 배경에 구멍을 낸다.** CSS가 없어도 그랬다 —
`examples/slideshow`는 슬라이드 배경(`#141428`) 위의 글자 칸 279개가 터미널 배경으로
뚫려 있었다. 새 채우기가 그 구멍에 CSS 배경을 부으면서 드러났을 뿐이다.

**고침:** `RenderContext::fill_box_background(bg)` — 박스를 칠하는 것과 함께 **"이
박스의 배경은 bg다"를 선언한다.** 위젯이 다 그린 뒤 빈 칸은 CSS 배경이 아니라 선언된
배경으로 채운다. 빌더 > 스타일시트 > 위젯 기본값이라는 우선순위와 같다. 박스 전체를
배경으로 칠하던 위젯 8개의 루프를 이것으로 바꿨다(`ZenMode`, `Card`, `Presentation`,
`MarkdownPresentation`, `TextArea`, `CodeEditor`, `RichTextEditor`, 차트).

이 쪽은 **기본 경로에서도** 동작한다(`render_child`와 루트). 기본 모드에서 main과 예제를
대조한 결과 바뀐 것은 `slideshow`의 구멍 279칸이 메워진 것뿐이다.

## 뒤집기 전에 남은 것

이 대조로 보면 기본값 전환이 깨뜨리는 예제는 없다. 바뀌는 것은 전부 "CSS가 적힌 대로
동작하기 시작한다"이다. 사용자 앱도 같은 성격의 변화를 겪는다 — 이제까지 조용히
무시되던 자기 CSS가 적용된다. 마이그레이션 가이드에 적을 내용이다.

곁가지로 본 것: `text_editor`는 두 모드 모두 첫 화면 본문이 비어 있다("52 lines"라고
표시하면서). 플래그와 무관하다.

## 뒤집은 뒤 — 3.0

기본값을 뒤집은 PR에서 같은 방법(100×30, 첫 화면 + 같은 키 입력, `pyte`, 속성에 밑줄·취소선·
기울임 추가)으로 내용 크기 스택이 기본인 `main`과 다시 대조했다. 달라진 것은 이번에도 위의
셋뿐이고, 원인도 같다. 반복 실행으로 실행마다 달라지는 예제(`showcase` 시계, `theme_switcher`
키 입력 뒤, `demo`, `reactive_form`, `tasks_usage`, `todo`, `qrcode_showcase`)를 걸러 냈다.

CSS가 적용되기 시작하면서 **적힌 CSS 자체가 틀렸던** 예제가 드러났고, 같은 PR에서 고쳤다.

| 예제 | 틀린 것 | 고침 |
|---|---|---|
| `css_features` | `.card` 규칙을 쓰는 위젯이 없었다 | 세 `Border`에 `.class("card")` |
| `plugins` | `bold: true`(CSS가 아니다), 아무도 쓰지 않는 `.plugin-title`, 적용되지 않는 `padding` | 죽은 선언 삭제 |
| `theme_switcher` | 정의된 적 없는 `var(--theme-primary)`, 노드가 없는 위임 본문의 `.container` | 테마 색을 `render`에서 직접 읽는다 |
| `templates/counter`, `form-app` | `Border { … }` — `Border`가 뷰의 위임 본문이라 노드가 뷰의 것이고, `Border::panel()` 빌더가 이긴다 | 뷰 타입으로 선택(`CounterApp { … }`), `Border::new()` |
