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
use serde::{Deserialize, Serialize};

// Reading the cert
use rustls_pemfile::{certs, pkcs8_private_keys};
use std::fs::File;
use std::io::BufReader;
use std::io::Read;

// AES encryption
use openssl::rsa::{Padding, Rsa};
type Aes128CfbEnc = cfb_mode::Encryptor<aes::Aes128>;

// Certify
use hex_literal::hex;
use md5::{Digest, Md5};

#[post("/basicinfo")]
async fn basicinfo() -> HttpResponse {
    /*
        let pem = "-----BEGIN PUBLIC KEY-----
    MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAy63nybDg2d0l5Em5RTsx
    0QJ4WhuT4DwrzJD/SdPDbOotXE5BiVycfNxcfXVSa74SvqThyQs4KasZyK/NWJN6
    Xyi7NQgh2xKYc3eVj8b8MSkhz5Y7631dscLQRR9sDiTf2+jR8umd6U9op/ZucaOU
    zaEcyHalryeeRwD8q7mtlBccL+5dSVVWuPaJ/Oh4Oivk4qNunYHygQ/iw2vBgN3f
    6tB1yiKlUe0T51FS1yJcavWilp2JA6XGEhh0OmFJX6wf5vPu9heTXGqnriClinXn
    XV1zUPDaa0udD8n2OV9NphozqD7TT4pE68G65Xz/iLAaEudSg7f1Shu+VFtt/cF4
    NwIDAQAB
    -----END PUBLIC KEY-----";
        */
    let mut key_file = File::open("priv.pem").unwrap();

    let mut key_buffer = Vec::new();
    key_file.read_to_end(&mut key_buffer).unwrap();

    // Load the private key from the PEM data
    let rsa = Rsa::private_key_from_pem(&key_buffer).unwrap();

    let plaintext = r#"
    {'result':200,'response':{'base_url':'http://10.3.0.53/game/info','download_url':'http://10.3.0.53/download','key':'01234567890123456789012345678901','iv':'0123456789012345','tenpo_index':1337}}
    "#;
    let mut ciphertext = vec![0; rsa.size() as usize];
    rsa.public_encrypt(plaintext.as_bytes(), &mut ciphertext, Padding::PKCS1).unwrap();

    println!("{:?}", String::from_utf8_lossy(&ciphertext));

    //let mut rng = rand::thread_rng();
    //let pub_key = RsaPublicKey::from_public_key_pem(pem).unwrap();
    //let ciphertext = pub_key.encrypt(&mut rng, Pkcs1v15Encrypt, &plaintext.as_bytes()).expect("failed to encrypt");
    HttpResponse::Ok().append_header(ContentType::octet_stream()).body(ciphertext)
}

#[macro_export]
macro_rules! resp {
    ($str:expr) => {
        //HttpResponse::Ok().append_header(ContentType(mime::TEXT_PLAIN)).body($str)
        HttpResponse::Ok().append_header(ContentType::octet_stream()).body($str)
    };
}

#[get("/alive/{id}/Alive.txt")]
async fn alive(id: web::Path<String>, req: actix_web::HttpRequest) -> HttpResponse {
    println!("---");
    println!("Method: {:?}", req.method());
    println!("Host: {:?}", req.head().uri.host());
    println!("Path: {:?}", req.path());

    println!("/alive/{}/Alive.txt", id);
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
#[post("/game/info")]
async fn game_info() -> HttpResponse {
    // JSON type that is AES encrypted
    let plaintext = r#"{"result":200,"response":{"base_url":"http://10.3.0.53/game/next","information":[],"event_information":[],"encore_expiration_date":"2033-05-27"}}"#;

    // Crypto constants
    let key: &[u8] = "0123456789012345".as_bytes();
    let iv: &[u8] = "0123456789012345".as_bytes();

    // Encrypt
    let mut ciphertext = plaintext.as_bytes().to_vec();
    Aes128CfbEnc::new(key.into(), iv.into()).encrypt(&mut ciphertext);

    //println!("{:?}", String::from_utf8_lossy(&ciphertext));
    HttpResponse::Ok().append_header(ContentType::octet_stream()).body(ciphertext)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Certify {
    pub gid: u32,
    pub mac: String,
    pub r: u32,
    pub md: String,
    pub cn: String,
}

#[get("/server/certify.php")]
async fn certify(data: web::Query<Certify>, req: HttpRequest) -> HttpResponse {
    println!("Certify!");
    dbg!(&data);
    // Need to
    let mut hasher = Md5::new();
    let gid_bytes = "303807".as_bytes(); // LL game nesys id
    hasher.update(gid_bytes);
    let hash_result = hasher.finalize();
    for byte in hash_result {
        print!("{:x?}", &byte);
    }
    println!("");
    let res = format!(
        "host=http://10.3.0.53
no=1337
name=LLServer
pref=nesys
addr=Local
x-next-time=15
x-img=http://10.3.0.53/test.png
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
    //dbg!(&req);
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
    //env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));
    let config = load_rustls_config();
    info!("Certificates loaded.");
    println!("Started!");
    HttpServer::new(|| {
        App::new()
            .service(alive)
            .service(alive_i)
            .service(incomALL)
            .service(respone)
            .service(fire_alert)
            .service(cursel)
            .service(gameinfo)
            .service(game_info)
            .service(certify)
            .service(server_data)
            .service(basicinfo)
            .route("{path:.*}", web::get().to(index))
    })
    .bind("0.0.0.0:80")?
    .bind("0.0.0.0:5107")?
    .bind_rustls("0.0.0.0:443", config)?
    .run()
    .await
}
