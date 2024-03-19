///////////////// actix /////////////////
#[cfg(feature = "actix")]
use actix_files::Files;
#[cfg(feature = "actix")]
use actix_web::{get, web::ServiceConfig};
#[cfg(feature = "actix")]
use shuttle_actix_web::ShuttleActixWeb;

#[cfg(feature = "actix")]
#[shuttle_runtime::main]
async fn actix_web() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(hello_world).service(
            Files::new("/", "./dist")
                .index_file("index.html")
                .prefer_utf8(true),
        );
    };

    Ok(config.into())
}

///////////////// axum /////////////////
#[cfg(feature = "axum")]
use axum::{routing::get, Router};
#[cfg(feature = "axum")]
use tower_http::services::ServeDir;

#[cfg(feature = "axum")]
#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/hi-axum", get(hello_world))
        .nest_service("/", ServeDir::new("./dist"));

    Ok(router.into())
}

///////////////// rocket /////////////////
#[cfg(feature = "rocket")]
use rocket::{
    fs::{relative, NamedFile},
    get, routes,
};
#[cfg(feature = "rocket")]
use std::path::{Path, PathBuf};

#[cfg(feature = "rocket")]
#[get("/<path..>")]
pub async fn serve(mut path: PathBuf) -> Option<NamedFile> {
    path.set_extension("html");
    let mut path = Path::new(relative!("./dist")).join(path);
    if path.is_dir() {
        path.push("index.html");
    }

    NamedFile::open(path).await.ok()
}

#[cfg(feature = "rocket")]
#[shuttle_runtime::main]
async fn rocket() -> shuttle_rocket::ShuttleRocket {
    let rocket = rocket::build().mount("/", routes![hello_world, launch, serve]);

    Ok(rocket.into())
}

///////////////// all of them /////////////////
#[cfg_attr(feature = "actix", get("/hi-actix"))]
#[cfg_attr(feature = "rocket", get("/hi-rocket"))]
async fn hello_world() -> &'static str {
    "Hello, World!"
}

#[cfg(feature = "rocket")]
#[cfg_attr(feature = "rocket", get("/launch"))]
async fn launch() -> &'static str {
    "3... 2... 1... Liftoff!"
}
