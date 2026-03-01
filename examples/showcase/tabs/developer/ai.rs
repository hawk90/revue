//! AI widget demos

use crate::example::Example;
use crate::theme_colors;
use revue::prelude::*;

pub fn examples() -> Vec<Example> {
    let (primary, success, _warning, _error, info, muted, text, _) = theme_colors();

    vec![
        Example::new(
            "AI Stream",
            "Streaming AI responses with real-time code generation",
            Border::rounded().title(" AI Stream ").child(
                vstack()
                    .gap(1)
                    .child(Text::new("Prompt: Write a Rust function").fg(primary))
                    .child(Text::new(""))
                    .child(Text::new("Response:").fg(text))
                    .child(Text::new("Here's a Rust function to calculate fibonacci:").fg(text))
                    .child(Text::new(""))
                    .child(Text::new("fn fibonacci(n: u64) -> u64 {").fg(info))
                    .child(Text::new("    match n {").fg(info))
                    .child(Text::new("        0 => 0,").fg(info))
                    .child(Text::new("        1 => 1,").fg(info))
                    .child(Text::new("        _ => fibonacci(n-1) + fibonacci(n-2),").fg(info))
                    .child(Text::new("    }").fg(info))
                    .child(Text::new("}").fg(info))
                    .child(Spinner::new().label("Streaming..."))
                    .child(Text::new(""))
                    .child(Text::new("• Streaming responses").fg(muted))
                    .child(Text::new("• Code generation").fg(muted))
                    .child(Text::new("• Real-time display").fg(muted)),
            ),
        ),
        Example::new(
            "AI Response",
            "Complete AI response with token count and duration",
            Border::rounded().title(" AI Response ").child(
                vstack()
                    .gap(1)
                    .child(Text::new("Model: claude-3-opus").fg(primary))
                    .child(Text::new("Prompt: Explain Rust ownership").fg(text))
                    .child(Text::new(""))
                    .child(Text::new("Response:").fg(text))
                    .child(Text::new("Rust ownership is a memory management feature...").fg(text))
                    .child(Text::new(""))
                    .child(Text::new("Tokens: 150 | Duration: 1200ms").fg(muted))
                    .child(Text::new(""))
                    .child(Text::new("• Complete response").fg(muted))
                    .child(Text::new("• Token count").fg(muted))
                    .child(Text::new("• Duration tracking").fg(muted)),
            ),
        ),
        Example::new(
            "Chat Interface",
            "Conversational view with user and assistant roles",
            Border::rounded().title(" Chat Interface ").child(
                vstack()
                    .gap(1)
                    .child(Text::new("User: What is Revue?").fg(primary))
                    .child(Text::new("Assistant: Revue is a Rust TUI framework").fg(text))
                    .child(Text::new("with 92+ widgets for building beautiful").fg(text))
                    .child(Text::new("terminal interfaces.").fg(text))
                    .child(Text::new(""))
                    .child(Text::new("User: How do I get started?").fg(primary))
                    .child(Text::new("Assistant: Add revue to your Cargo.toml").fg(text))
                    .child(Text::new("and use the prelude:").fg(text))
                    .child(Text::new(""))
                    .child(Text::new("• Conversation view").fg(muted))
                    .child(Text::new("• User/assistant roles").fg(muted))
                    .child(Text::new("• Input field").fg(muted)),
            ),
        ),
        Example::new(
            "Message Types",
            "Chat message styles for system, user, and assistant",
            Border::rounded().title(" Message Types ").child(
                vstack()
                    .gap(1)
                    .child(Text::new("Chat message styles:").fg(primary))
                    .child(Text::new(""))
                    .child(Text::new("[SYSTEM] System message").fg(muted))
                    .child(Text::new("[USER] User message").fg(primary))
                    .child(Text::new("[ASSISTANT] Assistant response").fg(success))
                    .child(Text::new(""))
                    .child(Text::new("• System (neutral)").fg(muted))
                    .child(Text::new("• User (highlighted)").fg(muted))
                    .child(Text::new("• Assistant (colored)").fg(muted)),
            ),
        ),
        Example::new(
            "Model Selection",
            "AI model picker with capability comparison",
            Border::rounded().title(" Model Selection ").child(
                vstack()
                    .gap(1)
                    .child(Text::new("AI model picker:").fg(primary))
                    .child(Text::new(""))
                    .child(Text::new("(*) claude-3-opus - Most capable").fg(success))
                    .child(Text::new("( ) claude-3-sonnet - Balanced").fg(text))
                    .child(Text::new("( ) claude-3-haiku - Fast").fg(text))
                    .child(Text::new(""))
                    .child(Text::new("• Model comparison").fg(muted))
                    .child(Text::new("• Capability info").fg(muted))
                    .child(Text::new("• Quick selection").fg(muted)),
            ),
        ),
        Example::new(
            "Usage Stats",
            "Token usage tracking with cost estimation",
            Border::rounded().title(" Usage Stats ").child(
                vstack()
                    .gap(1)
                    .child(Text::new("Token usage:").fg(primary))
                    .child(Text::new(""))
                    .child(Text::new("Input tokens: 256").fg(text))
                    .child(Progress::new(0.33))
                    .child(Text::new("Output tokens: 512").fg(text))
                    .child(Progress::new(0.67))
                    .child(Text::new("Total: 768").fg(text))
                    .child(Progress::new(0.50))
                    .child(Text::new(""))
                    .child(Text::new("Cost: $0.02").fg(success))
                    .child(Text::new(""))
                    .child(Text::new("• Input/output tokens").fg(muted))
                    .child(Text::new("• Cost estimation").fg(muted))
                    .child(Text::new("• Rate limits").fg(muted)),
            ),
        ),
    ]
}
