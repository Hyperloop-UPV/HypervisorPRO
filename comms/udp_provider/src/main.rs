use std::env;
use std::io;
use std::io::Write;
use std::thread;
use std::sync::Arc;
use std::sync::Mutex;
use tokio::net::UdpSocket;
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> io::Result<()> {


    let args : Vec<String> = env::args().collect();
    let address_and_port : String = String::from(String::clone(&args[1]) + ":" + &args[2]);

    let client_list : Arc<Mutex<Vec<SocketAddr>>> = Arc::new(Mutex::new(vec![]));
    println!("Server ip address is {address_and_port}");


    let sock = UdpSocket::bind(address_and_port).await?;
    println!("Started HypervisorPRO udp server");


    let listening_client_ref = Arc::clone(&client_list);
    let console_client_ref   = Arc::clone(&client_list);

    tokio::spawn(async move {
        let mut buf = [0; 1024];
        loop {
            let (len, addr) = sock.recv_from(&mut buf).await?;
            println!("new Listener with ip {addr}");
            new_listener_callback(listening_client_ref, &addr);
        }
        Ok(())
    });

    let console_thread_h = thread::spawn(move || console_thread(console_client_ref));

    let _ = console_thread_h.join();
    loop {}
}


async fn new_listener_callback(listener_list : Arc<Mutex<Vec<SocketAddr>>>, new_ip : &SocketAddr) -> () {

    listener_list.lock().unwrap().push(*new_ip);

}

fn console_thread(client_list : Arc<Mutex<Vec<SocketAddr>>>) -> () {
     let stdin = io::stdin();

    loop {
 
        println!("1) List clients");
        println!("q) Quit");
        print!("\nChoose: ");
        let _ = io::stdout().flush();

        let mut input = String::new();
        _ = stdin.read_line(&mut input);

        match input.trim() {
            "1" => {
                let data = client_list.lock().unwrap();
                println!("{:?}", *data);

                let mut pause = String::new();
                _ = stdin.read_line(&mut pause);
            }
            "q" => { break }
            _ => { println!("Invalid option");}
        }

        print!("\x1B[2J\x1B[H");
        let _ = io::stdout().flush();
    }
}
