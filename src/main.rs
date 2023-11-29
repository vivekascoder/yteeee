use actix_files as fs;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use anyhow::{bail, Result};
use log::info;
use std::process::Command;

#[get("/")]
async fn ping() -> impl Responder {
    HttpResponse::Ok().body("pong!!!")
}

#[actix_web::main]
async fn main() -> Result<()> {
    env_logger::init();
    let output = Command::new("which")
        .arg("yt-dlp")
        .output()
        .expect("can't find `yt-dlp` binary in your system.");

    if !output.status.success() {
        bail!("can't find `yt-dlp` binary in your system.");
    }
    let yt_dlp_path = String::from_utf8(output.stdout)?;
    info!("`yt-dlp` binary found at '{yt_dlp_path}'");

    let result = HttpServer::new(|| {
        App::new()
            .service(ping)
            .service(fs::Files::new("/static", "./static").show_files_listing())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await;
    if result.is_err() {
        bail!("issue: {:?}", result);
    }
    Ok(())
}
