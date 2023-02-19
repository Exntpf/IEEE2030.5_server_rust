/**
 * IEEE 2030.5 client
 * 
 * A real client (such as the one made by EPRI) would be Event based
 * and would only send requests to the server in response to self-generated
 * or server-generated events.
 * This one simply sends a request to the server and prints it's response
 * standard out for testing purposes. A series of requests can be sent 
 * if required.
 * 
 * Obvious optimisations include taking additional arguments such as the
 * servers IP address, the ciphers supported, the path and method of requests
 * to send, and other testing variables.
 * 
 * Other improvements include encapsulating mbedtls operations into the 
 * tls.rs file for better coupling.
 */

 use std::{
    fs,
    // fs::File,
    io::{ Write, BufRead, BufReader},
};

extern crate mbedtls;
extern crate libc;

use crate::tcp::*;
use crate::errs::*;

use mbedtls::{rng::{CtrDrbg, OsEntropy}};
use mbedtls::pk::Pk;
use mbedtls::ssl::config::{Endpoint, Preset, Transport};
use mbedtls::ssl::{Config, Context, ciphersuites::{CipherSuite}};
use mbedtls::x509::{Certificate};

use std::sync::Arc;


pub fn run_client() -> i32{
    connect_to_server()
}

/**Connects to the server using the base IEEE 2030.5 cipher suite and 
 * the client and SERCA certificates in the certs folder.
 * 
 * This could be refactored using the reqwest or http crates to make
 *  this part simpler and clearer.
 */
fn connect_to_server() -> i32{ // to make this return Result in the future so we can recover from this error

    println!("Client: Establishing TCP connection with server IP: 127.0.0.1:7877");
    // not doing any error checking here, unlike earlier implementation.
    let stream = connect("127.0.0.1:7877").unwrap();
    // if let Err(e) = stream {
    //     errs::print_err(true, e, "couldn't connect to server address");
    // }

    // let stream = TcpStream::connect("127.0.0.1:7877").unwrap();
    println!("Client: TCP connection established");
    let entropy = OsEntropy::new();
    let rng = Arc::new(CtrDrbg::new(Arc::new(entropy), None).unwrap());

    let mut config = Config::new(Endpoint::Server, Transport::Stream, Preset::Default);
    config.set_rng(rng);
    let ciphersuite_list: Vec<i32> = vec![
    CipherSuite::EcdheEcdsaWithAes128Ccm8.into(),
    // CipherSuite::EcdheEcdsaWithAes256CbcSha384.into(), CipherSuite::EcdheEcdsaWithAes128CbcSha256.into(),
    // CipherSuite::EcdheEcdsaWithAes256GcmSha384.into(), CipherSuite::EcdheEcdsaWithAes128GcmSha256.into(), 
        0];
    config.set_ciphersuites(Arc::new(ciphersuite_list));


    // 2030.5 requires at minimum: TLS_ECDHE_ECDSA_WITH_AES_128_CCM_8 = EcdheEcdsaWithAes128Ccm8 = 0xC0AE
    // Firefox does not consider this secure communication, so cannot establish a connection with this server.
    // let ciphersuite_list: Vec<i32> = vec![CipherSuite::EcdheEcdsaWithAes128Ccm8.into(), CipherSuite::EcdheEcdsaWithAes256GcmSha384.into(), CipherSuite::EcdheEcdsaWithAes128GcmSha256.into(), 
    // CipherSuite::EcdheEcdsaWithAes256CbcSha384.into(), CipherSuite::EcdheEcdsaWithAes128CbcSha256.into(), 0];
    // config.set_ciphersuites(Arc::new(ciphersuite_list));

    /* 
     as 2030.5 client cannot connect to a server with a self-signed
     certificate, the client must set who the CA is. this can be done with:
     rust-mbedtls/mbedtls/tests/client_server.rs:79
     */
    
    println!("Client: Setting CA");
    let mut cert_content = fs::read("../../IEEE2030.5_server_rust/certs/serca_cert.pem")
        .expect("Client: ERR: Could not read SERCA certificate file\n");
    cert_content.append(&mut vec![0u8]); // becuase certificates must be \0 terminated
    let cert_content_bytes: &[u8] = &cert_content;
    let ca_cert = Arc::new(Certificate::from_pem_multiple(cert_content_bytes)
        .expect("Client: ERR: SERCA cert file corrupted"));
    config.set_ca_list(ca_cert, None);
    
    // read .pem files
    println!("Client: loading client certificate and private key");
    let mut cert_content = fs::read("../../IEEE2030.5_server_rust/certs/client_cert.pem")
        .expect("Client: ERR: Could not read client certificate .pem file\n");
    cert_content.append(&mut vec![0u8]); // becuase certificates must be \0 terminated
    let cert_content_bytes: &[u8] = &cert_content;
    
    let mut private_key_contents = fs::read("../../IEEE2030.5_server_rust/certs/client_private_key.pem")
        .expect("Client: ERR: Could not read client private key .pem file\n");
    private_key_contents.append(&mut vec![0u8]);
    let private_key_bytes: &[u8] = &private_key_contents;

    // client is reading in the certificates.
    
    let server_cert = Arc::new(Certificate::from_pem_multiple(cert_content_bytes)
        .expect("Client: ERR: Client certificate file corrupted"));
    let key = Arc::new(Pk::from_private_key(private_key_bytes, None)
        .expect("Client: ERR: Client private key file corrupted"));
    if let Err(a) = config.push_cert(server_cert, key){
        print_err(true, a, "Could not load certificate to mbedtls config");
        return -1;
    }
    
   
    println!("Client: making new ctx context");
    let mut ctx = Context::new(Arc::new(config));
    // so far so good - above code aligns with client_server.rs test on mbedtls GitHub repo.
    println!("Client: ctx setup complete. Attempting to establish connection TLS with server");
    match ctx.establish(stream, None) {
        Ok(()) => {
            println!("Client: Conneciton Established!");
        },
        Err(a) => {
            print_err(true, a, "Error establishing connection");
            return -1;
        },
    };


    let get_msg_str= "GET /dcap HTTP/1.1\r\n";
    match ctx.write_all(get_msg_str.as_bytes()){
        Ok(_) => println!("Client: GET message sent successfully."),
        Err(a) => print_err(true, a, "Could not send GET request to server"),
    }

    let mut buf = vec![0u8;200];
    let mut client_buf_reader = BufReader::new(ctx);
    let bytes_read = match client_buf_reader.read_until(0u8, &mut buf){
        Ok(a) => a,
        Err(a) =>{ print_err(true, a, "Could not read server data into buffer"); 0} ,
    };

    

    // if let Err(a) = ctx.read_exact(&mut buf){
    //     println!("client: ERR: {a}\ncould not read server data into buffer");
    // }
    // assert_eq!(&buf, format!("Cipher suite: c0ae").as_bytes());

    println!("Client: Bytes received: {bytes_read}\nReceived server response:\n{}", String::from_utf8(buf).unwrap());
    return 0;
}