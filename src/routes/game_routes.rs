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

///////////////////////////////////////////////////////////////////////////////////////////
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct GameData {
    game: Game,
    param: Param,
    protocol: String,
    terminal: Terminal,
}

#[derive(Debug, Deserialize)]
struct Game {
    eventcode: String,
    version: String,
}

#[derive(Debug, Deserialize)]
struct Param {}

#[derive(Debug, Deserialize)]
struct Terminal {
    tenpo_id: String,
    tenpo_index: i32,
    terminal_attrib: i32,
    terminal_id: String,
}

#[post("/game")]
pub async fn game_stuff(body: web::Bytes, req: actix_web::HttpRequest) -> HttpResponse {
    // For getting the game online, we need to give it a json type encrypted!
    let ct = String::from_utf8_lossy(&body).trim().replace("\n", "").replace("\0", "");
    println!("{}",format!("Ciphertext:").black().on_red());
    println!("{}", &ct.red());
    println!("{}",format!("Plaintext:").black().on_green());
    let pt = aes_dec(&body);
    let cleaned = &pt.trim().replace("\n", "").replace("\0", "");
    println!("{}", &cleaned.green());
     
    // Given the plaintext of the request body
    // Attempt to deserialize the JSON into your custom struct
    match serde_json::from_str::<GameData>(&cleaned) {
        Ok(data) => {
            // You can now work with the deserialized data
            println!("{}",format!("data.protocol -> {}", data.protocol).black().bold().on_magenta());
            // Respond with success or any other logic you need
            return resp!("");
        },
        Err(err) => {
            // Handle deserialization error
            println!("Deserialization error: {}", err);
            // Respond with a JSON error message or other appropriate response
            return resp!("");
        }
    }
}
