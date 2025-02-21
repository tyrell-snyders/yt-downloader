mod handles;
mod utils;
mod routes;
use handles::handles::video_info;
use routes::routes::{download_video, download_yt};
use actix_web::{ error, web, App, HttpResponse, HttpServer, Responder };


async fn welcome() ->  impl Responder {
    HttpResponse::Ok().body("Welcome to the YouTube Downloader API!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let json_config = web::JsonConfig::default()
            .limit(4096)
            .error_handler(|err, _req| {
                error::InternalError::from_response(err, HttpResponse::Conflict().finish())
                    .into()
            });
        App::new()
            .service(
                web::resource("/video_info")
                    .app_data(json_config)
                    .route(web::get().to(video_info))
            )
            .service(
                web::resource("/")
                .route(web::get().to(welcome))
            )       
            .service(download_video)
            .service(download_yt)
    }).bind(("127.0.0.1", 3000))?
    .run()
    .await
}
