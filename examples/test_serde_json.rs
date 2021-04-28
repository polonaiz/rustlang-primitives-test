use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct SomeStruct {
    a: u8,
    b: u8,
}

fn main() {
    let v = SomeStruct { a: 10, b: 20 };
    let s = serde_json::to_string(&v).unwrap();
    println!("j={}", &s);

    let d:SomeStruct = serde_json::from_str(&s).unwrap();
    println!("d.a={},d.b={}", &d.a, &d.b);
}
