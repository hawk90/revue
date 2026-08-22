# 렌더 파이프라인 조사 결과

> **상태:** F-1·F-2·F-3은 `fix(render): always repaint from the view`로 해결됐다.
> F-4·F-5는 남아 있고, `tests/render_pipeline.rs`가 계속 고정하고 있다.
> 아래 조사 기록은 그대로 둔다 — 무엇이 왜 틀렸는지가 다음 단계의 근거다.

Phase 1을 기본 on으로 뒤집기 전에 `examples/`를 손으로 확인하려다 발견한 것들.
**Phase 1·2보다 큰 문제이고, 3.0 계획의 순서를 바꿔야 한다.**

카탈로그(`docs/anti-patterns/catalog.yaml`)가 "일어날 수 있는 실패 모드"의 목록인 것과 달리,
아래는 **재현해서 확인한 것**이다. 특성화 테스트는 `tests/render_pipeline.rs`에 있다.

## 요약

**출시된 예제에서 키를 눌러도 화면이 바뀌지 않는다.**

| | |
|---|---|
| 대상 | `examples/counter`, `examples/todo` (릴리스된 그대로) |
| 방법 | 실제 PTY 100×30, 자식 프로세스로 실행, 마스터에 키 바이트 주입 |
| 결과 | 키 입력당 터미널로 나가는 바이트 **0** |
| 대조 1 | 터미널 리사이즈 → **+2,861 바이트** (그리기 경로는 작동함) |
| 대조 2 | Ctrl+C → 프로세스 종료 (입력 경로도 작동함) |

두 예제 모두 핸들러가 해당 키에 `true`(= 다시 그려라)를 반환한다.

```
frame 1          : 5540 bytes
after Down       : +0 bytes
after Down       : +0 bytes
after Up         : +0 bytes
after space      : +0 bytes
after Tab        : +0 bytes
after resize     : +2861 bytes   <-- 그리기 경로 정상
Ctrl+C           : exited=True   <-- 입력 경로 정상
```

## 원인

렌더 파이프라인이 **서로 연결되지 않은 두 반쪽**으로 되어 있다.

| | 뷰 반쪽 | DOM 반쪽 |
|---|---|---|
| 무엇을 순회하나 | `root.render(ctx)` — 실제 위젯 트리 | `View::children()`로 만든 노드 트리 |
| 무엇을 결정하나 | **무엇을 어디에 그릴지** (`ctx.sub_area`) | **다시 그릴지, 어디를** (dirty rect, layout) |

DOM 반쪽이 "다시 그릴지"를 결정하는데, 그 반쪽은 위젯의 **내용**도 모르고 위젯 트리의
**대부분**도 모른다. 그래서 대개 "바뀐 것 없음"으로 판정한다.

### F-1. 내용만 바뀌면 다시 그려지지 않는다

첫 프레임 이후 `collect_dirty_regions`(`src/core/app/mod.rs`)는 DOM 노드의 dirty 플래그만
읽는다. dirty는 메타데이터 — id, class, 구조 — 가 바뀔 때 서고, **위젯의 텍스트는
`WidgetMeta`에 존재하지 않는다.** dirty rect가 비면 `render_to_buffer`는 이전 버퍼를
복사하고 렌더 없이 반환한다.

`PipelineHarness`로 반응성 시스템을 배제하고 확인했다 — Signal 없이 `Counter{n:0}` →
`{n:1}` → `{n:2}`를 넘겨도 화면은 `Count: 0`에 고정된다. `incremental_dom` on/off 무관.

강제 redraw가 서는 곳은 리사이즈, CSS 핫리로드, 그리고 public `App::request_redraw()`뿐이다.
**`request_redraw()`의 호출부는 `src/`에도 `examples/`에도 없다.**

### F-2. 구조가 바뀌면 버퍼에는 반영되지만 터미널에는 도달하지 않는다

구조 변경은 dirty rect를 만들므로 `render_to_buffer`가 돈다. 그런데 버퍼는 **뷰 전체를
순회해서** 칠하고, 뒤따르는 diff는 **dirty rect로 마스킹**된다. dirty rect는 레이아웃
엔진에서 오고, 위젯이 실제로 칠한 위치를 덮는다는 보장이 없다.

