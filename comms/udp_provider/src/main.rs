use std::env;
use std::io;
use std::io::Write;
use std::mem::size_of;
use std::net::SocketAddr;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::thread;
use std::time::Duration;
use tokio::net::UdpSocket;

use crossterm::event::{self, Event};
use serde::{Deserialize, Serialize};
use serde_json;

// Los argumentos que se le deben pasar al script son (ip+puerto) del provider y (ip+puerto) del
// backend

#[derive(Serialize, Deserialize)]
struct BackendData {
    demo_id: u8,
    levitation_mm: f32,
    acceleration_kmh: f32,
    speed_kmh: f32,
}

const REFRESH_RATE: u64 = 200;

#[tokio::main]
async fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let provider_addr: String = String::from(&args[1]);
    let backend_addr: String = String::from(&args[2]);
    let mut headerless_flag: bool = false;

    let argc = env::args().count();
    if argc == 4 && args[3] == "--headerless" || argc == 4 && args[3] == "-hl" {
        headerless_flag = true;
    }

    let backend_data: Arc<Mutex<BackendData>> = Arc::new(Mutex::new(BackendData {
        demo_id: 0,
        levitation_mm: 0.0,
        acceleration_kmh: 0.0,
        speed_kmh: 0.0,
    }));
    let client_list: Arc<Mutex<Vec<SocketAddr>>> = Arc::new(Mutex::new(vec![]));

    let backend_sock: UdpSocket = UdpSocket::bind(backend_addr.clone()).await?;
    let provider_sock: UdpSocket = UdpSocket::bind(provider_addr.clone()).await?;
    println!("Started HypervisorPRO udp server");

    // let listening_client_ref = Arc::clone(&client_list);
    let console_client_ref = Arc::clone(&client_list);
    let console_bk_data_ref = Arc::clone(&backend_data);
    let bk_data_listener_ref = Arc::clone(&backend_data);
    let provider_bk_data_ref = Arc::clone(&backend_data);

    // listener to new devices Connecting
    tokio::spawn(async move {
        backend_listener_thread(bk_data_listener_ref, backend_sock).await;
    });
    tokio::spawn(async move {
        provider_thread(provider_bk_data_ref, provider_sock).await;
    });
    if !headerless_flag {
        thread::spawn(move || {
            console_thread(
                console_client_ref,
                console_bk_data_ref,
                provider_addr,
                backend_addr,
            )
        });
    }

    loop {
        thread::sleep(Duration::from_millis(1000));
    }
}

async fn provider_thread(backend_data: Arc<Mutex<BackendData>>, socket: UdpSocket) -> () {
    let mut buf = [0; size_of::<BackendData>()];
    loop {
        let (_, addr) = socket.recv_from(&mut buf).await.unwrap();

        let serialized_data = {
            let data = match backend_data.lock() {
                Ok(data) => data,
                Err(err) => {
                    eprintln!("Error al bloquear backend_data: {err}");
                    continue;
                }
            };

            match serde_json::to_vec(&*data) {
                Ok(bytes) => bytes,
                Err(err) => {
                    eprintln!("Error serializando datos: {err}");
                    continue;
                }
            }
        }; // El bloqueo se libera aquí.

        if let Err(err) = socket.send_to(&serialized_data, addr).await {
            eprintln!("Error enviando respuesta: {err}");
        }
    }
}

async fn backend_listener_thread(backend_data: Arc<Mutex<BackendData>>, socket: UdpSocket) -> () {
    loop {
        let mut buf = [0; 1024];
        let (n, _) = socket.recv_from(&mut buf).await.unwrap();

        let packet: BackendData = serde_json::from_slice(&buf[0..n]).unwrap();
        // lock pod data to write (its inside a scope because the mutex unloks automatically when it
        // falls out of scope)
        {
            let mut data = backend_data.lock().unwrap();
            data.demo_id = packet.demo_id;
            data.levitation_mm = packet.levitation_mm;
            data.acceleration_kmh = packet.acceleration_kmh;
            data.speed_kmh = packet.speed_kmh;
        }

        thread::sleep(Duration::from_millis(REFRESH_RATE));
    }
}

fn console_thread(
    client_list: Arc<Mutex<Vec<SocketAddr>>>,
    backend_data: Arc<Mutex<BackendData>>,
    provider_addr: String,
    backend_addr: String,
) -> () {
    let stdin = io::stdin();

    print!("\x1B[2J\x1B[H");
    let _ = io::stdout().flush();

    loop {
        println!("Provider address : {provider_addr}");
        println!("Backend  address : {backend_addr}");
        println!("1) Log of clients");
        println!("2) Show live pod data");
        println!("q) Quit");
        print!("\nChoose: ");
        let _ = io::stdout().flush();

        let mut input = String::new();
        _ = stdin.read_line(&mut input);

        match input.trim() {
            "1" => {
                print!("\x1B[2J\x1B[H");
                let _ = io::stdout().flush();

                println!("Log of clients");
                let data = client_list.lock().unwrap();
                println!("{:?}", *data);
                println!("Press any key...");
                let mut pause = String::new();
                _ = stdin.read_line(&mut pause);
            }
            "2" => {
                print!("\x1B[2J\x1B[H");
                let _ = io::stdout().flush();

                // dispach local thread and wait for any input,
                // meanwhile show live data with a refresh rate
                let stop = AtomicBool::new(false);

                thread::scope(|s| {
                    s.spawn(|| {
                        while let Ok(event) = event::read() {
                            if matches!(event, Event::Key(_)) {
                                stop.store(true, Ordering::Relaxed);
                                break;
                            }
                        }
                    });

                    // update data
                    while !stop.load(Ordering::Relaxed) {
                        {
                            let data = backend_data.lock().unwrap();
                            println!("Live Data");
                            println!("[ ] Demo Id       : [{}] ", data.demo_id);
                            println!("[ ] Speed         : [{}] KmH", data.speed_kmh);
                            println!("[ ] Acceleration  : [{}] KmH", data.acceleration_kmh);
                            println!("[ ] Levitation    : [{}] mm", data.levitation_mm);
                        }

                        println!("Press Enter...");

                        thread::sleep(Duration::from_millis(REFRESH_RATE));

                        print!("\x1B[2J\x1B[H");
                        let _ = io::stdout().flush();
                    }
                });
            }
            "q" => {
                std::process::exit(1);
            }
            _ => {
                println!("Invalid option");
            }
        }

        print!("\x1B[2J\x1B[H");
        let _ = io::stdout().flush();
    }
}
