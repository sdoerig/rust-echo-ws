use actix_web::{error, Error, web, FromRequest, HttpResponse, Responder};
use serde::{Deserialize,Serialize};
use chrono::{Utc};
use futures::stream::once;
use futures::stream::Once;
use bytes::Bytes;

#[derive(Deserialize)]
struct EchoRequest {
    username: String,
}

#[derive(Deserialize)]
struct EchoRequestPath {
    id: u32
}

#[derive(Serialize)]
struct EchoResponse<'a> {
    id: u32,
    username: &'a str,
    echoed_at: String
}

impl<'a> EchoResponse<'a> {
    pub fn new(id: u32,username: &'a str) -> Self {
        EchoResponse{ id: id, username: username, 
        echoed_at: Utc::now().to_rfc2822()}
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
fn index((path, req): (web::Path<EchoRequestPath>, web::Json<EchoRequest>)) -> impl Responder {
    let resp = EchoResponse::new(path.id, &req.username );
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