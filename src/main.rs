use std::io::{Read, Write};
use std::net::TcpStream;

use actix_web::web::{BufMut, Bytes, BytesMut};
use actix_web::{App, HttpResponse, HttpServer, Responder, post};

#[post("/")]
async fn root(bytes: Bytes) -> impl Responder {
    println!("body: {:?}", bytes);

    let mut stream = TcpStream::connect("127.0.0.1:1337").unwrap();

    let mut m = BytesMut::with_capacity(bytes.len() + 1);
    m.extend_from_slice(&bytes);
    m.put_u8(0x00);
    stream.write_all(&m).unwrap();

    let mut buf: Vec<u8> = vec![];
    let n = stream.read(&mut buf).unwrap();
    println!("got {} bytes: {:?}", n, &buf[..n]);

    HttpResponse::Ok().body(String::from_utf8(buf).unwrap_or("".into()))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(root))
        .bind(("127.0.0.1", 8081))?
        .run()
        .await
}
