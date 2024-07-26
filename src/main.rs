use std::{
    env,
    thread,
    net::{
        TcpListener,
        TcpStream
    },
    time::{
        SystemTime,
        UNIX_EPOCH
    },
    path::PathBuf,
    io::Read
};
use rusqlite::{
    Connection,
    params
};
use serde::{
    Serialize, 
    Deserialize
};

#[derive(Serialize, Deserialize, Debug)]
struct TempDataStruct {
    temperature: f64,
    humidity: f64,
    co2: i32,
}

fn insert_temp_data(conn: &Connection, temp_data: TempDataStruct, unix_time: u64) {
    let query = "
        INSERT INTO tempData (temperature, humidity, co2, time)
        VALUES (?1, ?2, ?3, ?4);
    ";
    
    conn.execute(query, params![temp_data.temperature, temp_data.humidity, temp_data.co2, unix_time]).unwrap();
}

fn get_unix_time() -> u64 {
    let start = SystemTime::now();

    match start.duration_since(UNIX_EPOCH) {
        Ok(duration) => {
            duration.as_secs()
        }
        Err(_) => {
            0
        }
    }
}

fn handle_client(mut stream: TcpStream, conn: Connection) {
    let mut buffer = [0; 1024];
    
    while match stream.read(&mut buffer) {
        Ok(size) if size > 0 => {
            let temp_data_json = String::from_utf8_lossy(&buffer[..size]);
            let temp_data: TempDataStruct = serde_json::from_str(&temp_data_json).unwrap();

            insert_temp_data(&conn, temp_data, get_unix_time());

            println!("Received: {}", temp_data_json);
            
            true
        },
        _ => false,
    } {}
}

fn get_relative_path() -> PathBuf {
    let exe_path = env::current_exe().expect("Failed to get current executable path");
    let exe_dir = exe_path.parent().expect("Failed to get executable directory");
    exe_dir.to_path_buf()
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:873").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let conn = Connection::open(get_relative_path().join("temps.db")).unwrap();
                conn.execute("CREATE TABLE IF NOT EXISTS tempData (id INTEGER PRIMARY KEY, temperature REAL, humidity REAL, co2 INTEGER, time INTEGER)", [],).unwrap();
                thread::spawn(move || handle_client(stream, conn));
            }
            Err(e) => eprintln!("Failed to accept connection: {:?}", e),
        }
    }
}
