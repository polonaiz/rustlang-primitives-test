fn main() {}

#[test]
fn test_read_file() {
    use std::io::Read;
    let path = std::path::Path::new("./src/test.txt");
    let mut file = std::fs::File::open(path).unwrap();
    let mut buf = [0; 1024];
    let size = file.read(&mut buf).unwrap();
    println!("size: {}", size);
    println!("buf: {:x?}", &buf[..size]);
}
