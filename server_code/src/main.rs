
// starts up either a client or server instance.
// client given IP address: 127.0.0.1:1111
// server given IP address: 127.0.0.1:7878, which is hardcoded into client. 

use core::panic;
/*
mbedtls config.rs options:

MBEDTLS_KEY_EXCHANGE_ECDHE_ECDSA_ENABLED -> defined
MBEDTLS_ECP_DP_SECP256R1_ENABLED -> defined
MBEDTLS_CIPHER_MODE_CBC -> defined
MBEDTLS_AES_C -> defined
MBEDTLS_CCM_C -> defined
*/
use std::{
    time::{SystemTime, UNIX_EPOCH},
    env,
    fs,
    io::{ prelude::*, Write, BufRead, BufReader},
    net::{TcpListener, TcpStream},
};


extern crate mbedtls;

use mbedtls::rng::{CtrDrbg, OsEntropy};
use mbedtls::{Error};
use mbedtls::alloc::{List, Box};
use mbedtls::pk::Pk; 
use mbedtls::ssl::config::{Endpoint, Preset, Transport};
use mbedtls::ssl::{Config, Context};
use mbedtls::x509::Certificate;
use mbedtls::Result as TlsResult;

use std::sync::Arc;

const RSA_KEY_SIZE: u32 = 3072;
const RSA_KEY_EXP: u32 = 0x10001;
const DAYS_TO_SES: u64 = 86400;
const CERT_VAL_SECS: u64 = (365 * DAYS_TO_SES);

/// The below generates a (Private? || Public?) key and a self signed certificate
/// to configure the TLS context.
// let mut key = Pk::private_from_ec_components(EcGroup::new(SecP256R1).unwrap(), Mpi::new(198248952));
// let mut key = Pk::generate_rsa(&mut rng, RSA_KEY_SIZE, RSA_KEY_EXP).unwrap();
// let mut key_i = Pk::generate_rsa(&mut rng, RSA_KEY_SIZE, RSA_KEY_EXP).unwrap();

// let (not_before, not_after) = get_validity();

// task now: To get server_cert.pem data as [u8]
fn get_key_and_cert(private_key_path: &str, cert_path: &str) -> Result<(Pk, List<Certificate>), i32> {

    // let rng = Arc::new(CtrDrbg::new(Arc::new(entropy), None)?);
    // let cert = Arc::new(Certificate::from_pem_multiple(keys::PEM_CERT.as_bytes())?);
    // let key = Arc::new(Pk::from_private_key(keys::PEM_KEY.as_bytes(), None)?);
    // let mut config = Config::new(Endpoint::Server, Transport::Stream, Preset::Default);
    // config.set_rng(rng);
    // config.push_cert(cert, key)?;

    // let rc_config = Arc::new(config);


    let cert_content = fs::read(cert_path)
        .expect("Could not read server certificate .pem file\n");
    let cert_content_bytes: &[u8] = &cert_content;
    
    let private_key_contents = fs::read(private_key_path)
        .expect("Could not read private key .pem file\n");
    let private_key_bytes: &[u8] = &private_key_contents;

    
    if let Ok(key) = Pk::from_private_key(private_key_bytes, None){
        if let Ok(server_cert) = Certificate::from_pem_multiple(cert_content_bytes){

            return Ok((key, server_cert));
        } else {
            println!("Could not generate certificate from {}", cert_path);
        }
    } else {
        println!("Could not generate private key from {}", private_key_path);
    }
    Err(-1)
}



fn run_client() -> i32{
    return connect_to_server();
}

fn connect_to_server() -> i32{

    match TcpStream::connect("127.0.0.1:7878") {
        Ok(mut stream) => {
            
            println!("Client: Connection to server established.");

            let get_msg_str= "GET /dcap HTTP/1.1\r\n";

            stream.write(get_msg_str.as_bytes()).unwrap();

            println!("Client: Get message sent successfully.");

            let buf_reader = BufReader::new(&mut stream);

            match buf_reader.lines().next().unwrap() {
                Ok(server_response) => {
                    println!("Client: Received server response.");
                    match fs::write("client_output.txt", server_response.as_bytes()){
                        Ok(_) => {
                            println!("Client: Server response successfully saved to client_output.txt.");
                            return 0;
                        },
                        Err(err_code) => {
                            println!("Client: Could not save server response to file -> {}", err_code);
                            return -1;

                        },
                    };
                },
                Err(err_code) => {
                    println!("Client: Server response error -> {}", err_code);
                    return -1;
                },
            }
        },
        Err(err_code) => {
            println!("Client: Could not connect to server -> {}", err_code);
            return -1;
        }
    }
}


