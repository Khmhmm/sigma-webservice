use actix_web::{web, App, HttpServer};

mod sigma_frontend;
mod data;
mod config;
mod api;

// a set of useful macro_rules and functions
#[macro_use]
mod tool;
pub use tool::*;

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


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use crate::api::*;
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
            .service(api::index::get_index).service(api::index::get_index_css).service(api::index::get_index_js)
            .service(api::login_page::get_login).service(api::login_page::get_login_style).service(api::login_page::get_login_script)
            .service(api::signup).service(api::login).service(api::autologin)
            .service(api::cabinet::get_cabinet_index).service(api::cabinet::get_cabinet_style).service(api::cabinet::get_cabinet_script)
            .service(api::get_active_orders).service(api::have_rights)
            .service(construct_post_onlyrights!("/api/newOrder", new_order, 2))
            .service(construct_post_onlyrights!("/api/newTypography", new_typography, 3))
            .service(construct_post_onlyrights!("/api/newAuthor", new_author, 4))
            .service(construct_post_onlyrights!("/api/newOrdermaker", new_ordermaker, 5))
            .service(api::get_actions)
            .service(api::get_authors).service(api::get_categories).service(api::get_types).service(api::get_typographies).service(api::get_ordermakers)
    })
    .bind_openssl("127.0.0.1:8080", builder)?
    .run()
    .await
}
