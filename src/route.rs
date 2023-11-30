use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result};
use serde::{Deserialize, Serialize};

use crate::yt_dlp;

#[get("/")]
pub async fn ping() -> impl Responder {
    HttpResponse::Ok().body("pong!!!")
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetSubtitleReq {
    video_url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetSubtitleRes {
    subtitle: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetVideoInfoReq {
    video_url: String,
}

#[post("/get_subtitle")]
pub async fn get_subtitle(data: web::Json<GetSubtitleReq>) -> Result<impl Responder> {
    println!("Got req., {:?}", data);
    // Download the subtitle using yt-dlp.
    let text =
        yt_dlp::YtDlp::download_subtitle(&data.video_url).expect("can't download the video.");
    Ok(web::Json(GetSubtitleRes { subtitle: text }))
}

#[post("/get_video_info")]
pub async fn get_video_info(data: web::Json<GetVideoInfoReq>) -> Result<impl Responder> {
    println!("Got req., {:?}", data);
    // Download the subtitle using yt-dlp.
    let vid = yt_dlp::YtDlp::get_video_info(&data.video_url).expect("can't download the video.");
    Ok(web::Json(vid))
}
