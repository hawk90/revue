// Stores are shared between threads (`Store: Send + Sync`): a field that
// cannot be is reported at the derive.
use std::rc::Rc;

#[derive(revue::Store)]
struct Settings {
    shared: Rc<u32>,
}

fn main() {}
