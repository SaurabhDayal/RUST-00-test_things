use actix_web::{get, HttpServer, App, HttpRequest};

#[get("/")]
async fn login(req: HttpRequest) -> String {
    let value = req.headers().get("keynation").unwrap().to_str().unwrap();
    format!("Request was sent at {}", value.to_string())
}

#[actix_web::main]
async fn main()-> std::io::Result<()> {

HttpServer::new(|| App::new()
.service(login))
.bind(("127.0.0.1", 8080))?
.run()
.await

}
