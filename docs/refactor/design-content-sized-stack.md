# Stack은 자식의 내용 크기를 따라야 한다

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

### 2. Stack이 그 답을 쓴다 — 3.0, 기본값 변경

`ChildSize::Auto` 자식에 대해:

| 자식의 `measure` | 주축 크기 |
|---|---|
| `Some((w, h))` | 그 크기 (남은 공간을 넘지 않게) |
| `None` | 측정된 자식들을 뺀 나머지를 `None` 자식끼리 균등 분배 — 지금 규칙 그대로 |

- `child_sized` / `child_flex`는 그대로 우선한다
- 교차축은 지금처럼 전체를 준다(stretch)
- 3.0 전까지는 `Stack::content_sized(true)`로 옵트인, 3.0에서 기본 on. `false`로 2.x 동작

이 규칙이면 `vstack().child(header).child(Border::new().child(list)).child(footer)`에서
`header`·`footer`는 한 줄씩, 측정하지 않는 `list`를 감싼 `Border`는 `None`이라 나머지
전부를 받는다 — 예제 작성자들이 기대한 그대로다.

### 왜 엔진이 아니라 컨테이너인가

(A) 엔진을 배치의 권위로 삼기는 3.x로 미뤘다. 그 전제 조건인 "위젯이 배치 의도를 DOM에
싣는다"(L-2)와 "내재 크기"(L-3) 중 **내재 크기는 이 `measure`가 그대로 재료가 된다.**
(A)를 시작할 때 엔진의 auto 크기를 `measure`로 채우면 L-3이 풀린다. 버리는 작업이 아니다.

## 순서

1. 예제를 `child_sized`로 고친다 — 지금 깨진 화면을 바로잡는다(#681, #682, #683)
2. `View::measure` 추가, 기본 위젯이 답하게 함, `Stack::content_sized` 옵트인 — 2.x (이 문서와 같은 PR)
3. 3.0: `content_sized` 기본 on. [`docs/migration/v3.0.0.md`](../migration/v3.0.0.md)에 추가

1에서 넣은 `child_sized`는 3.0 이후에도 옳다 — 명시한 크기는 계속 우선한다.
