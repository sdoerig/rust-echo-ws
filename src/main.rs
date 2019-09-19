use actix_web::{error, Error, web, FromRequest, HttpResponse, Responder};
use serde::{Deserialize,Serialize};
use chrono::{DateTime, Utc};
use futures::stream::once;
use futures::stream::Once;
use bytes::Bytes;

#[derive(Deserialize)]
struct EchoRequest {
    username: String,
}

#[derive(Serialize)]
struct EchoResponse<'a> {
    jaco123: &'a str,
    echoedAt: String
}

impl<'a> EchoResponse<'a> {
    pub fn new(jaco123: &'a str) -> Self {
        EchoResponse{ jaco123: jaco123, 
        echoedAt: Utc::now().to_rfc2822()}
    }

    pub fn to_json(&self) -> Once<Bytes, Error> {
        let body = match serde_json::to_string(self) {
            Ok(_json) => once::<Bytes, Error>(Ok(Bytes::from( _json.as_bytes() ))),
            Err(_e) => once::<Bytes, Error>(Ok(Bytes::from( "error".as_bytes() ))),
        };
        body
    }
}

/// deserialize `Info` from request's body, max payload size is 4kb
fn index(req: web::Json<EchoRequest>) -> impl Responder {
    let resp = EchoResponse::new( &req.username );
    //format!("Welcome {}!", req.username)
    HttpResponse::Ok()
        .content_type("application/json")
        .streaming(Box::new(resp.to_json()))
}

pub fn main() {
    use actix_web::{App, HttpServer};

    HttpServer::new(|| {
        App::new().service(
            web::resource("/ibis/rest/rc/setTaskLock/{id}/true")
                .data(
                    // change json extractor configuration
                    web::Json::<EchoRequest>::configure(|cfg| {
                        cfg.limit(4096).error_handler(|err, _req| {
                            // <- create custom error response
                            error::InternalError::from_response(
                                err,
                                HttpResponse::Conflict().finish(),
                            )
                            .into()
                        })
                    }),
                )
                .route(web::post().to(index)),
        )
    })
    .bind("127.0.0.1:8088")
    .unwrap()
    .run()
    .unwrap();
}