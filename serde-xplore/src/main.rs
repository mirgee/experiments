use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
enum Foo {
    Bar { u: u64 },
    Baz { s: String },
}

fn main() {
    println!("Hello, world!");
}