결과: 변경이 백버퍼에 들어가고 거기서 끝난다. **그 시점부터 두 버퍼는 서로 일치하고
터미널과만 불일치하므로, 이후 어떤 diff도 이를 복구할 수 없다.**

```
f1  screen="N0"      terminal_output=244
f2  screen="N0\nN9"  terminal_output=244   <-- 버퍼엔 있고 터미널엔 없음
```

### F-3. `request_redraw()`는 어긋난 터미널을 복구하지 못한다

`collect_dirty_regions`가 `needs_force_redraw`를 **소비해서** 전체 화면 dirty rect로
바꾸고 플래그를 끈다. 그 뒤 `draw_to_terminal`이 플래그를 읽을 때는 이미 false다. 그래서
diff 경로를 타는데, 그때는 두 버퍼가 이미 같은 내용이라 diff가 아무것도 찾지 못한다.

리사이즈만 빠져나가는 이유는 버퍼를 **리사이즈**해서 내용 자체가 달라지기 때문이다.

논리가 뒤집혀 있다 — `request_redraw()`는 *다른 무언가가 이미 dirty일 때만* 동작한다.

### F-4. DOM에는 `View::children()`으로 노출된 위젯만 들어간다

`render()` 안에서 트리를 조립하는 관용적 패턴 — 모든 튜토리얼과 예제가 그렇게 쓰여 있다 —
은 노드가 정확히 **1개**인 DOM을 만든다. 위젯 라이브러리 전체에서 `children()`을 구현한
것은 **`Stack` 하나뿐**이다.

즉 CSS 매칭, `:focus`/`:hover`, dirty-rect 추적, devtools가 전부 앱을 설명하지 않는 트리
위에서 돈다.

### F-5. 계산된 스타일이 자식 위젯에 전달되지 않는다

`DomRenderer::render`는 **루트 뷰에만** `RenderContext::style`을 채운다. `Stack::render`는
자식마다 `RenderContext::child_ctx_with_overflow(...)`를 새로 만드는데, 여기에는 스타일도
상태도 실리지 않는다.

CSS cascade는 `DomRenderer::styles`에 전부 계산되고 **아무도 읽지 않는다.** (참고: 위젯
파일 ~470개 중 `ctx.style`을 읽는 것은 9개다.)

## 3.0 계획에 미치는 영향

- **Phase 2는 지금 설 자리가 없다.** `NodeState`를 상호작용 상태의 유일한 소유자로 만들려는
  것인데, 실제 앱의 DOM에는 노드가 하나뿐이다. 소유할 노드가 없다.
- **Phase 1의 매 프레임 reconcile은 필요조건이지 충분조건이 아니다.** 켜도 1노드 트리를
  reconcile할 뿐이다. Phase 1은 옳고 유지하되, 그것만으로는 아무것도 고쳐지지 않는다.
- Phase 0의 invariant 테스트가 쉽게 통과한 이유의 일부도 이것이다.
  `inv06_unchanged_tree_is_not_dirty_after_draw`는 아무것도 dirty가 되지 않는 세계에서
  자명하게 참이다.

## 해결 — F-1·F-2·F-3

**렌더와 diff를 무조건 수행한다.**

- `render_to_buffer`는 항상 백버퍼를 비우고 뷰 전체를 그린다
- `draw_to_terminal`은 마스크 없이 두 버퍼 전체를 diff한다
- `collect_dirty_regions` / `collect_transition_rects` 삭제

버퍼에 칠하는 건 메모리 트래픽이고, 프레임의 비싼 부분은 터미널로 나가는 바이트다.
그건 버퍼 diff가 이미 정확히 처리한다 — 무변경 프레임은 0바이트, 한 글자 변경은 커서 이동
한 번과 글자 하나.

`needs_force_redraw`를 `collect_dirty_regions`가 소비하던 문제(F-3)도 그 함수가 사라지면서
해소됐다. 이제 `draw_to_terminal`만 읽고 끈다.

