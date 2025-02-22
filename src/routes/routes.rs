use actix_web::{ get, http::header::{ ContentDisposition, DispositionType }, web, HttpRequest, HttpResponse };
use actix_files::NamedFile;
use rustube::{ Id, Video };
use crate::utils::utils::{Error, Info, Query};
use youtube_dl::YoutubeDl;
use std::path::PathBuf;



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

    let id = match Id::from_raw(&info.data) {
        Ok(id) => id.into_owned(),
        Err(_) => return Ok(HttpResponse::BadRequest().body("Invalid YouTube URL")),
    };

    let video = match Video::from_id(id).await {
        Ok(video) => video,
        Err(e) => return Ok(HttpResponse::InternalServerError().body(format!("Failed to fetch video: {:?}", e))),
    };

    let path_to_video = match video.best_quality().unwrap().download().await {
        Ok(path) => path,
        Err(e) => return Ok(HttpResponse::InternalServerError().body(format!("Download failed: {:?}", e))),
    };

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(format!("Downloaded video: {:?}", path_to_video)))
}
#[get("/download/yt")]
pub async fn download_yt(req: HttpRequest, query: web::Query<Query>) -> Result<HttpResponse, Error> {
    let yt_dl_path = ""; //yt-dlp.exe directory
    let out_dir = ""; // output directory

    println!("Downloading video: {:?}", query.data);

    let output = YoutubeDl::new(&query.data)
        .youtube_dl_path(yt_dl_path)
        .output_template("downloads")
        .socket_timeout("20")
        .run();

    match output {
        Ok(video) => {
            if let Some(single_video) = video.into_single_video() {
                let title = single_video.title.unwrap_or("Unknown".to_string());
                let file_name = format!("{}.mp4", title);
                let file_path = format!("{}/{}", out_dir, file_name);

                println!("Download completed: {}", file_path);

                let path: PathBuf = file_path.into();
                match NamedFile::open(path) {
                    Ok(file) => Ok(file
                        .use_last_modified(true)
                        .set_content_disposition(ContentDisposition {
                            disposition: DispositionType::Attachment,
                            parameters: vec![],
                        })
                        .into_response(&req)),
                    Err(_) => Ok(HttpResponse::InternalServerError().body("File not found")),
                }
            } else {
                Ok(HttpResponse::InternalServerError().body("Failed to get video info."))
            }
        }
        Err(e) => Ok(HttpResponse::InternalServerError().body(format!("Error: {:?}", e))),
    }
}
