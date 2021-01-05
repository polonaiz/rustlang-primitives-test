use flate2::read::GzDecoder;
use itertools::Itertools;
use std::io::BufRead;
use std::io::Write;
use std::{io::Read, process::Command};

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
#[ignore]
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
#[ignore]
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
#[ignore]
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
#[ignore]
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

#[test]
fn require_mysql_56() {
    let command = format!("docker ps -q --filter name=mysql-5.6");
    let output = Command::new("sh").arg("-c").arg(&command).output().unwrap();
    let stdout = String::from_utf8(output.stdout)
        .unwrap()
        .trim_end()
        .to_string();
    let stdoutlen = stdout.len();
    let mysql_exists = stdoutlen > 0;
    if !mysql_exists {
        let command = format!("docker run --rm -d -e MYSQL_ROOT_PASSWORD=mysql-password -v /tmp:/tmp -p 33061:3306 --name 'mysql-5.6' mysql:5.6");
        let output = Command::new("sh").arg("-c").arg(&command).output().unwrap();
        println!(
            "command: {}\n output: {}",
            command,
            match output.status.code().unwrap() {
                0 => String::from_utf8(output.stdout)
                    .unwrap()
                    .trim_end()
                    .to_string(),
                _ => String::from_utf8(output.stderr)
                    .unwrap()
                    .trim_end()
                    .to_string(),
            }
        );

        // wait start mysql
        std::thread::sleep(std::time::Duration::from_secs(15));
        println!("mysql started.");
    } else {
        println!("mysql already started.");
    }
}

#[test]
fn execute_mysql_56() {
    let command = format!(
        "docker exec mysql-5.6 sh -c '{}'",
        "cat /tmp/mysql.setup.sql | MYSQL_PWD=mysql-password mysql"
    );
    let output = Command::new("sh").arg("-c").arg(&command).output().unwrap();
    println!(
        "command: {}\n output: {}",
        command,
        match output.status.code().unwrap() {
            0 => String::from_utf8(output.stdout)
                .unwrap()
                .trim_end()
                .to_string(),
            _ => String::from_utf8(output.stderr)
                .unwrap()
                .trim_end()
                .to_string(),
        }
    );
}

#[test]
pub fn test_write_data_file() {
    require_mysql_56();

    // setup.sql
    let path = std::path::Path::new("/tmp/mysql.setup.sql");
    let mut file = std::fs::File::create(path).unwrap();

    // test table schema
    let columns = vec![
        Column {
            name: "bigint_col",
            data_type: "BIGINT",
        },
        Column {
            name: "varchar_col",
            data_type: "VARCHAR(255)",
        },
    ];

    let sql = "CREATE DATABASE IF NOT EXISTS TEST;\n";
    println!("{}", sql.trim_end());
    file.write(sql.as_bytes()).unwrap();

    let sql = "USE TEST;\n";
    println!("{}", sql.trim_end());
    file.write(sql.as_bytes()).unwrap();

    let sql = "DROP TABLE IF EXISTS TEST;\n";
    println!("{}", sql);
    file.write(sql.as_bytes()).unwrap();

    // create table
    let mut sql = String::new();
    sql.push_str("CREATE TABLE TEST (\n");
    sql.push_str(
        columns
            .iter()
            .map(|column| format!("  {} {}", column.name, column.data_type))
            .join(",\n")
            .as_str(),
    );
    sql.push_str("\n);\n");
    println!("{}", sql.trim_end());
    file.write(sql.as_bytes()).unwrap();

    // insert
    for idx in 0..10 {
        let vals = vec![
            idx.to_string(),
            format!("'{:x}'", md5::compute(idx.to_string())),
        ];
        let mut sql = String::new();
        sql.push_str("INSERT INTO TEST VALUES (");
        sql.push_str(vals.iter().join(",").as_str());
        sql.push_str(");\n");
        println!("{}", sql.trim_end());
        file.write(sql.as_bytes()).unwrap();
    }

    let command = format!(
        "docker exec mysql-5.6 sh -c '{}'",
        "cat /tmp/mysql.setup.sql | MYSQL_PWD=mysql-password mysql"
    );
    let output = Command::new("sh").arg("-c").arg(&command).output().unwrap();
    println!(
        "command: {}\n output: {}",
        command,
        match output.status.code().unwrap() {
            0 => String::from_utf8(output.stdout)
                .unwrap()
                .trim_end()
                .to_string(),
            _ => String::from_utf8(output.stderr)
                .unwrap()
                .trim_end()
                .to_string(),
        }
    );
}

struct Column {
    name: &'static str,
    data_type: &'static str,
}
