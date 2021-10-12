#[macro_use]

use actix_web::{web, HttpResponse, Responder};
use std::collections::HashMap;
use crate::DB_CONNECTION;
use crate::data::DBConnection;

#[macro_export]
macro_rules! check_cookie{
    {$req: ident, $if_valid: block} => {{
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
    }};

    ($req: ident) => {{
        let cookie = $req.headers().get("Cookie");

        if let Some(cookie) = cookie.as_ref().and_then(|cookie| cookie.to_str().map_or(None, |s| Some(s))) {
            let hash = &cookie[cookie.find('=').unwrap()+1..];
            let q = r##"SELECT public."validHash"('"##.to_owned() + hash + r##"');"##;
            let is_valid: bool = DB_CONNECTION.query_get(&q, &[]).unwrap().get(0);
            if is_valid {
                true
            } else {
                false
            }
        } else {
            false
        }

    }};
}

#[macro_export]
macro_rules! construct_post_resource{
    ($route: expr, $api: ident) => {
        web::resource($route).route(
                web::route().guard(actix_web::guard::fn_guard(|req| {
                    req.method == actix_web::http::Method::POST && check_cookie!(req)
                })).to($api)
        )
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

pub fn construct_json_order(new_obj: &mut HashMap<String, String>, row: postgres::row::Row) {
    crate::append_json!(author, "author", row, 0, String, new_obj);
    crate::append_json!(ord, "ord", row, 1, String, new_obj);
    crate::append_json!(category, "category", row, 2, String, new_obj);
    crate::append_json!(yr, "yr", row, 3, i32, new_obj);
    crate::append_json!(ty, "ty", row, 4, String, new_obj);
    crate::append_json!(typography, "typography", row, 5, String, new_obj);
    crate::append_json!(ordermaker, "ordermaker", row, 6, String, new_obj);
    crate::append_json!(price, "price", row, 7, f32, new_obj);
    crate::append_json!(status, "status", row, 8, i16, new_obj);
}

pub fn construct_json_nameonly(new_obj: &mut HashMap<String, String>, row: postgres::row::Row) {
    crate::append_json!(id, "id", row, 0, i64, new_obj);
    crate::append_json!(name, "name", row, 1, String, new_obj);
}

#[macro_export]
macro_rules! append_json{
    ($row_ident: ident, $obj_row: expr, $row: ident, $col_num: expr, $col_type: ty, $obj_name: ident) => {
        let $row_ident: $col_type = $row.get($col_num);
        $obj_name.insert($obj_row.to_string(), format!("{}", $row_ident));
    }
}

pub fn construct_query_nameonly(req: web::HttpRequest, q: &str) -> impl Responder {
    check_cookie!{ req,
        {
            let data = DB_CONNECTION.query_get_each(q, &[]).unwrap();
            let mut output = Vec::<HashMap<String, String>>::new();

            for row in data {
                let mut new_obj = HashMap::new();
                crate::construct_json_nameonly(&mut new_obj, row);
                output.push(new_obj);
            }

            let json = serde_json::to_string(&output).unwrap();
            return HttpResponse::Ok().body(&json);
        }
    }
}
