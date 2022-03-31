#[macro_use]
extern crate diesel;

pub mod schema;
pub mod models;

use tera::Tera;
use dotenv::dotenv;
use std::env;

use diesel::prelude::*;
use diesel::pg::PgConnection;

use diesel::r2d2::{self, ConnectionManager};
use diesel::r2d2::Pool;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

use self::models::{Post, NewPost, NewPostHandler};
use self::schema::posts;
use self::schema::posts::dsl::*;


#[get("/prueba-tera")]
async fn tera_test(template_manager: web::Data<tera::Tera>) -> impl Responder {

    let mut ctx = tera::Context::new();

    HttpResponse::Ok().content_type("text/html").body(
        template_manager.render("index.html", &ctx).unwrap()
    )
}


#[get("/")]
async fn index(pool: web::Data<DbPool>) -> impl Responder {
    let conn = pool.get().expect("Problemas al traer la base de datos");

    match web::block(move || {posts.load::<Post>(&conn)}).await {
        Ok(data) => {
            return HttpResponse::Ok().body(format!("{:?}", data));
        },
        Err(err) => HttpResponse::Ok().body("Error al recibir la data")
    }
}


#[post("/new_post")]
async fn new_post(pool: web::Data<DbPool>, item: web::Json<NewPostHandler>) -> impl Responder {
    let conn = pool.get().expect("Problemas al traer la base de datos");

    match web::block(move || {Post::create_post(&conn, &item)}).await {
        Ok(data) => {
            return HttpResponse::Ok().body(format!("{:?}", data));
        },
        Err(err) => HttpResponse::Ok().body("Error al recibir la data")
    }
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("db url variable no encontrada");

    let connection = ConnectionManager::<PgConnection>::new(db_url);

    let pool = Pool::builder().build(connection).expect("No se pudo construir la Pool");

    HttpServer::new(move || {
        
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();
        
        App::new()
        .service(index)
        .service(new_post)
        .service(tera_test)
        .data(pool.clone())
        .data(tera)
    }).bind(("0.0.0.0", 9900)).unwrap().run().await

}
