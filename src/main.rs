use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
mod sigma_frontend;
mod data;
mod config;

use config::Config;
use data::{DBConnection, PostgresEnvelop};
use sigma_frontend::{FrontendData, ReadFrontend};
use lazy_static::lazy_static;
use openssl::ssl::*;

lazy_static! {
    static ref GLOBAL_FRONTEND: FrontendData = ReadFrontend::create_data();
    static ref CFG: Config = if let Ok(cfg) = Config::from_file("Config.json") {
        cfg
    } else {
        let def_cfg: String = Config::default().generate();
        std::fs::write("Config.json", def_cfg).unwrap();
        panic!("**\tPlease edit Config.json before running!\t**");
    };
    static ref DB_CONNECTION: PostgresEnvelop = PostgresEnvelop::init(&CFG).unwrap();
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

    // TODO: add business logic layer

    HttpServer::new(move || {
        App::new()
            .service(index::get_index).service(index::get_index_css).service(index::get_index_js)
            .service(login::get_login).service(login::get_login_style).service(login::get_login_script)
            .service(login::signup).service(login::login)
            .service(cabinet::get_cabinet_index).service(cabinet::get_cabinet_style).service(cabinet::get_cabinet_script)
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
    use super::{GLOBAL_FRONTEND, DB_CONNECTION, data::DBConnection, CFG};
    use serde::{Serialize, Deserialize};
    use scrypt::{
        password_hash::{ rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString, Encoding },
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

    #[post("/api/signup")]
    pub async fn signup(req: web::Json<Credentials>) -> impl Responder {
        if &req.u == "" || &req.p == "" {
            return HttpResponse::BadRequest().finish();
        }

        println!(" signup {:?}", req);
        // TODO: store these strings in static vars
        let q = r##"SELECT public."existsUserByName"('"##.to_owned() + &req.u + r##"');"##;
        let is_existing_user: bool = DB_CONNECTION.query_get(&q, &[]).unwrap().get(0);
        if is_existing_user {
            return HttpResponse::Forbidden().finish();
        }


        let user_pass = (&req.u).to_owned() + &req.p;
        let salt = SaltString::generate(&mut OsRng);
        let password_hash = Scrypt.hash_password(user_pass.as_bytes(), &salt).unwrap().to_string();

        let result_id = DB_CONNECTION.query_edit(
            &format!(r##"CALL public."insertUser"('{}','{}','{}');"##,req.u,password_hash,salt.as_str()), &[]
        ).unwrap();

        HttpResponse::Ok().body(&password_hash)
    }

    #[post("/api/login")]
    pub async fn login(req: web::Json<Credentials>) -> impl Responder {
        println!("Login attempt: {}", req.u);
        let q = format!(r##"SELECT salt, hash from public."getSaltAndHash"('{}');"##, req.u);
        let salt_and_hash = DB_CONNECTION.query_get_each(
            &q, &[]
        ).unwrap();

        if let Some(salt_and_hash) = salt_and_hash.get(0).as_ref() {
            let stored_hash: String = salt_and_hash.get(1);
            let stored_hash = PasswordHash::parse(&stored_hash, Encoding::B64).unwrap();

            println!("  Going to compare pass...");
            let user_pass = (&req.u).to_owned() + &req.p;

            if Scrypt.verify_password(user_pass.as_bytes(), &stored_hash).is_ok() {
                println!("  Right password");
                HttpResponse::Ok().body(&stored_hash.to_string())
            } else {
                println!("  Wrong password");
                HttpResponse::BadRequest().finish()
            }
        } else {
            println!("  No db record");
            return HttpResponse::BadRequest().finish();
        }
    }
}

mod cabinet {
    use actix_web::{get, post, web, HttpResponse, HttpRequest, Responder};
    use super::sigma_frontend::FrontendData;
    use std::ops::Deref;
    use super::{GLOBAL_FRONTEND, DB_CONNECTION, data::DBConnection, CFG};
    use serde::{Serialize, Deserialize};
    use scrypt::{
        password_hash::{ rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString, Encoding },
        Scrypt
    };

    #[get("/cabinet")]
    pub async fn get_cabinet_index(req: HttpRequest) -> impl Responder {
        // TODO: add journal record
        let cookie = req.headers().get("Cookie");

        if let Some(cookie) = cookie.as_ref().and_then(|cookie| cookie.to_str().map_or(None, |s| Some(s))) {
            let hash = &cookie[cookie.find('=').unwrap()+1..];
            let q = r##"SELECT public."validHash"('"##.to_owned() + hash + r##"');"##;
            let is_valid: bool = DB_CONNECTION.query_get(&q, &[]).unwrap().get(0);
            println!("{}", is_valid);

            return match &GLOBAL_FRONTEND.get_cabinet().as_ref().and_then(|bts| Some(&bts.html)) {
                 Some(html) if is_valid =>  { HttpResponse::Ok().body(bytes::Bytes::deref(html)) },
                _=> HttpResponse::NoContent().finish()
            };
        } else {
            HttpResponse::NoContent().finish()
        }
    }

    #[get("/cabinet/style.css")]
    pub async fn get_cabinet_style() -> impl Responder {
        if let Some(css) = &GLOBAL_FRONTEND.get_cabinet().as_ref().and_then(|bts| bts.css.as_ref()) {
            HttpResponse::Ok().body(bytes::Bytes::deref(css))
        } else {
            HttpResponse::NoContent().finish()
        }
    }

    #[get("/cabinet/script.js")]
    pub async fn get_cabinet_script() -> impl Responder {
        if let Some(js) = &GLOBAL_FRONTEND.get_cabinet().as_ref().and_then(|bts| bts.js.as_ref()) {
            HttpResponse::Ok().body(bytes::Bytes::deref(js))
        } else {
            HttpResponse::NoContent().finish()
        }
    }
}
