use serde::Serialize;

#[derive(Serialize)]
struct SomeStruct {
    a: u8,
    b: u8,
}

fn main() {
    let s = SomeStruct { a: 10, b: 20 };
    let j = serde_json::to_string(&s).unwrap();
    println!("{}", j);
}
