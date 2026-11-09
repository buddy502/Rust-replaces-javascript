use tokio::net::{TcpListener};
use std::env;
use serde::Deserialize;
use meval;

use tower_http::services::ServeDir;

use axum::{
    Router,
    Form,
    routing::get,
    response::Html
};

const WEBSITEFILE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/website/index.html");

// calc is short for calculator
#[derive(Deserialize)]
struct CalcS {
    key: String,
    #[serde(default)]
    input: char,
    calcaction: Option<String>,
}

// calc is short for calculator
// simple calculator for html
async fn calculator(Form(mut calcform): axum::Form<CalcS>) -> Html<String> {
    // Access user response through a string
    // User can only type certain characters inside form

    // calc is short for calculator

    // brute force: iterate through string and find symbols in PEMDAS order
    // Why I am not doing this and gave up after thinking about it
    // 1. You need to iterate through each parenthesis and use a stack to evaluate each inner paren
    // 2. You need to evaulte each operation, which isn't hard but it's time consuming
    // 3. This isn't the purpose of the project
    // 4. Edge cases like divide by 0 and ending paren automatically closing, I wouldn't be
    //    satisfied without solving those

    match calcform.calcaction.as_deref() {
        Some("calcclear") => {
            calcform.key.clear();
            calcform.key.push('0');
        }
        Some("calcequal") => {
            calcform.key = match meval::eval_str(&calcform.key) {
                Ok(v) if v.is_finite() => v.to_string(),
                _ => String::from("Error"),
            };
        }
        _ => {
            // keep 0 at start of page
            if calcform.key == "0" {
                calcform.key.clear();
            }
            if calcform.input != '\0' {
                calcform.key.push(calcform.input);
            }
        }
    }

    let website = tokio::fs::read_to_string(WEBSITEFILE).await.unwrap();
    return Html(website
        .replace("{{calcsubmit}}", &calcform.key));

    // curly brace won't go down when indenting and it's really anoying bro
    }

async fn index() -> Html<String> {
    let website = tokio::fs::read_to_string(WEBSITEFILE).await.unwrap();
    Html(website
        .replace("{{calcsubmit}}", "0"))
}

#[tokio::main]
async fn main() {
    let router = Router::new()
        .route("/", get(index))
        .route("/calculator", get(index).post(calculator))
        .fallback_service(ServeDir::new(concat!(env!("CARGO_MANIFEST_DIR"), "/website")));
    let tcpconnection = TcpListener::bind("127.0.0.1:8080").await.unwrap();

    axum::serve(tcpconnection, router).await.unwrap();
}
