use std::{env, net::TcpStream};

use fshare::{download_file, get_message, send_file, send_message, ActionDescrtiptor, FileIndex};

fn main() {
    let addr = "192.168.0.94:10000";
    let mut stream = TcpStream::connect(addr).unwrap();

    let args: Vec<String> = env::args().collect();

    match args.get(1) {
        Some(protocol) => match protocol.as_str() {
            "upload" => match args.get(2) {
                Some(filepath) => {
                    send_message(&mut stream, ActionDescrtiptor::Upload);
                    send_file(filepath, &mut stream);
                }
                None => {
                    println!("Failed to parse file path");
                }
            },
            "download" => match args.get(2) {
                Some(filename) => {
                    send_message(&mut stream, ActionDescrtiptor::Download(filename.into()));
                    download_file(&mut stream, "./".into());
                }
                None => {
                    println!("Failed to parse file name");
                }
            },
            "index" => {
                send_message(&mut stream, ActionDescrtiptor::Index);
                let fileindex: FileIndex = get_message(&mut stream);
                let col1 = "filename";
                let col2 = "size (bytes)";
                println!("{col1:15}{col2}");
                for file in fileindex {
                    println!(
                        "{fname:15}{filesize}",
                        fname = file.filename,
                        filesize = file.filelen
                    );
                }
            }
            _ => {
                println!("Failed to parse protocol, Use \"upload\", \"download\" or \"index\" to get avalable files");
            }
        },
        None => {
            println!("Failed to parse protocol, Use \"upload\", \"download\" or \"index\" to get avalable files");
        }
    }
}
