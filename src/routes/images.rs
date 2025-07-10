use axum::{
    body::Body, extract::{Multipart, Path, State}, http::{header, StatusCode}, response::{Html, IntoResponse, Response}, routing::{get, post}, Router
};
use tokio::fs::File;
use std::{fs, io::Write, sync::Arc};
use tera::{Context, Tera};
use tokio_util::io::ReaderStream;
use uuid::Uuid;

pub fn routes() -> Router<Arc<Tera>> {
    Router::new()
        .route("/upload", post(upload))
        .route("/images", get(images_home))
        .route("/gallery", get(serve_image_gallery))
        .route("/images/{filename}", get(serve_image))
}

async fn upload(mut multipart: Multipart) -> impl IntoResponse {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let file_name = field.file_name().unwrap_or("upload.jpg").to_string();

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
            sanitize_filename::sanitize(&file_name) 
        );

        if let Err(e) = std::fs::create_dir_all("images") {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to create images directory: {}", e),
            );
        }

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
    
async fn images_home(State(tera): State<Arc<Tera>>) -> impl IntoResponse {
    let ctx = Context::new();
    // 8. Added proper error handling for template rendering
    match tera.render("home.html", &ctx) {
        Ok(rendered) => Html(rendered),
        Err(e) => {
            tracing::error!("Template error: {}", e);
            Html(format!("Template error: {}", e).into())
        }
    }
}
async fn serve_all_images() -> Result<Vec<String>, std::io::Error>
{
    let mut images = Vec::new();
    let entries =   fs::read_dir("images")?;

    for entry in entries{
       let entry = entry?;
       let path = entry.path();

       if path.is_file() {
           if let Some(file_name) = path.file_name().and_then(|s| s.to_str()){
               images.push(file_name.to_string());
           }
       }
    }   
    Ok(images)
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
async fn serve_image_gallery() -> Result<Html<String>, StatusCode> {
    match serve_all_images().await {
        Ok(images) => {
            let mut html = String::from(r#"
                <div id="image-gallery" class="row">
            "#);
            
            for image in images {
                html.push_str(&format!(r#"
                    <div class="col-md-6 col-sm-12 col-xl-4">
                        <img src="/images/{}" alt="{}" class="img-thumbnail" 
                             onclick="openModal(this.src, this.alt)">
                        <div class="p-3">
                            <p class="text-sm text-gray-600 truncate">{}</p>
                        </div>
                    </div>
                "#, image, image, image));
            }
            
            html.push_str("</div>");
            Ok(Html(html))
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
