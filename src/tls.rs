extern crate mbedtls;
extern crate libc;
#[allow(unused_imports)]
use mbedtls::{rng::{CtrDrbg, OsEntropy}, ssl::{context, Context}};
use mbedtls::pk::Pk;
// use mbedtls::Error as MbedtlsError;
use mbedtls::ssl::config::{Endpoint, Preset, Transport};
use mbedtls::ssl::{Config, ciphersuites::{CipherSuite}};
use mbedtls::x509::{Certificate};

use std::{sync::Arc, fs, net::TcpStream};

use crate::errs::*;

pub fn tls_setup() -> Config{
    let mut config = Config::new(Endpoint::Server, Transport::Stream, Preset::Default);
    
    // set rng
    let entropy = OsEntropy::new();
    let rng = Arc::new(CtrDrbg::new(Arc::new(entropy), None).unwrap());
    config.set_rng(rng);
    return config;
}

pub fn set_2030_ciphers(mut config: Config) -> Config{
    let ciphersuite: Vec<i32> = vec![
        // 2030.5 requires at minimum: TLS_ECDHE_ECDSA_WITH_AES_128_CCM_8 = EcdheEcdsaWithAes128Ccm8 = 0xC0AE
        // Firefox has deprecated the least secure cipher 2030.5 allows and won't allow a connection using it.
        // any other cipher that uses elliptic curve p256 will work with the certificates we have generated.
        CipherSuite::EcdheEcdsaWithAes128Ccm8.into(),
        // CipherSuite::EcdheEcdsaWithAes256CbcSha384.into(), CipherSuite::EcdheEcdsaWithAes128CbcSha256.into(),
        // CipherSuite::EcdheEcdsaWithAes256GcmSha384.into(), CipherSuite::EcdheEcdsaWithAes128GcmSha256.into(), 
         0];
    config.set_ciphersuites(Arc::new(ciphersuite));
    config
}

pub fn set_cert(mut config: Config, private_key_path: &str, cert_path: &str) -> Config{

    // read .pem files

    let mut cert_content = fs::read(cert_path).expect("Could not read server certificate .pem file\n");
    let mut private_key_contents = fs::read(private_key_path).expect("Could not read private key .pem file\n");

    if !cert_content.ends_with(&[0u8]){
        cert_content.append(&mut vec![0u8]); // becuase certificates must be \0 terminated
    }
    if !private_key_contents.ends_with(&[0u8]){
        private_key_contents.append(&mut vec![0u8]); // becuase keys must be \0 terminated
    }
    
    let cert_content_bytes: &[u8] = &cert_content;
    let private_key_bytes: &[u8] = &private_key_contents;

    // generate certificate, private key, and push to Config.
    // could do some error checking here.
    let server_cert = Arc::new(Certificate::from_pem_multiple(cert_content_bytes).unwrap());
    let key = Arc::new(Pk::from_private_key(private_key_bytes, None).unwrap());
    if let Err(a) = config.push_cert(server_cert, key){
        println!("server: ERR: {a}\nCould not load certificate to mbedtls config");
        panic!();
    }
    config
}

pub fn establish_tls_server(tcp_stream: TcpStream, private_key_path: &str, cert_path: &str) -> Context{
    let config = tls_setup();
    let config = set_2030_ciphers(config);
    let config = set_cert(config, private_key_path, cert_path); // change this to return Result at a later stage.

    let mut ctx = Context::new(Arc::new(config));
    if let Err(a) = ctx.establish(tcp_stream, None) {
        panic_err(false, a, "could not \"establish\" tls client");
    }
    ctx
}
