use std::env;
use std::io;
use std::io::Write;
use std::thread;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Mutex;
use tokio::net::UdpSocket;
use std::net::SocketAddr;
use std::time::Duration;

use rand;
use crossterm::event::{self, Event};


// Los argumentos que se le deben pasar al script son (ip+puerto) del backend y (ip+puerto) del
// punto de acceso


struct BackendData {

    levitation_mm : f32,
    speed_kmh     : f32,

}

const REFRESH_RATE : u64 = 200;

#[tokio::main]
async fn main() -> io::Result<()> {


    let args : Vec<String> = env::args().collect();
    let provider_addr : String = String::from(&args[1]);
    let backend_addr  : String = String::from(&args[2]);

    let backend_data  : Arc<Mutex<BackendData>> = Arc::new(Mutex::new(BackendData { levitation_mm: 1.0, speed_kmh: 100.0}));
    let client_list   : Arc<Mutex<Vec<SocketAddr>>> = Arc::new(Mutex::new(vec![]));

    let backend_sock : UdpSocket = UdpSocket::bind(backend_addr.clone()).await?;
    let provider_sock : UdpSocket = UdpSocket::bind(provider_addr.clone()).await?;
    println!("Started HypervisorPRO udp server");


    let listening_client_ref = Arc::clone(&client_list);
    let console_client_ref   = Arc::clone(&client_list);
    let console_bk_data_ref  = Arc::clone(&backend_data);
    let bk_data_listener_ref = Arc::clone(&backend_data);

    // listener to new devices Connecting
    let _backend_listener_h  = tokio::spawn(async move { backend_listener_thread(bk_data_listener_ref, backend_sock).await; } );
    let _provider_h = tokio::spawn(async move { provider_thread(listening_client_ref, provider_sock).await; } );
    let _console_thread_h    = thread::spawn(move || console_thread(console_client_ref, console_bk_data_ref, provider_addr, backend_addr));

    // broadcast loop

    loop {}
}


async fn provider_thread(listener_list : Arc<Mutex<Vec<SocketAddr>>>, socket : UdpSocket) -> () {

        let mut buf = [0; 1024];
        loop {
            let (len, addr) = socket.recv_from(&mut buf).await.unwrap();
            println!("new Listener with ip {addr}");
            listener_list.lock().unwrap().push(addr);

            let _ = socket.send_to(&buf[..len], addr).await.unwrap();
        }
}

async fn backend_listener_thread(backend_data : Arc<Mutex<BackendData>>, socket : UdpSocket) -> () {

    loop {
        //let mut buf = [0; 1024];
        //let (buf, addr) = socket.recv_from(&mut buf).await.unwrap();

        let random_levitation: f32 = rand::random_range(0.0..1.0); // Generate a random
        let random_velocity : f32 = rand::random_range(0.0..100.0);
        {
        let mut data = backend_data.lock().unwrap();
        data.levitation_mm = random_levitation;
        data.speed_kmh = random_velocity;
        }
        thread::sleep(Duration::from_millis(REFRESH_RATE));
    }

}

fn console_thread(client_list : Arc<Mutex<Vec<SocketAddr>>>, backend_data : Arc<Mutex<BackendData>> , provider_addr : String, backend_addr : String) -> () {
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
                    println!("[ ] Speed      : [{}] KmH", data.speed_kmh);
                    println!("[ ] Levitation : [{}] mm", data.levitation_mm);
                    }
                    println!("Press any key...");

                    thread::sleep(Duration::from_millis(REFRESH_RATE));
                    print!("\x1B[2J\x1B[H");
                    let _ = io::stdout().flush();
                }
                });



            }
            "q" => { std::process::exit(1); }
            _ => { println!("Invalid option");}
        }

        print!("\x1B[2J\x1B[H");
        let _ = io::stdout().flush();
    }
}
