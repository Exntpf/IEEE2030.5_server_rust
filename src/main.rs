
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
use std::{
    env,
    fs::{File, self},
    io::{ Write, BufRead, BufReader, Read},
    net::{TcpListener, TcpStream},
};


extern crate mbedtls;

use mbedtls::{rng::{CtrDrbg, OsEntropy}};
use mbedtls::pk::Pk; 
use mbedtls::ssl::config::{Endpoint, Preset, Transport};
use mbedtls::ssl::{Config, Context, ciphersuites::{CipherSuite}, Version};
use mbedtls::x509::{Certificate, VerifyError};

use std::sync::Arc;

const RSA_KEY_SIZE: u32 = 3072;
const RSA_KEY_EXP: u32 = 0x10001;
const DAYS_TO_SES: u64 = 86400;
const CERT_VAL_SECS: u64 = 365 * DAYS_TO_SES;


fn run_client() -> i32{
    return connect_to_server();
}

fn connect_to_server() -> i32{

    // not doing any error checking here, unlike earlier implementation.
    let stream = TcpStream::connect("127.0.0.1:7877").unwrap();

    let entropy = OsEntropy::new();
    let rng = Arc::new(CtrDrbg::new(Arc::new(entropy), None).unwrap());

    let mut config = Config::new(Endpoint::Server, Transport::Stream, Preset::Default);
    config.set_rng(rng);

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
    config.push_cert(server_cert, key);
    
    /* 
     * as 2030.5 client cannot connect to a server with a self-signed
     * certificate, the client must set who the CA is. this can be done with:
     * rust-mbedtls/mbedtls/tests/client_server.rs:79
     * Code to be inserted here.
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
            println!("Error establishing connection. Code: {}", a);
            return -1;
        },
    };

    // let get_msg_str= "GET /dcap HTTP/1.1\r\n";
    // ctx.write_all(get_msg_str.as_bytes());
    

    println!("Client: Get message sent successfully.");

    let mut buf = [0u8; 13 + 1 + 4];
    ctx.read_exact(&mut buf);
    // assert_eq!(&buf, format!("Cipher suite: c0ae").as_bytes());

    println!("Client: Received server response: {:#?}", buf);
    println!("expected suite: c0ae = TLS_ECDHE_ECDSA_WITH_AES_128_CCM_8");
    // let mut output_file = File::create(&"client_output.txt").unwrap();

            // let mut output_file = fs::OpenOptions::new()
            //     .write(true)
            //     .create(true)
            //     .truncate(true)
            //     .append(true)
            //     .open("client_output.txt")
            //     .unwrap();

    // for line in response_lines {
    //     match line {
    //         Ok(output_line) => {
    //             if let Err(err_code) = writeln!(output_file, "{}", &output_line){
    //                     println!("Client: Could not save server response to file -> {}", err_code);
    //                     return -1;
    //             }
    //             continue;
    //         },
    //         Err(err_code) => {
    //             println!("Client: Server response error -> {}", err_code);
    //             return -1;
    //         },
    //     }
    // }
    // println!("Client: Server response successfully saved to client_output.txt.");
    return 0;
}

fn run_server() -> i32{
    
    let listener = TcpListener::bind("127.0.0.1:7877").unwrap();
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


// fn get_key_and_cert(private_key_path: &str, cert_path: &str) -> Result<(Pk, List<Certificate>), i32> {

//     let cert_content = fs::read(cert_path)
//         .expect("Could not read server certificate .pem file\n");
//     let cert_content_bytes: &[u8] = &cert_content;
    
//     let private_key_contents = fs::read(private_key_path)
//         .expect("Could not read private key .pem file\n");
//     let private_key_bytes: &[u8] = &private_key_contents;

    
//     if let Ok(key) = Pk::from_private_key(private_key_bytes, None){
//         if let Ok(server_cert) = Certificate::from_pem_multiple(cert_content_bytes){

//             return Ok((key, server_cert));
//         } else {
//             println!("Could not generate certificate from {}", cert_path);
//         }
//     } else {
//         println!("Could not generate private key from {}", private_key_path);
//     }
//     Err(-1)
// }

// fn server_handle_connection(mut stream: TcpStream, key: Pk, cert: List<Certificate>){
fn server_handle_connection(mut stream: TcpStream){
    
    let mut config = Config::new(Endpoint::Server, Transport::Stream, Preset::Default);
    
    // set rng
    let entropy = OsEntropy::new();
    let rng = Arc::new(CtrDrbg::new(Arc::new(entropy), None).unwrap());
    config.set_rng(rng);

    // 2030.5 requires at minimum: TLS_ECDHE_ECDSA_WITH_AES_128_CCM_8 = EcdheEcdsaWithAes128Ccm8 = 0xC0AE
    // Firefox does not consider this secure communication, so cannot establish a connection with this server.
    let ciphersuite_list: Vec<i32> = vec![
        CipherSuite::EcdheEcdsaWithAes128Ccm8.into(),
        // CipherSuite::EcdheEcdsaWithAes256CbcSha384.into(), CipherSuite::EcdheEcdsaWithAes128CbcSha256.into(),
        // CipherSuite::EcdheEcdsaWithAes256GcmSha384.into(), CipherSuite::EcdheEcdsaWithAes128GcmSha256.into(), 
         0];

    config.set_ciphersuites(Arc::new(ciphersuite_list));


    // read .pem files
    let mut cert_content = fs::read("/home/neel/Desktop/unswCasualProfessional_Files/server_casProf/tls_server_client/certs/server_cert_selfSigned.pem")
        .expect("Could not read server certificate .pem file\n");
    let mut private_key_contents = fs::read("/home/neel/Desktop/unswCasualProfessional_Files/server_casProf/tls_server_client/certs/server1_private_key.pem")
        .expect("Could not read private key .pem file\n");
    
    cert_content.append(&mut vec![0u8]); // becuase certificates must be \0 terminated
    private_key_contents.append(&mut vec![0u8]);
    let cert_content_bytes: &[u8] = &cert_content;
    let private_key_bytes: &[u8] = &private_key_contents;

    // Ok, we know that till here, the server has read in the right information.

    // generate certificate, private key, and push to Config.
    // could do some error checking here.
    let server_cert = Arc::new(Certificate::from_pem_multiple(cert_content_bytes).unwrap());
    let key = Arc::new(Pk::from_private_key(private_key_bytes, None).unwrap());
    config.push_cert(server_cert, key);

    let mut ctx = Context::new(Arc::new(config));
    // so far so good - above code aligns with client_server.rs test on mbedtls GitHub repo.
    
    println!("Server: ctx setup complete. Listening on TLS connection.");
    match ctx.establish(stream, None) {
        Ok(()) => {
            println!("Conneciton Established!");
        },
        Err(a) => {
            println!("Error establishing connection. Code: {}", a);
            return;
        },
    };

    let mut server_buf = [0u8; 100];
    ctx.read_exact(&mut server_buf).unwrap();
    let full_request = String::from_utf8_lossy(&server_buf);
    println!("Server recieved request: {}", full_request);

    let http_request_line = full_request.lines().next().unwrap();
    let (status_line, http_file) = match http_request_line {
        "GET /dcap HTTP/1.1" => {
            ("HTTP/1.1 200 OK", "devCapMsg.html")
        },
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };
    println!("request line: {}\nreturned status line: {}",http_request_line, status_line);
    
    let content = fs::read_to_string(http_file).unwrap(); // this might return Err if file not found.

    // for the server, could use the error here as a way to check if the file/resource exists
    // but it still feels better to have a whitelist of the services offered by the server, compare 
    // the request against that,
    // and then have a function encapsulate the getting of the resource from the file.
    let content_length = content.len();
    let response = format!("{status_line}\r\nContent-Type: application/sep+xml\r\nContent-Length: {content_length}\r\n\r\n{content}");
    // let response = format!("{content}");
    println!("full server response: {}",  response);
    ctx.write_all(response.as_bytes()).unwrap();

    // let ciphersuite = ctx.ciphersuite().unwrap();
    // ctx.write_all(format!("Cipher suite: {:4x}", ciphersuite).as_bytes());
    // let buf_reader = BufReader::new(session);
    // let http_request_line = buf_reader
    //     .lines()
    //     .next()
    //     .unwrap()
    //     .unwrap();
    //     // first unwrap is for Option because lines() might return None
    //     // second unwrap is to return the actual str that gets assigned.
    //     // unwrap() in the above line for commit 8610b4aef42b51dbe13231b84d2a6dba5fb94bb2 
    //     // means that we are not handling the None that .lines() could return.

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
