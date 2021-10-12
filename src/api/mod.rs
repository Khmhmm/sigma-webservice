#[macro_use]

use actix_web::{get, post, web, HttpResponse, HttpRequest, Responder};

use crate::data::DBConnection;

use crate::{
    {GLOBAL_FRONTEND, DB_CONNECTION, CFG},
    data::model::{Typography, Order},
    get_content, check_cookie, construct_query_nameonly,
    append_json
};

use serde_json;
use scrypt::{
    password_hash::{ rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString, Encoding },
    Scrypt
};
use std::collections::HashMap;

#[post("/api/login")]
pub async fn login(req: web::Json<self::login_page::Credentials>) -> impl Responder {
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
pub async fn signup(req: web::Json<self::login_page::Credentials>) -> impl Responder {
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

    let _result_id = DB_CONNECTION.query_edit(
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
            crate::construct_json_order(&mut new_obj, row);
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

// #[post("/api/newTypography")]
pub async fn new_typography(req: web::Json<Typography>) -> impl Responder {
    let _result_id = DB_CONNECTION.query_edit(
        &format!(r##"CALL public."insertTypography"('{}','{}','{}');"##,req.name,req.address,req.phone), &[]
    ).unwrap();
    return HttpResponse::Ok().finish();
}

// #[post("/api/newOrder]
pub async fn new_order(req: web::Json<Order>) -> impl Responder {
    let _result_id = DB_CONNECTION.query_edit(
        &format!(r##"CALL public."insertOrder"('{}','{}','{}','{}','{}','{}','{}','{}');"##,req.author_id,req.name,req.category_id,req.year,req.type_id,req.typography_id,req.ordermaker_id,req.price), &[]
    ).unwrap();
    return HttpResponse::Ok().finish();
}

#[get("/api/getAuthors")]
pub async fn get_authors(req: HttpRequest) -> impl Responder {
    construct_query_nameonly(req, r##"SELECT id, name from public."getTotalAuthors"();"##)
}

#[get("/api/getCategories")]
pub async fn get_categories(req: HttpRequest)-> impl Responder {
    construct_query_nameonly(req, r##"SELECT id, name from public."getTotalCategories"();"##)
}

#[get("/api/getTypes")]
pub async fn get_types(req: HttpRequest)-> impl Responder {
    construct_query_nameonly(req, r##"SELECT id,  name from public."getTotalTypes"();"##)
}

#[get("/api/getTypographies")]
pub async fn get_typographies(req: HttpRequest)-> impl Responder {
    construct_query_nameonly(req, r##"SELECT id, name from public."getTotalTypographies"();"##)
}

#[get("/api/getOrdermakers")]
pub async fn get_ordermakers(req: HttpRequest)-> impl Responder {
    construct_query_nameonly(req, r##"SELECT id, name from public."getTotalOrdermakers"();"##)
}


pub mod index {
    use actix_web::{get, HttpResponse, Responder};
    
    use std::ops::Deref;
    use crate::{GLOBAL_FRONTEND, get_content};

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


pub mod login_page {
    use actix_web::{get, post, web, HttpResponse, HttpRequest, Responder};
    
    use std::ops::Deref;
    use crate::{GLOBAL_FRONTEND, get_content};
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

pub mod cabinet {
    use actix_web::{get, HttpResponse, HttpRequest, Responder};
    
    use std::ops::Deref;
    use crate::{
        {GLOBAL_FRONTEND, DB_CONNECTION, data::DBConnection},
        get_content, check_cookie
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
