# 위젯이 캐스케이드 위에 덧칠한다

계획서 **Phase 2-2**("위젯 읽기 경로 전환")의 첫 조각. 앞선 작업들이 상태를 DOM에 싣고
캐스케이드를 올바르게 돌게 만들었지만, **위젯이 그 결과를 묻지 않는 곳**이 남아 있었다.

## 증상

```css
#save:disabled { color: red; }
```

캐스케이드는 이걸 올바르게 계산한다 — `computed_color("save")`가 빨강을 답한다. 화면은
회색이다.

`WidgetState::resolve_fg`가 `disabled`가 서면 **`css_style`을 보기도 전에** 고정 회색을
반환했기 때문이다.

```rust
pub fn resolve_fg(&self, css_style: Option<&Style>, default: Color) -> Color {
    if self.disabled {
        return DISABLED_FG;   // css_style은 여기서 죽는다
    }
    ...
}
```

`resolve_bg`, `resolve_colors_interactive`, `state_colors` 네 곳에 같은 단축이 있었다.

## 무엇이 틀렸나 — 우선순위의 자리

색을 정할 수 있는 출처가 셋이고, 순서가 중요하다.

| | | CSS 대응물 |
|---|---|---|
| 빌더의 `.fg(색)` | 최상 | 인라인 `style=` 속성 |
| 매칭된 스타일시트 규칙 | 중간 | author stylesheet |
| 위젯 자신의 기본값 | 최하 | user-agent stylesheet |

**disabled 회색은 맨 아래 줄에 속한다.** 스타일시트가 아무 말도 하지 않을 때 위젯이 어떻게
보이는가지, 스타일시트를 침묵시키는 것이 아니다. `<button disabled>`가 브라우저 기본 회색으로
보이지만 `button:disabled { color: red }`를 쓰면 빨강이 되는 것과 같다.

**고침:** 단축을 없애고 기본값을 바꾼다.

```rust
let default = if self.disabled { DISABLED_FG } else { default };
```

한 줄이지만 우선순위 표 전체가 제자리를 찾는다.

## 유지한 것 — 상호작용 효과

`resolve_colors_interactive`의 단축에는 **옳은 부분이 하나 있었다.** 비활성 위젯은 포인터에
반응하면 안 된다. 그래서 캐스케이드가 정한 색은 그대로 받되, hover/press 틴트는 받지 않는다.

```rust
let fg = self.resolve_fg(css_style, default_fg);
let bg = self.resolve_bg(css_style, default_bg);
if self.disabled {
    return (fg, bg);          // 색은 받고, 틴트는 안 받는다
}
let bg = bg.with_interaction(self.pressed, self.hovered, self.focused);
```

## 동작 변경

**비활성 위젯에 매칭되는 `color` / `background` 규칙이 이제 적용된다.** 그런 규칙을 쓴 앱은
화면이 달라진다 — 다만 그건 그 규칙을 **의도적으로 썼는데 지금까지 무시당하고 있던** 경우다.
`.fg()` / `.bg()` 인라인 오버라이드도 마찬가지로 비활성 위젯에서 살아난다.

아무 규칙도 매칭되지 않으면 전과 똑같이 회색이다.

## 남은 것

이 크레이트의 위젯 ~470개 중 `ctx.style`을 읽는 것은 아직 소수다
(`findings-render-pipeline.md` F-5). 이번 것은 **공유 경로**(`WidgetState`) 하나를 고쳐
그걸 쓰는 모든 위젯이 한꺼번에 캐스케이드를 존중하게 만든 것이고, 자기 색을 직접 계산하는
위젯들은 카테고리 단위로 따로 옮겨야 한다.

`tests/cascade_precedence.rs`가 우선순위 표를 고정한다.
