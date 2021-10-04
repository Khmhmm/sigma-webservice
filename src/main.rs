use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
mod sigma_frontend;
use sigma_frontend::{FrontendData, ReadFrontend};
use lazy_static::lazy_static;
use openssl::ssl::*;

lazy_static! {
    static ref GLOBAL_FRONTEND: FrontendData = ReadFrontend::create_data();
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // create it by openssl:
    // openssl req -x509 -newkey rsa:4096 -nodes -keyout key.pem -out cert.pem -days 365 -subj '/CN=localhost'
    // note: use doubleslash with -subj: '//CN=localhost'
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("key.pem", SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();

    HttpServer::new(|| {
        App::new()
            .service(index::get_index)
            .service(index::get_index_css)
            .service(index::get_index_js)
            .service(login::get_login)
            .service(login::get_login_style)
            .service(login::get_login_script)
            .service(login::try_login)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind_openssl("127.0.0.1:8080", builder)?
    .run()
    .await
}


mod index {
    use actix_web::{get, post, web, HttpResponse, Responder};
    use super::sigma_frontend::{ReadFrontend, FrontendData};
    use std::ops::Deref;
    use super::GLOBAL_FRONTEND;

    #[get("/")]
    pub async fn get_index() -> impl Responder {
        HttpResponse::Ok().body(GLOBAL_FRONTEND.get_index().unwrap().html.deref())
    }

    #[get("/style.css")]
    pub async fn get_index_css() -> impl Responder {
        if let Some(style) = &GLOBAL_FRONTEND.get_index().unwrap().css {
            HttpResponse::Ok().body(style.deref())
        } else {
            HttpResponse::NoContent().finish()
        }
    }

    #[get("script.js")]
    pub async fn get_index_js() -> impl Responder {
        if let Some(script) = &GLOBAL_FRONTEND.get_index().unwrap().js {
            HttpResponse::Ok().body(script.deref())
        } else {
            HttpResponse::NoContent().finish()
        }
    }
}


mod login {
    use actix_web::{get, post, web, HttpResponse, HttpRequest, Responder};
    use super::sigma_frontend::FrontendData;
    use std::ops::Deref;
    use super::GLOBAL_FRONTEND;
    use serde::{Serialize, Deserialize};
    use scrypt::{
        password_hash::{ rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString },
        Scrypt
    };

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Credentials{
        u: String,
        p: String
    }

    #[get("/login")]
    pub async fn get_login() -> impl Responder {
        if let Some(html) = &GLOBAL_FRONTEND.get_login().as_ref().and_then(|bts| Some(&bts.html)) {
            HttpResponse::Ok().body(bytes::Bytes::deref(html))
        } else {
            HttpResponse::NoContent().finish()
        }
    }

    #[get("/login/style.css")]
    pub async fn get_login_style() -> impl Responder {
        if let Some(Some(css)) = &GLOBAL_FRONTEND.get_login().as_ref().and_then(|bts| Some(bts.css.as_ref())) {
            HttpResponse::Ok().body(bytes::Bytes::deref(css))
        } else {
            HttpResponse::NoContent().finish()
        }
    }

    #[get("/login/script.js")]
    pub async fn get_login_script() -> impl Responder {
        if let Some(Some(js)) = &GLOBAL_FRONTEND.get_login().as_ref().and_then(|bts| Some(bts.js.as_ref())) {
            HttpResponse::Ok().body(bytes::Bytes::deref(js))
        } else {
            HttpResponse::NoContent().finish()
        }
    }

    #[post("/api/login")]
    pub async fn try_login(req: web::Json<Credentials>) -> impl Responder {
        println!("{:?}", req);
        let user_pass = (&req.u).to_owned() + &req.p;
        let salt = SaltString::generate(&mut OsRng);
        let password_hash = Scrypt.hash_password(user_pass.as_bytes(), &salt).unwrap().to_string();

        HttpResponse::NoContent().finish()
    }
}