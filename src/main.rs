use std::io::Read;
use flate2::read::GzDecoder;

fn main() {}

#[test]
fn test_read_file() {
    let path = std::path::Path::new("./src/test.txt");
    let mut file = std::fs::File::open(path).unwrap();

    let mut buf = [0; 1024];
    let size = file.read(&mut buf).unwrap();
    println!("size: {}", size);
    println!("buf: {:x?}", &buf[..size]);
}

#[test]
fn test_read_file_using_reader() {
    let path = std::path::Path::new("./src/test.txt");
    let file = std::fs::File::open(path).unwrap();
    let mut reader = std::io::BufReader::new(file);

    let mut buf: [u8; 1024] = [0; 1024];
    let size = reader.read(&mut buf).unwrap();
    println!("size: {}", size);
    println!("buf: {:x?}", &buf[..size]);
    println!("buf: {}", std::str::from_utf8(&buf).unwrap());
}

#[test]
fn test_read_file_from_ftp() {
    let ftp_addr = "localhost:21";
    let ftp_user = "myuser";
    let ftp_pass = "mypass";
    let filename = "asset/test.txt";

    let mut ftp_stream = ftp::FtpStream::connect(ftp_addr).unwrap();
    ftp_stream.login(ftp_user, ftp_pass).expect("login fail");
    let mut reader = ftp_stream.get(filename).unwrap();

    let mut buf: [u8; 1024] = [0; 1024];
    let size = reader.read(&mut buf).unwrap();
    println!("size: {}", size);
    println!("buf: {:x?}", &buf[..size]);
    println!("buf: {}", std::str::from_utf8(&buf).unwrap());
}

#[test]
fn test_read_gz_file_from_ftp() {
    let ftp_addr = "localhost:21";
    let ftp_user = "myuser";
    let ftp_pass = "mypass";
    let filename = "asset/test.txt.gz";

    let mut ftp_stream = ftp::FtpStream::connect(ftp_addr).unwrap();
    ftp_stream.login(ftp_user, ftp_pass).expect("login fail");
    let ftp_file_reader = ftp_stream.get(filename).unwrap();
    let mut reader = GzDecoder::new(ftp_file_reader);

    let mut buf: [u8; 1024] = [0; 1024];
    let size = reader.read(&mut buf).unwrap();
    println!("size: {}", size);
    println!("buf: {:x?}", &buf[..size]);
    println!("buf: {}", std::str::from_utf8(&buf).unwrap());
}
