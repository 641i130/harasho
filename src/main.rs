#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_imports)]
use actix_web::{Result, get, http::header::ContentType, post, web, App, HttpRequest, HttpResponse, HttpServer};
use aes::cipher::{AsyncStreamCipher, KeyIvInit};
use log::{debug, error, info, log_enabled, Level};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use rsa::Pkcs1v15Encrypt;
use rsa::RsaPublicKey;
use rsa::pkcs8::DecodePublicKey;
use actix_files::NamedFile;
use std::path::PathBuf;

// Reading the cert
//use rustls_pemfile::{certs, pkcs8_private_keys};
//use rustls::{Certificate, PrivateKey, ServerConfig};

// AES encryption
use openssl::rsa::{Padding, Rsa};
type Aes128CfbEnc = cfb_mode::Encryptor<aes::Aes128>;

// Certify
use hex_literal::hex;
use md5::{Digest, Md5};

#[post("/basicinfo/")]
async fn basicinfo() -> HttpResponse {
    // This function is technically decrypting the plaintext into cipher text for the client to
    // encrypt to read it. It's very backwards, but this is how the game works. I hate it.
    println!("____________________________");
    println!("/basicinfo");
    let mut key_file = File::open("priv.pem").unwrap();
    let mut key_buffer = Vec::new();
    key_file.read_to_end(&mut key_buffer).unwrap();
    // Load the private key from the PEM data
    let rsa = Rsa::private_key_from_pem(&key_buffer).unwrap();
    let plaintext = r#"{"result":200,"response":{"base_url":"http://data.nesys.jp/game","download_url":"http://data.nesys.jp/download","key":"01234567890123456789012345678901","iv":"0123456789012345","tenpo_index":1337}}"#;
    let mut ciphertext = vec![0; rsa.size() as usize];
    rsa.private_encrypt(plaintext.as_bytes(), &mut ciphertext, Padding::PKCS1).unwrap();
    println!("\t{:?}", &plaintext);
    // println!("{:?}", String::from_utf8_lossy(&ciphertext));
    HttpResponse::Ok().append_header(ContentType::octet_stream()).body(ciphertext)
}

fn aes_en(plaintext: &&str) -> Vec<u8> {
    // Encodes string with aes 128 cfb encryption
    // Return encrypted text
    // Crypto constants
    let mut ciphertext = plaintext.as_bytes().to_vec();
    let key: &[u8] = "0123456789012345".as_bytes();
    let iv: &[u8] = "0123456789012345".as_bytes();

    // Encrypt
    Aes128CfbEnc::new(key.into(), iv.into()).encrypt(&mut ciphertext);
    ciphertext.into()
}

fn aes_dec(ciphertext: &&str) ->  () {
    ()
}
#[macro_export]
macro_rules! resp {
    ($str:expr) => {
        //HttpResponse::Ok().append_header(ContentType(mime::TEXT_HTML)).body($str)
        HttpResponse::Ok().append_header(ContentType::octet_stream()).body($str)
    };
}

#[get("/alive/{id}/Alive.txt")]
async fn alive(id: web::Path<String>, req: actix_web::HttpRequest) -> HttpResponse {
    println!("____________________________");
    println!("get -> /alive/{}/Alive.txt", id);
    resp!("")
}

#[get("/alive/i.php")]
async fn alive_i() -> HttpResponse {
    println!("____________________________");
    println!("get -> /alive/i.php");
    resp!("REMOTE ADDRESS:10.3.0.53\nSERVER NAME:harasho\nSERVER ADDR:10.3.0.53")
}

#[post("/service/incom/incomALL.php")]
async fn incomALL() -> HttpResponse {
    println!("____________________________");
    println!("post -> /service/incom/incomALL.php\n1+1");
    resp!("1+1")
}

#[post("/service/respone/respone.php")]
async fn respone() -> HttpResponse {
    println!("____________________________");
    println!("post -> /service/respone/respone.php\n1");
    resp!("1")
}

#[get("/server/FireAlert.php")]
async fn fire_alert() -> HttpResponse {
    println!("____________________________");
    println!("get -> /server/FireAlert.php");
    resp!("Success")
}

