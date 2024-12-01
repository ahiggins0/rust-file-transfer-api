use axum::{
    extract::Path,
    response::{IntoResponse, Response},
    routing::{get},
    http::{header, StatusCode},
    Json, Router,
    body::Body,
};
use tokio::fs::File;
use tokio_util::io::ReaderStream;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{PathBuf};
use std::env;

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `GET /list-dir` goes to `list_dir`
        .route("/list-dir", get(list_dir))
        .route("/download/:file_path", get(download_file));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}

// handler to list the contents of a directory
async fn list_dir() -> Result<Json<Vec<String>>, StatusCode> {
    // Change this path to the directory you want to list
    let base_path = env::var("FILE_DIR").expect("FILE_DIR environment variable must be set");
    let path = PathBuf::from(base_path);

    if path.is_dir() {
        let entries = fs::read_dir(path)
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .filter_map(|entry| entry.ok()) // filter out errors
            .filter_map(|entry| entry.path().to_str().map(String::from)) // convert paths to Strings
            .collect::<Vec<String>>();
        
        Ok(Json(entries))
    } else {
        Err(StatusCode::BAD_REQUEST)
    }
}

// handler to serve files for download
async fn download_file(Path(file_path): Path<String>) -> impl IntoResponse {
    let base_path = env::var("FILE_DIR").expect("FILE_DIR environment variable must be set");
    let full_path = PathBuf::from(base_path).join(&file_path);

    if full_path.is_file() {
        match File::open(&full_path).await {
            Ok(file) => {
                // Create a stream from the file
                let stream = ReaderStream::new(file);
                let body = Body::from_stream(stream);

                // Build the response
                Response::builder()
                    .status(StatusCode::OK)
                    .header(header::CONTENT_TYPE, "application/octet-stream")
                    .header(
                        header::CONTENT_DISPOSITION,
                        format!("attachment; filename=\"{}\"", file_path),
                    )
                    .body(body)
                    .unwrap()
            }
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    } else {
        StatusCode::NOT_FOUND.into_response()
    }
}
