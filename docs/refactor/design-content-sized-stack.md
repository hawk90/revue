# Stack은 자식의 내용 크기를 따라야 한다

> **상태: 구현 완료.** `View::measure`·`View::fills`와 자주 쓰는 위젯의 답은 2.x에,
> `content_sized` 기본 on은 3.0에 들어갔다. 아래 "순서" 참고.

예제 전수 대조([`findings-default-flip.md`](findings-default-flip.md)) 뒤 화면을 하나씩
읽다가 나온 것. **예제 35개 중 20여 개가 깨져 있었고, v1.0부터 그랬다.**

## 증상

```rust
vstack()
    .child(Text::new("Hello, Revue!"))
    .child(Text::new("Press q to quit"))
    .child(Text::new("v1.0"))
```

세 줄이 1행, 12행, 22행에 흩어진다. `hstack().child(Text::new("1 ")).child(Text::new(line))`은
줄 번호가 화면 절반을 가져간다. 탭 예제 여섯 개는 본문 테두리가 안쪽 3행을 받아 정작
보여주려던 위젯이 화면에 없다.

## 원인

`Stack::calculate_sizes`는 크기를 지정받지 않은 자식(`ChildSize::Auto`)에게 남은 공간을
**균등하게** 나눈다(`src/widget/layout/stack.rs`). 문서에도 그렇게 적혀 있다. 결함이 아니라
설계다.

그런데 예제 작성자 전원이 반대로 가정했다 — 웹의 블록 흐름, SwiftUI의 `VStack`, Flutter의
`Column`처럼 자식이 **자기 크기만큼** 차지한다고. 사용자도 같은 코드를 쓸 것이다. 예제를
`child_sized`로 고치는 것은 증상을 가릴 뿐이다.

엔진 쪽 L-3("auto 교차축 1셀 붕괴", [`findings-layout.md`](findings-layout.md))과 뿌리가
같다 — **내재 크기를 물을 수단이 없다.** 다만 이쪽은 엔진이 아니라 컨테이너 자신의 문제이고,
(B) 방침(흐름은 컨테이너가 결정) 안에서 풀 수 있다.

## 설계

### 1. 위젯이 자기 크기를 답한다 — 2.x, 추가만

```rust
pub trait View {
    /// The size this view wants within `max_width` x `max_height`, or `None`
    /// if it fills whatever it is given.
    fn measure(&self, max_width: u16, max_height: u16) -> Option<(u16, u16)> {
        None
    }
}
```

- 기본 `None` = "주는 만큼 채운다" — 지금 모든 위젯의 동작이다. 추가만 하므로 breaking이
  아니다. 같은 이름의 inherent 메서드는 크레이트에 없다(확인함)
- 먼저 답하는 위젯: `Text`(줄 수 × 가장 긴 줄의 표시 폭, 줄바꿈 설정 반영), `Border`(자식 +
  2, 제목 폭), `Stack`(주축 합 + gap, 교차축 최대 — 자식 중 하나라도 `None`이면 `None`),
  빈 `Text`, 구분선 류
- 나머지 위젯은 필요할 때 하나씩 답하게 한다. 답하지 않는 위젯은 지금처럼 채운다

#### 지금 답하는 위젯

답은 `render`가 실제로 칠하는 크기다(테두리·여백·괄호 포함, 폭은 표시 폭). 최대값을 넘지
않는다. 각 답은 `tests/widget_measure.rs`가 칠해진 영역과 대조하고, `height()` 헬퍼가 있는
위젯은 `tests/widget_size_agreement.rs`가 헬퍼와 대조한다.

