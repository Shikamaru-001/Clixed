use axum::{
    Router,
    body::Body,
    extract::{Multipart, Path},
    http::{StatusCode, header},
    response::{IntoResponse, Response},
    routing::{get, post},
};
use std::{io::Write, sync::Arc};
use tera::Tera;
use tokio::fs::File;
use tokio_util::io::ReaderStream;
use uuid::Uuid;

pub fn routes() -> Router<Arc<Tera>> {
    Router::new()
        .route("/upload", post(upload))
        .route("/images/{filename}", get(serve_image))
}

async fn upload(mut multipart: Multipart) -> impl IntoResponse {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let file_name = field.file_name().unwrap_or("upload.jpg").to_string();

        // 3. Fixed content type validation
        let content_type = field.content_type().map(|s| s.to_string());
        if let Some(ref ct) = content_type {
            if ct != "image/jpeg" {
                return (
                    StatusCode::UNSUPPORTED_MEDIA_TYPE,
                    format!("Only JPEG images are supported, you sent me: {ct}"),
                );
            }
        } else {
            return (StatusCode::BAD_REQUEST, "Missing content type".to_string());
        }
        println!("{}", file_name);
        let data = field.bytes().await.unwrap();
        let unique_name = format!(
            "images/{}_{}",
            Uuid::new_v4(),
            sanitize_filename::sanitize(&file_name) // 4. Added sanitization
        );

        // 5. Proper error handling for directory creation
        if let Err(e) = std::fs::create_dir_all("images") {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to create images directory: {}", e),
            );
        }

        // 6. Proper error handling for file operations
        match std::fs::File::create(&unique_name) {
            Ok(mut file) => {
                if let Err(e) = file.write_all(&data) {
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Failed to write file: {}", e),
                    );
                }
                tracing::info!("File saved '{}' ({} bytes)", unique_name, data.len());
                return (StatusCode::OK, format!("File saved as {}", unique_name));
            }
            Err(e) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Failed to create file: {}", e),
                );
            }
        }
    }
    (
        StatusCode::BAD_REQUEST,
        "No file field found in multipart body".to_string(),
    )
}

async fn serve_image(Path(filename): Path<String>) -> Result<Response, StatusCode> {
    let file_path = format!("./images/{}", filename);

    match File::open(&file_path).await {
        Ok(file) => {
            let stream = ReaderStream::new(file);
            let body = Body::from_stream(stream);

            let content_type = match file_path.split('.').last() {
                Some("jpg") | Some("jpeg") => "image/jpeg",
                Some("png") => "image/png",
                Some("gif") => "image/gif",
                _ => "application/octet-stream",
            };

            Response::builder()
                .header(header::CONTENT_TYPE, content_type)
                .body(body)
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
        }
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}
