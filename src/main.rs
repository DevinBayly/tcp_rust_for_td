use rand::prelude::*;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::{Duration, SystemTime};

fn main() {
    let listener = TcpListener::bind("10.42.0.1:8000").expect("no bind");
    // this is when a connection happens, but perhaps push into a thread and continue to send it data instead of just closing
    for stream in listener.incoming() {
        println!("got connection ");
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}
fn handle_connection(mut s: TcpStream) {
    // empty array
    loop {
        let mut data = [0u8; 512*512];
        println!("{}",data.len());
        let mut thread = rand::thread_rng();
        thread.fill_bytes(&mut data);
        // use gen from thread to make data
        s.write(&data).unwrap();
        s.flush().unwrap();
        thread::sleep(Duration::from_secs(1));
    }
}
