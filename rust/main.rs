use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use image::{DynamicImage, ImageFormat};
use std::io::Cursor;
use std::time::Instant;

async fn compress_image(req: HttpRequest, body: web::Bytes) -> impl Responder {
    let start = Instant::now();
    
    let img = match image::load_from_memory(&body) {
        Ok(img) => img,
        Err(_) => return HttpResponse::BadRequest().body("Invalid image"),
    };
    
    let mut buf = Cursor::new(Vec::new());
    if img.write_to(&mut buf, ImageFormat::Jpeg).is_err() {
        return HttpResponse::InternalServerError().body("Compression failed");
    }
    
    let duration = start.elapsed();
    println!("Compression took: {:?}", duration);
    
    HttpResponse::Ok().content_type("image/jpeg").body(buf.into_inner())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().route("/compress", web::post().to(compress_image))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
