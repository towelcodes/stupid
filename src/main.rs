use actix_web::{App, HttpResponse, HttpServer, Responder, get, rt::net::TcpStream};
use std::env;

#[get("/")]
async fn root(body: String) -> impl Responder {
    // stream body to server
    let Ok(stream) = TcpStream::connect("127.0.0.1:1337").await else {
        return HttpResponse::InternalServerError();
    };
    if let Err(why) = stream.writable().await {
        println!("failed to wait for writable: {:?}", why);
        return HttpResponse::InternalServerError();
    }
    if let Err(why) = stream.try_write(body.as_bytes()) {
        println!("failed to send string: {:?}", why);
        return HttpResponse::InternalServerError();
    }
    if let Err(why) = stream.try_write(&[0u8]) {
        println!("failed to send terminator: {:?}", why);
        return HttpResponse::InternalServerError();
    }
    HttpResponse::Ok()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(root))
        .bind((
            env::var("HTTP_ADDRESS").unwrap_or("127.0.0.1".into()),
            env::var("HTTP_PORT")
                .unwrap_or("8080".into())
                .parse()
                .unwrap_or(8080),
        ))?
        .run()
        .await
}
