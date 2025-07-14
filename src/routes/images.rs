use axum::{
    body::Body,
    extract::{Multipart, Path, State},
    http::{header, StatusCode}, 
    response::{Html, IntoResponse, Response},
    routing::{delete, get, post},
    Router
};
use mozjpeg::{ColorSpace, Compress};
use std::{fs, io::Write, sync::Arc};
use tera::{Context, Tera};
use tokio::fs::File;
use tokio_util::io::ReaderStream;
use uuid::Uuid;

const IMAGE_SIZE: usize = 40;

pub fn routes() -> Router<Arc<Tera>> {
    Router::new()
        .route("/image/{filename}", delete(delete_image))
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
        let compressed_image = std::panic::catch_unwind(|| -> std::io::Result<Vec<u8>> {
            let mut comp = Compress::new(ColorSpace::JCS_RGB);
            comp.set_size(IMAGE_SIZE, IMAGE_SIZE);
            let mut comp = comp.start_compress(data.to_vec())?;
            let pixels = vec![0u8; IMAGE_SIZE * IMAGE_SIZE * 3];
            comp.write_scanlines(&pixels[..])?;
            let writer = comp.finish()?;
            Ok(writer)
        });

        match compressed_image {
            Ok(Ok(compressed_data)) => {
                let output_path = std::path::Path::new(&unique_name);
                match std::fs::File::create(output_path)
                    .and_then(|mut file| file.write_all(&compressed_data))
                {
                    Ok(_) => return (StatusCode::OK, format!("file saved as {}", unique_name)),
                    Err(e) => {
                        return (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            format!("failed to save image {}", e),
                        );
                    }
                }
            }
            Ok(Err(e)) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("failed compressing image {}", e),
                );
            }
            Err(_) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Panic!! image compression"),
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
    match tera.render("home.html", &ctx) {
        Ok(rendered) => Html(rendered),
        Err(e) => {
            tracing::error!("Template error: {}", e);
            Html(format!("Template error: {}", e).into())
        }
    }
}

async fn serve_all_images() -> Result<Vec<String>, std::io::Error> {
    let mut images = Vec::new();
    let entries = fs::read_dir("images")?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            if let Some(file_name) = path.file_name().and_then(|s| s.to_str()) {
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
            let mut html = String::from(
                r#"
                <div id="image-gallery" class="row">
            "#,
            );

            for image in images {
                html.push_str(&format!(
                    r#"
                    <div class="col-md-6 col-sm-12 col-xl-4">
                        <img src="/images/{}" alt="{}" class="img-thumbnail" 
                             onclick="openModal(this.src, this.alt)">
                        <div class="p-3">
                            <p class="text-sm text-gray-600 truncate">{}</p>
                        </div>
                    </div>
                "#,
                    image, image, image
                ));
            }

            html.push_str("</div>");
            Ok(Html(html))
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn delete_image(Path(filename): Path<String>) -> Result<Html<String>, StatusCode> {
    let file_path = format!("./images/{}", filename);
    match std::fs::remove_file(file_path) {
        Ok(_) => todo!(),
        Err(_) => todo!(),
    }
}
