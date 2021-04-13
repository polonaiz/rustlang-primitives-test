fn main() {
    let s: SomeStruct = SomeStruct { a: 1 };
    println!("{:?}", s);
}

#[derive(Debug)]
struct SomeStruct {
    a: u8,
}
