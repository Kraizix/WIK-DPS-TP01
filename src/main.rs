use std::{
    env,
    thread,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};
use dotenv::dotenv;
fn main() {
    dotenv().ok();
    let host;
    match env::var("PING_LISTEN_PORT"){
        Ok(val) => host = format!("0.0.0.0:{val}"),
        Err(_e) => host = "0.0.0.0:7878".to_string(),
    };
    let listener = TcpListener::bind(host).unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        thread::spawn(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request: Vec<_> = buf_reader.lines().map(|result | result.unwrap()).take_while(|line| !line.is_empty()).collect();
    let request_line = &request[0];
    if request_line == "GET /ping HTTP/1.0" {
        println!("Hostname: {:?}", hostname::get());
        let status_line = "HTTP/1.1 200 OK";
        let mut res: Vec<String> = Vec::new();
        for data in request[1..].into_iter() {
            let (s1, s2) = data.split_once(": ").unwrap();
            res.push(format!("\"{}\": \"{}\"",s1,s2));
        }
        let response = format!("{status_line}\r\nContent-Type: application/json\r\n\r\n{{{}}}",res.join(","));
        stream.write_all(response.as_bytes()).unwrap();
    } else {
        let status_line = format!("HTTP/1.1 404 NOT FOUND\r\n\r\n");
        stream.write_all(status_line.as_bytes()).unwrap();
    }
}
