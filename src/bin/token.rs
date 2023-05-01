use rand::distributions::{Alphanumeric, DistString};
use pwhash::bcrypt;

use hyper::header::{Headers, Authorization, Bearer};

use actix_web::{post, web, App, HttpServer, Responder, Result, http::{header::{ContentType}}, HttpResponse};
use serde::{Serialize, Deserialize};

#[derive(Serialize,Deserialize)]
struct MyObj {
    name: String,
    pwd: String
}

#[post("/login/{name}/{pwd}")]
async fn login(info: web::Path<MyObj>) -> Result<impl Responder> {

    let obj = MyObj {
        name: info.name.clone(),
        pwd: bcrypt::hash(info.pwd.clone()).unwrap()
    };

    let token = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);
    
    let mut headers = Headers::new();
    headers.set(
       Authorization(
           Bearer {
               token: token.to_owned()
           }
       )
    );

    Ok( HttpResponse::Ok()
    .content_type(ContentType::plaintext())
    .insert_header(("Authorization", token))
    .body(serde_json::to_string(&obj).unwrap()))
}

#[actix_web::main]
async fn main()-> std::io::Result<()> {

// let string = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);
// println!("{}", string);

HttpServer::new(|| App::new()
.service(login))
.bind(("127.0.0.1", 8080))?
.run()
.await

}