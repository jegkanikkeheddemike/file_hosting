use std::{
    net::{TcpListener, TcpStream},
    thread,
};

use fshare::{download_file, get_message, ActionDescrtiptor, send_file};

fn main() {
    let addr = "0.0.0.0:10000";
    let listener = TcpListener::bind(addr).unwrap();

    while let Ok((stream, _)) = listener.accept() {
        thread::spawn(move || {
            accept(stream);
        });
    }
}

fn accept(mut stream: TcpStream) {

    let action:ActionDescrtiptor = get_message(&mut stream);
    match action {
        ActionDescrtiptor::Upload => {
            let filename = download_file(&mut stream);
            println!("downloaded {filename}");
        },
        ActionDescrtiptor::Download(filename) => {
            let filepath = format!("./{filename}");
            send_file(&filepath, &mut stream);
            println!("uploaded {filename}");
        },
    }
}
