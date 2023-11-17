#[macro_use]
extern crate rocket;
mod calcprob;
mod wakachi;
use tokio::sync::Mutex;
use std::sync::Arc;
use calcprob::Model;
use once_cell::sync::Lazy;
use rocket::{serde::json::Json, futures};
use serde::{Deserialize, Serialize};
use wakachi::Wakachi;

// static MODEL: Lazy<Arc<Mutex<Model>>> = Lazy::new(|| Arc::new(Mutex::new(Model::new())));
static WAKACHI: Lazy<Arc<Mutex<Wakachi>>> = Lazy::new(|| Arc::new(Mutex::new(Wakachi::new())));

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
async fn your_handler(request: Json<WakachiReq>) -> Json<WakachiRes> {
    // リクエストを処理してレスポンスを生成するコードをここに書く
    let response = WakachiRes {
        // ここにレスポンスのデータを設定
        result: WAKACHI.lock().await.wakachi(&request.text),
    };

    Json(response)
}

//闇鍋生成API
#[post("/generate", data = "<request>")]
async fn generate(request: Json<GenReq>) -> Json<GenRes> {
    let wakachi = Arc::clone(&WAKACHI);
    let mut model = Model::new();
    let _ = model.make("static/corpus.txt");

    let quiz_futures = request.questions.iter().map(|q| {
        let wakachi_clone = Arc::clone(&wakachi);
        async move {
            let wakachi = wakachi_clone.lock().await;
            wakachi.wakachi(q)
                .iter()
                .map(String::from)
                .collect::<Vec<String>>()
        }
    });

    let quiz: Vec<Vec<String>> = futures::future::join_all(quiz_futures).await.into_iter().collect();
    println!("{:?}", quiz);
    let mut result = String::new();
    model.main(&quiz, &mut result).await;
    println!("{:?}", result);
    Json(GenRes { result })
    
}

#[rocket::main]
async fn main() {
    // let _ = MODEL.lock().await.make("static/corpus.txt");
    rocket::build()
        .mount("/", routes![your_handler, generate])
        .launch()
        .await
        .unwrap();
}
