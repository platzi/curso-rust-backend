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

    use self::models::Post;
    use self::schema::posts::dsl::*;

    // Select * from posts
    let posts_result = posts.load::<Post>(&conn).expect("Error al ejecutar la query");

    for post in posts_result {
        println!("{}", post.title);
    }


}
