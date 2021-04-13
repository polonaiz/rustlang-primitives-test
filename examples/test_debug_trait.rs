fn main() {
    let mut s: SomeStruct = SomeStruct { a: 1 };
    println!("{:?}", s);
}

#[derive(Debug)]
struct SomeStruct {
    a: u8,
}
