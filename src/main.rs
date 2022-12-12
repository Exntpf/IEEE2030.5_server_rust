
// starts up either a client or server instance.
// client given IP address: 127.0.0.1:1111
// server given IP address: 127.0.0.1:7877, which is hardcoded into client. 


/*
mbedtls config.rs options:

MBEDTLS_KEY_EXCHANGE_ECDHE_ECDSA_ENABLED -> defined
MBEDTLS_ECP_DP_SECP256R1_ENABLED -> defined
MBEDTLS_CIPHER_MODE_CBC -> defined
MBEDTLS_AES_C -> defined
MBEDTLS_CCM_C -> defined
*/
#![allow(unused_imports)]
use std::{
    env,
    fs,
    // fs::File,
    io::{ Write, BufRead, BufReader, Read, ErrorKind},
    net::{TcpListener, TcpStream},
};


extern crate mbedtls;
extern crate libc;

use tls_server_client::{tcp::*, tls::*};
use tls_server_client::errs::*;

use mbedtls::{rng::{CtrDrbg, OsEntropy}};
use mbedtls::pk::Pk;
use mbedtls::ssl::config::{Endpoint, Preset, Transport};
use mbedtls::ssl::{Config, Context, ciphersuites::{CipherSuite}};
use mbedtls::x509::{Certificate};

use std::sync::Arc;

fn run_client() -> i32{
    connect_to_server()
}

fn connect_to_server() -> i32{ // to make this return Result in the future so we can recover from this error


    // not doing any error checking here, unlike earlier implementation.
    let stream = connect("127.0.0.1:7877").unwrap();
    // if let Err(e) = stream {
    //     errs::print_err(true, e, "couldn't connect to server address");
    // }

    // let stream = TcpStream::connect("127.0.0.1:7877").unwrap();

    let entropy = OsEntropy::new();
    let rng = Arc::new(CtrDrbg::new(Arc::new(entropy), None).unwrap());

    let mut config = Config::new(Endpoint::Server, Transport::Stream, Preset::Default);
    config.set_rng(rng);
    let ciphersuite_list: Vec<i32> = vec![
    CipherSuite::EcdheEcdsaWithAes128Ccm8.into(),
    CipherSuite::EcdheEcdsaWithAes256CbcSha384.into(), CipherSuite::EcdheEcdsaWithAes128CbcSha256.into(),
    CipherSuite::EcdheEcdsaWithAes256GcmSha384.into(), CipherSuite::EcdheEcdsaWithAes128GcmSha256.into(), 
        0];
    config.set_ciphersuites(Arc::new(ciphersuite_list));


    // 2030.5 requires at minimum: TLS_ECDHE_ECDSA_WITH_AES_128_CCM_8 = EcdheEcdsaWithAes128Ccm8 = 0xC0AE
    // Firefox does not consider this secure communication, so cannot establish a connection with this server.
    // let ciphersuite_list: Vec<i32> = vec![CipherSuite::EcdheEcdsaWithAes128Ccm8.into(), CipherSuite::EcdheEcdsaWithAes256GcmSha384.into(), CipherSuite::EcdheEcdsaWithAes128GcmSha256.into(), 
    // CipherSuite::EcdheEcdsaWithAes256CbcSha384.into(), CipherSuite::EcdheEcdsaWithAes128CbcSha256.into(), 0];
    // config.set_ciphersuites(Arc::new(ciphersuite_list));

    
    println!("Client: Setting CA");
    let mut cert_content = fs::read("/home/neel/Desktop/unswCasualProfessional_Files/server_casProf/tls_server_client/certs/serca_cert.pem")
    .expect("Could not read server certificate .pem file\n");
    cert_content.append(&mut vec![0u8]); // becuase certificates must be \0 terminated
    let cert_content_bytes: &[u8] = &cert_content;
    let ca_cert = Arc::new(Certificate::from_pem_multiple(cert_content_bytes).unwrap());
    config.set_ca_list(ca_cert, None);
    
    // read .pem files
    println!("Client: loading certificate and private key");
    let mut cert_content = fs::read("/home/neel/Desktop/unswCasualProfessional_Files/server_casProf/tls_server_client/certs/client_cert.pem")
    .expect("Could not read server certificate .pem file\n");
    cert_content.append(&mut vec![0u8]); // becuase certificates must be \0 terminated
    let cert_content_bytes: &[u8] = &cert_content;
    
    let mut private_key_contents = fs::read("/home/neel/Desktop/unswCasualProfessional_Files/server_casProf/tls_server_client/certs/client_private_key.pem")
    .expect("Could not read private key .pem file\n");
    private_key_contents.append(&mut vec![0u8]);
    let private_key_bytes: &[u8] = &private_key_contents;
    // client is reading in the certificates.
    
    // generate certificate, private key, and push to Config.
    // could do some error checking here.
    let server_cert = Arc::new(Certificate::from_pem_multiple(cert_content_bytes).unwrap());
    let key = Arc::new(Pk::from_private_key(private_key_bytes, None).unwrap());
    if let Err(a) = config.push_cert(server_cert, key){
        println!("client: ERR: {a}\nCould not load certificate to mbedtls config");
    }
    
    /* 
     as 2030.5 client cannot connect to a server with a self-signed
     certificate, the client must set who the CA is. this can be done with:
     rust-mbedtls/mbedtls/tests/client_server.rs:79
     Code to be inserted here.
     */
    println!("Client: making new context");
    let mut ctx = Context::new(Arc::new(config));
    // so far so good - above code aligns with client_server.rs test on mbedtls GitHub repo.
    println!("Client: ctx setup complete. Attempting to establish connection TLS with server");
    match ctx.establish(stream, None) {
        Ok(()) => {
            println!("Conneciton Established!");
        },
        Err(a) => {
            print_err(true, a, "error establishing connection");
            return -1;
        },
    };

    let get_msg_str= "GET /dcap HTTP/1.1\r\n";
    match ctx.write_all(get_msg_str.as_bytes()){
        Ok(_) => println!("Client: GET message sent successfully."),
        Err(a) => println!("Client: ERR: {a}\ncould not send GET request to server"),
    }

    let mut buf = vec![0u8;200];
    let mut client_buf_reader = BufReader::new(ctx);
    let bytes_read = match client_buf_reader.read_until(0u8, &mut buf){
        Ok(a) => a,
        Err(a) =>{ println!("client: Err: {a}\ncould not read server data into buffer"); 0} ,
    };

    

    // if let Err(a) = ctx.read_exact(&mut buf){
    //     println!("client: ERR: {a}\ncould not read server data into buffer");
    // }
    // assert_eq!(&buf, format!("Cipher suite: c0ae").as_bytes());

    println!("Client: Bytes receivec: {bytes_read}\nReceived server response:\n{}", String::from_utf8(buf).unwrap());
    return 0;
}

