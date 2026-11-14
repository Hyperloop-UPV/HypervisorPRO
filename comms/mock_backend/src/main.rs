use crossterm::event;
use crossterm::event::Event;
use serde::Serialize;
use std::env;
use std::io;
use std::io::Write;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::thread;
use std::time::Duration;
use tokio::net::UdpSocket;

#[derive(Serialize)]
struct BackendData {
    demo_id: u8,
    levitation_mm: f32,
    acceleration_kmh: f32,
    speed_kmh: f32,
}

const MOCK_SEND_RATE: u64 = 200;
const MOCK_UPDATE_RATE: u64 = 50;

const MOCK_MAX_VELOCITY: f32 = 10.0;
const MOCK_ACCELERATION_RATE: f32 = 0.05;
const MOCK_MAX_ACCELERATION: f32 = 1.0;

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
        levitation_mm: 0.0,
        acceleration_kmh: 0.0,
        speed_kmh: 0.0,
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
                println!("1) Connection Test");
                println!("2) Levitation");
                println!("3) Oscilating movement");
                println!("4) Full Speed");

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

    {
        let mut data = backend_data.lock().unwrap();
        data.demo_id = demo_index.as_bytes()[0] - b'0';
    }

    match demo_index.trim() {
        "1" => {
            demo_random(Arc::clone(&backend_data));
        }

        "2" => {
            demo_levitation(Arc::clone(&backend_data));
        }

        "3" => {
            demo_oscilation(Arc::clone(&backend_data));
        }

        "4" => {}

        _ => {
            println!("Invalid Demo Index");
        }
    }

    {
        let mut data = backend_data.lock().unwrap();
        data.demo_id = 0;
        data.acceleration_kmh = 0.0;
        data.speed_kmh = 0.0;
        data.levitation_mm = 0.0;
    }
}

fn demo_random(backend_data: Arc<Mutex<BackendData>>) -> () {
    // dispach local thread and wait for any input,
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

        s.spawn(|| {
            // update data
            while !stop.load(Ordering::Relaxed) {
                println!("Press Enter to stop simulation...");

                thread::sleep(Duration::from_millis(MOCK_SEND_RATE));

                print!("\x1B[2J\x1B[H");
                let _ = io::stdout().flush();
            }
        });

        while !stop.load(Ordering::Relaxed) {
            let rand_levitation = rand::random_range(1.0..MOCK_MAX_LEVITATION_MM);
            let rand_velocity = rand::random_range(1.0..MOCK_MAX_VELOCITY);
            let rand_acceleration = rand::random_range(1.0..MOCK_MAX_ACCELERATION);

            {
                let mut live_data = backend_data.lock().unwrap();
                live_data.levitation_mm = rand_levitation;
                live_data.acceleration_kmh = rand_acceleration;
                live_data.speed_kmh = rand_velocity;
            }
            thread::sleep(Duration::from_millis(MOCK_SEND_RATE));
        }
    });
}

fn demo_levitation(backend_data: Arc<Mutex<BackendData>>) -> () {
    let mut loc_levitation: f32 = 0.0;

    let stop = AtomicBool::new(false);

    thread::scope(|s| {
        // ask for input and lock demo loop
        s.spawn(|| {
            while let Ok(event) = event::read() {
                if matches!(event, Event::Key(_)) {
                    stop.store(true, Ordering::Relaxed);
                    break;
                }
            }
        });

        // print console
        s.spawn(|| {
            // update data
            while !stop.load(Ordering::Relaxed) {
                println!("Press Enter to stop simulation...");

                thread::sleep(Duration::from_millis(MOCK_SEND_RATE));

                print!("\x1B[2J\x1B[H");
                let _ = io::stdout().flush();
            }
        });

        while !stop.load(Ordering::Relaxed) {
            if loc_levitation <= MOCK_MAX_LEVITATION_MM {
                loc_levitation += 1.0;
            }
            {
                let mut live_data = backend_data.lock().unwrap();
                live_data.levitation_mm = loc_levitation;
                live_data.acceleration_kmh = 0.0;
                live_data.speed_kmh = 0.0;
            }
            thread::sleep(Duration::from_millis(MOCK_UPDATE_RATE)); // for performance reasons
        }
    });
}

fn demo_oscilation(backend_data: Arc<Mutex<BackendData>>) -> () {
    let loc_levitation: f32 = MOCK_MAX_LEVITATION_MM;
    let mut loc_speed: f32 = 0.0;
    let mut loc_acceleration = 0.0;

    let stop = AtomicBool::new(false);

    thread::scope(|s| {
        // ask for input and lock demo loop
        s.spawn(|| {
            while let Ok(event) = event::read() {
                if matches!(event, Event::Key(_)) {
                    stop.store(true, Ordering::Relaxed);
                    break;
                }
            }
        });

        // print console
        s.spawn(|| {
            // update data
            while !stop.load(Ordering::Relaxed) {
                println!("Press Enter to stop simulation...");

                thread::sleep(Duration::from_millis(MOCK_SEND_RATE));
                print!("\x1B[2J\x1B[H");
                let _ = io::stdout().flush();
            }
        });

        let mut direction: f32 = 1.0;
        let mut accelerating: f32 = 1.0;
        // main demo loop
        while !stop.load(Ordering::Relaxed) {
            // reached top speed, need to decelerate
            if (loc_speed >= MOCK_MAX_VELOCITY || loc_speed <= -MOCK_MAX_VELOCITY)
                && accelerating == 1.0
            {
                accelerating = -1.0;
            }

            // reached bottom speed, need to accelerate and change direction
            if ((loc_speed <= 0.0 && direction == 1.0) || (loc_speed >= 0.0 && direction == -1.0))
                && accelerating == -1.0
            {
                if direction == -1.0 {
                    direction = 1.0;
                } else {
                    direction = -1.0;
                }
                accelerating = 1.0;
            }

            loc_acceleration += MOCK_ACCELERATION_RATE * accelerating;
            loc_speed += loc_acceleration * direction;

            loc_acceleration =
                loc_acceleration.clamp(-MOCK_MAX_ACCELERATION, MOCK_MAX_ACCELERATION);
            {
                let mut live_data = backend_data.lock().unwrap();
                live_data.levitation_mm = loc_levitation;
                live_data.acceleration_kmh = loc_acceleration;
                live_data.speed_kmh = loc_speed;
            }
            thread::sleep(Duration::from_millis(MOCK_UPDATE_RATE));
        }
    });
}
