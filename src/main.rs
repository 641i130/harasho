//! Restart PC and see if installed certs work
//! get rid of HTTP cert issue in access logs
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_imports)]
use actix_web::{get, http::header::ContentType, post, web, App, HttpRequest, HttpResponse, HttpServer};
use aes::cipher::{AsyncStreamCipher, KeyIvInit};
use log::{debug, error, info, log_enabled, Level};
use rustls::{Certificate, PrivateKey, ServerConfig};
use rustls_pemfile::{certs, pkcs8_private_keys};

use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;

type Aes128CfbEnc = cfb_mode::Encryptor<aes::Aes128>;

#[derive(Serialize, Deserialize)]
struct BasicInfo {
    BaseUrl: String,
    DownloadUrl: String,
    Key: String,
    Iv: String,
    TenpoIndex: u16,
}

#[post("/basicinfo")]
async fn basicinfo() -> HttpResponse {
    // Encrypt or something first...
    // Very possible PGP is needed I think/? or aes portion ... idk
    let data: BasicInfo = BasicInfo {
        BaseUrl: "http://10.3.0.53/game/info".to_string(),
        DownloadUrl: "http://10.3.0.53/download".to_string(),
        Key: "0123456789012345".to_string(),
        Iv: "0123456789012345".to_string(),
        TenpoIndex: 1337u16,
    };
    let plaintext: String = serde_json::to_string(&data).unwrap();

    // Crypto constants
    let key: &[u8] = "0123456789012345".as_bytes();
    let iv: &[u8] = "0123456789012345".as_bytes();

    // Encrypt
    let mut ciphertext = plaintext.as_bytes().to_vec();
    Aes128CfbEnc::new(key.into(), iv.into()).encrypt(&mut ciphertext);

    print_valid_chars!(ciphertext.iter());
    HttpResponse::Ok().content_type("application/octet-stream").body(ciphertext)
}

#[macro_export]
macro_rules! resp {
    ($str:expr) => {
        HttpResponse::Ok().append_header(ContentType(mime::TEXT_PLAIN)).body($str)
    };
}

#[macro_export]
macro_rules! print_valid_chars {
    ($slice:expr) => {{
        print!("{{{{");
        let mut valid_chars = String::new();
        for &byte in $slice {
            if let Ok(chr) = std::str::from_utf8(&[byte]) {
                if chr.is_ascii() && &byte >= &32 {
                    valid_chars.push_str(chr);
                }
            } else {
                valid_chars.push_str(".");
            }
        }
        println!("{}}}}}", valid_chars);
    }};
}

#[get("/alive/303807/Alive.txt")]
async fn alive() -> HttpResponse {
    resp!("")
}

#[get("/alive/i.php")]
async fn alive_i() -> HttpResponse {
    resp!("REMOTE ADDRESS:10.3.0.53\nSERVER NAME:LLSIFAC\nSERVER ADDR:10.3.0.53")
}

#[post("/service/card/incomALL.php")]
async fn incomALL() -> HttpResponse {
    resp!("1+1")
}

#[post("/service/respone/respone.php")]
async fn respone() -> HttpResponse {
    resp!("1")
}

#[get("/server/FireAlert.php")]
async fn fire_alert() -> HttpResponse {
    resp!("OK")
}

#[get("/server/cursel.php")]
async fn cursel() -> HttpResponse {
    resp!("1\n")
}

#[get("/server/gameinfo.php")]
async fn gameinfo() -> HttpResponse {
    resp!("0\n3\n301000,test1\n302000,test2\n303000,test3\n")
}

#[get("/server/certify.php")]
async fn certify() -> HttpResponse {
    let res = format!(
        "host=http://10.3.0.53
no=1337
name=LLServer
pref=nesys
addr=Local
x-next-time=15
x-img=https://static.wikia.nocookie.net/houkai-star-rail/images/1/18/Character_March_7th_Splash_Art.png
x-ranking=http://10.3.0.53/ranking/ranking.php
ticket=9251859b560b33b031516d05c2ef3c28"
    );
    resp!(res)
}

#[get("/server/data.php")]
async fn server_data() -> HttpResponse {
    resp!("count=0\nnexttime=0\n")
}

async fn index(req: actix_web::HttpRequest) -> HttpResponse {
    println!("---");
    println!("Method: {:?}", req.method());
    println!("Host: {:?}", req.head().uri.host());
    println!("Path: {:?}", req.path());
    dbg!(&req);
    HttpResponse::Ok().append_header(ContentType(mime::TEXT_PLAIN)).body("shit")
}

fn load_rustls_config() -> rustls::ServerConfig {
    // init server config builder with safe defaults
    let config = ServerConfig::builder().with_safe_defaults().with_no_client_auth();

    // load TLS key/cert files
    let cert_file = &mut BufReader::new(File::open("./certs/nesica1.crt").expect("Certificate not found!"));
    let key_file = &mut BufReader::new(File::open("./certs/nesica1.key").expect("Key not found!"));

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
            .service(alive_i)
            .service(incomALL)
            .service(respone)
            .service(fire_alert)
            .service(cursel)
            .service(gameinfo)
            .service(certify)
            .service(server_data)
            .service(basicinfo)
            .route("{path:.*}", web::get().to(index))
    })
    .bind("127.0.0.1:80")?
    .bind("127.0.0.1:5107")?
    .bind_rustls("0.0.0.0:443", config)?
    .run()
    .await
}
