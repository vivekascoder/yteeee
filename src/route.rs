use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result};
use anyhow::bail;
use chrono::{prelude::*, Duration};
use jsonwebtoken::{encode, EncodingKey, Header};
use log::info;
use serde::{Deserialize, Serialize};
use std::env;

use crate::google_oauth::{get_google_user, request_token};
use crate::yt_dlp;

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

#[derive(Serialize, Deserialize, Debug)]
pub struct SummarizeVideoReq {
    video_url: String,
}

#[derive(Serialize, Deserialize, Debug)]
enum SummarizeVideoResData {
    ChatGptCompletionRes(ChatGptCompletionResFormatted),
    SummarizeVideoResError(String),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SummarizeVideoRes {
    status: bool,
    data: SummarizeVideoResData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}

#[derive(Debug, Deserialize)]
pub struct QueryCode {
    pub code: String,
    pub state: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct RoleContent {
    role: String,
    content: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ChatGptCompletionReq {
    model: String,
    messages: Vec<RoleContent>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ChatGptCompletionRes {
    model: String,
    choices: Vec<ChatGptResChoices>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ChatGptCompletionResFormatted {
    result: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ChatGptResChoices {
    index: u64,
    message: RoleContent,
    finish_reason: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct GoogleOauthResponse {
    token: String,
}

#[get("/")]
pub async fn ping() -> impl Responder {
    HttpResponse::Ok().body("pong!!!")
}

#[post("/get_subtitle")]
pub async fn get_subtitle(data: web::Json<GetSubtitleReq>) -> Result<impl Responder> {
    info!("Got req., {:?}", data);
    // Download the subtitle using yt-dlp.
    let text =
        yt_dlp::YtDlp::download_subtitle(&data.video_url).expect("can't download the video.");
    Ok(web::Json(GetSubtitleRes { subtitle: text }))
}

#[post("/summarize_video")]
pub async fn summarize_video(data: web::Json<GetSubtitleReq>) -> Result<impl Responder> {
    info!("Got req., {:?}", data);

    let a = tokio::task::spawn(async move {
        // Download the subtitle using yt-dlp.
        info!("task: starting to download subs.");
        let text =
            yt_dlp::YtDlp::download_subtitle(&data.video_url).expect("can't download the video.");
        info!("task: downloaded subs.");

        // Sent openAi request to summarize the video.
        let client = reqwest::Client::new();
        let data_req = ChatGptCompletionReq {
            model: "gpt-3.5-turbo".to_string(),
            messages: vec![RoleContent {
                role: "system".to_string(),
                content:
                    "you're an ai that can summarise youtube videos from caption, title of the video."
                        .to_string(),
            }, RoleContent {
                role: "user".to_string(),
                content: format!("Summarise a youtube video that has the following caption in 200-300 words. \n\n  {}", text)
            }],
        };

        let res = client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Content-Type", "application/json")
            .header(
                "Authorization",
                format!("Bearer {}", env::var("OPENAI_KEY_YTEEEE").unwrap()),
            )
            .json(&data_req)
            .send()
            .await
            .expect("can't send request to openai");
        info!("task: Got response from openai");

        if res.status().is_success() {
            let output = res
                .json::<ChatGptCompletionRes>()
                .await
                .expect("didn't got anything from api");
            info!("this is the summary of the video: {:?}", output);
            let choices = output.choices;

            // return Ok(web::Json(SummarizeVideoRes {
            //     status: true,
            //     data: SummarizeVideoResData::ChatGptCompletionRes(ChatGptCompletionResFormatted {
            //         result: choices
            //             .iter()
            //             .map(|v| v.message.content.clone())
            //             .collect::<Vec<String>>(),
            //     }),
            // }));

            // Ok(web::Json(SummarizeVideoRes {
            //     status: false,
            //     data: SummarizeVideoResData::SummarizeVideoResError("can't fetch".to_string()),
            // }))
        }
    });
    Ok(web::Json(SummarizeVideoRes {
        status: false,
        data: SummarizeVideoResData::SummarizeVideoResError("can't fetch".to_string()),
    }))
}

#[post("/get_video_info")]
pub async fn get_video_info(data: web::Json<GetVideoInfoReq>) -> Result<impl Responder> {
    info!("Got req., {:?}", data);
    // Download the subtitle using yt-dlp.
    let vid = yt_dlp::YtDlp::get_video_info(&data.video_url).expect("can't download the video.");
    Ok(web::Json(vid))
}

// Login related routes goes here

#[get("/sessions/oauth/google")]
async fn google_oauth_handler(query: web::Query<QueryCode>) -> impl Responder {
    let code = &query.code;
    let state = &query.state;

    if code.is_empty() {
        return HttpResponse::Unauthorized().json(
            serde_json::json!({"status": "fail", "message": "Authorization code not provided!"}),
        );
    }

    let token_response = request_token(code.as_str()).await;

    if token_response.is_err() {
        let message = token_response.err().unwrap().to_string();
        return HttpResponse::BadGateway()
            .json(serde_json::json!({"status": "fail", "message": message}));
    }

    let token_response = token_response.unwrap();

    let google_user = get_google_user(&token_response.access_token, &token_response.id_token).await;

    if google_user.is_err() {
        let message = google_user.err().unwrap().to_string();
        return HttpResponse::BadGateway()
            .json(serde_json::json!({"status": "fail", "message": message}));
    }

    let google_user = google_user.unwrap();

    let email = google_user.email.to_lowercase();

    // TODO: save user or do any other db stuff

    let jwt_secret = "hello world";
    // TODO ^ move this to env file please

    let now = Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + Duration::minutes(10000)).timestamp() as usize;
    let claims: TokenClaims = TokenClaims {
        sub: "user id goes here".to_string(),
        exp,
        iat,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_ref()),
    )
    .unwrap();

    let mut response = HttpResponse::Ok();
    response.json(GoogleOauthResponse { token: token });
    response.finish()
}