| 종류 | 위젯 | 답 |
|---|---|---|
| 내용 크기 | `Button` | 라벨(+아이콘) + 좌우 2칸씩, `width()`보다 작지 않게 × 1행 |
| | `Badge`, `Tag` | 글자 + 좌우 여백/가장자리 × 1행 (`Badge::dot`은 1×1) |
| | `Checkbox`, `Switch`, `RadioGroup` | 상자/트랙 + 라벨. 포커스 표시(`> `, 오른쪽 `]`)가 칸을 더하면 포함 |
| | `Spinner`, `Rating`, `Slider` | 글리프/별/트랙 + 라벨·값. `Slider`는 `length` 고정, 눈금이 있으면 2행 |
| | `Link`, `Breadcrumb`, `RichText`, `StatusIndicator` | 보이는 글자 폭(`RichText`는 줄 수만큼 행) |
| | `Gauge` (Bar, Battery, Segments, Dots, Vertical, Thermometer) | `width`/`height`/`segments` + 제목 1행 |
| 가로로 늘어남 (`fills` = `WIDTH`) | `Input`, `Progress`, `Sparkline`, 가로 `Divider` | 주어진 폭 × 1행 (`Divider::length`가 있으면 그 길이, `fills` = `NONE`) |
| 세로로 늘어남 (`fills` = `HEIGHT`) | 세로 `Divider` | 1열 × 주어진 높이 |
| 헬퍼 재사용 | `Alert`, `Callout`, `EmptyState` | 주어진 폭 × `height()` (닫힌 `Alert`는 0×0) |
| | `StatusBar` | 주어진 폭 × `height` |
| | `BigText` | Figlet: 가장 긴 줄 × `height()`. 텍스트 크기 프로토콜: 주어진 폭 |
| | `Digits` | 가장 넓은 글리프 행 × `height()` (+ 접두/접미 1행) |
| | `Card` | 주어진 폭 × 테두리·제목·부제·헤더·구분선·본문 측정·푸터(+`Elevated` 그림자), 최소 3행. 본문이 `None`이면 `None` |
| 감싸기 | `ErrorBoundary` | 보여 주는 쪽(자식, 패닉 뒤에는 대체 뷰) |
| | `ZenMode`, `DebugOverlay` | 꺼져/숨겨져 있으면 안쪽 뷰, 켜져 있으면 `None` |

답하지 않는(채우는) 위젯: 리스트·테이블·트리·에디터·차트처럼 크기가 데이터와 스크롤에 달린
것, `Positioned`(부모 영역 안의 오프셋으로 놓으므로 자식 크기가 자기 크기가 아님),
`Layers`, `Gauge`의 Arc·Circle.

#### 어느 축을 채우는가 — `View::fills`

`measure`만으로는 "1행이지만 폭은 주는 만큼"을 말할 수 없다. 늘어나는 위젯이 주어진 폭
전부를 답하면 열(`vstack`)에서는 1행으로 옳지만, 행(`hstack`)에서는 그 폭이 고정 크기가 되어
뒤의 형제를 밀어낸다(`[` `Progress` `]`에서 `]`가 사라졌다).

그래서 `View::fills() -> Fill`(기본 `Fill::NONE`)이 **주는 만큼 차지하는 축**을 말한다.
`measure`는 다른 축의 자연 크기를 그대로 답한다. `Fill`은 `NONE`/`WIDTH`/`HEIGHT`/`BOTH`.

- `WIDTH`: `Input`, `Progress`, `Sparkline`, 가로 `Divider`, `Alert`(닫히지 않았을 때),
  `Callout`, `EmptyState`, `StatusBar`, `Card`, 텍스트 크기 프로토콜로 그리는 `BigText`
- `HEIGHT`: 세로 `Divider`. `length`가 있는 `Divider`는 `NONE`
- `Box<dyn View>`, `ErrorBoundary`, `ZenMode`, `DebugOverlay`는 `measure`와 같은 조건으로 전달
  (켜진 `ZenMode`, 보이는 `DebugOverlay`는 `BOTH`)
- `Stack::fills`: `child`로 넣은 자식 중 하나라도 채우는 축은 스택도 채운다. 그래서 `Input`을
  담은 `hstack`은 바깥 `hstack`에서도 남은 폭을 나눠 받는다. `child_sized`/`child_flex` 자식은
  세지 않는다

### 2. Stack이 그 답을 쓴다 — 3.0, 기본값 변경

