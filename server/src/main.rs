use std::{
    net::{TcpListener, TcpStream},
    thread, fs::create_dir,
};

use fshare::{download_file, get_message, ActionDescrtiptor, send_file, send_index};

fn main() {
    let addr = "0.0.0.0:10000";
    let listener = TcpListener::bind(addr).unwrap();

    let _ = create_dir("./files");

    while let Ok((stream, _)) = listener.accept() {
        thread::spawn(move || {
            accept(stream);
        });
    }
}

fn accept(mut stream: TcpStream) {

    let action:ActionDescrtiptor = get_message(&mut stream).unwrap();
    match action {
        ActionDescrtiptor::Upload => {
            let filename = download_file(&mut stream, "./files/".to_string()).unwrap();
            println!("downloaded {filename}");
        },
        ActionDescrtiptor::Download(filename) => {
            let filepath = format!("./files/{filename}");
            send_file(&filepath, &mut stream);
            println!("uploaded {filename}");
        },
        ActionDescrtiptor::Index => {
            send_index(&mut stream);
            println!("Send index");
        }
    }
}
