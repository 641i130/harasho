use actix_web::{web, App, HttpServer, HttpResponse};
use rustls::ServerConfig;
use rustls::Certificate;
use rustls_pemfile::pkcs8_private_keys;
use rustls_pemfile::certs;
use std::fs::File;
use std::io::BufReader;

async fn index(req: actix_web::HttpRequest) -> HttpResponse {
    println!("~");
    //println!("Method: {:?}", req.method());
    //println!("Host: {:?}", req.head().uri.host());
    //println!("Path: {:?}", req.path());
    dbg!(&req);

    HttpResponse::Ok().body("3")
}

async fn certify(req: actix_web::HttpRequest) -> HttpResponse {
    // Will be read in as a string of length 1040 bytes 
    // Can use = or : (it will use both)
    // if error is in the body, it will break and return an error
    let res = r"$host
    "
    HttpResponse::Ok().body()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    async {
        println!("Starting!!!");
        // Load key files
        let cert_file = &mut BufReader::new(
            File::open("cert.pem").unwrap());
        let key_file = &mut BufReader::new(
            File::open("key.pem").unwrap());

        // Parse the certificate and set it in the configuration
        let cert_chain = certs(cert_file).unwrap();
        let mut keys = pkcs8_private_keys(key_file).unwrap();
        let config = ServerConfig::builder()
            .with_safe_defaults()
            .with_no_client_auth()
            .with_single_cert(vec![Certificate(cert_chain.into_iter().next().expect("Cert parsing error"))], rustls::PrivateKey(keys.remove(0)))
            .expect("bad certificate/key");
    }.await;
    HttpServer::new(|| {
        App::new()
            .route("{path:.*}", web::get().to(index))
            .route("/server/certify.php", web::get().to(certify))
            // add /server/certify.php for cert3.nesys.jp
    })
    .bind("127.0.0.1:80")?
    .bind_rustls("127.0.0.1:443", config)?
    .run()
    .await
}