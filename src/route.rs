use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

use crate::yt_dlp;

#[get("/")]
pub async fn ping() -> impl Responder {
    HttpResponse::Ok().body("pong!!!")
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetSubtitleData {
    video_url: String,
}

#[post("/get_subtitle")]
pub async fn get_subtitle(data: web::Json<GetSubtitleData>) -> impl Responder {
    println!("Got req., {:?}", data);
    // Download the subtitle using yt-dlp.
    HttpResponse::Ok()
        .body(yt_dlp::YtDlp::download_subtitle(&data.video_url).expect("can't download the video."))
}
