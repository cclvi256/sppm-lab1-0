use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

fn main() {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                println!("accepted new connection");
                std::thread::spawn(|| handle_connection(_stream));
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    println!("Request: {:#?}", http_request);

    let start_line = http_request.first().unwrap();
    let path = start_line.split_whitespace().nth(1).unwrap();

    let response: String;

    response = match path {
        "/" => "HTTP/1.1 200 OK\r\n\r\n".to_string(),
        "/user-agent" => {
            let user_agent = http_request
                .iter()
                .find(|line| line.starts_with("User-Agent"))
                .unwrap();

            format!(
                "\
                HTTP/1.1 200 OK\r\n\
                Content-Type: text/plain\r\n\
                Content-Length: {}\r\n\r\n\
                {}",
                user_agent.len() - 12,
                &user_agent[12..]
            )
        }
        _ if path.starts_with("/echo/") => {
            format!(
                "\
                HTTP/1.1 200 OK\r\n\
                Content-Type: text/plain\r\n\
                Content-Length: {}\r\n\r\n\
                {}",
                path.len() - 6,
                &path[6..]
            )
        }
        _ => "HTTP/1.1 404 Not Found\r\n\r\n".to_string(),
    };

    stream.write_all(response.as_bytes()).unwrap();
}
