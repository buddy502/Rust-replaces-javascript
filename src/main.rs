use tokio::net::{TcpListener};
use std::env;
use serde::Deserialize;

use axum::{
    Router,
    routing::get,
    routing::post,
    response::Html
};

const WEBSITEFILE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/website/index.html");

// calc is short for calculator
#[derive(Deserialize)]
struct CalcS {
    input: String,
}
// calc is short for calculator
const CALC_LETTERS: &[u8] = b"+-*/1234567890";
// simple calculator for html
async fn calculator(calcform: axum::Form<CalcS>) -> Html<String> {
    // Access user response through a string
    // User can only type certain characters inside form

   // let character_map: String = CALC_LETTERS.iter()
   //     .map();

    // !!!  Can't find a way to make the pattern with regex
    //      You're still able to type anything, just not submit it      
    let calculatorhtml: &str = r#"
        <input id=calcinput name="input" pattern="[0-9]{6}"></input>
        "#;

    // !edit calcform.input to be users input

    let calcanswer: String = format!(r#"
        <p id=calcanswe>{}</input>"#,
        calcform.input);

    // divide by 0 is bad :(

    let website = tokio::fs::read_to_string(WEBSITEFILE).await.unwrap();
    return Html(website
        .replace("{{calculator}}", calculatorhtml)
        .replace("{{answer}}", &calcanswer));
}

async fn index() -> Html<String> {
    let website = tokio::fs::read_to_string(WEBSITEFILE).await.unwrap();
    Html(website
        .replace("{{calculator}}", r#"<input id="calcinput" name="input">"#)
        .replace("{{answer}}", ""))
}

#[tokio::main]
async fn main() {
    let router = Router::new()
        .route("/", get(index))
        .route("/calculator", get(index).post(calculator));
    let tcpconnection = TcpListener::bind("127.0.0.1:8080").await.unwrap();

    axum::serve(tcpconnection, router).await.unwrap();
}
