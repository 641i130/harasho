use colored::Colorize;
use actix_web::{web, HttpResponse, dev::Service, dev::ServiceRequest, dev::ServiceResponse, Error, Result, get, http::header::ContentType, post, App, HttpRequest, HttpServer};
// AES
use crate::cry::aes::{aes_en, aes_dec};

macro_rules! resp {
    ($str:expr) => {
        //HttpResponse::Ok().append_header(ContentType(mime::TEXT_HTML)).body($str)
        HttpResponse::Ok().append_header(ContentType::octet_stream()).body($str)
    };
}

#[post("/game")]
pub async fn game_stuff(body: web::Bytes, req: actix_web::HttpRequest) -> HttpResponse {
    // For getting the game online, we need to give it a json type encrypted!
    println!("{}",format!("____________________________").black().on_white());
    println!("{}",format!("post -> /game").cyan());
    let ct = String::from_utf8_lossy(&body);
    println!("{}",format!("Ciphertext:").black().on_red());
    println!("{}", &ct.red());
    println!("{}",format!("Plaintext:").black().on_green());
    let pt = aes_dec(&body);
    println!("{}", &pt.green());
    resp!("")
}
