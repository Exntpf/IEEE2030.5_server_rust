use std::fs;
use std::str;
use std::io::Read;
use std::io::Write;
use std::net::TcpStream;

use crate::backend;
use crate::tcp::*;
use crate::tls::*;
use crate::errs::*;

/**
 * IEEE 2030.5 server code. Multiple instances of this can be run from
 * the calling file by executing the `run_server` function.
 * 
 * The server is currently quite brittle, and is only equipped to handle
 * a working faultless client. Future improvements should make it more
 * robust to unexpected connection closures, failed TLS handshakes or TCP
 * connection establishment, and other network errors.
 */

 pub fn run_server() -> i32{
    let address = "127.0.0.1:7877";
    let listener = match listen(address){
        Ok(a) => a,
        Err(e) =>{
            print_err(false, e, "could not listen on given address");
            return -1;
        },
    };

    // let listener = TcpListener::bind("127.0.0.1:7877").unwrap();
    println!("server running on: 127.0.0.1:7877");
    
    for stream in listener.incoming(){
        let stream = stream.unwrap();
//  }       let _ = serve(stream, &mut key, &mut cert).unwrap();
        println!("Server: Established TCP connection with client");
        server_handle_connection(stream);

        // if let Ok((key, cert)) = get_key_and_cert(&"../server_private_key.pem", &"../server_cert.pem"){
        //     server_handle_connection(stream, key, cert);
        // } else {
        //     return -1;
        // }

    }

    return 0;
}

// fn server_handle_connection(mut stream: TcpStream, key: Pk, cert: List<Certificate>){
fn server_handle_connection(stream: TcpStream){

    let cert_path = "../../IEEE2030.5_server_rust/certs/server_cert.pem";
    let private_key_path = "../../IEEE2030.5_server_rust/certs/server_private_key.pem";

    let mut ctx = establish_tls_server(stream, private_key_path, cert_path);

    let mut server_buf = [0u8; 100];
    let bytes_read = match ctx.read(&mut server_buf){
        Ok(a) => a,
        Err(a) => {
            print_err(false, a, "Could not read client request");
            0
        },
    };

    println!("server: read in {bytes_read} bytes");

    let full_request = String::from_utf8_lossy(&server_buf);
    println!("Server recieved request ->\n{}", full_request);

    let http_header_line = full_request
                                .lines()
                                .next()
                                .unwrap();
    let mut http_header_words = http_header_line.split_whitespace();
    let method = http_header_words
                .next()
                .expect("Client request was malformed");
    let path = http_header_words
                .next()
                .expect("Client request was malformed");
    println!("path: {path}, method: {method}");
    
    let (status_code, content) = match method{
        "GET" => {
            println!("server: input get_request received.");
            backend::service_response(path, method, None)
        },
        "PUT" =>{
            let mut http_request_lines = full_request.lines();
            http_request_lines.position(|l| l.is_empty());
            let mut body = String::new();
            for line in http_request_lines{
                body.push_str(line);
                body.push('\n');
            }
            backend::service_response(path, method, Some(&body))
        },
        "POST" =>{
            println!("input post_request received.");
            let mut http_request_lines = full_request.lines();
            http_request_lines.position(|l| l.is_empty());
            let mut body = String::new();
            for line in http_request_lines{
                body.push_str(line);
                body.push('\n');
            }
            backend::service_response(path, method, Some(&body))
        },
        "DELETE" =>{
            // following RFC 9110 (June 2022), a client should not put
            // anything in the body of DELETE request, so we are going 
            // enforce this by ignoring the body.
            backend::service_response(path, method, None)
        },
        _ => { panic!("Client method had no chill (was not RESTful)") },
    };

    let content = str::from_utf8(&content);
    let mut response: String;
    if let Err(_) = content{
        response = "HTTP/1.1 500 Internal Server Error".to_owned();
    }
    
    // easy refactor here is turning status_code into an enum of valid
    // status codes.
    let response = match status_code{
        200 => {
            let status_line = "HTTP/1.1 200 OK";
            let content = content.unwrap();
            let content_len = content.len();
            format!("{status_line}\r\nContent-Type: application/sep+xml\r\nContent-Length: {content_len}\r\n\r\n{content}")
        },
        201 => {
            let status_line = "HTTP/1.1 201 Created";
            let content = content.unwrap();
            format!("{status_line}\r\n{content}")
        },
        400 => {
            let status_line = "HTTP/1.1 400 Bad Request";
            format!("{status_line}")
        },
        401 => {
            let status_line = "HTTP/1.1 401 Unauthorized";
            format!("{status_line}")
        },
        404 => {
            let status_line = "HTTP/1.1 404 Not Found";
            format!("{status_line}")
        },
        405 => {
            let status_line = "HTTP/1.1 405 Method Not Allowed";
            format!("{status_line}")
        },
        500 => {
            let status_line = "HTTP/1.1 500 Internal Server Error";
            format!("{status_line}")
        }
        _ => {
            let status_line = "HTTP/1.1 500 Internal Server Error";
            format!("{status_line}")
        }
    };

    match ctx.write_all(response.as_bytes()){
        Ok(_) => println!("server: response sent to client."),
        Err(a) => print_err(false, a, "Response not sent to client"),
    }
    println!("full server response: {}",  response);
    

}

