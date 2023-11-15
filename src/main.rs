#[macro_use]
extern crate rocket;

mod calcprob;
mod wakachi;

use std::sync::Mutex;

use calcprob::Model;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use once_cell::sync::Lazy;

static MODEL: Lazy<Mutex<Model>> = Lazy::new(|| Mutex::new(Model::new()));

// APIで受け取るデータの形式と返すデータの形式を規定
#[derive(Debug, Serialize, Deserialize)]
struct WakachiReq {
    text: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct WakachiRes {
    result: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct GenReq {
    questions: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct GenRes {
    result: String,
}

//分かち書きAPI
#[post("/wakachi", data = "<request>")]
fn your_handler(request: Json<WakachiReq>) -> Json<WakachiRes> {
    // リクエストを処理してレスポンスを生成するコードをここに書く
    let response = WakachiRes {
        // ここにレスポンスのデータを設定
        result: wakachi::wakachi(&request.text),
    };

    Json(response)
}

//闇鍋生成API
#[post("/generate", data = "<request>")]
fn generate(request: Json<GenReq>) -> Json<GenRes> {
    // リクエストを処理してレスポンスを生成するコードをここに書く

    //Stringをスペース区切りでVecに変換
    let quiz = request
        .questions
        .iter()
        .map(|q| {
            q.split_whitespace()
                .map(String::from)
                .collect::<Vec<String>>()
        })
        .collect::<Vec<Vec<String>>>();

    let response = GenRes {
        // ここにレスポンスのデータを設定
        result: MODEL.lock().unwrap().main(&quiz),
    };

    Json(response)
}

#[rocket::main]
async fn main() {
    let _ = MODEL.lock().unwrap().make("static/corpus.txt");
    rocket::build()
        .mount("/", routes![your_handler, generate])
        .launch()
        .await
        .unwrap();
}