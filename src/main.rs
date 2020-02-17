
use std::fs::File;
use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;

fn main() {
    let addr: &str = "127.0.0.1:8080";
    let listener = TcpListener::bind(addr).unwrap();
    
    println!("Server Runing on {}\n", addr);

    for stream in listener.incoming() {
        let _stream = stream.unwrap();
        handle_connection(_stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    /* stream needs to
be mut because its internal state might change */


    let mut buffer = [0; 512];

    // read 512 bite from the stream
    stream.read(&mut buffer).unwrap();
    /*
     * from_utf8_lossy => convert from &[u8] to string.
     * (lossy) mean replace the invalid UTF-8 sequence with ?
     */
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));


    // -----------     Response      -----------
    

    let get = b"GET / HTTP/1.1\r\n";

    // Validating the Request and Selectively Responding
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "statics/index.html")
    } else { 
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "statics/404.html")
    };

    let mut file = File::open(filename).unwrap();
    let mut body = String::new();

    file.read_to_string(&mut body).unwrap();
  
    /* 
    * Response Pattern => [
    *        HTTP-Version Status-Code Reason-Phrase CRLF
    *        headers CRLF
    *        message-body
    *    ]
    */
    let response = format!("{}{}", status_line, body);
  
    stream.write(response.as_bytes()).unwrap();
  
    //flush will wait and prevent the program from continuing until allthe bytes are written to the connection
    stream.flush().unwrap();

}