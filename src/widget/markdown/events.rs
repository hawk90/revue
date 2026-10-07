//! Turning pulldown-cmark events into styled lines: the event loop, the tag,
//! text, code, HTML and footnote-reference handlers, and the footnote list

use super::{parser, AdmonitionType, FootnoteDefinition, Line, Markdown, StyledText};
use crate::render::Modifier;
use crate::utils::syntax::Language;
use crate::widget::theme::{DARK_GRAY, PLACEHOLDER_FG};

// Import pulldown-cmark types for parser
#[cfg(feature = "markdown")]
use pulldown_cmark::{CodeBlockKind, Tag, TagEnd};

impl Markdown {
    /// Parse markdown into styled lines with current options
    pub(super) fn parse_with_options(&self) -> Vec<Line> {
        #[allow(unused_imports)]
        #[cfg(feature = "markdown")]
        use pulldown_cmark::{Event, HeadingLevel, Parser, Tag, TagEnd};

        let parser = Parser::new_ext(&self.source, parser::ParserContext::parser_options());
        let mut ctx = parser::ParserContext::new(&self.source, &self.config);

        if self.config.show_toc {
            self.render_toc(&mut ctx);
        }

        for event in parser {
            // Text held back as a possible callout marker is plain quote
            // text once anything but more text follows it
            if !matches!(event, Event::Text(_)) {
                ctx.flush_pending_quote();
            }
            match event {
                Event::Start(tag) => self.handle_start_tag(&mut ctx, tag),
                Event::End(tag_end) => self.handle_end_tag(&mut ctx, tag_end),
                Event::Text(text) => self.handle_text(&mut ctx, &text),
                Event::Code(text) => self.handle_code(&mut ctx, &text),
                Event::Html(text) => self.handle_html(&mut ctx, &text),
                Event::FootnoteReference(text) => self.handle_footnote_reference(&mut ctx, &text),
                Event::Rule => {
                    ctx.flush_line();
                    let rule_line = Line::new();
                    ctx.lines.push(rule_line);
                }
                Event::SoftBreak => {
                    if ctx.in_blockquote || ctx.current_admonition.is_some() {
                        ctx.flush_line();
                        ctx.new_line();
                    } else {
                        ctx.add_text(" ");
                    }
                }
                Event::HardBreak => {
                    ctx.flush_line();
                    ctx.new_line();
                }
                Event::TaskListMarker(checked) => {
                    // Task list item - the checkbox stands in for the bullet
                    ctx.item_needs_bullet = false;
                    if checked {
                        ctx.add_text("[x] ");
                    } else {
                        ctx.add_text("[ ] ");
                    }
                }
                // Handle remaining events
                _ => {}
            }
        }

        // Render footnotes at the end if any
        if !ctx.footnote_definitions.is_empty() {
            ctx.new_line();
            ctx.flush_line();

            // Separator
            let mut sep_line = Line::new();
            sep_line.push(
                StyledText::new("────────────────────────────────────────").with_fg(DARK_GRAY),
            );
            ctx.lines.push(sep_line);
            ctx.new_line();

            // Sort footnotes by reference number
            let mut sorted_definitions: Vec<_> = ctx.footnote_definitions.iter().collect();
            sorted_definitions
                .sort_by_key(|d| ctx.footnote_label_map.get(&d.label).copied().unwrap_or(999));

            for (idx, def) in sorted_definitions.iter().enumerate() {
                let mut footnote_line = Line::new();
                footnote_line.push(
                    StyledText::new(format!("[{}] ", idx + 1))
                        .with_fg(ctx.link_fg)
                        .with_modifier(Modifier::BOLD),
                );
                footnote_line.push(StyledText::new(&def.content));
                ctx.lines.push(footnote_line);
            }
        }

        while ctx.lines.last().map(|l| l.is_empty()).unwrap_or(false) {
            ctx.lines.pop();
        }

        ctx.lines
    }

