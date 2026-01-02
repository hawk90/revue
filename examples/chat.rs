//! Chat App example - Messaging interface demo
//!
//! Run with: cargo run --example chat

use revue::prelude::*;

#[derive(Clone)]
struct Message {
    sender: String,
    content: String,
    timestamp: String,
    is_self: bool,
}

#[derive(Clone)]
struct Contact {
    name: String,
    status: String,
    online: bool,
    unread: usize,
}

struct ChatApp {
    contacts: Vec<Contact>,
    messages: Vec<Message>,
    input: Input,
    selected_contact: usize,
    scroll_offset: usize,
    editing: bool,
}

impl ChatApp {
    fn new() -> Self {
        Self {
            contacts: vec![
                Contact { name: "Alice".into(), status: "Working on Revue".into(), online: true, unread: 2 },
                Contact { name: "Bob".into(), status: "Away".into(), online: true, unread: 0 },
                Contact { name: "Charlie".into(), status: "Last seen 2h ago".into(), online: false, unread: 0 },
                Contact { name: "Diana".into(), status: "Busy coding".into(), online: true, unread: 5 },
            ],
            messages: vec![
                Message { sender: "Alice".into(), content: "Hey! Have you seen the new Revue framework?".into(), timestamp: "10:30".into(), is_self: false },
                Message { sender: "You".into(), content: "Yes! It's amazing for building TUIs".into(), timestamp: "10:31".into(), is_self: true },
                Message { sender: "Alice".into(), content: "The CSS styling is so intuitive".into(), timestamp: "10:32".into(), is_self: false },
                Message { sender: "You".into(), content: "And the reactive state management is clean".into(), timestamp: "10:33".into(), is_self: true },
                Message { sender: "Alice".into(), content: "We should build something cool with it!".into(), timestamp: "10:35".into(), is_self: false },
            ],
            input: Input::new().placeholder("Type a message..."),
            selected_contact: 0,
            scroll_offset: 0,
            editing: false,
        }
    }

    fn handle_key(&mut self, key: &Key) -> bool {
        if self.editing {
            match key {
                Key::Enter => {
                    let text = self.input.text().trim().to_string();
                    if !text.is_empty() {
                        self.messages.push(Message {
                            sender: "You".into(),
                            content: text,
                            timestamp: "Now".into(),
                            is_self: true,
                        });
                        self.input.clear();
                    }
                    true
                }
                Key::Escape => {
                    self.editing = false;
                    true
                }
                _ => self.input.handle_key(key),
            }
        } else {
            match key {
                Key::Tab => {
                    self.editing = true;
                    true
                }
                Key::Up | Key::Char('k') => {
                    if self.selected_contact > 0 {
                        self.selected_contact -= 1;
                    }
                    true
                }
                Key::Down | Key::Char('j') => {
                    if self.selected_contact < self.contacts.len() - 1 {
                        self.selected_contact += 1;
                    }
                    true
                }
                Key::PageUp => {
                    if self.scroll_offset > 0 {
                        self.scroll_offset -= 1;
                    }
                    true
                }
                Key::PageDown => {
                    if self.scroll_offset < self.messages.len().saturating_sub(5) {
                        self.scroll_offset += 1;
                    }
                    true
                }
                _ => false,
            }
        }
    }
}

impl View for ChatApp {
    fn render(&self, ctx: &mut RenderContext) {
        // Header
        let current_contact = &self.contacts[self.selected_contact];
        let status_dot = if current_contact.online { "●" } else { "○" };
        let status_color = if current_contact.online { Color::GREEN } else { Color::rgb(100, 100, 100) };

        let header = Border::rounded()
            .child(hstack()
                .child(Text::new(status_dot).fg(status_color))
                .child(Text::new(format!(" {} ", current_contact.name)).bold())
                .child(Text::new(format!("- {}", current_contact.status)).fg(Color::rgb(128, 128, 128)))
                .child(Text::new("          Chat App").fg(Color::CYAN)));

        // Contact list (sidebar)
        let mut contact_list = vstack();
        for (i, contact) in self.contacts.iter().enumerate() {
            let dot = if contact.online { "●" } else { "○" };
            let dot_color = if contact.online { Color::GREEN } else { Color::rgb(80, 80, 80) };

            let name_text = if contact.unread > 0 {
                format!("{} {} ({})", dot, contact.name, contact.unread)
            } else {
                format!("{} {}", dot, contact.name)
            };

            let row = if i == self.selected_contact {
                Text::new(name_text).fg(Color::CYAN).bold()
            } else if contact.unread > 0 {
                Text::new(name_text).fg(Color::WHITE).bold()
            } else {
                Text::new(name_text).fg(dot_color)
            };
            contact_list = contact_list.child(row);
        }

        let sidebar = Border::rounded()
            .title("Contacts")
            .child(contact_list);

        // Messages area
        let visible_messages = self.messages.iter()
            .skip(self.scroll_offset)
            .take(10);

        let mut messages_content = vstack().gap(1);
        for msg in visible_messages {
            let sender_color = if msg.is_self { Color::CYAN } else { Color::YELLOW };
            let prefix = if msg.is_self { "  > " } else { "    " };

            let msg_line = Text::new(format!("{}{} [{}]: {}",
                prefix, msg.sender, msg.timestamp, msg.content))
                .fg(sender_color);

            messages_content = messages_content.child(msg_line);
        }

        let messages_panel = Border::rounded()
            .title(format!("Messages ({}/{})", self.scroll_offset + 1, self.messages.len()))
            .child(messages_content);

        // Input area
        let input_border = if self.editing {
            Border::rounded().fg(Color::CYAN)
        } else {
            Border::rounded().fg(Color::rgb(60, 60, 60))
        };
        let input_area = input_border.child(self.input.clone());

        // Help bar
        let help = if self.editing {
            Text::new("Enter: Send | Esc: Back to contacts").fg(Color::rgb(80, 80, 80))
        } else {
            Text::new("Tab: Type message | j/k: Select contact | PgUp/PgDn: Scroll | q: Quit").fg(Color::rgb(80, 80, 80))
        };

        // Layout - sidebar and main content
        let main_content = vstack()
            .child(messages_panel)
            .child(input_area);

        let body = hstack()
            .gap(1)
            .child(sidebar)
            .child(main_content);

        let layout = vstack()
            .gap(1)
            .child(header)
            .child(body)
            .child(help);

        layout.render(ctx);
    }
}

fn main() -> Result<()> {
    let mut app = App::builder().build();
    let chat = ChatApp::new();

    app.run_with_handler(chat, |key_event, chat| {
        chat.handle_key(&key_event.key)
    })
}
