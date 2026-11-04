//! Chat view

use revue::prelude::*;

/// Width of the users sidebar, borders included.
const SIDEBAR_WIDTH: u16 = 20;

#[derive(Clone)]
pub struct Message {
    user: String,
    text: String,
    timestamp: String,
    is_self: bool,
}

impl Message {
    fn new(user: &str, text: &str, timestamp: &str, is_self: bool) -> Self {
        Self {
            user: user.into(),
            text: text.into(),
            timestamp: timestamp.into(),
            is_self,
        }
    }
}

#[derive(Clone)]
pub struct User {
    name: String,
    status: UserStatus,
}

#[derive(Clone, Copy)]
pub enum UserStatus {
    Online,
    Away,
    Offline,
}

pub struct ChatApp {
    messages: Signal<Vec<Message>>,
    users: Vec<User>,
    input: Input,
    current_user: String,
}

impl ChatApp {
    pub fn new() -> Self {
        let user = |name: &str, status| User {
            name: name.into(),
            status,
        };
        Self {
            messages: signal(vec![
                Message::new("Alice", "Hey everyone!", "10:30", false),
                Message::new("Bob", "Hi Alice! How's it going?", "10:31", false),
                Message::new(
                    "You",
                    "Hello! Just built this chat with Revue!",
                    "10:32",
                    true,
                ),
            ]),
            users: vec![
                user("Alice", UserStatus::Online),
                user("Bob", UserStatus::Online),
                user("Charlie", UserStatus::Away),
                user("Diana", UserStatus::Offline),
            ],
            input: Input::new().placeholder("Type a message...").focused(true),
            current_user: "You".into(),
        }
    }

    /// Handles a key press and returns whether the view changed.
    pub fn handle_key(&mut self, key: &Key) -> bool {
        match key {
            Key::Enter => {
                self.send_message();
                true
            }
            _ => self.input.handle_key(key),
        }
    }

    fn send_message(&mut self) {
        let text = self.input.text().trim().to_string();
        if text.is_empty() {
            return;
        }
        let message = Message::new(&self.current_user, &text, "now", true);
        self.messages.update(|messages| messages.push(message));
        self.input.clear();
    }

    fn render_user(user: &User) -> Stack {
        let avatar = match user.status {
            UserStatus::Online => Avatar::new(&user.name).online(),
            UserStatus::Away => Avatar::new(&user.name).away(),
            UserStatus::Offline => Avatar::new(&user.name).offline(),
        };
        // A small avatar is the initial plus a status dot: 2 columns.
        hstack()
            .gap(1)
            .child_sized(avatar.small(), 2)
            .child(Text::new(&user.name))
    }

    fn render_message(message: &Message) -> Stack {
        let name = format!("{}:", message.user);
        let name_width = name.chars().count() as u16;
        hstack()
            .gap(1)
            .child_sized(Text::new(&message.timestamp).class("timestamp"), 5)
            .child_sized(
                Text::new(name).class(if message.is_self { "self" } else { "username" }),
                name_width,
            )
            .child(Text::new(&message.text))
    }
}

impl Default for ChatApp {
    fn default() -> Self {
        Self::new()
    }
}

impl View for ChatApp {
    fn render(&self, ctx: &mut RenderContext) {
        let messages = self.messages.get();

        // Show the newest messages that fit: the screen minus the help line,
        // the input box (3 lines) and the message panel's border (2 lines).
        let fit = ctx.area.height.saturating_sub(1 + 3 + 2) as usize;
        let mut message_list = vstack();
        for message in messages.iter().skip(messages.len().saturating_sub(fit)) {
            message_list = message_list.child_sized(Self::render_message(message), 1);
        }

        let mut user_list = vstack();
        for user in &self.users {
            user_list = user_list.child_sized(Self::render_user(user), 1);
        }

        // Fixed-size pieces use `child_sized`; the sidebar and the message
        // panel take the rest of the screen with `child_flex`.
        let sidebar = Border::rounded().title(" Users ").child(user_list);
        let chat = vstack()
            .child_flex(
                Border::rounded().title(" # general ").child(message_list),
                1.0,
            )
            .child_sized(
                Border::rounded()
                    .class("composer")
                    .child(self.input.clone()),
                3,
            );

        vstack()
            .child_flex(
                hstack()
                    .gap(1)
                    .child_sized(sidebar, SIDEBAR_WIDTH)
                    .child_flex(chat, 1.0),
                1.0,
            )
            .child_sized(Text::muted("[Enter] send  [Esc] quit"), 1)
            .render(ctx);
    }

    fn meta(&self) -> WidgetMeta {
        WidgetMeta::new("ChatApp")
    }
}