    fn handle_start_tag(&self, ctx: &mut parser::ParserContext, tag: Tag) {
        match tag {
            Tag::Heading { level, .. } => {
                ctx.in_heading = true;
                ctx.heading_level = match level {
                    pulldown_cmark::HeadingLevel::H1 => 1,
                    pulldown_cmark::HeadingLevel::H2 => 2,
                    pulldown_cmark::HeadingLevel::H3 => 3,
                    pulldown_cmark::HeadingLevel::H4 => 4,
                    pulldown_cmark::HeadingLevel::H5 => 5,
                    pulldown_cmark::HeadingLevel::H6 => 6,
                };
                ctx.heading_text.clear();
                ctx.heading_is_figlet =
                    ctx.figlet_font.is_some() && ctx.heading_level <= ctx.figlet_max_level;
                if !ctx.heading_is_figlet {
                    // The line starts with its level's `#` marker, dimmed;
                    // the heading's content follows as it arrives, so inline
                    // code, emphasis and footnote references keep their
                    // place and style
                    let marker = "#".repeat(ctx.heading_level as usize);
                    ctx.current_fg = Some(PLACEHOLDER_FG);
                    ctx.current_modifier = Modifier::empty();
                    ctx.add_text(&format!("{marker} "));
                }
                ctx.current_modifier |= Modifier::BOLD;
                ctx.current_fg = Some(ctx.heading_fg);
            }
            Tag::Strong => {
                ctx.current_modifier |= Modifier::BOLD;
            }
            Tag::Emphasis => {
                ctx.current_modifier |= Modifier::ITALIC;
            }
            Tag::Strikethrough => {
                // Strikethrough not supported by terminal modifier
            }
            Tag::Link { .. } => {
                ctx.current_fg = Some(ctx.link_fg);
                ctx.current_modifier |= Modifier::UNDERLINE;
            }
            Tag::Image { .. } => {
                ctx.current_fg = Some(ctx.link_fg);
            }
            Tag::CodeBlock(kind) => {
                ctx.in_code_block = true;
                match kind {
                    // The info string's first word names the language
                    CodeBlockKind::Fenced(info) => {
                        let lang = info.split_whitespace().next().unwrap_or("");
                        ctx.code_block_lang = Language::from_fence(lang);
                    }
                    CodeBlockKind::Indented => {
                        ctx.code_block_lang = Language::Unknown;
                    }
                }
            }
            Tag::List(num) => {
                ctx.list_depth += 1;
                ctx.ordered_list_num = num;
            }
            Tag::Item => {
                // A nested item must not continue its parent's line
                ctx.flush_line();
                let indent = "  ".repeat(ctx.list_depth.saturating_sub(1));
                ctx.add_text(&indent);

                // For ordered lists, add the number now
                if let Some(n) = ctx.ordered_list_num {
                    ctx.ordered_list_num = Some(n + 1);
                    ctx.add_text(&format!("{}. ", n));
                    ctx.item_needs_bullet = false;
                } else {
                    // For unordered lists, wait to see if it's a task list;
                    // `add_text` emits the bullet before the item's first text
                    ctx.item_needs_bullet = true;
                }
            }
            Tag::Paragraph => {
                // Start new line if not empty
                ctx.flush_line();
            }
            Tag::Table(alignments) => {
                ctx.flush_line();
                ctx.in_table = true;
                ctx.table_alignments = alignments;
                ctx.table_rows.clear();
            }
            Tag::TableHead => {
                ctx.in_table_head = true;
                ctx.table_row.clear();
            }
            Tag::TableRow => {
                ctx.table_row.clear();
            }
            Tag::TableCell => {
                ctx.current_cell.clear();
            }
            Tag::FootnoteDefinition(name) => {
                ctx.in_footnote_definition = true;
                ctx.current_footnote_label = name.to_string();
                ctx.current_footnote_content.clear();
            }
            Tag::BlockQuote(_) => {
                ctx.in_blockquote = true;
                ctx.blockquote_first_text = true;
                ctx.current_admonition = None;
            }
            _ => {}
        }
    }

    fn handle_end_tag(&self, ctx: &mut parser::ParserContext, tag_end: TagEnd) {
        match tag_end {
            TagEnd::Heading(_) => {
                if let Some(font) = ctx.figlet_font.filter(|_| ctx.heading_is_figlet) {
                    self.render_figlet_heading(ctx, font);
                    return;
                }
                // The marker and content are already on the line
                ctx.in_heading = false;
                ctx.current_modifier &= !Modifier::BOLD;
                ctx.current_fg = None;
                ctx.new_line();
            }
            TagEnd::Paragraph => {
                ctx.flush_line();
                ctx.new_line();
            }
            TagEnd::Strong => {
                // A heading is bold throughout, strong text or not
                if !ctx.in_heading {
                    ctx.current_modifier &= !Modifier::BOLD;
                }
            }
            TagEnd::Emphasis => {
                ctx.current_modifier &= !Modifier::ITALIC;
            }
            TagEnd::Strikethrough => {
                ctx.current_modifier &= !Modifier::CROSSED_OUT;
            }
            TagEnd::Link => {
                ctx.current_fg = ctx.in_heading.then_some(ctx.heading_fg);
                ctx.current_modifier &= !Modifier::UNDERLINE;
            }
            TagEnd::Image => {
                ctx.current_fg = ctx.in_heading.then_some(ctx.heading_fg);
            }
            TagEnd::CodeBlock => {
                self.render_code_block(ctx);
            }
            TagEnd::FootnoteDefinition => {
                ctx.footnote_definitions.push(FootnoteDefinition {
                    label: ctx.current_footnote_label.clone(),
                    content: ctx.current_footnote_content.clone(),
                });
                ctx.in_footnote_definition = false;
            }
            TagEnd::BlockQuote(_) => {
                ctx.in_blockquote = false;
                ctx.blockquote_first_text = false;
                ctx.flush_line();
                // Add empty line after admonition
                if ctx.current_admonition.is_some() {
                    ctx.lines.push(std::mem::take(&mut ctx.current_line));
                    let empty_line = Line::new();
                    ctx.lines.push(empty_line);
                }
                ctx.current_admonition = None;
                ctx.accumulated_blockquote.clear();
                // The quote / admonition styling ends with the quote
                ctx.current_fg = None;
                ctx.current_modifier &= !(Modifier::ITALIC | Modifier::BOLD);
            }
            TagEnd::List(_) => {
                ctx.list_depth = ctx.list_depth.saturating_sub(1);
                ctx.flush_line();
            }
            TagEnd::Item => {
                ctx.flush_line();
                ctx.item_needs_bullet = false;
            }
            TagEnd::TableCell => {
                let cell = std::mem::take(&mut ctx.current_cell);
                ctx.table_row.push(cell.trim().to_string());
            }
            TagEnd::TableHead | TagEnd::TableRow => {
                let row = std::mem::take(&mut ctx.table_row);
                ctx.table_rows.push(row);
                ctx.in_table_head = false;
            }
            TagEnd::Table => {
                self.render_table(ctx);
            }
            _ => {}
        }
    }

