#[macro_use]
extern crate diesel;

pub mod schema;
pub mod models;

use dotenv::dotenv;
use std::env;

use diesel::prelude::*;
use diesel::pg::PgConnection;


fn main() {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("db url variable no encontrada");

    let conn = PgConnection::establish(&db_url).expect("No nos pudimos conectar a la base de datos");

    use self::models::{Post, NewPost, PostSimplificado};
    use self::schema::posts;
    use self::schema::posts::dsl::*;

    diesel::delete(posts.filter(slug.like("%-post%"))).execute(&conn).expect("Ha fallado la eliminacion del el tercer post");


    // Select * from posts
    println!("Query sin limites");
    let posts_result = posts.load::<Post>(&conn).expect("Error al ejecutar la query");

    for post in posts_result {
        println!("{:?}", post);
    }

}
