# 터미널 백엔드 및 이미지 프로토콜 확장 계획

## 현재 상태

### 터미널 백엔드
- ✅ Crossterm (유일한 백엔드, 하드코딩됨)

### 이미지 프로토콜
- ✅ Kitty Graphics Protocol
- ❌ Sixel
- ❌ iTerm2 Inline Images

---

## Phase 1: 터미널 백엔드 추상화

### 1.1 Backend trait 정의
**파일**: `src/render/backend/traits.rs` ✅ (생성됨)

```rust
pub trait Backend: Write {
    fn init(&mut self) -> Result<()>;
    fn init_with_mouse(&mut self, enable_mouse: bool) -> Result<()>;
    fn restore(&mut self) -> Result<()>;
    fn size(&self) -> Result<(u16, u16)>;
    fn clear(&mut self) -> Result<()>;
    fn hide_cursor(&mut self) -> Result<()>;
    fn show_cursor(&mut self) -> Result<()>;
    fn set_cursor(&mut self, x: u16, y: u16) -> Result<()>;
    fn set_fg(&mut self, color: Color) -> Result<()>;
    fn set_bg(&mut self, color: Color) -> Result<()>;
    fn reset_fg(&mut self) -> Result<()>;
    fn reset_bg(&mut self) -> Result<()>;
    fn set_modifier(&mut self, modifier: Modifier) -> Result<()>;
    fn reset_style(&mut self) -> Result<()>;
    fn enable_mouse(&mut self) -> Result<()>;
    fn disable_mouse(&mut self) -> Result<()>;
    fn write_hyperlink_start(&mut self, url: &str) -> Result<()>;
    fn write_hyperlink_end(&mut self) -> Result<()>;
    fn capabilities(&self) -> BackendCapabilities;
    fn name(&self) -> &'static str;
}
```

### 1.2 CrosstermBackend 구현
**파일**: `src/render/backend/crossterm.rs` ✅ (생성됨)

- 기존 `terminal.rs` 로직을 Backend trait으로 이전
- crossterm 의존성 사용
- feature flag: `crossterm-backend`

### 1.3 TermionBackend 구현
**파일**: `src/render/backend/termion.rs` ✅ (생성됨)

- Unix 전용 (`#[cfg(unix)]`)
- termion 의존성 사용
- feature flag: `termion-backend`
- 제한사항: 마우스 동적 토글 불가 (초기화 시점에 결정)

### 1.4 Terminal 리팩토링
**파일**: `src/render/terminal.rs`

```rust
// 변경 전
pub struct Terminal<W: Write> {
    writer: W,
    current: Buffer,
    ...
}

// 변경 후
pub struct Terminal<B: Backend> {
    backend: B,
    current: Buffer,
    ...
}

// 하위 호환성 유지
pub type CrosstermTerminal = Terminal<CrosstermBackend<io::Stdout>>;
```

---

## Phase 2: 이미지 프로토콜 확장

### 2.1 ImageProtocol enum 및 trait 정의
**파일**: `src/widget/image/protocol.rs`

```rust
/// 지원되는 이미지 프로토콜
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageProtocol {
    /// Kitty Graphics Protocol
    Kitty,
    /// Sixel Graphics
    Sixel,
    /// iTerm2 Inline Images
    ITerm2,
    /// 플레이스홀더 (이미지 미지원)
    None,
}

/// 이미지 인코더 trait
pub trait ImageEncoder {
    /// 이미지 데이터를 터미널 이스케이프 시퀀스로 인코딩
    fn encode(
        &self,
        data: &[u8],
        width: u32,
        height: u32,
        cols: u16,
        rows: u16,
    ) -> String;

    /// 프로토콜 이름
    fn name(&self) -> &'static str;
}

impl ImageProtocol {
    /// 환경에서 최적의 프로토콜 자동 감지
    pub fn detect() -> Self { ... }
}
```

### 2.2 Kitty 프로토콜 리팩토링
**파일**: `src/widget/image/kitty.rs`

- 기존 `image.rs`의 `kitty_escape()` 로직 분리
- `ImageEncoder` trait 구현

```rust
pub struct KittyEncoder;

impl ImageEncoder for KittyEncoder {
    fn encode(&self, data: &[u8], width: u32, height: u32, cols: u16, rows: u16) -> String {
        // 기존 kitty_escape() 로직
        // APC 시퀀스: \x1b_G...params...;base64_data\x1b\\
    }

    fn name(&self) -> &'static str { "kitty" }
}
```

### 2.3 Sixel 프로토콜 구현
**파일**: `src/widget/image/sixel.rs`

