cfg_if::cfg_if! { if #[cfg(feature = "ssr")] {
use axum::{response::{IntoResponse, Response}, extract::Path, body::{self, Empty, Full}};
use http::{StatusCode, header, HeaderValue};
use include_dir::{include_dir, Dir};

static POSTS_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/posts");

// TODO: Sanitize input so that users cannot access CARGO_MANIFEST_DIR");
pub async fn get_post_file(Path(file_path): Path<String>) -> impl IntoResponse {
    let path = file_path.trim_start_matches('/');
    let mime_type = mime_guess::from_path(path).first_or_text_plain();

    match POSTS_DIR.get_file(path) {
        None => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(body::boxed(Empty::new()))
            .unwrap(),
        Some(file) => Response::builder()
            .status(StatusCode::OK)
            .header(
                header::CONTENT_TYPE,
                HeaderValue::from_str(mime_type.as_ref()).unwrap(),
            )
            .body(body::boxed(Full::from(file.contents())))
            .unwrap(),
    }
}
}}
