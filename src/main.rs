#![allow(unused_variables)]
use actix_web::{web, get, App, HttpServer, HttpResponse, HttpRequest};
use rustls::ServerConfig;
use rustls::Certificate;
use rustls_pemfile::pkcs8_private_keys;
use rustls_pemfile::certs;
use std::fs::File;
use std::io::BufReader;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Alive {
    mac: String,
    ip: String,
    errcnt: u32,
    errcode: u32,
    errstr: String,
    access: String,
    speed: u32,
    total_down: u32,
    game_down: u32,
    process_num: i32,
    OS_Phys: i32,
    OS_Virtual: i32,
    AP_Phys: i32,
    AP_Virtual: i32,
    SV_Phys: i32,
    SV_Virtual: i32,
    free_space: i32,
    uptime: String,
    ver: String,
    libver: String,
    game_hash: String,
}


async fn certify(req: HttpRequest, info: web::Path<(String, String, String, String, String)>) -> HttpResponse {
    let id:u32 = req.match_info().query("user_id").parse().unwrap();
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

#[get("/alive/{id}/alive.txt")]
async fn alive(info: web::Path<(String,)>, query: web::Query<Alive>) -> String {
    dbg!(&query);
    //let (mac, ip, errcnt, errcode, errstr, access, speed, total_down, game_down, process_num, OS_Phys, OS_Virtual, AP_Phys, AP_Virtual, SV_Phys, SV_Virtual, free_space, uptime, ver, libver, game_hash) = query.into_inner();
    println!("ALIVE REQUEST");
    if info.0 == "303801" {
        return "1".to_string()
    }
    "0".to_string()
}

async fn fire_alert(info: web::Query<(u32, String, u32, u32)>) -> HttpResponse {
    println!("FireAlert REQUEST");
    let (game_id, mac_addr, tick_count, status) = info.into_inner();

    // Perform actions based on the passed in parameters

    // Return a response
    HttpResponse::Ok().body("OK")
}

async fn index(req: actix_web::HttpRequest) -> HttpResponse {
    println!("~");
    //println!("Method: {:?}", req.method());
    //println!("Host: {:?}", req.head().uri.host());
    //println!("Path: {:?}", req.path());
    dbg!(&req);

    HttpResponse::Ok().body("OK")
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
            .service(alive)
            .route("/server/FireAlert.php", web::get().to(fire_alert))
            .route("/server/certify.php", web::get().to(certify))
            .route("{path:.*}", web::get().to(index))
    })
    .bind("127.0.0.1:80")?
    .bind_rustls("127.0.0.1:443", config)?
    .run()
    .await
}