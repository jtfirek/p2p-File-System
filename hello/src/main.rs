use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream}
};

fn main() {
    // listener to TCP connects on the IP:port(127.0.0.1:787)
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established");
    }
}