fn run_server() -> i32{
    
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("server running on: 127.0.0.1:7878");
    
    for stream in listener.incoming(){
        let stream = stream.unwrap();
//         let _ = serve(stream, &mut key, &mut cert).unwrap();
        if let Ok((key, cert)) = get_key_and_cert(&"../server_private_key.pem", &"../server_cert.pem"){
            server_handle_connection(stream, key, cert);
        } else {
            return -1;
        }

    }

    return 0;
}



fn server_handle_connection(mut stream: TcpStream, key: Pk, cert: List<Certificate>){
    let random_num: [u8; 100];

    let rng = Arc::new(CtrDrbg::new(Arc::new(OsEntropy::new()), None)?);

    let mut config = Config::new(Endpoint::Server, Transport::Stream, Preset::Default);
    config.set_rng(rng);
    config.push_cert(Arc::new(cert), Arc::new(key));
    let mut ctx = Context::new(Arc::new(config));

    let mut buf = String::new();
    let session = ctx.establish(&mut stream, None)?;

    println!("Connection established!");
    let buf_reader = BufReader::new(session);
    let http_request_line = buf_reader
        .lines()
        .next()
        .unwrap()
        .unwrap();
        // first unwrap is for Option because lines() might return None
        // second unwrap is to return the actual str that gets assigned.
        // unwrap() in the above line for commit 8610b4aef42b51dbe13231b84d2a6dba5fb94bb2 
        // means that we are not handling the None that .lines() could return.


    println!("server received request: {:#?}", http_request_line);
    let (status_line, http_file) = match http_request_line.as_str() {
        "GET /dcap HTTP/1.1" => ("HTTP/1.1 200 OK", "devCapMsg.html"),
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let content = fs::read_to_string(http_file).unwrap(); // this might return Err if file not found.
    // for the server, could use the error here as a way to check if the file/resource exists
    // but it still feels better to have a whitelist of the services offered by the server, compare 
    // the request against that,
    // and then have a function encapsulate the getting of the resource from the file.
    let content_length = content.len();
    let response = format!("{status_line}\r\nContent-Type: application/sep+xml\r\nContent-Length: {content_length}\r\n\r\n{content}");
    stream.write_all(response.as_bytes()).unwrap(); // error handling not done here either. write_all might return Err
}


fn main(){
    let args: Vec<String> = env::args().collect();
    dbg!(args.len());
    if !args.len() == 2 {
        println!("Invalid arguments provided. Usage: -- \"client\"||\"server\"");
        return;
    };
    match args[1].as_str() {
        "client" => run_client(),
        "server" => run_server(),
        _ => {
            println!("Invalid arguments provided. Usage: -- \"client\"||\"server\"");
            return;
        },
    };   
}


// Establish a TLS connection with a randomly generated key and
// a self signed certificate.
// After a session is established, echo the incoming stream to the client.
// till EOF is detected.
// 
// fn serve(mut conn: TcpStream, key: &mut Pk, cert: &mut Certificate) -> TlsResult<()> {
//     let mut rng = Rdrand;

//     let mut config = Config::new(Endpoint::Server, Transport::Stream, Preset::Default);
//     config.set_rng(Some(&mut rng));
//     config.push_cert(&mut **cert, key)?;
//     let mut ctx = Context::new(&config)?;

//     let mut buf = String::new();
//     let session = ctx.establish(&mut conn, None)?;
//     println!("Connection established!");
//     let mut reader = BufReader::new(session);
//     while let Ok(1..=std::usize::MAX) = reader.read_line(&mut buf) {
//         let session = reader.get_mut();
//         session.write_all(&buf.as_bytes()).unwrap();
//         buf.clear();
//     }
//     Ok(())
// }
