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

use web_sys::{Document, Window, console, HtmlElement, Element};

const CALCFILE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/website/calc.html");
const CHESSFILE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/website/chess.html");

#[derive(Deserialize, Default)]
struct ChessS {
    #[serde(default)]
    addr: String,

    folderinput: String,
}

/*
    get the folder input event 
*/
impl ChessS {
    fn new(addr: impl Into<String>) -> Self {
        Self {
            addr: addr.into(),
            ..Default::default()
        }
    }

    async fn chess_file() -> Html<String> {
        let website = tokio::fs::read_to_string(CHESSFILE).await.unwrap();
        Html(website
            .replace("{{folders}}", " "))
    }

    fn chess_router(&self) -> Router {
        Router::new()
            .route("/", get(ChessS::chess_file).post(folder_chess))
            .fallback_service(ServeDir::new(concat!(env!("CARGO_MANIFEST_DIR"), "/website")))
    }

    async fn chess_run(self) {
        let app = self.chess_router();
        let listener = tokio::net::TcpListener::bind(&self.addr).await.unwrap();
        axum::serve(listener, app).await.unwrap();
    }

    fn create_tile_colors() {
        // hmm lookin a lot like javascript, idk though
        let window = web_sys::window().expect("No window available");
        let doc = window.document().expect("Failed to get document");
        let tile_element: Element = doc.get_element_by_id("chessboard").ok_or("Element not found")
            .expect("Element tile error");

        let tile = doc.create_element("div");
    }
}

async fn folder_chess() -> Html<String> {
    // replace folders with folders
    Html(CHESSFILE.replace("{{folders}}", " "))
}


// calc is short for calculator
#[derive(Deserialize, Default)]
struct CalcS {
    #[serde(default)]
    addr: String,

    key: String,
    #[serde(default)]
    input: char,
    calcaction: Option<String>,
}

// wasm wasn't used here
impl CalcS {
    fn new(addr: impl Into<String>) -> Self {
        Self {
            addr: addr.into(),
            ..Default::default()
        }
    }

    async fn calc_file() -> Html<String> {
        let website = tokio::fs::read_to_string(CALCFILE).await.unwrap();
        Html(website
            .replace("{{calcsubmit}}", "0"))
    }

    fn calc_router(&self) -> Router {
        Router::new()
            .route("/", get(CalcS::calc_file))
            .route("/calculator", get(CalcS::calc_file).post(calculator))
            .fallback_service(ServeDir::new(concat!(env!("CARGO_MANIFEST_DIR"), "/website")))
    }

    async fn calc_run(self) {
        let app = self.calc_router();
        let listener = tokio::net::TcpListener::bind(&self.addr).await.unwrap();
        axum::serve(listener, app).await.unwrap();
    }
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

    let website = tokio::fs::read_to_string(CALCFILE).await.unwrap();
    return Html(website.replace("{{calcsubmit}}", &calcform.key));

}


#[tokio::main]
async fn main() {
    //CalcS::new("127.0.0.1:8080").calc_run().await;
    TreeS::new("127.0.0.1:8080").chess_run().await;
}
