use rustube::{ Id, VideoFetcher };
use actix_web::{ web, HttpResponse };
use crate::utils::utils::{Info, VideoResponse, Error };

/// Fetches video information from YouTube using the provided video ID.
///
/// # Parameters
///
/// * `info` - A JSON object containing the video ID to fetch information for.
///
/// # Returns
///
/// * `Result<HttpResponse, Error>` - On success, returns an HTTP response containing the video information in JSON format.
///   On error, returns an error response.
pub async fn video_info(info: web::Json<Info>) -> Result<HttpResponse, Error> {
    let id = Id::from_raw(&info.data)?;

    let id_owned = id.clone().into_owned(); // Clone before moving
    let descrambler = VideoFetcher::from_id(id_owned)?.fetch().await?;
    let video_details = descrambler.video_details().clone(); // Clone to avoid borrowing issues

    let response = VideoResponse {
        id: id.as_str().to_string(), // Use original `id`
        details: video_details,
    };
    let json_response = serde_json::to_string(&response)?;

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(json_response))
}

