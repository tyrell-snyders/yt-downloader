use actix_web::{ get, web, HttpResponse };
use rustube::{ Id, Video };
use crate::utils::utils::{Info, Error};


/// This function handles the download of a video from YouTube based on the provided video information.
///
/// # Parameters
///
/// * `info` - A JSON object containing the video information. The `Info` struct is expected to have a field `data`
///   which contains the YouTube video ID as a string.
///
/// # Return
///
/// * `Result<HttpResponse, Error>` - On success, returns an `HttpResponse` with a status code of 200 and a JSON
///   body containing the message "Downloaded video: {path_to_video}". On failure, returns an `Error`.
///
/// # Example
///
/// ```rust
/// use actix_web::{ get, web, HttpResponse };
/// use rustube::{ Id, Video };
/// use crate::utils::utils::{Info, Error};
///
/// #[get("/download")]
/// pub async fn download_video(info: web::Json<Info>) -> Result<HttpResponse, Error> {
///     println!("Downloading video: {:?}", info.data);
///
///     let id = Id::from_str(&info.data)?;
///     let video = Video::from_id(id.into_owned()).await?;
///     let path_to_video = video.worst_audio().unwrap().download().await?;
///
///     Ok(HttpResponse::Ok()
///         .content_type("application/json")
///         .body(format!("Downloaded video: {:?}", path_to_video)))
/// }
/// ```
#[get("/download")]
pub async fn download_video(info: web::Json<Info>) -> Result<HttpResponse, Error> {
    println!("Downloading video: {:?}", info.data);

    let id = Id::from_str(&info.data)?;
    let video = Video::from_id(id.into_owned()).await?;
    let path_to_video = video.worst_audio().unwrap().download().await?;

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(format!("Downloaded video: {:?}", path_to_video)))
}

