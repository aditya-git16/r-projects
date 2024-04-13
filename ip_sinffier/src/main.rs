use std::{env, io, net::{IpAddr , TcpStream}, process, str::FromStr};
use std::thread;
use std::sync::mpsc::{Sender , channel};
use std::io::{Read,Write};

// Max port we can sniff
const MAX: u16 = 65535;

struct Arguements{
    flag : String,
    ipaddr : IpAddr,
    threads: u16, 
}

// Implementation allows us to create methods to instanciate the struct

impl Arguements{
    fn new(args : &[String]) -> Result<Arguements , &'static str>{
        if args.len() < 2 {
            return Err("Not enough arguements")
        }
        else if args.len() > 4 {
            return Err("Too mant arguements")
        }

        let f = args[1].clone();
        if let Ok(ipaddr) = IpAddr::from_str(&f){
            return Ok(Arguements{
                flag : String::from(""),
                ipaddr,
                threads : 4,    
            });
        }else {
            let flag = args[1].clone();
            if flag.contains("-h") || flag.contains("-help") && args.len() == 2 {
                println!("Usage : -j to select how many threads you want
                \r\n      -h or -help to show this help message");
                return Err("help");
            }else if flag.contains("-h") || flag.contains("-help"){
                return Err("Too many arguements");
            }else if flag.contains("-j") {
                let ipaddr = match IpAddr::from_str(&args[3]){
                    Ok(s) => s,
                    Err(_) => return Err("Invalid IP address; must be IPv4 or IPv6")
                };
                let threads = match args[2].parse::<u16>(){
                    Ok(s) => s,
                    Err(_) => return Err("Failed to parse number of threads")
                };
                return Ok(Arguements{threads, flag , ipaddr});
                } else {
                    return Err("Invalid syntax");
                }                
            }
        }
}  

// The scan function
fn scan(tx: Sender<u16> , start_port: u16 , addr: IpAddr , num_threads: u16){
    let mut port = start_port + 1;
    loop{
        match TcpStream::connect((addr , port)){
            Ok(_) => {
                print!(".");
                io::stdout().flush().unwrap();
                tx.send(port).unwrap();
            }
            Err(_) => {}

        }

        if (MAX - port)  <= num_threads {
            break;
        }
        port += num_threads;

    }
}
fn main() {
    let args : Vec<String> = env::args().collect();

    let program = args[0].clone();

    let arguements = Arguements::new(&args).unwrap_or_else(
        |err| {
            if err.contains("help"){
                process::exit(0);
            }else {
                eprintln!("{} Problem parsing arguements: {}", program ,  err);
                process::exit(0);
            }
        }
    );

    // binding the number of threads to a variable
    let num_threads = arguements.threads;

    //ip address allocation
    let addr = arguements.ipaddr;

    //now will make a channel
    // tx is the transmitter and rx is the receiver

    let (tx , rx) = channel();
    for i in 0..num_threads{
        // we will clone the transmitter so that every thread has its own transmitter
        let tx = tx.clone();

        thread::spawn(move || {
            scan(tx , i, addr , num_threads);
        });
    }

    let mut out = vec![];
    drop(tx);
    for p in rx{
        out.push(p);
    }

    println!("");
    out.sort();
    for v in out{
        println!("{} is open" , v);
    }
}