fn run_server() -> i32{
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

    let cert_path = "/home/neel/Desktop/unswCasualProfessional_Files/server_casProf/tls_server_client/certs/server1_cert.pem";
    let private_key_path = "/home/neel/Desktop/unswCasualProfessional_Files/server_casProf/tls_server_client/certs/server1_private_key.pem";

    let mut ctx = establish_tls_server(stream, private_key_path, cert_path);

    let mut server_buf = [0u8; 100];
    let bytes_read = match ctx.read(&mut server_buf){
        Ok(a) => a,
        Err(a) => {
            println!("server: ERR: {a}\ncould not read client request");
            0
        },
    };

    println!("server: read in {bytes_read} bytes");

    let full_request = String::from_utf8_lossy(&server_buf);
    println!("Server recieved request ->\n{}", full_request);

    let http_request_line = full_request.lines().next().unwrap();
    let (status_line, http_file) = match http_request_line {
        "GET /dcap HTTP/1.1" => {
            ("HTTP/1.1 200 OK", "devCapMsg.html")
        },
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
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
        Err(a) => println!("server: ERR: {a}\nresponse not sent to client"),
    }
    println!("full server response: {}",  response);
    

}


fn main(){
    let args: Vec<String> = env::args().collect();
    dbg!(args.len());
    if !args.len() == 2 {
        println!("Invalid arguments provided. Usage: -- \"client\"||\"server\"");
        return;
    };
    match args[1].as_str() {
        "client" => {
            if run_client() < 0{
                println!("client generated an error");
            }
            return
        },
        "server" => run_server(),
        _ => {
            println!("Invalid arguments provided. Usage: -- \"client\"||\"server\"");
            return;
        },
    };   
}
