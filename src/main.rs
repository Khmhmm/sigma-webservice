use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
mod sigma_frontend;
mod data;
mod config;

use config::Config;
use data::{DBConnection, PostgresEnvelop};
use sigma_frontend::{FrontendData, ReadFrontend};
use lazy_static::lazy_static;
use openssl::ssl::*;
use std::collections::HashMap;

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

#[macro_export]
macro_rules! check_cookie{
    {$req: ident, $if_valid: block} => {
        let cookie = $req.headers().get("Cookie");

        if let Some(cookie) = cookie.as_ref().and_then(|cookie| cookie.to_str().map_or(None, |s| Some(s))) {
            let hash = &cookie[cookie.find('=').unwrap()+1..];
            let q = r##"SELECT public."validHash"('"##.to_owned() + hash + r##"');"##;
            let is_valid: bool = DB_CONNECTION.query_get(&q, &[]).unwrap().get(0);
            if is_valid {
                $if_valid
            } else {
                return HttpResponse::Forbidden().finish();
            }
        } else {
            return HttpResponse::Forbidden().finish();
        }
    }
}

#[macro_export]
macro_rules! get_content{
    ($frontend_data: expr, $content: ident) => {
        if let Some($content) = &$frontend_data.as_ref().and_then(|bts| bts.$content.as_ref()) {
            HttpResponse::Ok().body(bytes::Bytes::deref($content))
        } else {
            HttpResponse::NoContent().finish()
        }
    }
}

pub fn construct_json(new_obj: &mut HashMap<String, String>, row: postgres::row::Row) {
    append_json!(author, "author", row, 0, String, new_obj);
    append_json!(ord, "ord", row, 1, String, new_obj);
    append_json!(category, "category", row, 2, String, new_obj);
    append_json!(yr, "yr", row, 3, i32, new_obj);
    append_json!(ty, "ty", row, 4, String, new_obj);
    append_json!(typography, "typography", row, 5, String, new_obj);
    append_json!(ordermaker, "ordermaker", row, 6, String, new_obj);
    append_json!(price, "price", row, 7, f32, new_obj);
    append_json!(status, "status", row, 8, i16, new_obj);
}

#[macro_export]
macro_rules! append_json{
    ($row_ident: ident, $obj_row: expr, $row: ident, $col_num: expr, $col_type: ty, $obj_name: ident) => {
        let $row_ident: $col_type = $row.get($col_num);
        $obj_name.insert($obj_row.to_string(), format!("{}", $row_ident));
    }
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
            .service(api::signup).service(api::login)
            .service(cabinet::get_cabinet_index).service(cabinet::get_cabinet_style).service(cabinet::get_cabinet_script)
            .service(api::get_active_orders).service(api::have_rights).service(api::new_typography)
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
    use super::{GLOBAL_FRONTEND, get_content};

    #[get("/")]
    pub async fn get_index() -> impl Responder {
        get_content!(GLOBAL_FRONTEND.get_index(), html)
    }

    #[get("/style.css")]
    pub async fn get_index_css() -> impl Responder {
        get_content!(GLOBAL_FRONTEND.get_index(), css)
    }

    #[get("script.js")]
    pub async fn get_index_js() -> impl Responder {
        get_content!(GLOBAL_FRONTEND.get_index(), js)
    }
}


mod login {
    use actix_web::{get, post, web, HttpResponse, HttpRequest, Responder};
    use super::sigma_frontend::FrontendData;
    use std::ops::Deref;
    use super::{GLOBAL_FRONTEND, DB_CONNECTION, data::DBConnection, CFG, get_content};
    use serde::{Serialize, Deserialize};

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Credentials{
        pub u: String,
        pub p: String
    }

    #[get("/login")]
    pub async fn get_login() -> impl Responder {
        get_content!(GLOBAL_FRONTEND.get_login(), html)
    }

    #[get("/login/style.css")]
    pub async fn get_login_style() -> impl Responder {
        get_content!(GLOBAL_FRONTEND.get_login(), css)
    }