```rust
pub struct SixelEncoder {
    /// 팔레트 크기 (16, 256, 또는 무제한)
    palette_size: u16,
}

impl SixelEncoder {
    /// DCS 시퀀스 시작
    fn dcs_start(&self) -> String {
        // ESC P q
        "\x1bPq".to_string()
    }

    /// DCS 시퀀스 종료
    fn dcs_end(&self) -> String {
        // ESC \
        "\x1b\\".to_string()
    }

    /// RGB를 팔레트 인덱스로 변환
    fn rgb_to_palette(&self, r: u8, g: u8, b: u8) -> u8 { ... }

    /// 이미지를 Sixel 데이터로 변환
    fn encode_scanlines(&self, pixels: &[u8], width: u32, height: u32) -> String { ... }
}

impl ImageEncoder for SixelEncoder {
    fn encode(&self, data: &[u8], width: u32, height: u32, cols: u16, rows: u16) -> String {
        // 1. PNG/RGB 데이터를 픽셀로 디코딩
        // 2. 팔레트 정의 (색상 맵핑)
        // 3. Sixel 인코딩 (6줄씩 처리)
        // 포맷: DCS q <palette> <sixel_data> ST
    }

    fn name(&self) -> &'static str { "sixel" }
}
```

**Sixel 인코딩 알고리즘**:
1. 이미지를 6픽셀 높이 단위로 분할
2. 각 열에서 6비트 값 계산 (각 픽셀 = 1비트)
3. 63(0x3F) 더해서 ASCII 문자로 변환
4. 반복 압축: `!count char` 형식

**지원 터미널 감지**:
- `TERM` 변수에 `xterm`, `mlterm`, `mintty` 포함
- DA1 응답에서 Sixel 지원 확인 (4번 비트)

### 2.4 iTerm2 프로토콜 구현
**파일**: `src/widget/image/iterm2.rs`

```rust
pub struct ITerm2Encoder {
    /// 파일명 (옵션)
    name: Option<String>,
    /// 인라인 표시 여부
    inline: bool,
    /// 종횡비 유지
    preserve_aspect_ratio: bool,
}

impl ITerm2Encoder {
    fn build_params(&self, width: Option<u32>, height: Option<u32>) -> String {
        // name=<base64_name>;size=<bytes>;width=<cols>;height=<rows>;
        // preserveAspectRatio=<0|1>;inline=<0|1>
    }
}

impl ImageEncoder for ITerm2Encoder {
    fn encode(&self, data: &[u8], width: u32, height: u32, cols: u16, rows: u16) -> String {
        // OSC 1337 ; File=<params>:<base64> BEL
        // 포맷: \x1b]1337;File=...:<base64>\x07
        let params = self.build_params(Some(cols as u32), Some(rows as u32));
        let b64 = base64::encode(data);
        format!("\x1b]1337;File={}:{}\x07", params, b64)
    }

    fn name(&self) -> &'static str { "iterm2" }
}
```

**지원 터미널 감지**:
- `TERM_PROGRAM` = `iTerm.app`
- `LC_TERMINAL` = `iTerm2`
- `WEZTERM_PANE` 존재
- `GHOSTTY_RESOURCES_DIR` 존재

### 2.5 Image 위젯 수정
**파일**: `src/widget/image.rs`

```rust
pub struct Image {
    data: Vec<u8>,
    width: u32,
    height: u32,
    format: ImageFormat,
    scale: ScaleMode,
    placeholder: char,
    id: u32,
    /// 사용할 프로토콜 (None = 자동 감지)
    protocol: Option<ImageProtocol>,
    props: WidgetProps,
}

impl Image {
    /// 프로토콜 강제 지정
    pub fn protocol(mut self, protocol: ImageProtocol) -> Self {
        self.protocol = Some(protocol);
        self
    }

    /// 현재 환경에 맞는 이스케이프 시퀀스 생성
    pub fn escape_sequence(&self, cols: u16, rows: u16) -> String {
        let protocol = self.protocol.unwrap_or_else(ImageProtocol::detect);
        match protocol {
            ImageProtocol::Kitty => KittyEncoder.encode(&self.data, ...),
            ImageProtocol::Sixel => SixelEncoder::default().encode(&self.data, ...),
            ImageProtocol::ITerm2 => ITerm2Encoder::default().encode(&self.data, ...),
            ImageProtocol::None => String::new(),
        }
    }
}
```

---

## Phase 3: 터미널 기능 통합 감지

### 3.1 TerminalCapabilities 구조체
**파일**: `src/render/capabilities.rs`

