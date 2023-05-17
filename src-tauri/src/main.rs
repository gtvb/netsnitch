// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fmt;
use std::io::{self, BufRead, BufReader};
use std::net::{SocketAddr, TcpStream};
use std::str::FromStr;
use std::sync::mpsc;
use std::thread;
use tauri::{self, Manager};

#[derive(Debug)]
enum ResponseTypeOptions {
    Application,
    Protocol,
    IP,
}

impl fmt::Display for ResponseTypeOptions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ResponseTypeOptions::Application => write!(f, "application-event"),
            ResponseTypeOptions::Protocol => write!(f, "protocol-event"),
            ResponseTypeOptions::IP => write!(f, "ip-event"),
        }
    }
}

#[tauri::command]
fn init_socket_handler(window: tauri::Window) {
    thread::spawn(move || {
        let addrs = [
            (
                ResponseTypeOptions::Application,
                SocketAddr::from_str("127.0.0.1:50000").unwrap(),
            ),
            (
                ResponseTypeOptions::Protocol,
                SocketAddr::from_str("127.0.0.1:50001").unwrap(),
            ),
            (
                ResponseTypeOptions::IP,
                SocketAddr::from_str("127.0.0.1:50002").unwrap(),
            ),
        ];

        let (tx, rx) = mpsc::channel();

        for (response_type, addr) in addrs {
            let txi = tx.clone();
            thread::spawn(move || {
                println!(
                    "Handling {} stream on address {}",
                    response_type.to_string(),
                    addr
                );
                let connection = TcpStream::connect(addr).expect("Could not connect");

                handle_conn(connection, response_type, txi)
            });
        }

        drop(tx);

        for received in rx {
            window
                .emit(&received.0, received.1)
                .expect("could emit event");
        }
    });
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            #[cfg(debug_assertions)]
            {
                let window = app.get_window("main").unwrap();
                window.open_devtools();
                window.close_devtools();
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![init_socket_handler])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn handle_conn(
    stream: TcpStream,
    response_type: ResponseTypeOptions,
    sender: mpsc::Sender<(String, String)>,
) {
    let mut reader = BufReader::new(stream);

    loop {
        let received: Vec<u8> = reader.fill_buf().unwrap().to_vec();
        reader.consume(received.len());

        let response = String::from_utf8(received)
            .map_err(|_| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Couldn't parse received string as utf8",
                )
            })
            .unwrap();


        let mut response = response.split('\n').collect::<Vec<&str>>();
        response.pop().unwrap();

        let json_data = response.last().unwrap();
        let json_data = &json_data[2..json_data.len().checked_sub(2).unwrap_or(2)];

        sender
            .send((response_type.to_string(), json_data.to_string()))
            .expect("failed to send stream data");
    }
}
