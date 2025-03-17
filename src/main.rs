use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::str;
use std::time::Instant;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3030").expect("No se pudo iniciar el servidor");

    println!("Servidor HTTP corriendo en http://127.0.0.1:3030");

    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            handle_connection(stream);
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    if let Ok(size) = stream.read(&mut buffer) {
        let request = str::from_utf8(&buffer[..size]).unwrap_or("");

        if let Some(i) = parse_request(request) {
            let start_time = Instant::now();
            let pi = calculate_pi(i);
            let duration = start_time.elapsed().as_secs_f64();

            let response_body = format!(
                "Valor de Pi para el término {}: {} (Tiempo: {:.6} segundos)",
                i, pi, duration
            );

            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
                response_body.len(),
                response_body
            );
            stream.write_all(response.as_bytes()).unwrap();
        } else {
            let response = "HTTP/1.1 400 Bad Request\r\nContent-Length: 0\r\n\r\n";
            stream.write_all(response.as_bytes()).unwrap();
        }
    }
}

fn parse_request(request: &str) -> Option<u64> {
    let request_line = request.lines().next()?;

    if request_line.starts_with("GET /pi/") {
        let parts: Vec<&str> = request_line.split_whitespace().collect();
        if let Some(path) = parts.get(1) {
            if let Some(i_str) = path.strip_prefix("/pi/") {
                return i_str.parse().ok();
            }
        }
    }

    None
}

fn calculate_pi(n: u64) -> String {
    let mut sum = 0.0;

    for k in 0..n {
        let term = 1.0 / (2.0 * k as f64 + 1.0);
        sum += if k % 2 == 0 { term } else { -term };
    }

    format!("{}", 4.0 * sum)
}