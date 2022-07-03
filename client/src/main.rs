use std::{net::TcpStream, env};

use fshare::{send_file, send_message, ActionDescrtiptor, download_file};

fn main() {
    let addr = "0.0.0.0:10000";
    let mut stream = TcpStream::connect(addr).unwrap();


    let args:Vec<String> = env::args().collect();

    match args.get(1) {
        Some(protocol) => {
            match protocol.as_str() {
                "upload" => {
                    match args.get(2) {
                        Some(filepath) => {
                            send_message(&mut stream, ActionDescrtiptor::Upload);
                            send_file(filepath, &mut stream);
                        },
                        None => {
                            println!("Failed to parse file path");
                        },
                    }
                },
                "download" => {
                    match args.get(2) {
                        Some(filename) => {
                            send_message(&mut stream, ActionDescrtiptor::Download(filename.into()));
                            download_file(&mut stream);
                        },
                        None => {
                            println!("Failed to parse file name");
                        },
                    }
                },
                _ => {
                    println!("Failed to parse protocol, Use \"upload\" or \"download\"");
                }
            }
        },
        None => {
            println!("Failed to parse protocol, Use \"upload\" or \"download\"");
        },
    }
}
