#[macro_use]
extern crate rocket;
mod calcprob;
mod wakachi;
use std::sync::Mutex;
use calcprob::Model;
use once_cell::sync::Lazy;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use wakachi::Wakachi;

static MODEL: Lazy<Mutex<Model>> = Lazy::new(|| Mutex::new(Model::new()));
static WAKACHI: Lazy<Mutex<Wakachi>> = Lazy::new(|| Mutex::new(Wakachi::new()));

// APIで受け取るデータの形式と返すデータの形式を規定
#[derive(Debug, Serialize, Deserialize)]
struct WakachiReq {
    text: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct WakachiRes {
    result: Vec<String>,
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
        result: WAKACHI.lock().unwrap().wakachi(&request.text),
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
            WAKACHI.lock().unwrap().wakachi(q)
                .iter()
                .map(String::from)
                .collect::<Vec<String>>()
        })
        .collect::<Vec<Vec<String>>>();

    //中身を出力
    println!("{:?}", quiz);
    let response = GenRes {
        // ここにレスポンスのデータを設定
        result: MODEL.lock().unwrap().main(&quiz),
    };

    println!("{:?}", response);

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
