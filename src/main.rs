extern crate clap;

use clap::Parser;
use std::net::TcpStream;
use rayon::prelude::*;
use std::num::ParseIntError;

const N_THREADS: usize = 8;
const MIN_PORT: usize = 0;
const MAX_PORT: usize = 65535;

#[derive(Parser, Debug)]
struct Options {
    address: String,

    #[arg(short, long)]
    port_range: Option<String>,

    #[arg(short, long)]
    n_threads: Option<usize>,
}

fn check_port(addr: &String, port: usize) -> bool{
    match TcpStream::connect( format!("{}:{}", addr, port) ) {
        Ok(_) => return true,
        Err(_) => return false
    }
}

fn scan(addr: &String, range: std::ops::Range<usize>) -> Vec<bool>{
    let results: Vec<bool> = range.into_par_iter().map(|p| check_port(&addr, p) ).collect();
    results
}

fn main() -> Result<(), ParseIntError> {
    rayon::ThreadPoolBuilder::new().num_threads(N_THREADS).build_global().unwrap();

    let args = Options::parse();
    let port_range: std::ops::Range<usize>;
    
    let address = args.address;
    
    match args.port_range {
        None => { port_range = MIN_PORT..MAX_PORT;
        },
        Some(_) =>  { let tmp = args.port_range.unwrap();
                      let v: Vec<&str> = tmp.split('-').collect();
                      port_range = v[0].parse::<usize>()?..v[1].parse::<usize>()?;
        },
    }

    println!("Scaning...");
    
    let results = scan(&address, port_range);

    for (idx, val) in results.iter().enumerate() {
        if *val {
            println!("{}:{} open", &address, idx);
        }
    }
    
    Ok(())
}
