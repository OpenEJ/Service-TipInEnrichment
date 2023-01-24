use actix_cors::Cors; 
use actix_web::{error, web, App, HttpResponse, HttpServer, Responder};
use serde_json::Result;

mod models;
mod tip_in_enrichment;

async fn index() -> impl Responder {
    HttpResponse::Ok().body("tip-in-enrichment actix-web server is live!")
}

async fn receive_data(info: web::Json<Vec<models::Log>>) -> Result<impl Responder> {
    let obj_resp = tip_in_enrichment::begin(info.into_inner());
    return serde_json::to_string(&obj_resp);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        /*
        let cors = Cors::default()
              .allow_any_origin()
              .allow_any_header()
              .allow_any_method();
        */
        let json_config = web::JsonConfig::default()
            .limit(100000000) //100M limit
            .error_handler(|err, _req| {
                // create custom error response
                error::InternalError::from_response(err, HttpResponse::Conflict().finish())
                    .into()
            });
        App::new()
            //.wrap(cors)
            .service(
                web::resource("/api/analyze/2/")
                    .app_data(json_config)
                    .route(web::post().to(receive_data)),
            ).service(
                web::resource("/heartbeat/2/")
                    .route(web::get().to(index)),
            )
    })
    .bind(("0.0.0.0", 8002))?
    .run()
    .await
}

