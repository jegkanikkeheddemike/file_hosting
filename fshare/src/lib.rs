use std::{
    fmt::Debug,
    fs::{self, File, OpenOptions},
    io::{Read, Write},
    mem::size_of,
    net::TcpStream,
    thread,
    time::Duration,
};

use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct FileDescriptor {
    pub filename: String,
    pub filelen: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ActionDescrtiptor {
    Upload,
    Download(String),
    Index,
}

pub type FileIndex = Vec<FileDescriptor>;

const PARTSIZE: usize = 512; //

pub fn send_file(filepath: &str, stream: &mut TcpStream) {
    let mut file = File::open(filepath.clone()).unwrap();

    let filename = filepath.split("/").last().unwrap().to_string();
    let filelen = file.metadata().unwrap().len();

    let filedescriptor = FileDescriptor { filename, filelen };

    send_message(stream, filedescriptor);

    let mut buffer = vec![0u8; PARTSIZE];
    loop {
        let read_count = file.read(&mut buffer).unwrap();
        let mut send_buffer = vec![0u8; read_count];
        send_buffer.copy_from_slice(&buffer[..read_count]);
        send_message(stream, send_buffer);

        if read_count != PARTSIZE {
            break;
        }
    }
}

pub fn download_file(stream: &mut TcpStream, mut path: String) -> Result<String,String> {
    let filedescriptor: FileDescriptor = get_message(stream)?;

    if !path.ends_with("/") {
        path = format!("{path}/")
    }

    let filepath = format!("{path}{}", filedescriptor.filename);

    //remove old file if it exists
    let _ = fs::remove_file(filepath.clone());

    while let Ok(_) = fs::metadata(filepath.clone()) {
        thread::sleep(Duration::from_millis(10));
        println!("Waiting to delete old");
    }

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(filepath)
        .unwrap();

    loop {
        let bin: Vec<u8> = get_message(stream)?;
        file.write(&bin).unwrap();
        if bin.len() != PARTSIZE {
            break;
        }
    }
    Ok(filedescriptor.filename)
}

pub fn send_message<T: Serialize + DeserializeOwned + Debug>(stream: &mut TcpStream, message: T) {
    let msg_bin = bincode::serialize(&message).unwrap();
    let length = msg_bin.len() as u32;

    let len_bin = bincode::serialize(&length).unwrap();

    stream.write_all(&len_bin).unwrap();
    stream.write_all(&msg_bin).unwrap();
}

pub fn get_message<T: DeserializeOwned + Serialize + Debug>(stream: &mut TcpStream) -> Result<T,String> {
    let mut len_bin = vec![0u8; size_of::<u32>()];

    stream.read_exact(&mut len_bin).unwrap();
    let length: u32 = bincode::deserialize(&len_bin).unwrap();

    let mut msg_bin = vec![0u8; length as usize];

    stream.read_exact(&mut msg_bin).unwrap();

    let message: T = bincode::deserialize(&msg_bin).unwrap();

    Ok(message)
}

pub fn send_index(stream: &mut TcpStream) {
    let paths = fs::read_dir("./files").unwrap();

    let mut fileindex: FileIndex = vec![];

    for path in paths {
        let path = path.unwrap();
        let md = path.metadata().unwrap();
        let filename = path
            .file_name()
            .into_string()
            .unwrap()
            .split("/")
            .last()
            .unwrap()
            .to_string();

        let filelen;
        if md.is_file() {
            filelen = md.len();
        } else {
            filelen = 0;
        }
        fileindex.push(FileDescriptor { filename, filelen })
    }
    send_message(stream, fileindex);
}