`ChildSize::Auto` 자식에 대해:

| 자식의 `measure` | 주축 크기 |
|---|---|
| `Some((w, h))` | 그 크기 (남은 공간을 넘지 않게) |
| `Some(..)`, 그러나 `fills()`가 주축을 덮음 | `None`과 같다 — 남은 공간을 나눠 받는다 |
| `None` | 측정된 자식들을 뺀 나머지를 `None` 자식끼리 균등 분배 — 지금 규칙 그대로 |

- `child_sized` / `child_flex`는 그대로 우선한다
- `css_layout`에서 주축의 명시적 CSS 크기(열은 `height`, 행은 `width`)는 `fills`보다 우선한다
- 측정된 자식들과 고정 자식들의 합이 스택을 넘으면, **측정된 자식만** 각자의 크기에 비례해
  줄어든다(CSS `flex-shrink: 1`). 고정 자식(`child_sized`)은 줄지 않는다 — 빌더가 말한
  크기다. 주축에 CSS 명시 크기나 `min-*`이 있는 자식도 줄지 않는다 — 박스 모델이 그 크기를
  되돌려 놓으므로 슬롯만 줄면 다음 형제가 그 안에서 시작한다. 이것이 위 표의 "남은 공간을 넘지 않게"다: 내용이 한 줄 넘치는 본문이 그 줄을
  잃고, 뒤의 푸터가 화면 밖으로 밀리지 않는다(`examples/showcase`에서 본문이 1행 넘쳐
  푸터가 사라졌다). 줄바꿈하는 행(`flex-wrap`)은 줄이지 않고 다음 행으로 넘긴다
- 교차축은 지금처럼 전체를 준다(stretch)
- 2.x에서는 `Stack::content_sized(true)`로 옵트인, **3.0에서 기본 on**. `false`로 2.x 동작

이 규칙이면 `vstack().child(header).child(Border::new().child(list)).child(footer)`에서
`header`·`footer`는 한 줄씩, 측정하지 않는 `list`를 감싼 `Border`는 `None`이라 나머지
전부를 받는다 — 예제 작성자들이 기대한 그대로다.

### 3. `css_layout`과 함께 — 자식의 CSS 박스를 슬롯에 접어 넣는다

`css_layout`에서 CSS 박스 속성은 페인트 패스가 **스택이 이미 고른 area 위에**
적용한다(`box_model::apply`: 마진으로 들이고 → `height`/`width`로 바꾸고 → `min-*`/`max-*`로
자른다). 균등 분배에서는 그 area가 몫이라 괜찮았지만, 내용 크기에서는 area가 곧 내용
크기다. 그래서 한 줄짜리 `Text`에 `margin-top: 2`를 주면 한 줄을 0줄로 들여 글자가
사라지고, `height: 3`을 주면 한 줄 area가 세 줄로 자라 다음 형제를 덮었다.

**규칙: 스택은 예약하고, 박스 모델은 배치한다.** 내용 크기 스택은 `Auto` 자식의 주축
슬롯을 정할 때 그 자식의 계산된 스타일을 미리 읽어(`RenderContext::peek_child_styles`)
박스를 접어 넣는다.

1. `size` = 주축의 명시 크기(열이면 `height`, 행이면 `width`)가 있으면 그것, 없으면
   `measure` 결과. 측정은 박스 모델이 남겨 줄 교차축 폭과, 주축 마진을 뺀 길이 안에서 한다
2. `max-*`, 그다음 `min-*`으로 자른다 — 박스 모델과 같은 순서
3. 슬롯 = 앞 마진 + `size` + 뒤 마진

그러면 나중에 도는 박스 모델이 같은 마진으로 들여 `size`를 되찾고, 같은 명시 크기로
바꾸고 같은 경계로 잘라도 값이 변하지 않는다. **두 번 적용되는 것은 없다.** 교차축은
지금처럼 박스 모델에 맡긴다.

예외 — 아래 경우 자식은 접지 않고 채운다(`Auto`, 남은 공간의 균등 몫):