    fn handle_text(&self, ctx: &mut parser::ParserContext, text: &str) {
        if ctx.in_code_block {
            // Collected verbatim; split into lines when the block ends
            ctx.code_block_lines.push(text.to_string());
        } else if ctx.in_footnote_definition {
            ctx.current_footnote_content.push_str(text);
        } else if ctx.in_table {
            ctx.current_cell.push_str(text);
        } else if ctx.in_heading {
            ctx.heading_text.push_str(text);
            if !ctx.heading_is_figlet {
                ctx.add_text(text);
            }
        } else if ctx.in_blockquote && ctx.blockquote_first_text {
            // Hold the quote's opening text back only while it can still
            // become a callout marker like `[!NOTE]`
            ctx.accumulated_blockquote.push_str(text);
            let pending = ctx.accumulated_blockquote.clone();
            if let Some(admonition) = AdmonitionType::from_exact_marker(&pending) {
                ctx.current_admonition = Some(admonition);
                ctx.flush_line();
                // Render admonition header with icon and label
                ctx.current_fg = Some(admonition.color());
                ctx.current_modifier |= Modifier::BOLD;
                ctx.add_text(&format!("{} {}", admonition.icon(), admonition.label()));
                ctx.new_line();
                ctx.accumulated_blockquote.clear();
                ctx.blockquote_first_text = false;
            } else if !AdmonitionType::is_marker_prefix(&pending) {
                ctx.flush_pending_quote();
            }
        } else if ctx.in_blockquote {
            // Blockquote / admonition content
            ctx.add_quote_text(text);
        } else {
            ctx.add_text(text);
        }
    }

    fn handle_code(&self, ctx: &mut parser::ParserContext, text: &str) {
        if ctx.in_table {
            ctx.current_cell.push_str(text);
        } else if ctx.in_heading && ctx.heading_is_figlet {
            // Part of the big text, drawn when the heading ends
            ctx.heading_text.push_str(text);
        } else if !ctx.in_code_block {
            if ctx.in_heading {
                ctx.heading_text.push_str(text);
            }
            // Inline code is drawn in the code color
            let fg = ctx.current_fg;
            ctx.current_fg = Some(ctx.code_fg);
            ctx.add_text(text);
            ctx.current_fg = fg;
        }
    }

    fn handle_html(&self, ctx: &mut parser::ParserContext, text: &str) {
        if let Some(admonition) = AdmonitionType::from_marker(text) {
            ctx.current_admonition = Some(admonition);
            ctx.flush_line();
            // Render admonition header with icon and label
            let color = admonition.color();
            ctx.current_fg = Some(color);
            ctx.current_modifier |= Modifier::BOLD;
            ctx.add_text(&format!("{} {}", admonition.icon(), admonition.label()));
            ctx.new_line();
            ctx.blockquote_first_text = false;
        }
    }

    fn handle_footnote_reference(&self, ctx: &mut parser::ParserContext, text: &str) {
        // Track footnote references
        if !ctx.footnote_label_map.contains_key(text) {
            ctx.footnote_counter += 1;
            ctx.footnote_label_map
                .insert(text.to_string(), ctx.footnote_counter);
        }

        let num = ctx.footnote_label_map.get(text).copied().unwrap_or(1);
        // Big text has no room for a reference mark; the numbering above
        // still counts it so later references keep their numbers
        if ctx.in_heading && ctx.heading_is_figlet {
            return;
        }
        ctx.add_text(&format!("[^{}]", num));
    }
}
