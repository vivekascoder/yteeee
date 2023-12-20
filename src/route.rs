use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result};
use anyhow::bail;
use chrono::{prelude::*, Duration};
use diesel::OptionalExtension;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use jsonwebtoken::{encode, EncodingKey, Header};
use log::info;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::env;
use uuid::Uuid;

use crate::db::{establish_connection, Task};
use crate::google_oauth::{get_google_user, request_token};
use crate::schema::tasks;
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
    TaskSpwaned(String),
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

#[get("/task/{uuid}")]
pub async fn get_task_details(path: web::Path<String>) -> Result<impl Responder> {
    use crate::schema::tasks::dsl::*;

    let conn = &mut establish_connection();
    let uuid = path.into_inner();
    let task: Option<Task> = tasks
        .find(&uuid)
        .select(Task::as_select())
        .first(conn)
        .optional()
        .expect("error while getting task");
    println!("p: {}, task: {:?}", &uuid, &task);
    Ok(web::Json(json!(task)))
}

#[post("/summarize_video")]
pub async fn summarize_video(data: web::Json<GetSubtitleReq>) -> Result<impl Responder> {
    info!("Got req., {:?}", data);

    //
    // Spawn a thread for now.
    //
    let task = Task {
        id: Uuid::new_v4().to_string(),
        youtube_url: data.video_url.clone(),
        status: "start".to_string(),
        result: None,
    };
    let task_cloned = task.clone();
    tokio::task::spawn(async move {
        let task = task_cloned;
        use crate::schema::tasks::dsl::*;
        // Download the subtitle using yt-dlp.
        info!("task: starting to download subs.");

        // Create new task
        let conn = &mut establish_connection();
        diesel::insert_into(tasks)
            .values(&task)
            .execute(conn)
            .expect("Error saving new post");

        let text =
            yt_dlp::YtDlp::download_subtitle(&data.video_url).expect("can't download the video.");
        info!("task: downloaded subs.");

        // diesel::update(posts).set(draft.eq(false)).execute(conn)
        diesel::update(tasks)
            .filter(id.eq(&task.id))
            .set(status.eq("Subtitles downloaded".to_string()))
            .execute(conn);

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
            let formatted_result = ChatGptCompletionResFormatted {
                result: output
                    .choices
                    .iter()
                    .map(|v| v.message.content.clone())
                    .collect::<Vec<String>>(),
            };
            diesel::update(tasks)
                .filter(id.eq(&task.id))
                .set((
                    status.eq("summarized".to_string()),
                    result.eq(Some(serde_json::to_string(&formatted_result).unwrap())),
                ))
                .execute(conn);
            info!("this is the summary of the video: {:?}", output);

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
        // return Ok(web::Json(SummarizeVideoRes {
        //     status: false,
        //     data: SummarizeVideoResData::SummarizeVideoResError("can't fetch".to_string()),
        // }));
    });

    return Ok(web::Json(SummarizeVideoRes {
        status: true,
        data: SummarizeVideoResData::TaskSpwaned(task.id),
    }));
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