    #[get("/login/script.js")]
    pub async fn get_login_script() -> impl Responder {
        get_content!(GLOBAL_FRONTEND.get_login(), js)
    }
}

mod cabinet {
    use actix_web::{get, post, web, HttpResponse, HttpRequest, Responder};
    use super::sigma_frontend::FrontendData;
    use std::ops::Deref;
    use super::{
        {GLOBAL_FRONTEND, DB_CONNECTION, data::DBConnection, CFG},
        get_content, check_cookie, login::Credentials
    };
    use serde::{Serialize, Deserialize};
    use scrypt::{
        password_hash::{ rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString, Encoding },
        Scrypt
    };

    #[get("/cabinet")]
    pub async fn get_cabinet_index(req: HttpRequest) -> impl Responder {
        check_cookie!{ req, {get_content!(GLOBAL_FRONTEND.get_cabinet(), html)} }
    }

    #[get("/cabinet/style.css")]
    pub async fn get_cabinet_style() -> impl Responder {
        get_content!(GLOBAL_FRONTEND.get_cabinet(), css)
    }

    #[get("/cabinet/script.js")]
    pub async fn get_cabinet_script() -> impl Responder {
        get_content!(GLOBAL_FRONTEND.get_cabinet(), js)
    }
}

mod model {
    use serde::{Serialize, Deserialize};

    #[derive(Serialize, Deserialize)]
    pub struct Typography {
        pub name: String,
        pub address: String,
        pub phone: String
    }
}

mod api {
    use actix_web::{get, post, web, HttpResponse, HttpRequest, FromRequest, Responder};
    use super::sigma_frontend::FrontendData;
    use std::ops::Deref;
    use super::{
        {GLOBAL_FRONTEND, DB_CONNECTION, data::DBConnection, CFG},
        model::{Typography},
        get_content, check_cookie, login::Credentials
    };
    use serde::{Serialize, Deserialize};
    use serde_json;
    use scrypt::{
        password_hash::{ rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString, Encoding },
        Scrypt
    };
    use std::collections::HashMap;

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

    #[get("/api/activeOrders")]
    pub async fn get_active_orders(req: HttpRequest) -> impl Responder {
        check_cookie!(req, {
            let q = r##"SELECT author,ord,category,yr,ty,typography,ordermaker,price,status from public."getTotalOrders"();"##;
            let data = DB_CONNECTION.query_get_each(q, &[]).unwrap();
            let mut output = Vec::<HashMap<String, String>>::new();

            for row in data {
                let mut new_obj = HashMap::new();
                super::construct_json(&mut new_obj, row);
                output.push(new_obj);
            }

            let json = serde_json::to_string(&output).unwrap();
            return HttpResponse::Ok().body(&json);
        });
    }

    #[get("/api/haveRights")]
    pub async fn have_rights(req: HttpRequest) -> impl Responder {
        let cookie = req.headers().get("Cookie");

        if let Some(cookie) = cookie.as_ref().and_then(|cookie| cookie.to_str().map_or(None, |s| Some(s))) {
            let hash = &cookie[cookie.find('=').unwrap()+1..];
            let rights_id: i16 = DB_CONNECTION.query_get(&(r##"SELECT public."getUserRights"('"##.to_owned()+hash+r##"');"##), &[]).unwrap().get(0);
            HttpResponse::Ok().body(format!("{}",rights_id))
        } else {
            HttpResponse::Forbidden().finish()
        }
    }

    #[post("/api/newTypography")]
    pub async fn new_typography(req: HttpRequest) -> impl Responder {
        check_cookie!{ req,
            {
                println!("{:?}", req);
                let req = web::Json::<Typography>::extract(&req).await.unwrap();
                // TODO: add check
                let result_id = DB_CONNECTION.query_edit(
                    &format!(r##"CALL public."insertTypography"('{}','{}','{}');"##,req.name,req.address,req.phone), &[]
                ).unwrap();
                println!("Inserted new typography: {}", req.name);
                return HttpResponse::Ok().finish();
            }
        }
    }
}