```rust
use crate::text::width::TerminalType;
use crate::widget::image::ImageProtocol;

/// 터미널 기능 정보
#[derive(Debug, Clone)]
pub struct TerminalCapabilities {
    /// 터미널 유형
    pub terminal_type: TerminalType,
    /// 지원되는 이미지 프로토콜
    pub image_protocol: ImageProtocol,
    /// True Color (24-bit RGB) 지원
    pub true_color: bool,
    /// 하이퍼링크 (OSC 8) 지원
    pub hyperlinks: bool,
    /// 유니코드 버전
    pub unicode_version: UnicodeVersion,
    /// 마우스 지원
    pub mouse: bool,
}

impl TerminalCapabilities {
    /// 환경에서 자동 감지
    pub fn detect() -> Self {
        let terminal_type = TerminalType::detect();
        Self {
            terminal_type,
            image_protocol: ImageProtocol::detect(),
            true_color: Self::detect_true_color(),
            hyperlinks: Self::detect_hyperlinks(&terminal_type),
            unicode_version: UnicodeVersion::detect(),
            mouse: true, // 대부분 지원
        }
    }

    fn detect_true_color() -> bool {
        // COLORTERM=truecolor 또는 24bit
        // 또는 TERM_PROGRAM이 알려진 truecolor 터미널
    }

    fn detect_hyperlinks(terminal: &TerminalType) -> bool {
        // 대부분의 현대 터미널 지원
        matches!(terminal,
            TerminalType::ITerm2 | TerminalType::Kitty |
            TerminalType::WezTerm | TerminalType::Alacritty | ...
        )
    }
}

#[derive(Debug, Clone, Copy)]
pub enum UnicodeVersion {
    V9,   // 기본
    V14,  // 이모지 14.0
    V15,  // 최신
}
```

---

## 파일 구조

```
src/render/
├── mod.rs                    # 모듈 정의
├── terminal.rs               # Terminal<B: Backend>
├── buffer.rs                 # 버퍼 (변경 없음)
├── cell.rs                   # 셀 (변경 없음)
├── diff.rs                   # diff 알고리즘 (변경 없음)
├── capabilities.rs           # [NEW] 터미널 기능 감지
└── backend/
    ├── mod.rs                # 백엔드 모듈
    ├── traits.rs             # Backend trait
    ├── crossterm.rs          # Crossterm 구현
    └── termion.rs            # Termion 구현 (unix)

src/widget/
├── image.rs                  # Image 위젯 (수정)
└── image/
    ├── mod.rs                # [NEW] 이미지 모듈
    ├── protocol.rs           # [NEW] ImageProtocol enum
    ├── kitty.rs              # [NEW] Kitty 인코더
    ├── sixel.rs              # [NEW] Sixel 인코더
    └── iterm2.rs             # [NEW] iTerm2 인코더
```

---

## Cargo.toml 변경

```toml
[features]
default = ["crossterm-backend"]
crossterm-backend = ["dep:crossterm"]
termion-backend = ["dep:termion"]
all-backends = ["crossterm-backend", "termion-backend"]

[dependencies]
crossterm = { version = "0.28", optional = true }
termion = { version = "3.0", optional = true }

# 이미지 처리 (기존)
image = "0.25"
base64 = "0.22"
```

---

## 구현 순서

1. ✅ Backend trait 정의 (`backend/traits.rs`)
2. ✅ CrosstermBackend 구현 (`backend/crossterm.rs`)
3. ✅ TermionBackend 구현 (`backend/termion.rs`)
4. ⬜ Terminal<B> 리팩토링 (`terminal.rs`)
5. ⬜ ImageProtocol enum 정의 (`image/protocol.rs`)
6. ⬜ Kitty 인코더 분리 (`image/kitty.rs`)
7. ⬜ Sixel 인코더 구현 (`image/sixel.rs`)
8. ⬜ iTerm2 인코더 구현 (`image/iterm2.rs`)
9. ⬜ Image 위젯 수정 (`image.rs`)
10. ⬜ TerminalCapabilities 구현 (`capabilities.rs`)
11. ⬜ Cargo.toml feature flags 추가
12. ⬜ 테스트 작성 및 검증

---

## 참고 자료

- [Kitty Graphics Protocol](https://sw.kovidgoyal.net/kitty/graphics-protocol/)
- [Sixel Graphics](https://en.wikipedia.org/wiki/Sixel)
- [iTerm2 Inline Images](https://iterm2.com/documentation-images.html)
- [Termion Docs](https://docs.rs/termion/latest/termion/)
