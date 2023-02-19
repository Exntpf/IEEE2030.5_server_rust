
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

use std::env;

use source_code::{
    server::run_server,
    client::run_client,
};

fn main(){
    let args: Vec<String> = env::args().collect();
    if !(args.len() == 2) {
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
