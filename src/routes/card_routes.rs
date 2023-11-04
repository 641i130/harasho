use colored::Colorize;
use actix_web::{web, HttpResponse, dev::Service, dev::ServiceRequest, dev::ServiceResponse, Error, Result, get, http::header::ContentType, post, App, HttpRequest, HttpServer};
// AES
use crate::cry::aes::{aes_en, aes_dec};
use serde::Deserialize;

macro_rules! resp {
    ($str:expr) => {
        //HttpResponse::Ok().append_header(ContentType(mime::TEXT_HTML)).body($str)
        HttpResponse::Ok().append_header(ContentType::octet_stream()).body($str)
    };
}
///////////////////////////////////////////////////////////////////////////////////////////

// Card Command Codes
#[derive(Debug, Deserialize)]
pub enum CardCmd {
    READ = 256,
    REGISTER = 512,
    REISSUE = 1536,
}

impl CardCmd {
    fn from_u16(cmd_str: u16) -> Option<Self> {
        match cmd_str {
            256 => Some(CardCmd::READ),
            512 => Some(CardCmd::REGISTER),
            1536 => Some(CardCmd::REISSUE),
            _ => None, // Handle unknown values
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CardVals {
    cmd_str: u16, // Commands for card functions
    card_no: u64, // Example: 7020392002385103
}

#[post("/service/card/cardn.cgi")]
async fn cardn(web::Form(form): web::Form<CardVals>) -> HttpResponse {
    dbg!(&form);
    match CardCmd::from_u16(form.cmd_str) {
        Some(CardCmd::READ) => {
            println!("READ");
            resp!(format!("1\n1,1\n{}",form.card_no))
        },
        Some(CardCmd::REISSUE) => {
            println!("REISSUE");
            resp!("27")
        },
        Some(CardCmd::REGISTER) => {
            println!("REGISTER");
            // Add user into database later
            resp!(format!("1\n1,1\n{}",form.card_no))
        },
        _ => HttpResponse::NotFound().into()
    }
}