### 비용

`cargo bench --bench frame`, 120×40 화면:

| rows | 변경 있는 프레임 | 변경 없는 프레임 | 이전 |
|---:|---:|---:|---:|
| 10 | 20.5 µs | 19.6 µs | 14.6 µs |
| 50 | 31.5 µs | 28.7 µs | 14.6 µs |
| 200 | 55.8 µs | 52.8 µs | 14.6 µs |

**이전 수치가 행 개수와 무관하게 14.6 µs로 평평한 것 자체가 버그의 증거다** — 내용을
그리지 않았으니 내용에 비용이 들지 않았다. 14.6 µs는 순수 오버헤드(DOM reconcile, 레이아웃,
버퍼 복사)였다.

프레임당 1.3~3.8배 비싸졌다. 60fps 예산 16,667 µs에서 최악이 **0.34%**다. 그리기는 매 틱이
아니라 이벤트가 요청할 때만 일어난다.

영역 기반 스킵은 DOM이 실제 위젯 트리를 서술하게 된 뒤에 되돌아올 수 있다. 그전까지는
정확한 쪽이 영리한 쪽보다 낫다.

### 확인

수정 후 같은 PTY 하네스로:

```
counter:  frame 1 = 5093 bytes,  k → +127,  k → +102,  ↑ → +102
todo:     frame 1 = 5540 bytes,  ↓ → +100,  ↓ → +115,  ↑ → +115,  space → +81
```

캡처된 바이트에 `Count` 0→1→2→3, `Doubled` 0→2→4→6이 보이고, diff가 바뀐 글자만 쓴다.

## 남은 것 — F-4·F-5



F-4가 뿌리다. 그것이 풀리면 F-5는 따라오고, 영역 기반 렌더 스킵을 다시 켤 근거도 생긴다.

1. **DOM을 렌더 순회에서 짓는다.** `View::children()`이 아니라 `render()`가 실제로 만드는
   트리에서. `RenderContext`가 노드 커서를 들고 다니며, 위젯이 자식을 그릴 때 노드를
   등록하고 그 노드의 계산된 스타일·상태를 자식 컨텍스트에 실어준다. 이것이 F-4와 F-5를
   동시에 푼다.
2. **dirty를 실제로 칠해진 영역에서 얻는다.** 레이아웃이 예측한 rect가 아니라. 최소한
   diff 마스킹은 "칠해진 곳"의 상위집합이어야 한다 — 아니면 F-2가 남는다.
3. **내용 변화를 감지할 수단.** 노드가 생기면 reconcile이 감지할 수 있지만, `WidgetMeta`에
   내용 해시를 넣을지 `needs_render()`를 실제로 활용할지는 설계 판단이 필요하다.
**이 순서는 사람이 결정해야 한다.** 1번은 렌더/DOM 관계의 재설계이고, 되돌리기 어렵다.

## 재현

```bash
cargo test --test render_pipeline     # 리페인트 계약 4개 + 남은 결함 3개
cargo bench --bench frame             # 프레임 비용
```

PTY 스크립트는 세션 스크래치패드에 있었고 커밋하지 않았다. 요지는 다음과 같다.

- `pty.openpty()` + `fork` + `setsid` + `ioctl(TIOCSCTTY)` — python의 `pty.fork()`는
  `TIOCSCTTY`를 걸지 않아서 macOS에서 제어 터미널이 잡히지 않는다
- `ioctl(TIOCSWINSZ)`로 창 크기를 **반드시** 설정할 것 — 안 하면 0×0이라 앱이 아무것도
  그리지 않고, 그것을 버그로 오인하게 된다
- 생존 확인은 **Ctrl+C**로. `is_quit_key`는 `key.is_ctrl_c()`만 본다 (`'q'`가 아니다)

> 조사 중 위 두 가지로 두 번 잘못된 결론에 도달할 뻔했다. 하네스를 `cat`과 raw-mode `dd`로
> 먼저 검증한 뒤에야 신뢰할 수 있었다.
