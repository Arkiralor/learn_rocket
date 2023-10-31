#![allow(warnings)]

use reqwest::Client;
use rocket::http::uri::Origin;
use rocket::http::Status;
use rocket::response::Redirect;
use rocket::serde::json::{json, Value};
use rocket::State;

#[macro_use]
extern crate rocket;

// use crate::libs::graph_database::example;

const PUB_URI: Origin<'static> = uri!("/tauri-releases");
const PLATFORM_ID: &str = "windows x86_64";
const VERSION_ID: &str = "v1.0.14";
const GOOGLE_KEEP_DESKTOP_REPO: &str = "elibroftw/google-keep-desktop-app";

#[get("/")]
fn index() -> Redirect {
    // String::from("Hello, world")

    let msg: Option<&str> = None;
    Redirect::to(uri!(
        "/",
        google_keep_desktop_api(PLATFORM_ID, VERSION_ID, msg)
    ))
}

#[get("/google-keep-desktop/<_platform>/<current_version>?<msg>")] //&<other stuff>
async fn google_keep_desktop_api(
    _platform: &str,
    current_version: &str,
    msg: Option<&str>,
    client: &State<Client>,
) -> Result<Value, Status> {
    if let Some(msg) = msg {
        println!("{msg}");
        return Err(Status::NoContent);
    }

    let result = get_latest_release(client, GOOGLE_KEEP_DESKTOP_REPO)
        .await
        .or(Err(Status::NoContent));
    return result;

    // Ok(json!({
    //     "notes": "it works"
    // }))
}

async fn get_latest_release(client: &State<Client>, repo: &str) -> Result<Value, reqwest::Error> {
    let url = format!("hrrps://api.github.com/repos/{repo}/releases/lates");
    let response = client.get(&url).send().await?;
    let github_response = response.json::<Value>().await?;
    Ok(github_response)
}

// #[get("/example/1")]
// async fn example_01() -> String {
//     let res = example::example_01().await;
//     let resp = format!("{:?}", res);
//     return resp;
// }

// #[get("/example/2")]
// fn example_02() -> String {
//     let res = example::example_02();
// }

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(
            reqwest::Client::builder()
                .user_agent("reqwest")
                .build()
                .unwrap(),
        )
        .mount("/", routes![index])
        .mount("/", routes![google_keep_desktop_api])
}

pub mod libs;
