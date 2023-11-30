use actix_files as fs;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use anyhow::{bail, Result};
use log::info;
use std::process::Command;
use yteeee::{
    route::{get_subtitle, get_video_info, ping},
    yt_dlp,
};

#[actix_web::main]
async fn main() -> Result<()> {
    env_logger::init();

    let yt_dlp_path = yt_dlp::YtDlp::get_binary_path()?;
    info!("`yt-dlp` binary found at '{yt_dlp_path}'");

    let result = HttpServer::new(|| {
        App::new()
            .service(ping)
            .service(get_subtitle)
            .service(get_video_info)
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
