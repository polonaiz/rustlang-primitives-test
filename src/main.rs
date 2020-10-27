use flate2::read::GzDecoder;
use std::io::BufRead;
use std::io::Read;

fn main() {}

#[test]
fn test_read_file() {
    let path = std::path::Path::new("./asset/test.txt");
    let mut file = std::fs::File::open(path).unwrap();

    let mut buf = [0; 1024];
    let size = file.read(&mut buf).unwrap();
    println!("size: {}", size);
    println!("buf: {:x?}", &buf[..size]);
}

#[test]
fn test_read_file_using_reader() {
    let path = std::path::Path::new("./asset/test.txt");
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

#[test]
fn test_drain_large_file() {
    let path = std::path::Path::new("large.sql.gz");
    let mut file = std::fs::File::open(path).unwrap();

    let mut buf = [0; 1024 * 1024];
    loop {
        let size = file.read(&mut buf).unwrap();
        if size == 0 {
            break;
        }
    }
}

#[test]
fn test_drain_large_file_using_reader() {
    let path = std::path::Path::new("asset/large/mysqldump.sql.gz");
    let file = std::fs::File::open(path).unwrap();
    let gz_decoder = GzDecoder::new(file);
    let reader = std::io::BufReader::new(gz_decoder);

    for line in reader.lines().take(10).map(|result| result.unwrap()) {
        println!("{}", line);
    }
}

#[test]
fn test_drain_gz_file_from_ftp() {
    let ftp_addr = "localhost:21";
    let ftp_user = "myuser";
    let ftp_pass = "mypass";
    let filename = "asset/large/mysqldump.sql.gz";

    let mut ftp_stream = ftp::FtpStream::connect(ftp_addr).unwrap();
    ftp_stream.login(ftp_user, ftp_pass).expect("login fail");
    let ftp_reader = ftp_stream.get(filename).unwrap();
    let gz_decoder = GzDecoder::new(ftp_reader);
    let reader = std::io::BufReader::new(gz_decoder);

    for line in reader.lines().take(65).map(|result| result.unwrap()) {
        println!("{}", &compact_line(&line));
    }
}

#[test]
fn test_parse_file() {
    let path = std::path::Path::new("./asset/large/mysqldump.sql.gz");
    let file = std::fs::File::open(path).unwrap();
    let gz_decoder = GzDecoder::new(file);
    let file_reader = std::io::BufReader::new(gz_decoder);

    let mut line_num = 0;
    for line in file_reader.lines().take(65).map(|result| result.unwrap()) {
        line_num += 1;

        println!("{}: {}", line_num, &compact_line(&line));
        if line.starts_with("INSERT INTO ") {
            // let line_reader = std::io::BufReader::new(stringreader::StringReader::new(&line));
            println!("  ---->");
        }
    }
}

fn compact_line(line: &String) -> String {
    let mut output = String::new();

    let lhs_size = 100;
    let rhs_size = 100;
    if line.len() <= lhs_size + rhs_size {
        output.push_str(&line);
    } else {
        output.push_str(&line.chars().take(lhs_size).collect::<String>());
        output.push_str(".............................");
        output.push_str(
            &line
                .chars()
                .rev()
                .take(rhs_size)
                .collect::<String>()
                .chars()
                .rev()
                .collect::<String>(),
        );
    }
    output
}
