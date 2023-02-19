use std::fs;
use std::io::Read;
use std::io::Write;
use std::net::TcpStream;

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
    
    let http_request_line = full_request
                                    .lines()
                                    .next()
                                    .unwrap();
    let (status_line, http_file) = match http_request_line {
        "GET /dcap HTTP/1.1" => {
            ("HTTP/1.1 200 OK", "../hello.html")
        },
        _ => ("HTTP/1.1 404 NOT FOUND", "../404.html"),
    };
    println!("returned status line -> {}", status_line);
    
    let content = fs::read_to_string(http_file).unwrap(); // this might return Err if file not found.

    // for the server, could use the error here as a way to check if the file/resource exists
    // but it still feels better to have a whitelist of the services offered by the server, compare 
    // the request against that,
    // and then have a function encapsulate the getting of the resource from the file.
    let content_length = content.len();
    let response = format!("{status_line}\r\nContent-Type: application/sep+xml\r\nContent-Length: {content_length}\r\n\r\n{content}");
    // let response = format!("{content}");
    
    match ctx.write_all(response.as_bytes()){
        Ok(_) => println!("server: response sent to client."),
        Err(a) => print_err(false, a, "Response not sent to client"),
    }
    println!("full server response: {}",  response);
    

}

