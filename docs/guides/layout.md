# Layout Guide

How a stack shares its space among its children, and how a widget of your own
tells the stack how big it is.

## How a stack sizes its children

A stack gives each child a size along its main axis (height in a `vstack`,
width in an `hstack`) and the whole cross axis.

| Added with | Main-axis size |
|---|---|
| `child_sized(view, n)` | exactly `n` |
| `child_flex(view, grow)` | a share of what is left, in proportion to `grow` |
| `child(view)`, and the view measures itself | its measured size |
| `child(view)`, and the view fills | an equal share of what is left |

A view fills when it does not measure itself (a list, a table, a scroll view),
or when it stretches along the stack's main axis: `Input` is one row in a
`vstack` but takes the width an `hstack` leaves.

This is `Stack::content_sized`, on by default since 3.0.
`vstack().content_sized(false)` gives every `child(view)` an equal share, as in
2.x.

### When the children do not fit

If the measured children and the fixed children together are larger than the
stack, the measured children shrink, each in proportion to its size, so a body
one row too tall loses that row instead of pushing the footer off the screen.

These never shrink:

- a `child_sized` child, because the size was given in the builder;
- under `css_layout`, a child with an explicit CSS size or `min-*` along the
  main axis, because the box model puts that size back whatever slot the child
  gets;
- the items of a wrapping row (`flex-wrap`), which move to the next row instead.

## A widget of your own

A custom widget fills whatever it is given until it answers two `View`
methods.

`measure(max_width, max_height)` returns the size the widget paints, clamped to
the maximum: borders, padding and display width included. `None`, the default,
means it fills.

`fills()` names the axes along which the widget takes whatever it is offered,
while `measure` still gives its natural size on the other axis. The default is
`Fill::NONE`; the others are `Fill::WIDTH`, `Fill::HEIGHT` and `Fill::BOTH`.

```rust
use revue::prelude::*;
use revue::utils::display_width;

/// One row, as wide as its label.
struct Tag {
    label: String,
}

impl View for Tag {
    fn render(&self, ctx: &mut RenderContext) {
        ctx.draw_text(0, 0, &self.label, Color::CYAN);
    }

    fn measure(&self, max_width: u16, max_height: u16) -> Option<(u16, u16)> {
        let width = display_width(&self.label) as u16;
        Some((width.min(max_width), 1.min(max_height)))
    }
}

/// One row, as wide as the stack lets it be.
struct Meter {
    ratio: f32,
}

impl View for Meter {
    fn render(&self, ctx: &mut RenderContext) {
        let filled = (ctx.area.width as f32 * self.ratio) as usize;
        ctx.draw_text(0, 0, &"█".repeat(filled), Color::GREEN);
    }

    fn measure(&self, max_width: u16, max_height: u16) -> Option<(u16, u16)> {
        Some((max_width, 1.min(max_height)))
    }

    fn fills(&self) -> Fill {
        Fill::WIDTH
    }
}
```

In a `vstack`, `Meter` takes one row. In an `hstack`, it shares the width the
other children leave, instead of taking it all and pushing later siblings out.

A stack passes its children's fill on: an `hstack` holding a `Meter` fills the
width of an outer `hstack` too. Children added with `child_sized` or
`child_flex` do not count.
