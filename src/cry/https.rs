// Reading the cert for HTTPS -> later...
//use rustls_pemfile::{certs, pkcs8_private_keys};
//use rustls::{Certificate, PrivateKey, ServerConfig};

/*
pub fn load_rustls_config() -> rustls::ServerConfig {
    // init server config builder with safe defaults
    let config = ServerConfig::builder().with_safe_defaults().with_no_client_auth();

    // load TLS key/cert files
    let cert_file = &mut BufReader::new(File::open("./certs/test/nesica1.crt").expect("Certificate not found!"));
    let key_file = &mut BufReader::new(File::open("./certs/test/nesica1.key").expect("Key not found!"));

    // convert files to key/cert objects
    let cert_chain = certs(cert_file).unwrap().into_iter().map(Certificate).collect();
    let mut keys: Vec<PrivateKey> = pkcs8_private_keys(key_file).unwrap().into_iter().map(PrivateKey).collect();

    // exit if no keys could be parsed
    if keys.is_empty() {
        eprintln!("Could not locate PKCS 8 private keys.");
        std::process::exit(1);
    }

    config.with_single_cert(cert_chain, keys.remove(0)).unwrap()
}
*/

