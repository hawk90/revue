// A store type has one id: a generic one is rejected with a clear error.
#[derive(revue::Store)]
struct Cache<T> {
    item: T,
}

fn main() {}
