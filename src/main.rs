#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_imports)]
use actix_web::{get, http::header::ContentType, web, App, HttpRequest, HttpResponse, HttpServer};
use log::{debug, error, info, log_enabled, Level};
use rustls::{Certificate, PrivateKey, ServerConfig};
use rustls_pemfile::{certs, pkcs8_private_keys};
use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;

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
    let _id: u32 = req.match_info().query("user_id").parse().unwrap();
    let (_gid, _mac, _r, _md, _cn) = info.into_inner(); // not used
    println!("CERTIFY REQUEST");
    // Will be read in as a string of length 1040 bytes
    // Can use = or : (it will use both)
    // if error is in the body, it will break and return an error
    // Probably used for tracking / telemetry... we don't care about that right now lmao
    let res =
        format!("host=\ncard_id=7020392000147361,relay_addr=localhost,relay_port=80\nno=1337\nname=123\npref=nesys\naddr=nesys@home\nx-next-time=15\nx-img=http://localhost/news.png\nx-ranking=http://localhost/ranking.php\nticket=123456");
    HttpResponse::Ok().body(res)
}

#[get("/alive/{id}/Alive.txt")]
async fn alive(_info: web::Path<(String,)>, _query: web::Query<Alive>) -> HttpResponse {
    //let (mac, ip, errcnt, errcode, errstr, access, speed, total_down, game_down, process_num, OS_Phys, OS_Virtual, AP_Phys, AP_Virtual, SV_Phys, SV_Virtual, free_space, uptime, ver, libver, game_hash) = query.into_inner();
    println!("ALIVE REQUEST");
    /*
    if info.0 == "303801" {
        return "".to_string()
    }
    "".to_string()*/
    HttpResponse::Ok().append_header(ContentType(mime::TEXT_PLAIN)).append_header(("Connection", "keep-alive")).finish()
}

async fn fire_alert(info: web::Query<(u32, String, u32, u32)>) -> HttpResponse {
    println!("FireAlert REQUEST");
    let (_game_id, _mac_addr, _tick_count, _status) = info.into_inner();

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

fn load_rustls_config() -> rustls::ServerConfig {
    // init server config builder with safe defaults
    let config = ServerConfig::builder().with_safe_defaults().with_no_client_auth();

    // load TLS key/cert files
    let cert_file = &mut BufReader::new(File::open("./certs/nesica1.csr").unwrap());
    let key_file = &mut BufReader::new(File::open("./certs/nesica1.key").unwrap());

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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let config = load_rustls_config();
    info!("Certificates loaded.");
    HttpServer::new(|| {
        App::new()
            .service(alive)
            .route("/server/FireAlert.php", web::get().to(fire_alert))
            .route("/server/certify.php", web::get().to(certify))
            .route("{path:.*}", web::get().to(index))
    })
    .bind("127.0.0.1:80")?
    .bind("127.0.0.1:5107")?
    .bind_rustls("0.0.0.0:443", config)?
    .run()
    .await
}
