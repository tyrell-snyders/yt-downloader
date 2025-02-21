mod handles;
mod utils;
mod routes;
use handles::handles::video_info;
use routes::routes::download_video;
use actix_web::{ web, App, HttpResponse, HttpServer, error };


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
                web::resource("/")
                    .app_data(json_config)
                    .route(web::get().to(video_info))
            )
            .service(download_video)
    }).bind(("127.0.0.1", 3000))?
    .run()
    .await
}
