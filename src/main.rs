#![allow(unused_variables)]
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

    HttpResponse::Ok().body("OK")
}

async fn certify(info: web::Path<(String, String, String, String, String)>) -> HttpResponse {
    let (gid, mac, r, md, cn) = info.into_inner(); // not used
    println!("CERTIFY REQUEST");
    // Will be read in as a string of length 1040 bytes 
    // Can use = or : (it will use both)
    // if error is in the body, it will break and return an error
    // Probably used for tracking / telemetry... we don't care about that right now lmao
    let res = format!("host=\ncard_id=7020392000147361,relay_addr=localhost,relay_port=80\nno=1337\nname=123\npref=nesys\naddr=nesys@home\nx-next-time=15\nx-img=http://localhost/news.png\nx-ranking=http://localhost/ranking.php\nticket=123456")
;
    HttpResponse::Ok().body(res)
}


async fn alive(info: web::Path<(String,)>) -> HttpResponse {
    println!("ALIVE REQUEST");
    if info.0 == "303801" {
        return HttpResponse::Ok().body("");
    }
    HttpResponse::Ok().body("Unsupported game.")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
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
    HttpServer::new(|| {
        App::new()
            .route("/alive/{id}/alive.txt", web::get().to(alive))
            .route("/server/certify.php", web::get().to(certify))
            .route("{path:.*}", web::get().to(index))
    })
    .bind("127.0.0.1:80")?
    .bind_rustls("127.0.0.1:443", config)?
    .run()
    .await
}