#[get("/server/cursel.php")]
async fn cursel() -> HttpResponse {
    println!("____________________________");
    println!("get -> /server/cursel.php");
    resp!("1\n")
}

#[get("/server/gameinfo.php")]
async fn gameinfo() -> HttpResponse {
    println!("____________________________");
    println!("get -> /server/gameinfo.php");
    resp!(r#"0
3
301000,test1
302000,test2
303000,test3"#)
}
/*
#[post("/game")]
async fn game_stuff(body: web::Bytes, req: actix_web::HttpRequest) -> HttpResponse {
    println!("____________________________");
    println!("post -> /game");
    println!("{:?}",String::from_utf8_lossy(&body));
    resp!("")
}
*/
#[post("/game/info")]
async fn game_info() -> HttpResponse {
    println!("____________________________");
    println!("post -> /game/info");
    // JSON type that is AES encrypted
    let plaintext = r#"{"result":200,"response":{"base_url":"http://10.3.0.53/game/next","information":[],"event_information":[],"encore_expiration_date":"2033-05-27"}}"#;
    let ciphertext = aes_en(&plaintext);

    println!("{:?}", String::from_utf8_lossy(&ciphertext));
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
async fn certify() -> HttpResponse {
    //async fn certify(data: web::Query<Certify>, req: HttpRequest) -> HttpResponse {
    println!("____________________________");
    println!("get -> /server/certify.php");
    /*
    dbg!(&data);
    let mut hasher = Md5::new();
    let gid_bytes = "303801".as_bytes(); // LL game nesys id
    hasher.update(gid_bytes);
    let hash_result = hasher.finalize();
    let mut ticket = String::new();
    for byte in hash_result {
        ticket.push_str(&format!("{:x?}", &byte));
    }*/
    let res = format!(
"host=http://ll.aoeu.top
no=1337
name=LLServer
pref=nesys
addr=Local
x-next-time=15
x-img=http://ll.aoeu.top/news.png
x-ranking=http://ll.aoeu.top/ranking/ranking.php
ticket=63c6598e9ddd2961e7dfa4d4eb8144a1");
    println!("Response:\n{}", &res);
    resp!(res)
}

#[get("/server/data.php")]
async fn server_data() -> HttpResponse {
    resp!("count=0\nnexttime=0\n")
}

async fn index(req: actix_web::HttpRequest) -> HttpResponse {
    println!("----------------------------");
    println!("Method: {:?}", req.method());
    println!("Host: {:?}", req.head().uri.host());
    println!("Path: {:?}", req.path());
    //dbg!(&req);
    HttpResponse::Ok().append_header(ContentType(mime::TEXT_PLAIN)).body("shit")
}

async fn test(req: HttpRequest) -> Result<NamedFile> {
    println!("---");
    println!("Method: {:?}", req.method());
    println!("Host: {:?}", req.head().uri.host());
    println!("Path: {:?}", req.path());

    let path: PathBuf = req.match_info().query("test.png").parse().unwrap();
    Ok(NamedFile::open(path)?)
}

async fn handle_post_request(body: web::Bytes,req: HttpRequest) -> HttpResponse {
    println!("----------------------------");
    println!("Method: {:?}", req.method());
    println!("Host: {:?}", req.head().uri.host());
    println!("Path: {:?}", req.path());
    println!("{:?}", String::from_utf8_lossy(&body));
    HttpResponse::Ok().append_header(ContentType(mime::TEXT_PLAIN)).body("shit")
}

/*
fn load_rustls_config() -> rustls::ServerConfig {
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
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    //let config = load_rustls_config();
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
            //.service(game_stuff)
            .service(certify)
            .service(server_data)
            .service(basicinfo)
            //.service(web::resource("/*").route(web::post().to(handle_post_request)))
            .route("{path:.*}",web::post().to(handle_post_request))
            .route("/{test.png}",web::get().to(test))
            .route("{path:.*}", web::get().to(index))
    })
    .bind("0.0.0.0:80")?
    .bind("0.0.0.0:5107")?
//    .bind_rustls("0.0.0.0:443", config)?
    .run()
    .await
}
