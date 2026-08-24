# Revue 아키텍처 방향 검토

[`README.md`](README.md)의 안티패턴 카탈로그가 "무엇이 깨질 수 있는가"라면, 이 문서는
그 다음 질문 — **"지금 구조 자체가 맞는가, 다시 짠다면 무엇을 고를 것인가"** — 에 대한 검토다.

> [!NOTE]
> 이 문서는 **결정 기록(ADR)이 아니라 외부 검토 의견**이다. 출처는
> [ChatGPT 대화](https://chatgpt.com/share/6a843f6f-64f8-83e9-a269-bb1cb8a7ba44) (2026-08-18 정리).
> 여기 담긴 판단은 Revue 코드베이스를 실측한 결과가 아니라 README와 공개 문서 기준의 평가다.
> 실제 채택 여부는 별도 ADR로 남겨야 한다 (`REV-PROJ-003`).
>
> 원문 응답을 **요약 없이 그대로** 옮겼다. 편집한 것은 헤딩 레벨 조정과,
> 대상 문서가 함께 공유되지 않아 검증할 수 없는 인용 마커 제거뿐이다.
> 대화 중 웹 검색 결과는 공유 시점에 이미 가려져 있어(`output redacted`) 복원할 수 없었다.

## 목차

- [1. 지금 구조가 rich하면서 nice한 아키텍처인가](#1-지금-구조가-rich하면서-nice한-아키텍처인가)
- [2. 처음부터 다시 짠다면 어떤 스택인가](#2-처음부터-다시-짠다면-어떤-스택인가)
- [3. Ratatui를 쓰지 않는다면](#3-ratatui를-쓰지-않는다면)

---

## 1. 지금 구조가 rich하면서 nice한 아키텍처인가

> **질문:** 근데 근본적인건데 이게 리치풀하면서 나이스한 아키텍쳐일까? 고민이긴하네 갑자기 요즘 또 트렌드가 바뀌어서

그 고민이 맞아. 지금은 기능이 풍부한 것보다 **핵심 모델이 단순하고 확장 지점이 명확한가**가 더 중요한 흐름이라, Revue가 “좋은 프레임워크”인지 “브라우저를 TUI에 재구현한 것인지”를 분리해서 봐야 해.

응. **Revue가 rich한 건 확실한데, 지금 구조가 반드시 nice architecture인지는 의심해볼 만해.**
내 판단은 이거야.

> **방향 자체가 낡은 것은 아니다. 다만 “retained + signals + CSS + 100개 widget”을 전부 핵심 정체성으로 묶은 건 요즘 흐름보다 다소 무겁다.**

Revue는 현재 스스로를 retained architecture, CSS styling, Signal/Computed/Effect, Flex/Grid, animation, worker pool, hot reload, devtools를 갖춘 통합 프레임워크로 정의하고 있다. 사실상 **Rust판 Textual 또는 터미널용 Vue**에 가깝다. citeturn300193view0turn300193view2

#### 요즘 트렌드가 바뀐 지점

요즘은 단순히 retained에서 immediate로 돌아갔다기보다, **사용자 API는 immediate/declarative하게 유지하고 내부만 필요한 상태를 보존하는 hybrid 방향**이 강해졌어.

Ratatui 자체도 immediate mode와 retained mode를 구분하지만, Ratatui 생태계에서 권장하는 상위 구조는 TEA나 component architecture처럼 비교적 명시적인 상태 흐름이다. citeturn955718search0turn955718search5turn955718search18

즉 예전의 선택지는 이랬다면,

```text
Immediate mode
vs
Retained mode
```

지금은 오히려 이렇게 보는 게 맞아.

```text
사용자 모델: 선언적이고 값 중심
내부 엔진: 캐시와 상태를 선택적으로 보존
변경 흐름: 명시적
최적화: 증분 처리
```

Conrod처럼 오래전부터 **immediate API 위에 retained state graph를 두는 혼합 구조**도 존재했고, 최근 Rust UI 쪽에서도 이 조합이 계속 매력적인 이유는 retained mode의 성능과 immediate mode의 단순성을 함께 가져갈 수 있기 때문이다. citeturn955718search10turn955718search14

#### Revue의 현재 구조에서 좋은 부분

##### 1. Rich TUI라는 포지션은 여전히 유효하다

터미널 앱이 단순한 CLI를 넘어 다음 수준으로 올라가고 있어.

- AI coding agent
- database explorer
- infrastructure dashboard
- observability console
- IDE 비슷한 개발 도구
- interactive installer
- local-first administration tool

이런 앱은 form, modal, table, tree, markdown, async task, theme 같은 상위 기능이 필요해. Ratatui는 의도적으로 저수준 library라 이런 부분을 직접 해결하지 않는다. Ratatui도 자신을 lightweight TUI library로 소개하며 특정 앱 구조를 강제하지 않는다. citeturn955718search16turn955718search19

그러므로 **Ratatui 위의 batteries-included framework**라는 Revue의 자리는 충분히 있다.

##### 2. Signals도 낡은 선택은 아니다

Signal 기반 fine-grained reactivity는 오히려 최근 UI 아키텍처에서 다시 강해진 흐름이야. 문제는 signal 자체가 아니라 다음이야.

- dependency가 너무 암묵적인가
- effect가 business logic으로 번지는가
- widget마다 signal이 흩어지는가
- signal graph가 실제 앱 상태의 source of truth가 되는가

즉:

```text
Signal = 좋거나 나쁜 아키텍처
```

가 아니라,

```text
Signal은 국소 UI 파생 상태에 좋음
Signal graph를 전체 애플리케이션 아키텍처로 쓰면 위험
```

이게 더 정확해.

##### 3. CSS도 사용자 경험 측면에서는 강점이다

터미널 UI를 만드는 사람이 Rust 코드 안에서 모든 색상, padding, border를 작성하는 건 번거롭다. CSS 파일, variable, hot reload는 생산성이 높다. 특히 디자이너나 웹 개발자가 접근하기 쉽다.

다만 **CSS 전체를 구현하는 것**과 **CSS 같은 스타일 언어를 제공하는 것**은 완전히 다른 문제야.

Revue가 전자를 향하면 부담이 지나치게 커지고, 후자로 한정하면 좋은 선택이 될 수 있어.

---

### 그런데 왜 지금 구조가 위험해 보이냐

#### 1. Feature-rich와 architecture-rich를 혼동할 위험

현재 README의 대표 비교 기준이 다음이야.

- widget 100+
- Flex/Grid
- form
- animation
- worker pool
- hot reload
- devtools

이건 제품 기능 비교로는 좋지만 아키텍처 품질은 설명하지 못해. citeturn300193view0

좋은 architecture의 핵심은 widget 개수가 아니라 다음이거든.

```text
상태는 어디에 있는가?
변경은 어떤 경로로 전달되는가?
누가 tree를 소유하는가?
어떤 변경이 layout을 무효화하는가?
이벤트 결과는 어떻게 상태 변경으로 이어지는가?
비동기 결과는 어느 시점에 commit되는가?
```

현재 Revue는 기능이 너무 빠르게 확장돼서 **설계 중심부보다 주변 기능의 표면적이 더 커질 가능성**이 있어.

#### 2. 웹의 복잡성을 TUI에 그대로 가져오고 있다

현재 Revue는 명시적으로 “web app처럼 TUI를 만든다”고 설명한다. citeturn300193view2

이것은 마케팅으로는 좋지만 아키텍처 목표로 삼으면 위험해.

웹 브라우저의 복잡성은 다음 요구 때문에 생긴 거야.

- 수십 년간 누적된 호환성
- DOM mutation
- CSS cascade
- accessibility tree
- network resource
- arbitrary document
- multiple scripting contexts
- pixel-level layout
- 다양한 input method

터미널은 훨씬 제한된 환경이야.

```text
정수 cell grid
keyboard-first
작은 viewport
제한된 color/style
상대적으로 작은 UI tree
보통 단일 application
```

그런데 CSS selector, cascade, retained tree, signal graph, animation, plugin, hot reload를 모두 넣으면 **브라우저의 비용은 가져오면서 브라우저의 시장과 생태계는 없는 상황**이 될 수 있어.

#### 3. Retained widget object가 Rust와 아주 잘 맞는다고 보긴 어렵다

Rust에서 retained UI는 흔히 다음 문제를 만든다.

- parent-child ownership
- callback lifetime
- `Arc<Mutex<_>>`
- `'static` closure
- trait object
- interior mutability
- tree mutation
- widget identity
- zombie subscription

반면 immediate UI는 app state만 명확히 소유하면 view는 매번 빌릴 수 있어.

```rust
fn view(state: &AppState, frame: &mut Frame)
```

이 구조가 Rust ownership과 굉장히 잘 맞아.

그래서 retained가 무조건 나쁘다는 게 아니라, **Rust에서는 retained object tree를 public programming model로 그대로 노출할수록 비용이 커진다**는 뜻이야.

#### 4. Revue Quick Start도 실제로는 반쯤 immediate다

현재 예제는 `Counter`가 상태를 소유하고, `View::render()` 때마다 `vstack()`, `Text`, `hstack()`을 다시 구성한다. citeturn300193view2

```rust
impl View for Counter {
    fn render(&self, ctx: &mut RenderContext) {
        vstack()
            .child(...)
            .render(ctx);
    }
}
```

이건 사용자가 widget tree를 한 번 만들어 보존하는 전통적인 retained API라기보다:

```text
상태 객체는 retained
View description은 매 render마다 생성
내부 tree 또는 engine state는 retained
```

에 가까워 보여.

사실 이건 나쁜 게 아니라, 오히려 **Revue가 이미 hybrid architecture 쪽으로 가고 있다는 신호**야.

그렇다면 아예 retained라는 말을 강조하기보다 다음처럼 재정의하는 게 정확할 수 있어.

> Declarative TUI framework with retained incremental runtime

또는

> Immediate-style view composition backed by a retained reactive runtime

---

### 내가 Revue라면 핵심 구조를 이렇게 바꾼다

#### 현재 인상

```text
Revue
├─ Widget framework
├─ Signal runtime
├─ CSS engine
├─ Flex/Grid engine
├─ animation
├─ worker pool
├─ hot reload
├─ devtools
├─ plugins
├─ macros
└─ 100+ widgets
```

이 구조는 **모든 것이 핵심**처럼 보인다.

#### 권장 구조

```text
                    Revue Application
                           │
                 ┌─────────┴─────────┐
                 │                   │
          Explicit App Model     Commands/Events
                 │                   │
                 └─────────┬─────────┘
                           │
                    Declarative View
                           │
                    View Description
                           │
          ┌────────────────┼────────────────┐
          │                │                │
    Reconciliation    Style Resolve      Layout
          │                │                │
          └────────────────┼────────────────┘
                           │
                     Retained Runtime
                           │
                    Terminal Renderer
```

여기서 signals는 root architecture가 아니라 **선택적 local primitive**가 되어야 해.

```text
Application state
    → 명시적 Model / Store / Update

Local derived state
    → Signal / Computed

Side effect
    → Command / Task

Rendering optimization
    → retained runtime
```

이 구분이 핵심이야.

---

### Revue가 취할 만한 세 가지 방향

#### A. 지금처럼 Textual/Vue for Rust TUI

```text
Retained
Signals
CSS
Flex/Grid
Large widget collection
Hot reload
```

장점:

- 차별화가 명확함
- 웹 개발자에게 익숙함
- 빠르게 풍부한 앱 제작 가능
- batteries-included

단점:

- 유지보수 범위가 매우 큼
- hidden magic 증가
- Rust ownership과 마찰
- CSS와 reactive graph의 복잡성
- 1인 프로젝트에서 감당하기 어려움

**제품으로는 매력 있지만 프레임워크 유지보수 리스크가 가장 크다.**

#### B. Ratatui용 Vue/React layer

```text
Declarative components
Signals/hooks
Ratatui backend
Moderate widget set
Inline typed style
```

reratui와 비슷한 영역이야. reratui도 React-inspired components와 hooks를 Ratatui 위에 제공한다. citeturn955718search6turn955718search22

장점:

- 구현 범위가 작음
- Ratatui 생태계 활용
- 사용자 mental model이 익숙함

단점:

- 차별화가 약함
- React 흉내로 보이기 쉬움
- CSS와 rich widget 포지션을 포기해야 할 수 있음

#### C. Hybrid incremental TUI engine

내가 보기엔 가장 좋은 방향이야.

```text
Immediate/declarative authoring
+ retained internal runtime
+ explicit app state
+ optional fine-grained signals
+ typed style core
+ optional CSS frontend
+ incremental layout/paint
```

사용자는 단순하게 작성한다.

```rust
fn view(model: &Model) -> impl View {
    column((
        text(model.title()),
        table(model.visible_rows()),
        button("Refresh", Msg::Refresh),
    ))
}
```

내부는 다음을 보존한다.

- node identity
- focus
- scroll state
- computed style
- layout cache
- paint cache
- signal dependencies

즉 **복잡성은 engine이 감당하되, 사용자의 application architecture까지 reactive graph로 삼키지 않는 방식**이야.

---

### CSS는 없애야 하나?

없앨 필요는 없어. 다만 core에서 떼는 게 좋아.

```text
revue-style-core
    typed property
    computed style
    inheritance
    terminal capability

revue-css
    CSS syntax parser
    selector compiler
    stylesheet hot reload
```

사용자는 둘 중 하나를 고를 수 있게 한다.

```rust
button("Save")
    .variant(ButtonVariant::Primary);
```

또는:

```css
button.primary {
    border: rounded;
    color: cyan;
}
```

둘 다 최종적으로 동일한 `ComputedStyle` IR로 내려가야 해.

그러면 CSS가 프레임워크의 필수 세계관이 아니라 **선택 가능한 authoring frontend**가 된다.

---

### 100개 widget은 장점일까?

초기에는 장점인데, 장기적으로는 오히려 부담일 수 있어.

실제로 중요한 건 100개가 아니라 다음 20개 정도야.

```text
Text
Container
Row / Column
Grid
Scroll
List
Table
Tree
Input
TextArea
Button
Select
Checkbox
Tabs
Modal
Popover
Progress
Canvas
Image
Custom
```

나머지는 조합으로 만들어져야 해.

예를 들어:

```text
Alert
Callout
Badge
Card
Notification
Toast
FormField
Toolbar
StatusBar
```

이런 것들이 각각 깊은 core widget이면 유지보수가 폭증해. 반대로 **primitive 조합으로 만든 compound component**라면 100개가 있어도 괜찮아.

따라서 목표는:

```text
100 built-in primitive widgets
```

가 아니라

```text
15~25 stable primitives
+ compound components
+ recipes
```

가 되어야 해.

---

### 최종 판단

내가 냉정하게 평가하면:

| 항목 | 평가 |
|---|---|
| 제품 아이디어 | 좋음 |
| Rich TUI 포지션 | 유효함 |
| Signals 채택 | 여전히 현대적 |
| CSS 채택 | UX에는 좋지만 core 결합은 위험 |
| Retained runtime | 필요할 수 있음 |
| Retained를 사용자 모델로 노출 | 신중해야 함 |
| 100+ widget | 마케팅 장점, 구조적 위험 |
| 모든 기능을 단일 프레임워크에 통합 | 과도함 |
| 지금 전면 재작성 필요 | 아님 |
| 아키텍처 재정의 필요 | 높음 |

그래서 나는 Revue의 방향을 버리기보다는 다음 한 문장으로 줄이겠어.

> **Revue는 retained UI framework가 아니라, declarative TUI API 뒤에 retained incremental engine을 둔 hybrid framework가 되어야 한다.**

그리고 세 가지를 바꿔야 해.

1. **명시적 application state/update를 기본값으로 둔다.**
2. **Signal은 local reactivity와 derived state에 제한한다.**
3. **CSS, worker, animation, rich widgets는 core 위의 optional layer로 내린다.**

이렇게 하면 rich함은 유지하면서도 구조는 훨씬 단단해져.

반대로 현재의 모든 기능을 하나의 reactive retained object model 아래 계속 넣으면, 결국 **작은 브라우저 엔진을 혼자 유지하는 프로젝트**가 될 가능성이 높아. 그게 지금 Revue의 가장 근본적인 위험이야.

---

## 2. 처음부터 다시 짠다면 어떤 스택인가

> **질문:** 만약 너가 처음부터 다시 짠다고 하면 추천하는 기술스택이나 아키텍쳐가 있어?

처음부터 다시 만든다는 가정이면, “무엇을 넣을지”보다 **복잡성을 어디에 숨기고 어디는 명시적으로 둘지**부터 정하겠습니다. 최신 Rust UI/TUI 흐름과 현재 생태계를 기준으로 현실적인 스택을 좁혀볼게요.

내가 **Revue를 처음부터 다시 설계한다면**, 특정 프레임워크를 그대로 복제하지 않고 다음 형태로 만들겠어.

> **Ratatui 기반 렌더러 + Elm식 명시적 상태 모델 + Xilem식 경량 View Tree/Retained Runtime + 선택적 Signal**

즉, 사용자에게는 단순한 선언형 API를 주고 내부적으로만 상태·레이아웃·포커스·캐시를 보존하는 **Hybrid Incremental TUI Framework**다.

---

### 1. 최종 추천 기술 스택

```text
Language           Rust
Terminal backend   Crossterm
Cell renderer      Ratatui Buffer/Backend
App architecture   Elm Architecture(Model–Message–Update–View)
View architecture  Ephemeral View Tree + Retained Runtime Tree
Reconciliation     Keyed structural reconciliation
Local reactivity   Optional typed Signal
Async              Command/Task + channel-based UI-thread commit
Layout             Own integer-cell Flex/Grid engine
Styling            Typed Style IR
CSS                 Optional frontend → Typed Style IR
Text                unicode-segmentation + unicode-width
Storage             Generational arena
Testing             Virtual terminal + snapshots + property tests + fuzzing
Instrumentation     Trace events + invalidation reasons
```

핵심은 Ratatui를 없애는 게 아니라, **Ratatui를 프레임워크의 최하단 렌더링 백엔드로 제한하는 것**이야.

Ratatui는 현재 immediate rendering과 intermediate buffer를 사용하며, 매 프레임 애플리케이션이 UI를 기술하고 최종 buffer 차이를 출력하는 구조다. 이 렌더링 및 terminal backend 부분은 이미 잘 해결되어 있으므로 Revue가 다시 구현할 이유가 적다. citeturn184238search3turn184238search14turn184238search17

---

### 2. 전체 아키텍처

```text
┌─────────────────────────────────────────────────────┐
│                   User Application                  │
│                                                     │
│   Model ── Message ── update() ── Command/Task      │
│     │                                      │        │
│     └────────────── view() ◀────────────────┘        │
└────────────────────────┬────────────────────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────────┐
│                Ephemeral View Description           │
│                                                     │
│   Element / Component / Key / Props / Children      │
└────────────────────────┬────────────────────────────┘
                         │ reconcile
                         ▼
┌─────────────────────────────────────────────────────┐
│                  Retained Runtime Tree              │
│                                                     │
│ identity │ focus │ scroll │ local state │ cache     │
└──────────┬──────────┬───────────┬───────────────────┘
           │          │           │
           ▼          ▼           ▼
      Style Pass   Layout Pass   Event Routing
           │          │
           └────┬─────┘
                ▼
             Paint IR
                │
                ▼
        Ratatui Cell Buffer
                │
                ▼
       Crossterm / Terminal
```

이 구조는 Xilem/Masonry의 중요한 아이디어와 닮아 있어. Xilem은 경량 view tree를 만들고, Masonry는 별도의 retained widget tree에서 event와 update pass를 처리한다. 즉, **사용자가 작성하는 선언적 tree와 실제 런타임 tree를 분리**한다. citeturn184238search2turn184238search10

다만 Xilem 자체는 GUI용이고 여전히 실험적이므로 직접 의존하는 게 아니라 **아키텍처 원리만 가져오는 것**이 좋아. Linebender도 현재 Xilem을 pre-alpha 성격의 실험적 UI 구조로 설명한다. citeturn184238search19

---

### 3. 애플리케이션 상태는 TEA로

Revue의 기본 프로그램 모델은 Signal graph가 아니라 다음이어야 해.

```rust
struct Model {
    selected_tab: Tab,
    query: String,
    rows: Vec<Row>,
    loading: bool,
}

enum Msg {
    KeyPressed(KeyEvent),
    QueryChanged(String),
    Refresh,
    RowsLoaded(Result<Vec<Row>, LoadError>),
    Quit,
}

fn update(model: &mut Model, msg: Msg) -> Command<Msg> {
    match msg {
        Msg::QueryChanged(query) => {
            model.query = query;
            Command::none()
        }

        Msg::Refresh => {
            model.loading = true;
            Command::perform(load_rows(), Msg::RowsLoaded)
        }

        Msg::RowsLoaded(result) => {
            model.loading = false;

            if let Ok(rows) = result {
                model.rows = rows;
            }

            Command::none()
        }

        Msg::Quit => Command::quit(),

        _ => Command::none(),
    }
}
```

그리고 view는 가능하면 pure function으로 둔다.

```rust
fn view(model: &Model) -> impl View<Msg> {
    column((
        tabs(model.selected_tab),
        text_input(&model.query)
            .on_change(Msg::QueryChanged),
        table(&model.rows),
        button("Refresh")
            .on_press(Msg::Refresh),
    ))
}
```

Ratatui 공식 문서도 TEA에서 `view()`가 model을 visual representation으로 변환하는 side-effect-free 함수가 되는 구조를 소개한다. 이 방식은 동일 model이 동일 화면을 만들기 때문에 reasoning과 debugging이 쉬워진다. citeturn184238search0

#### 왜 TEA가 기본이어야 하나

Signal-first 구조에서는 상태 변경이 다음처럼 흩어진다.

```text
Button callback
  → Signal A 변경
      → Computed B
          → Effect C
              → Signal D
                  → render
```

TEA에서는 다음처럼 보인다.

```text
Event
  → Msg
      → update()
          → Model 변경
              → view()
```

Revue 같은 디버깅·관리 도구용 TUI에서는 **숨은 반응성보다 추적 가능한 상태 변화가 더 중요하다.**

특히 사용자가 만들 가능성이 높은 다음 프로그램과 잘 맞는다.

- BitVue
- VQProbe
- 로그 분석기
- 패킷·비트스트림 분석기
- 시스템 모니터
- 디버거
- IDE 스타일 도구

이런 프로그램은 UI state보다 domain state가 훨씬 크기 때문에 Signal graph만으로 전체 상태를 관리하면 추적이 어려워진다.

---

### 4. Signal은 버리지 말고 범위를 제한

Signal을 완전히 제거할 필요는 없어.

다만 다음 용도로 제한하는 게 좋아.

```text
좋은 Signal 사용
─────────────────────────────────────
hover 상태
animation progress
local validation result
derived visibility
local text composition
component 내부 임시 상태
외부 데이터의 read-only binding

좋지 않은 Signal 사용
─────────────────────────────────────
애플리케이션 전체 domain model
파일 로딩 transaction
business rule
navigation 전체 상태
worker task orchestration
복수 상태를 변경하는 command
```

공식적인 기본 모델은:

```text
Model + Message + Update
```

선택적인 고급 모델은:

```text
LocalSignal<T>
Derived<T>
```

정도로 두는 게 좋아.

```rust
fn search_box(props: SearchBoxProps) -> impl View<Msg> {
    component(move |cx| {
        let focused = cx.local(false);
        let validation = cx.derived(|| validate(props.query));

        // local UI behavior
    })
}
```

중요한 점은:

```text
Signal 변경 → 직접 terminal render
```

가 아니라,

```text
Signal 변경
    → component dirty
        → scheduler
            → reconcile
```

로 들어가야 한다는 거야.

---

### 5. View Tree와 Runtime Tree를 반드시 분리

이게 가장 중요한 결정이야.

#### View Tree

매 `view()` 호출 때 만들어지는 값이다.

```rust
enum ViewNode<Msg> {
    Text(TextView),
    Container(ContainerView<Msg>),
    Button(ButtonView<Msg>),
    Component(ComponentView<Msg>),
    Fragment(Vec<ViewNode<Msg>>),
}
```

특징:

- 값 중심
- 가벼움
- 실제 focus 상태 없음
- 실제 scroll 위치 없음
- backend handle 없음
- 가능한 한 allocation이 적음

#### Runtime Tree

프레임워크 내부에 남아 있는 tree다.

```rust
struct RuntimeNode {
    id: NodeId,
    key: Option<Key>,
    kind: NodeKind,

    parent: Option<NodeId>,
    children: SmallVec<[NodeId; 4]>,

    state: NodeState,
    layout: LayoutState,
    style: StyleState,
    paint: PaintState,

    dirty: DirtyFlags,
}
```

여기에만 다음을 보관한다.

- focus
- scroll offset
- input composition
- animation
- lifecycle
- cached layout
- computed style
- event subscriptions
- async task generation
- previous paint output

이 방식이면 사용자는 retained object를 직접 소유하지 않아도 retained runtime의 장점을 얻는다.

---

### 6. Reconciliation은 React식 범용 diff보다 단순하게

TUI에서는 HTML DOM처럼 극단적으로 범용적인 diff가 필요하지 않아.

추천 규칙은 세 가지면 충분해.

```text
1. 같은 위치 + 같은 widget kind → update
2. 명시적 Key가 같음 → move/reuse
3. 그 외 → remove + create
```

예:

```rust
column(
    rows.iter().map(|row| {
        table_row(row)
            .key(row.id)
    })
)
```

Key가 없으면 positional identity를 사용하되, 개발 모드에서는 동적 collection에 key가 없다는 경고를 줄 수 있어.

```text
REVUE-W102:
Dynamic collection children do not have stable keys.
Scroll, input, or local component state may move between rows.
```

React처럼 복잡한 fiber scheduler를 만들 필요는 없어. 터미널 UI에서는 일반적으로 tree가 작고 cell output 자체가 제한적이기 때문에 **결정론적인 동기식 reconciliation**이 더 낫다.

---

### 7. 렌더링은 Ratatui를 그대로 활용

직접 terminal diff engine을 다시 만드는 건 추천하지 않아.

```text
Revue Paint Commands
        ↓
Ratatui Buffer
        ↓
Ratatui Terminal diff
        ↓
Crossterm Backend
```

Ratatui의 immediate-mode 구조는 persistent widget object를 요구하지 않으면서 매 frame 현재 상태를 표현할 수 있다는 장점이 있다. 또한 프레임워크가 호출할 때만 다시 그리므로 Revue가 scheduling 정책을 직접 결정할 수 있다. citeturn184238search3

다만 Revue widget이 Ratatui widget을 그대로 public API로 노출해서는 안 돼.

```rust
// 좋지 않음
pub fn raw_widget<W: ratatui::widgets::Widget>(widget: W);

// 권장
pub trait Paint {
    fn paint(&self, ctx: &mut PaintContext);
}
```

호환용 escape hatch는 별도 crate로 둔다.

```text
revue-ratatui-compat
```

---

### 8. Layout은 직접 만들되 정수 cell 기반

Ratatui의 기본 `Layout`만으로 rich framework 전체를 표현하기는 어렵다.

Revue에서는 다음 정도를 자체 layout engine으로 제공하는 게 좋아.

```text
Stack
Row
Column
Flex
Grid
Overlay
Absolute
Scroll viewport
Intrinsic content size
Min / Max / Fixed / Fraction / Content
```

하지만 브라우저 CSS layout 전체를 구현하지는 말아야 해.

#### 내부 크기 단위

```rust
struct Size {
    width: u16,
    height: u16,
}

struct Rect {
    x: u16,
    y: u16,
    width: u16,
    height: u16,
}
```

중간 계산은 overflow를 피하기 위해 `i32` 또는 `u32`를 써도 되지만, 최종 layout은 정수 cell이어야 해.

```rust
enum Length {
    Auto,
    Cells(u16),
    Percent(u8),
    Fraction(u16),
    MinContent,
    MaxContent,
}
```

브라우저처럼 다음까지 처음부터 지원하지 않는 게 좋아.

- `calc()` 전체
- CSS subgrid
- writing modes
- floats
- inline formatting context
- arbitrary transforms
- pixel geometry
- CSS containment 전체

TUI에 실제로 필요한 subset부터 고정해야 해.

---

### 9. Style은 Typed IR가 중심

CSS를 중심에 두지 말고 **Typed Style IR**을 중심에 둬야 한다.

```rust
struct Style {
    foreground: Option<Color>,
    background: Option<Color>,
    modifiers: ModifierSet,

    border: BorderStyle,
    padding: Edges<u16>,
    margin: Edges<u16>,

    width: Length,
    height: Length,
    min_width: Option<u16>,
    max_width: Option<u16>,

    display: Display,
    overflow: Overflow,
    alignment: Alignment,
}
```

사용자는 Rust API로 쓸 수 있다.

```rust
button("Save")
    .class("primary")
    .style(
        Style::new()
            .padding_x(2)
            .border(BorderStyle::Rounded)
    )
```

CSS는 별도 frontend다.

```text
CSS source
    ↓
Parser
    ↓
Selector IR
    ↓
Typed Declaration IR
    ↓
Computed Style
```

```css
button.primary {
    color: cyan;
    border: rounded;
    padding-inline: 2;
}
```

두 방식 모두 같은 `ComputedStyle`에 도착해야 한다.

#### CSS crate 구성

```text
revue-style
    Typed Style
    ComputedStyle
    Theme tokens
    inheritance
    terminal capability fallback

revue-css
    parser
    selectors
    cascade
    hot reload
```

그래야 CSS가 마음에 들지 않는 사용자도 Revue core를 쓸 수 있어.

---

### 10. 비동기는 Command 기반

Tokio를 프레임워크 세계관으로 만들지 않는 게 좋아.

사용자 API:

```rust
enum Command<Msg> {
    None,
    Emit(Msg),
    Task(Task<Msg>),
    Batch(Vec<Command<Msg>>),
    Quit,
}
```

```rust
Command::perform(
    async {
        repository.load_rows().await
    },
    Msg::RowsLoaded,
)
```

내부 동작:

```text
update()
   │
   ├─ Model 변경
   └─ Command 반환
          │
          ▼
      Task Executor
          │
          ▼
       Msg Channel
          │
          ▼
      UI Thread Queue
          │
          ▼
        update()
```

원칙은 하나야.

> **Model과 Runtime Tree는 UI thread에서만 변경한다.**

worker는 결과를 message로 반환할 뿐이다.

#### Runtime 선택

- 기본: 작은 자체 executor abstraction
- `revue-tokio`: Tokio integration
- `revue-async-std`: 필요할 경우 integration
- 동기 앱: executor 없이 사용 가능

내부 channel은 bounded channel이 좋고, animation tick이나 resize처럼 합칠 수 있는 event는 coalescing해야 해.

---

### 11. 이벤트는 Capture/Bubble보다 Command Routing 우선

웹 DOM의 완전한 capture/bubble 모델을 그대로 가져올 필요는 없어.

기본 우선순위를 명시적으로 둔다.

```text
1. System handler
2. Active modal scope
3. Focused node
4. Ancestor component
5. Screen/application handler
6. Default behavior
```

처리 결과는 bool 하나가 아니라 구조체로 둔다.

```rust
struct EventResponse<Msg> {
    message: Option<Msg>,
    propagation: Propagation,
    focus: Option<FocusRequest>,
    invalidation: Invalidation,
}
```

```rust
enum Propagation {
    Continue,
    Stop,
}

enum Invalidation {
    None,
    Paint,
    Layout,
    Tree,
}
```

이러면 이벤트 하나가 왜 render 또는 layout을 유발했는지 profiler가 추적할 수 있어.

---

### 12. Component 구조

Primitive와 Component를 분리해야 해.

#### Primitive widget

엔진이 직접 알고 있는 최소 요소:

```text
Text
Container
Flex
Grid
Scroll
Input
Canvas
Overlay
```

#### Behavior primitive

```text
Focusable
Clickable
Selectable
Scrollable
Editable
Draggable
```

#### Compound component

primitive를 조합해 만든 것:

```text
Button
Checkbox
Select
Table
Tree
Tabs
Modal
Dialog
Toast
FormField
CommandPalette
FilePicker
```

예를 들어 `Button`이 core paint engine에 박혀 있을 필요는 없어.

```rust
fn button<Msg>(label: impl Into<Text>, msg: Msg) -> impl View<Msg> {
    container(text(label))
        .class("button")
        .focusable()
        .on_activate(msg)
}
```

이렇게 해야 100개 component가 있어도 core complexity는 크게 증가하지 않아.

---

### 13. Crate 구성

내가 처음 만들면 다음 정도로 나눠.

```text
revue/
├── revue-core
│   ├── model/message runtime
│   ├── view traits
│   ├── node identity
│   └── commands
│
├── revue-tree
│   ├── runtime tree
│   ├── reconciliation
│   ├── generational arena
│   └── lifecycle
│
├── revue-layout
│   ├── flex
│   ├── grid
│   ├── intrinsic measurement
│   └── integer geometry
│
├── revue-style
│   ├── typed style IR
│   ├── computed style
│   └── themes
│
├── revue-render
│   ├── paint context
│   ├── clipping
│   ├── layers
│   └── ratatui buffer adapter
│
├── revue-events
│   ├── keyboard/mouse
│   ├── focus
│   ├── command routing
│   └── shortcuts
│
├── revue-components
│   ├── table
│   ├── tree
│   ├── input
│   ├── modal
│   └── ...
│
├── revue-css
│   ├── parser
│   ├── selector
│   ├── cascade
│   └── hot reload
│
├── revue-async
│   ├── command executor
│   ├── cancellation
│   └── task scope
│
├── revue-devtools
├── revue-testing
├── revue-macros
├── revue-ratatui
└── revue
    └── curated facade/prelude
```

하지만 처음부터 전부 만드는 건 아니야.

#### 1단계

```text
revue-core
revue-runtime
revue-components
revue-testing
```

#### 2단계

```text
revue-style
revue-async
revue-devtools
```

#### 3단계

```text
revue-css
revue-macros
hot reload
plugin ecosystem
```

---

### 14. 내부 저장소는 Generational Arena

Retained runtime tree의 node를 `Rc<RefCell<Node>>` 연결 리스트로 만들지 않을 거야.

```rust
struct NodeId {
    index: u32,
    generation: u32,
}
```

```rust
struct Runtime {
    nodes: Arena<RuntimeNode>,
    root: NodeId,
    focus: FocusManager,
    scheduler: Scheduler,
}
```

장점:

- stable ID
- stale handle 감지
- parent/child traversal 용이
- node 이동 시 포인터 불안정 없음
- `Rc<RefCell>` 확산 방지
- serialization·inspector 구현 용이

Tree mutation은 transaction으로 제한한다.

```text
Build Diff
   ↓
Mutation List
   ↓
Validate
   ↓
Commit
   ↓
Lifecycle
   ↓
Invalidation
```

---

### 15. Dirty flag는 처음부터 설계

```rust
bitflags! {
    struct DirtyFlags: u8 {
        const RECONCILE = 1 << 0;
        const STYLE     = 1 << 1;
        const MEASURE   = 1 << 2;
        const LAYOUT    = 1 << 3;
        const PAINT     = 1 << 4;
    }
}
```

변경 종류에 따라 최소 범위만 무효화한다.

```text
text color 변경
    → PAINT

padding 변경
    → MEASURE + LAYOUT + PAINT

child 추가
    → RECONCILE + MEASURE + LAYOUT + PAINT

focus 변경
    → STYLE 또는 PAINT

scroll offset 변경
    → PAINT
```

하지만 초기 버전부터 지나친 incremental optimization을 넣지는 않을 거야.

처음에는:

```text
View 전체 생성
→ tree 전체 reconcile
→ layout 전체
→ paint 전체
```

로 correctness를 먼저 확보하고, profiler로 실제 병목을 확인한 뒤 subtree invalidation을 추가한다.

아키텍처는 incremental할 수 있도록 만들되 구현은 단계적으로 하는 게 중요해.

---

### 16. 추천 dependency

정확한 crate 버전은 계속 변하므로 API 범주 기준으로 보면:

```toml
[dependencies]
ratatui = "..."
crossterm = "..."

unicode-segmentation = "..."
unicode-width = "..."
smallvec = "..."
slotmap = "..."          # 또는 자체 generational arena
bitflags = "..."
thiserror = "..."
tracing = "..."

tokio = { version = "...", optional = true }
serde = { version = "...", optional = true }
```

테스트:

```toml
[dev-dependencies]
insta = "..."
proptest = "..."
criterion = "..."
```

Fuzz:

```text
cargo-fuzz
```

CSS parser는 처음부터 거대한 브라우저용 parser를 끌어오기보다 지원 문법을 먼저 확정하고 선택하는 게 좋아. Revue가 지원하지 않을 CSS semantics까지 dependency를 통해 암묵적으로 약속하면 안 된다.

---

### 17. Proc macro와 JSX 비슷한 DSL은 늦게

처음부터 다음 같은 macro를 만들고 싶어질 수 있어.

```rust
view! {
    <Column>
        <Text>{model.title}</Text>
        <Button on_press={Msg::Save}>
            "Save"
        </Button>
    </Column>
}
```

하지만 초기에 이걸 넣으면:

- 실제 API 결함이 macro에 숨음
- 오류 진단이 어려움
- compile time 증가
- manual API와 macro API가 갈라짐
- AI가 생성한 코드가 지나치게 macro 중심이 됨

먼저 plain Rust API를 완성해야 해.

```rust
column((
    text(&model.title),
    button("Save").on_press(Msg::Save),
))
```

그 후 macro는 이 API와 같은 View IR을 생성하는 sugar로만 추가한다.

---

### 18. 외부 사용 API 예시

```rust
use revue::prelude::*;

struct App {
    files: Vec<FileEntry>,
    selected: usize,
    loading: bool,
}

#[derive(Clone)]
enum Msg {
    MoveUp,
    MoveDown,
    Open,
    Refresh,
    Loaded(Result<Vec<FileEntry>, LoadError>),
    Quit,
}

impl RevueApp for App {
    type Message = Msg;

    fn update(&mut self, msg: Msg) -> Command<Msg> {
        match msg {
            Msg::MoveUp => {
                self.selected = self.selected.saturating_sub(1);
                Command::none()
            }

            Msg::MoveDown => {
                self.selected =
                    (self.selected + 1).min(self.files.len().saturating_sub(1));
                Command::none()
            }

            Msg::Refresh => {
                self.loading = true;
                Command::perform(load_files(), Msg::Loaded)
            }

            Msg::Loaded(result) => {
                self.loading = false;
                self.files = result.unwrap_or_default();
                Command::none()
            }

            Msg::Quit => Command::quit(),

            Msg::Open => Command::none(),
        }
    }

    fn view(&self) -> impl View<Msg> {
        column((
            header("Files"),
            file_table(&self.files)
                .selected(self.selected)
                .on_submit(Msg::Open),
            status_bar(if self.loading {
                "Loading..."
            } else {
                "Ready"
            }),
        ))
        .on_key(key::UP, Msg::MoveUp)
        .on_key(key::DOWN, Msg::MoveDown)
        .on_key(key::CHAR_R, Msg::Refresh)
        .on_key(key::CHAR_Q, Msg::Quit)
    }
}
```

이 정도로 사용자는 쉽게 쓸 수 있어야 하고, 내부적으로만 Runtime Tree와 reconciliation이 작동해야 해.

---

### 19. 내가 선택하지 않을 것

#### Dioxus를 직접 기반으로 하지 않음

Dioxus식 component와 signal API에서 아이디어는 얻을 수 있지만, TUI 프레임워크 core를 Dioxus runtime에 종속시키지는 않을 거야.

이유:

- TUI에는 훨씬 단순한 scheduler로 충분함
- framework 내부 디버깅이 어려워짐
- external runtime lifecycle에 종속됨
- DOM/RSX 중심 mental model이 terminal primitive와 완전히 맞지는 않음

#### Tokio를 필수 dependency로 만들지 않음

TUI 전체가 network app인 것은 아니기 때문이야.

#### CSS를 core contract로 만들지 않음

Style IR이 contract이고 CSS는 frontend여야 해.

#### 모든 widget을 native primitive로 만들지 않음

대부분 compound component로 둔다.

#### 전역 Signal Runtime을 만들지 않음

App instance와 Component scope가 runtime을 소유해야 해.

#### plugin ABI를 초기에 만들지 않음

Rust trait 기반 plugin API도 core API가 안정되기 전에는 오히려 리팩터링을 막는다.

---

### 20. Revue 2.0의 한 문장 정의

내가 처음부터 만든다면 README 첫 문장은 이걸로 갈 거야.

> **Revue is a declarative Rust TUI framework with an explicit application model and an incremental retained runtime.**

한국어로 하면:

> **Revue는 명시적인 애플리케이션 상태 모델과 증분 retained runtime을 결합한 선언형 Rust TUI 프레임워크다.**

`retained framework`가 아니라 **retained runtime**이라는 게 중요해.

---

### 최종 추천안

#### 외부 프로그래밍 모델

```text
Elm Architecture
Model → Message → Update → View
```

#### View 표현

```text
가볍고 매 render마다 생성되는 declarative View Tree
```

#### 내부 엔진

```text
Keyed retained Runtime Tree
```

#### 반응성

```text
Application state: explicit messages
Local UI state: optional signals
```

#### 렌더링

```text
Own layout/style/paint abstraction
→ Ratatui Buffer
→ Crossterm
```

#### 스타일

```text
Typed Style IR
+ optional CSS frontend
```

#### 비동기

```text
Command/Task
→ bounded channel
→ UI-thread message commit
```

#### 위젯 전략

```text
7~10개 engine primitives
15~25개 stable base components
나머지는 compound components와 recipes
```

내 판단으로 이 구조가 **Rust의 ownership과 TUI의 단순함을 살리면서도, Revue가 원하는 rich framework 경험을 유지하는 가장 균형 잡힌 아키텍처**야.

현재 Revue를 완전히 버리고 재작성할 필요까지는 없지만, 새 브랜치를 판다면 UI부터 옮기지 말고 먼저 `Model–Msg–Update–Command`, `ViewNode`, `RuntimeNode`, `NodeId`, `DirtyFlags` 다섯 가지 계약을 만들어 보는 게 맞아. 이 다섯 가지가 깔끔하게 맞으면 나머지는 확장이고, 여기서 꼬이면 widget을 아무리 추가해도 다시 복잡해질 거야.

---

## 3. Ratatui를 쓰지 않는다면

> **질문:** 라따뚜이 사용하고 싶지 않는데?

그럼 방향이 완전히 달라져. **Ratatui를 대체하는 프레임워크**가 아니라, terminal I/O·입력 파서·cell buffer·diff renderer까지 직접 가진 독립 엔진으로 설계해야 해.

그렇다면 **Ratatui를 빼는 편이 오히려 Revue의 정체성에는 더 맞을 수 있어.**

Ratatui를 쓰면 결국 Revue가 아무리 상위 프레임워크를 잘 만들어도 아래쪽에는 계속 Ratatui의 전제가 남아.

- 매 프레임 `Frame`과 `Buffer`에 그리는 immediate-mode 모델
- Ratatui의 `Widget`·`StatefulWidget` 생태계
- Ratatui의 `Rect`, `Cell`, `Style`
- Ratatui 방식의 전체 화면 구성
- Ratatui 호환성을 위한 abstraction leakage

Revue가 독립적인 retained/incremental UI 엔진을 목표로 한다면, Ratatui는 꽤 애매한 중간 계층이 될 수 있어.

### 내가 새로 짠다면

#### 추천 구조

```text
OS terminal
    │
    ▼
Terminal Platform Layer
    │
    ├─ input decoding
    ├─ raw mode
    ├─ alternate screen
    ├─ terminal capability
    └─ output batching
    │
    ▼
Cell Surface
    │
    ├─ current surface
    ├─ previous surface
    ├─ damage tracking
    └─ diff renderer
    │
    ▼
Paint Engine
    │
    ├─ clipping
    ├─ layers
    ├─ text shaping
    └─ paint commands
    │
    ▼
Layout / Style / Runtime Tree
    │
    ▼
Declarative View API
    │
    ▼
Model – Message – Update
```

여기서 외부 crate는 **terminal 자체와 Unicode 처리에만 제한적으로 사용**한다.

---

### 추천 기술 스택

#### 1. Terminal I/O: Crossterm은 사용 가능

Ratatui는 사용하지 않더라도 Crossterm까지 버릴 필요는 없어.

Crossterm은 UI 프레임워크가 아니라 다음을 제공하는 비교적 얇은 terminal manipulation layer야.

- raw mode
- alternate screen
- cursor 이동
- terminal 크기
- key/mouse/resize event
- 색상과 terminal command
- Unix/Windows 추상화

공식 문서도 Crossterm을 크로스플랫폼 terminal manipulation library로 정의한다. citeturn441667search0turn441667search7

따라서 첫 버전은:

```text
Revue Engine
    ↓
revue-terminal-crossterm
    ↓
Crossterm
```

이렇게 두는 게 현실적이야.

중요한 건 Crossterm 타입을 core에 노출하지 않는 거야.

```rust
pub trait TerminalBackend {
    fn size(&self) -> Result<ScreenSize, TerminalError>;
    fn poll_event(&mut self, timeout: Duration)
        -> Result<Option<RawEvent>, TerminalError>;
    fn write(&mut self, commands: &[TerminalCommand])
        -> Result<(), TerminalError>;
    fn flush(&mut self) -> Result<(), TerminalError>;
}
```

Crossterm adapter만 별도로 구현한다.

```rust
pub struct CrosstermBackend {
    stdout: Stdout,
}
```

그러면 나중에 다음 backend도 가능해.

```text
revue-terminal-crossterm
revue-terminal-termwiz
revue-terminal-ansi
revue-terminal-test
revue-terminal-web
```

---

#### 2. Termwiz를 core로 사용하는 선택

Crossterm보다 더 높은 수준의 terminal abstraction이 필요하면 `termwiz`도 후보야.

Termwiz는 terminal 출력뿐 아니라 terminal emulator 구현까지 염두에 둔 라이브러리로, terminal, surface, escape sequence 및 line editing 기능을 제공한다. 다만 공식 문서 자체가 활발한 개발과 큰 API 변경 가능성을 경고하고 있다. citeturn441667search2turn441667search8

그래서 내 선택은:

```text
기본 backend       Crossterm
선택 backend       Termwiz
Revue core         둘 중 어느 쪽에도 종속되지 않음
```

이야.

Termwiz를 바로 core로 잡으면 기능은 풍부하지만 Revue가 WezTerm 계열의 비교적 큰 dependency와 설계에 끌려갈 수 있어.

---

### 3. 자체 Cell Surface

Ratatui를 빼려면 Revue가 반드시 직접 소유해야 하는 가장 중요한 부분이야.

```rust
pub struct Surface {
    width: u16,
    height: u16,
    cells: Vec<Cell>,
}

pub struct Cell {
    symbol: Glyph,
    style: StyleId,
    flags: CellFlags,
}
```

하지만 단순히 `char` 하나를 `Cell`에 넣으면 안 돼.

```rust
pub enum Glyph {
    Empty,
    Inline(InlineGlyph),
    Interned(GlyphId),
    Continuation,
}
```

필요한 상태:

```rust
bitflags! {
    pub struct CellFlags: u8 {
        const WIDE_START   = 1 << 0;
        const CONTINUATION = 1 << 1;
        const DIRTY        = 1 << 2;
        const TRANSPARENT  = 1 << 3;
    }
}
```

#### 왜 직접 Surface를 가져야 하나

그래야 다음을 Revue 기준으로 최적화할 수 있어.

- retained node별 damage tracking
- wide character 원자적 갱신
- layer 합성
- transparent overlay
- style interning
- unchanged row 생략
- scroll region 최적화
- terminal write batching
- profiler의 cell 변경 수 추적

Ratatui를 사용하면 최종 buffer는 Ratatui 소유 모델에 맞춰야 하지만, 직접 만들면 runtime tree와 renderer를 훨씬 밀접하게 최적화할 수 있어.

---

### 4. Double Buffer보다는 Damage-aware Surface

초기 구현은 두 개의 surface로 시작하면 돼.

```rust
pub struct Renderer {
    front: Surface,
    back: Surface,
}
```

렌더 과정:

```text
back 초기화
    ↓
paint
    ↓
front와 back 비교
    ↓
terminal commands 생성
    ↓
flush
    ↓
front/back 교체
```

하지만 장기적으로는 전체 surface 비교를 기본 아키텍처로 삼지 않을 거야.

```rust
pub struct DamageRegion {
    rows: Vec<RowDamage>,
}

pub struct RowDamage {
    start: u16,
    end: u16,
}
```

Runtime node가 paint invalidation을 발생시키면 해당 영역만 damage에 추가한다.

```text
Node paint invalidated
    ↓
old bounds ∪ new bounds
    ↓
DamageRegion
    ↓
damaged rows만 repaint
    ↓
damaged cells만 diff
```

터미널 자체는 cell grid라서 복잡한 임의 다각형 damage는 필요 없고, row별 구간이나 rectangle 집합이면 충분해.

---

### 5. 출력 엔진은 자체 ANSI command compiler

Crossterm의 command를 매 cell마다 생성하기보다는 내부 출력 IR을 둬.

```rust
pub enum TerminalOp {
    MoveTo { x: u16, y: u16 },
    SetStyle(ResolvedTerminalStyle),
    Print(GlyphRun),
    ClearToEndOfLine,
    HideCursor,
    ShowCursor,
}
```

Diff renderer는 변경된 cell을 연속 run으로 합친다.

```text
Cell 10 변경
Cell 11 변경
Cell 12 변경
Cell 13 동일
Cell 14 변경
```

결과:

```text
MoveTo(10)
SetStyle(...)
Print(cells 10..12)

MoveTo(14)
Print(cell 14)
```

추가 최적화:

- 현재 cursor 위치 추적
- 현재 foreground/background/modifier 추적
- 같은 style run 합치기
- trailing blanks는 `ClearToEndOfLine`
- 동일 row에서 불필요한 cursor 이동 제거
- full reset 대신 변경된 style만 출력
- 한 frame을 하나의 output buffer로 작성

```rust
pub struct AnsiEncoder {
    output: Vec<u8>,
    cursor: CursorState,
    style: TerminalStyle,
}
```

이 부분이 Revue의 성능 차별화 포인트가 될 수 있어.

---

### 6. 입력은 Crossterm Event를 그대로 쓰지 않기

Crossterm backend가 이벤트를 읽더라도 즉시 Revue의 normalized event로 변환해야 해.

```rust
pub enum InputEvent {
    Key(KeyEvent),
    Mouse(MouseEvent),
    Paste(PasteEvent),
    FocusGained,
    FocusLost,
    Resize(ScreenSize),
    Tick(Instant),
}
```

```rust
pub struct KeyEvent {
    pub code: KeyCode,
    pub modifiers: Modifiers,
    pub kind: KeyEventKind,
    pub physical: Option<PhysicalKey>,
    pub text: Option<SmolStr>,
}
```

Backend-specific event는 adapter 바깥으로 나오면 안 돼.

```text
crossterm::event::Event
    ↓
CrosstermEventDecoder
    ↓
revue_core::InputEvent
```

이렇게 해야 나중에 kitty keyboard protocol, enhanced key protocol, bracketed paste 같은 기능을 추가할 수 있어.

---

### 7. `vte`는 어디에 사용하는가

`vte`는 일반적인 TUI keyboard input parser의 직접 대체재라기보다, terminal emulator용 ANSI parser에 가깝다. 공식 설명도 Paul Williams의 ANSI parser state machine을 기반으로 virtual terminal emulator 구현을 위한 parser라고 한다. citeturn441667search4turn441667search17

따라서 Revue 기본 입력에 무조건 넣지는 않을 거야.

좋은 사용처는:

- embedded terminal widget
- subprocess output를 terminal처럼 렌더링
- ANSI log viewer
- PTY pane
- test terminal emulator

예를 들어:

```text
revue-terminal-widget
    ├─ portable-pty
    ├─ vte parser
    └─ virtual terminal screen
```

Revue 전체 terminal I/O와 PTY 안에서 실행되는 terminal emulator는 서로 다른 문제야.

---

### 8. Unicode/Text Engine

Ratatui를 빼면 이 부분도 직접 책임져야 해.

최소 dependency:

```text
unicode-segmentation
unicode-width
unicode-bidi       선택적
unicode-normalization 선택적
```

`unicode-width`는 Unicode Annex #11 등을 기준으로 terminal display width를 계산한다. citeturn441667search5turn441667search12

하지만 `unicode-width`만 호출하면 모든 문제가 해결되지는 않아.

처리 단위는 `char`가 아니라 grapheme cluster여야 해.

```rust
pub struct TextRun<'a> {
    text: &'a str,
    style: StyleId,
}

pub struct ShapedGlyph {
    grapheme_range: Range<usize>,
    display_width: u8,
}
```

과정:

```text
UTF-8 string
    ↓
grapheme segmentation
    ↓
control character filtering
    ↓
display width 계산
    ↓
line breaking
    ↓
clipping / ellipsis
    ↓
cell placement
```

반드시 별도 정책이 필요한 대상:

- 한글
- CJK
- emoji
- variation selector
- combining mark
- ZWJ emoji
- tab
- newline
- zero-width character
- control sequence
- 폭 2 문자의 clipping

#### 현실적인 제한

터미널마다 emoji 폭을 다르게 처리할 수 있어서 완벽한 결과는 불가능해. 그래서 width policy를 설정 가능하게 해야 해.

```rust
pub enum WidthPolicy {
    Unicode,
    Cjk,
    TerminalReported,
    Custom(Arc<dyn WidthResolver>),
}
```

---

### 9. Core Runtime 아키텍처

Ratatui를 안 쓰더라도 앞서 이야기한 상위 구조는 유지해.

```text
Model
  ↓
Message
  ↓
Update
  ↓
View Tree
  ↓
Reconciliation
  ↓
Runtime Tree
  ↓
Style
  ↓
Layout
  ↓
Paint Commands
  ↓
Surface
  ↓
Diff Renderer
  ↓
Terminal Backend
```

#### View Tree

```rust
pub trait View<Msg> {
    type State;

    fn build(
        &self,
        cx: &mut BuildCx<'_>,
    ) -> NodeId;

    fn rebuild(
        &self,
        previous: &Self,
        state: &mut Self::State,
        node: NodeId,
        cx: &mut RebuildCx<'_>,
    );
}
```

다만 Xilem식 trait를 그대로 복제할 필요는 없고, 초기에는 단순 enum IR로 시작해도 돼.

```rust
pub enum ViewNode<Msg> {
    Text(TextView),
    Container(ContainerView<Msg>),
    Flex(FlexView<Msg>),
    Input(InputView<Msg>),
    Custom(Box<dyn CustomView<Msg>>),
}
```

#### Runtime Tree

```rust
pub struct RuntimeNode {
    id: NodeId,
    parent: Option<NodeId>,
    children: SmallVec<[NodeId; 4]>,

    behavior: BehaviorSet,
    state: RuntimeState,

    style: ComputedStyle,
    layout: LayoutBox,
    previous_layout: LayoutBox,

    dirty: DirtyFlags,
}
```

---

### 10. Paint IR

Widget이 직접 `Surface`에 쓰게 하지 않는 것도 고려할 만해.

```rust
pub enum PaintCommand {
    FillRect {
        rect: Rect,
        glyph: Glyph,
        style: StyleId,
    },
    Text {
        origin: Point,
        layout: TextLayoutId,
        clip: Rect,
    },
    Border {
        rect: Rect,
        border: Border,
        style: StyleId,
    },
    PushClip(Rect),
    PopClip,
    PushLayer(LayerId),
    PopLayer,
}
```

장점:

- paint command inspection
- replay
- snapshot
- layer 처리
- backend 독립성
- profiler
- headless rendering
- future web renderer

단점:

- command allocation
- 별도 traversal
- 작은 UI에서 overhead

그래서 실제 구현에서는 persistent `Vec<PaintCommand>`보다 frame arena를 쓰는 게 좋아.

```rust
pub struct PaintList<'frame> {
    commands: BumpVec<'frame, PaintCommand<'frame>>,
}
```

또는 hot path에서는 바로 surface에 paint하되, devtools 빌드에서만 command logging을 추가할 수 있어.

내 선택은:

```text
초기 버전     PaintContext → Surface 직접 기록
추후          선택적 Paint recording
```

이야.

---

### 11. Layout Engine

Ratatui를 안 쓰면 layout도 독립적으로 가져가야 해.

#### 외부 layout crate를 사용할까?

`taffy` 같은 layout engine을 고려할 수 있지만, 내 추천은 **초기에는 사용하지 않는 것**이야.

GUI/web layout engine은 보통 floating-point size, pixel geometry, CSS Flexbox/Grid semantics를 중심으로 설계돼. Revue는 terminal cell 기반이고 다음 특성이 있어.

- 정수 width/height
- text intrinsic size가 중요
- 폭 변화에 따라 높이가 변함
- CJK와 grapheme width
- scroll viewport
- zero-size terminal
- border가 한 cell을 차지함

따라서 제한된 자체 layout protocol이 더 통제하기 쉬워.

```rust
pub trait LayoutNode {
    fn measure(
        &mut self,
        constraints: Constraints,
        cx: &mut MeasureCx<'_>,
    ) -> Size;

    fn layout(
        &mut self,
        rect: Rect,
        cx: &mut LayoutCx<'_>,
    );
}
```

```rust
pub struct Constraints {
    pub min: Size,
    pub max: Size,
}
```

기본 container:

```text
Stack
FlexRow
FlexColumn
Grid
Overlay
Scroll
Absolute
```

CSS Flex/Grid 완전 호환을 목표로 하지 말고 **Revue layout semantics**를 정의하는 게 좋아.

---

### 12. 스타일 엔진

Ratatui `Style`도 사용하지 않아야 해.

```rust
pub struct Style {
    pub foreground: Option<Color>,
    pub background: Option<Color>,
    pub attributes: TextAttributes,
    pub border: BorderStyle,
    pub padding: Edges<u16>,
    pub margin: Edges<u16>,
    pub width: Length,
    pub height: Length,
    pub overflow: Overflow,
}
```

렌더 단계에서 terminal capability에 따라 변환한다.

```text
ComputedStyle
    ↓ TerminalCapabilities
ResolvedTerminalStyle
    ↓ Backend
ANSI / Crossterm commands
```

```rust
pub struct TerminalCapabilities {
    pub colors: ColorCapability,
    pub italic: bool,
    pub underline_styles: bool,
    pub hyperlinks: bool,
    pub synchronized_output: bool,
    pub kitty_keyboard: bool,
}
```

이 계층을 두면 24-bit 색상이 없는 terminal에서 자동 downgrade할 수 있어.

---

### 13. 추천 crate 구조

```text
revue/
├── revue
│   └── public facade
│
├── revue-core
│   ├── geometry
│   ├── event
│   ├── message
│   ├── command
│   └── application
│
├── revue-terminal
│   ├── backend trait
│   ├── capabilities
│   ├── terminal session
│   └── terminal operations
│
├── revue-terminal-crossterm
│   └── Crossterm adapter
│
├── revue-surface
│   ├── cell
│   ├── glyph
│   ├── surface
│   ├── damage
│   └── diff
│
├── revue-text
│   ├── grapheme
│   ├── width
│   ├── wrapping
│   ├── clipping
│   └── text layout
│
├── revue-tree
│   ├── node arena
│   ├── reconciliation
│   ├── lifecycle
│   └── dirty propagation
│
├── revue-layout
│   ├── measure
│   ├── flex
│   ├── grid
│   ├── overlay
│   └── scroll
│
├── revue-style
│   ├── typed style
│   ├── theme
│   ├── computed style
│   └── capability resolution
│
├── revue-events
│   ├── focus
│   ├── shortcut
│   ├── routing
│   └── input normalization
│
├── revue-widgets
│   ├── primitives
│   └── compound components
│
├── revue-css
├── revue-async
├── revue-testing
├── revue-devtools
└── revue-macros
```

실제 repository를 처음부터 이렇게 모두 쪼개지는 말고, workspace 내부 모듈 경계만 먼저 잡고 안정된 뒤 crate로 분리하는 편이 좋아.

---

### 14. 초기 MVP 범위

처음부터 CSS, animation, 100개 widget을 넣으면 다시 현재 문제로 돌아가.

#### Phase 1: Terminal engine

```text
TerminalSession
InputEvent
Surface
Cell
Glyph
ANSI encoder
Diff renderer
Unicode text
Virtual terminal tests
```

성공 기준:

- 한글과 wide glyph가 깨지지 않음
- resize에 안전함
- panic 후 terminal 복구
- 동일 frame 재출력 시 output 0
- 변경된 row만 출력
- cursor와 style command 최소화

#### Phase 2: UI runtime

```text
Runtime Tree
NodeId
Key
View description
Reconciliation
Focus manager
Event routing
Layout protocol
```

성공 기준:

- keyed child 이동에도 상태 유지
- 제거된 node 상태 정리
- modal focus 격리
- scroll 위치 보존
- layout/paint invalidation 구분

#### Phase 3: Application model

```text
Model
Msg
Update
Command
Task cancellation
Scheduler
```

#### Phase 4: Rich layer

```text
Typed styling
CSS frontend
hot reload
animations
devtools
compound widgets
```

---

### 15. 직접 작성할 것과 빌릴 것

#### 직접 작성

```text
Cell / Surface
Damage tracking
Diff renderer
Paint context
Runtime tree
Reconciliation
Focus system
Event routing
Layout semantics
Style IR
Scheduler
Widget/component model
```

이게 Revue의 핵심 경쟁력이야.

#### 외부 crate 사용

```text
Crossterm             OS terminal abstraction
unicode-segmentation  grapheme segmentation
unicode-width         display width
smallvec              small child collections
slotmap 또는 자체 arena  generational ID
bitflags              dirty/cell flags
tracing               diagnostics
thiserror             typed errors
```

#### 선택적으로 사용

```text
vte                   embedded terminal widget
portable-pty          subprocess terminal
tokio                 async integration
serde                  state/devtools serialization
insta                  snapshot tests
proptest               state-machine properties
```

---

### 16. 가장 중요한 결정

Ratatui를 빼면 유혹이 두 가지 생겨.

#### 유혹 1: Ratatui를 다시 만들기

```text
Rect
Buffer
Widget trait
draw every frame
terminal diff
```

이 정도에서 끝나면 결국 이름만 다른 Ratatui가 돼.

Revue가 직접 renderer를 만드는 이유는 **상위 retained runtime의 invalidation 정보가 renderer까지 내려가게 하기 위해서**여야 해.

```text
Signal/Message 변경
    ↓
Node dirty reason
    ↓
Style/Layout/Paint 최소 invalidation
    ↓
Damage region
    ↓
Surface partial repaint
    ↓
Terminal minimal diff
```

이 연결이 있어야 독자 엔진의 가치가 생겨.

#### 유혹 2: 브라우저를 다시 만들기

반대로 DOM, CSSOM, 완전한 selector engine, 복잡한 layout, animation compositor까지 만들면 또 과해져.

따라서 경계는 이렇게 잡는 게 좋아.

```text
브라우저형 범용 문서 엔진      X
TUI 애플리케이션 엔진          O
```

---

### 결론

Ratatui를 사용하지 않는다면 내가 권하는 조합은 이거야.

```text
Rust
+ Crossterm adapter
+ 자체 TerminalBackend abstraction
+ 자체 Unicode-aware Cell Surface
+ 자체 Damage/Diff renderer
+ 자체 integer-cell layout
+ explicit Model–Msg–Update
+ ephemeral View Tree
+ retained Runtime Tree
+ typed Style IR
+ optional CSS
```

즉 Revue는 더 이상 “Ratatui 위의 프레임워크”가 아니라:

> **터미널 플랫폼 계층부터 독립적으로 소유하는 증분형 Rust UI 엔진**

이 되는 거야.

그리고 이 선택은 충분히 의미 있어. 다만 Ratatui를 안 쓰는 목표가 단순한 NIH가 아니라, 반드시 다음 차이를 만들어야 해.

```text
Retained tree의 dirty 정보가
layout → paint → cell damage → terminal output까지
끝까지 보존된다.
```

이게 구현된다면 Ratatui를 빼는 게 정당하고, 구현하지 않는다면 Ratatui의 buffer와 diff renderer를 다시 만드는 비용만 떠안게 돼.
