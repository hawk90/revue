//! Pinia Store Example
//!
//! Demonstrates the use of the `#[derive(Store)]` macro and `use_store()` function
//! for centralized state management.

use revue::prelude::*;

// Counter store example using the derive macro
#[derive(Store)]
struct CounterStore {
    count: Signal<i32>,
}

impl Default for CounterStore {
    fn default() -> Self {
        Self { count: signal(0) }
    }
}

impl CounterStore {
    fn new() -> Self {
        Self::default()
    }

    fn increment(&self) {
        self.count.update(|c| *c += 1);
    }

    fn decrement(&self) {
        self.count.update(|c| *c -= 1);
    }

    fn reset(&self) {
        self.count.set(0);
    }

    fn double(&self) -> Computed<i32> {
        let count = self.count.clone();
        computed(move || count.get() * 2)
    }
}

// User store example
#[derive(Store)]
struct UserStore {
    username: Signal<String>,
    email: Signal<String>,
    is_logged_in: Signal<bool>,
}

impl Default for UserStore {
    fn default() -> Self {
        Self {
            username: signal(String::from("Guest")),
            email: signal(String::from("")),
            is_logged_in: signal(false),
        }
    }
}

impl UserStore {
    fn new() -> Self {
        Self::default()
    }

    fn login(&self, username: String, email: String) {
        self.username.set(username);
        self.email.set(email);
        self.is_logged_in.set(true);
    }

    fn logout(&self) {
        self.username.set(String::from("Guest"));
        self.email.set(String::from(""));
        self.is_logged_in.set(false);
    }

    fn display_name(&self) -> Computed<String> {
        let username = self.username.clone();
        computed(move || {
            let name = username.get();
            if name.is_empty() {
                String::from("Guest")
            } else {
                format!("@{}", name)
            }
        })
    }
}

fn main() -> Result<()> {
    // Demonstrate CounterStore
    println!("=== Counter Store Demo ===");
    let counter = use_store::<CounterStore>();
    println!("Initial count: {}", counter.count.get());

    counter.increment();
    println!("After increment: {}", counter.count.get());

    counter.increment();
    counter.increment();
    println!("After 2 more increments: {}", counter.count.get());

    counter.decrement();
    println!("After decrement: {}", counter.count.get());

    println!("Double (computed): {}", counter.double().get());

    // Demonstrate singleton behavior
    let counter2 = use_store::<CounterStore>();
    println!(
        "\nSingleton test: counter2 has same value: {}",
        counter2.count.get()
    );

    // Demonstrate UserStore
    println!("\n=== User Store Demo ===");
    let user = use_store::<UserStore>();
    println!(
        "Initial user: {} (logged in: {})",
        user.username.get(),
        user.is_logged_in.get()
    );

    user.login("alice".to_string(), "alice@example.com".to_string());
    println!(
        "After login: {} ({}) - logged in: {}",
        user.username.get(),
        user.email.get(),
        user.is_logged_in.get()
    );

    println!("Display name (computed): {}", user.display_name().get());

    user.logout();
    println!(
        "After logout: {} (logged in: {})",
        user.username.get(),
        user.is_logged_in.get()
    );

    // Create a fresh store instance (not a singleton)
    println!("\n=== Fresh Store Instance Demo ===");
    let fresh_counter = create_store::<CounterStore>();
    println!("Fresh counter starts at: {}", fresh_counter.count.get());

    fresh_counter.increment();
    println!(
        "Fresh counter after increment: {}",
        fresh_counter.count.get()
    );

    println!("\nOriginal counter still at: {}", counter.count.get());

    Ok(())
}