- 측정하지 않고 명시 크기도 없는 자식. 마진·경계는 몫 위에 적용된다(균등 분배와 같다)
- 주축 크기 중 하나라도 퍼센트인 자식. 퍼센트는 기준이 필요하고 박스 모델은 슬롯을
  기준으로 푼다 — 퍼센트에서 계산한 슬롯은 박스 모델을 거치면 반드시 달라진다. 몫이
  균등 분배 스택이 쓰던 기준이다

그 밖에:

- `display: none` 자식은 공간도 gap도 차지하지 않는다
- `child_sized` / `child_flex`는 여기에 오지 않는다. 빌더가 스타일시트보다 우선이므로
  슬롯은 빌더가 말한 크기이고, 박스 모델은 지금처럼 그 안에서 조정한다(CSS `height`가 그
  슬롯보다 크면 지금처럼 다음 형제 쪽으로 넘친다)
- 균등 분배 스택(`content_sized(false)`)은 스타일을 읽지 않는다 — 동작이 그대로다
- 줄바꿈하는 행(`flex-wrap`)도 읽지 않는다. 행이 넘치면 수집 패스가 중간에 멈출 수 있어
  그 뒤 자식의 노드 위치를 셈할 수 없기 때문이다

**엿보기가 건전한 이유.** 페인트 패스의 `next`는 다음 `render_child`가 소비할 노드를
가리키고, k번째 자식의 노드는 `next` + 자식 0..k의 `subtree_len` 합이다 —
`render_child_with_overflow`가 자식마다 재동기화할 때 쓰는 바로 그 산술이다. 커서는
움직이지 않는다. 스택은 `needs_render()`가 거짓인 자식(수집 때도 렌더하지 않아 노드가
없다)을 셈에서 뺀다.

**한계: `Stack::measure`는 자식의 CSS를 모른다.** `measure`에는 렌더 컨텍스트가 없어
계산된 스타일에 닿을 수 없다. 그래서 내용 크기 스택이 다른 스택 안에 들어가면 바깥은
안쪽 스택을 자식들의 맨 내용 크기로 재고, 접혀 들어가는 것은 안쪽 스택 **자신의** 박스뿐이다.
안쪽 자식에게 `margin-top`을 주면 바깥이 그만큼 덜 예약해 마지막 줄이 잘린다. 풀려면
`measure`가 스타일을 받아야 한다(엔진을 권위로 삼는 (A)에서 같이 풀 문제).

### 왜 엔진이 아니라 컨테이너인가

(A) 엔진을 배치의 권위로 삼기는 3.x로 미뤘다. 그 전제 조건인 "위젯이 배치 의도를 DOM에
싣는다"(L-2)와 "내재 크기"(L-3) 중 **내재 크기는 이 `measure`가 그대로 재료가 된다.**
(A)를 시작할 때 엔진의 auto 크기를 `measure`로 채우면 L-3이 풀린다. 버리는 작업이 아니다.

## 순서

1. 예제를 `child_sized`로 고친다 — 지금 깨진 화면을 바로잡는다(#681, #682, #683)
2. `View::measure` 추가, 기본 위젯이 답하게 함, `Stack::content_sized` 옵트인 — 2.x (이 문서와 같은 PR).
   자주 쓰는 위젯이 답하게 한 것은 그다음 PR — 위 "지금 답하는 위젯"
3. 3.0: `content_sized` 기본 on. [`docs/migration/v3.0.0.md`](../migration/v3.0.0.md) 2절 — **완료.**
   예제 전부를 PTY에서 기본값 전후로 대조했다. 본문이 내용 크기라 퍼짐에 기대던 화면
   (탭 예제의 본문, 나란한 패널, 상태 줄의 오른쪽 끝 항목)은 `child_flex`로 원래 모습을
   지켰고, 나머지는 텍스트·컨트롤이 붙어 쌓이는 쪽으로 바뀌었다

1에서 넣은 `child_sized`는 3.0 이후에도 옳다 — 명시한 크기는 계속 우선한다.
