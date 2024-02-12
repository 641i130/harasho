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

pub fn encresp(content: &str) -> HttpResponse {
    let encrypted_content = aes_en(&content); // Encrypt the content.
    resp!(encrypted_content) // Use your macro here, if it can be adapted to work with Vec<u8>
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
fn clean_json_string(input: &str) -> String {
    // Remove whitespace characters, including line breaks and extra spaces
    let cleaned_str = input
        .chars()
        .filter(|c| !c.is_control() || c == &'\n' || c == &'\r')
        .collect();    
    cleaned_str
}

fn to_xxd_format(input: &[u8]) {
    let chunk_size = 16;
    for (i, chunk) in input.chunks(chunk_size).enumerate() {
        // Print the offset
        print!("{:08x}: ", i * chunk_size);

        // Print the hex representation
        for byte in chunk.iter() {
            print!("{:02x} ", byte);
        }

        // Pad the hex representation if the chunk is less than chunk_size
        if chunk.len() < chunk_size {
            for _ in 0..(chunk_size - chunk.len()) {
                print!("   ");
            }
        }

        print!(" |");

        // Print the ASCII representation
        for byte in chunk.iter() {
            if byte.is_ascii_graphic() || *byte == b' ' {
                print!("{}", *byte as char);
            } else {
                print!(".");
            }
        }

        println!("|");
    }
}

#[post("/game")]
pub async fn game_stuff(body: web::Bytes, req: actix_web::HttpRequest) -> HttpResponse {
    // For getting the game online, we need to give it a json type encrypted!
    //println!("{}",format!("Ciphertext:").black().on_red());
    //to_xxd_format(&body);
    println!("{}",format!("Plaintext:").black().on_green());
    let pt = aes_dec(&body);
    let cleaned = clean_json_string(&pt).replace("\n","");
    println!("{}", &cleaned.green());
    // Given the plaintext of the request body
    // Attempt to deserialize the JSON into your custom struct
    match serde_json::from_str::<GameData>(&cleaned) {
        Ok(data) => {
            // You can now work with the deserialized data
            println!("{}",format!("data.protocol -> {}", data.protocol).black().bold().on_magenta());
            match data.protocol.as_str() {
                "unlock" => return encresp("{'result':200,'response':{}}"), // 1st
                "gameconfig" => return encresp("{'result':200,'response':{'version':'2.4.1','season_event_uid':'0','level_cap':200,'skill_level_cap':999,'yell_rank_cap':999,'card_sells_value':100,'music_release_version':99999,'finale_focus_release_version':99999,'stage_release_version':99999,'member_card_release_version':99999,'memorial_card_release_version':99999,'skill_card_release_version':99999,'advertise_movie_status_0':true,'advertise_movie_status_1':false,'advertise_movie_status_2':false,'advertise_movie_status_3':false,'live_demo_table_id':5,'exp_mag':1,'exp_mag_target_level':5,'cheer_enable':false,'achievement_release_version':99999,'travel_pamphlet_release_version':99999,'live_break_max':1,'yell_achieve_mobile_point':100,'yell_point_mag':1,'dice_bonus_credit':100}}"), // 2nd
                "information" => return encresp("{'result':200,'response':{}}"), // 3rd
                "ranking" => return encresp("{'result':200,'response':{}}"), // 4th
                "auth" => return encresp("{'result':200,'response':{}}"),
                "achievement" => return encresp("{'result':200,'response':{}}"),
                "achievementyell" => return encresp("{'result':200,'response':{}}"),
                "checkword" => return encresp("{'result':200,'response':{}}"),
                "discard" => return encresp("{'result':200,'response':{}}"),
                "gacha.member" => return encresp("{'result':200,'response':{}}"),
                "gameentry" => return encresp("{'result':200,'response':{}}"),
                "gameentry.center" => return encresp("{'result':200,'response':{}}"),
                "gameresult" => return encresp("{'result':200,'response':{}}"),
                "gametotalresult" => return encresp("{'result':200,'response':{}}"),
                "gameexit" => return encresp("{'result':200,'response':{}}"),
                "getmembercard" => return encresp("{'result':200,'response':{}}"),
                "music.unlock" => return encresp("{'result':200,'response':{}}"),
                "present" => return encresp("{'result':200,'response':{}}"),
                "printcard" => return encresp("{'result':200,'response':{}}"),
                "profile.inquiry" => return encresp("{'result':200,'response':{}}"),
                "profile.print" => return encresp("{'result':200,'response':{}}"),
                "userranking" => return encresp("{'result':200,'response':{}}"),
                "registerafter" => return encresp("{'result':200,'response':{}}"),
                "scfescheck" => return encresp("{'result':200,'response':{}}"),
                "scfesregister" => return encresp("{'result':200,'response':{}}"),
                "sellcard" => return encresp("{'result':200,'response':{}}"),
                "setterminallog" => return encresp("{'result':200,'response':{}}"),
                "setterminalstatus" => return encresp("{'result':200,'response':{}}"),
                "travelstamp" => return encresp("{'result':200,'response':{}}"),
                "TravelStart" => return encresp("{'result':200,'response':{}}"),
                "TravelResult" => return encresp("{'result':200,'response':{}}"),
                "TravelSnap.commit" => return encresp("{'result':200,'response':{}}"),
                "TravelSnap.inquiry" => return encresp("{'result':200,'response':{}}"),
                "TravelSnap.share" => return encresp("{'result':200,'response':{}}"),
                "TravelSnap.print" => return encresp("{'result':200,'response':{}}"),
                "userdata.get" => return encresp("{'result':200,'response':{}}"),
                "userdata.initialize" => return encresp("{'result':200,'response':{}}"),
                "userdata.set" => return encresp("{'result':200,'response':{}}"),
                _ => return encresp("{'result':400,'response':{}}"),
            }
        },
        Err(err) => {
            // Handle deserialization error
            println!("Deserialization error: {}", err);
            // encrespond with a JSON error message or other appropriate response
            return encresp("{'result':400,'response':{}}");
        }
    }
} 
