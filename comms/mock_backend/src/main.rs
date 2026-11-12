use serde::Serialize;
use std::env;
use std::io;
use std::io::Write;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::{Duration, Instant};
use tokio::net::UdpSocket;

#[derive(Serialize)]
struct BackendData {
    demo_id: u8,
    levitation_mm: f32,
    speed_kmh: f32,
}

const MOCK_SEND_RATE: u64 = 200;

const MOCK_MAX_VELOCITY: f32 = 200.0;
const MOCK_ACCELERATION_RATE: f32 = 3.0;
const MOCK_MAX_ACCELERATION: f32 = 30.0;

const MOCK_MAX_LEVITATION_MM: f32 = 60.0;
const MOCK_LEVITATION_OFFSET_MM: f32 = 10.0;

#[tokio::main]
async fn main() -> io::Result<()> {
    println!("Hello, world!");

    let args: Vec<String> = env::args().collect();
    let provider_addr: String = String::from(&args[1]);
    let backend_addr: String = String::from(&args[2]);
    let provider_addr_clone: String = provider_addr.clone();

    let backend_data: Arc<Mutex<BackendData>> = Arc::new(Mutex::new(BackendData {
        demo_id: 0,
        levitation_mm: 1.0,
        speed_kmh: 100.0,
    }));
    // create shared pointer for console thread
    let bk_data_ref: Arc<Mutex<BackendData>> = Arc::clone(&backend_data);

    let backend_sock: UdpSocket = UdpSocket::bind(backend_addr.clone()).await?;

    thread::spawn(move || {
        console_thread(bk_data_ref, backend_addr, provider_addr);
    });

    loop {
        let serialized_data = {
            let data = match backend_data.lock() {
                Ok(data) => data,
                Err(err) => {
                    eprintln!("Error al bloquear {err}");
                    continue;
                }
            };

            match serde_json::to_vec(&*data) {
                Ok(bytes) => bytes,
                Err(err) => {
                    eprint!("Error serializando {err}");
                    continue;
                }
            }
        };

        if let Err(err) = backend_sock
            .send_to(&serialized_data, &provider_addr_clone)
            .await
        {
            eprintln!("Error enviando el paquete: {err}");
        }

        tokio::time::sleep(Duration::from_millis(MOCK_SEND_RATE)).await;
    }
}

fn console_thread(
    backend_data: Arc<Mutex<BackendData>>,
    backend_addr: String,
    provider_addr: String,
) -> () {
    let stdin = io::stdin();

    print!("\x1B[2J\x1B[H");
    let _ = io::stdout().flush();

    loop {
        println!("Provider address : {provider_addr}");
        println!("Backend  address : {backend_addr}");
        println!("1) Select Demo");
        println!("q) Quit");
        print!("\nChoose: ");
        let _ = io::stdout().flush();

        let mut input = String::new();
        _ = stdin.read_line(&mut input);

        match input.trim() {
            "1" => {
                print!("\x1B[2J\x1B[H");
                let _ = io::stdout().flush();

                println!("Choose Demo");
                println!("1) levitation");
                println!("2) Oscilating movement");
                println!("3) Full Sequence");

                print!("> ");
                let _ = io::stdout().flush();
                let mut demo_input: String = String::new();
                _ = stdin.read_line(&mut demo_input);

                let bk_ref = Arc::clone(&backend_data);
                let demo_thread_h = thread::spawn(move || demo_thread(bk_ref, demo_input.clone()));
                demo_thread_h.join().unwrap();
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

fn demo_thread(backend_data: Arc<Mutex<BackendData>>, demo_index: String) {
    // flush screen
    print!("\x1B[2J\x1B[H");
    let _ = io::stdout().flush();

    match demo_index.trim() {
        "1" => {
            demo_levitation(Arc::clone(&backend_data));
        }

        "2" => {}

        "3" => {}

        _ => {
            println!("Invalid Demo Index");
        }
    }

    // print progress and q to quit
}

fn demo_levitation(backend_data: Arc<Mutex<BackendData>>) -> () {
    let loc_velocity: f32 = 0.0;
    let mut loc_levitation: f32 = 0.0;

    let mut last_frame = Instant::now();
    let mut now_frame: Instant;
    let mut deltatime: Duration;
    let start = Instant::now();
    while start.elapsed() <= Duration::from_secs(2) {
        now_frame = Instant::now();
        deltatime = now_frame - last_frame;
        last_frame = now_frame;

        if loc_levitation < MOCK_MAX_LEVITATION_MM {
            loc_levitation += 50.0 * deltatime.as_secs_f32();
        }

        {
            let mut live_data = backend_data.lock().unwrap();
            live_data.levitation_mm = loc_levitation;
            live_data.speed_kmh = loc_velocity;
        }
    }
}
