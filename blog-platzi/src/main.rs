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

    // let new_post = NewPost {
    //     title: "Mi segundo blogpost",
    //     body: "2 Lorem impsiingsdd",
    //     slug: "segundo-post",
    // };

    // let post: Post = diesel::insert_into(posts::table).values(new_post).get_result(&conn).expect("La insertada, falló");

    // Select * from posts
    println!("Query sin limites");
    let posts_result = posts.load::<Post>(&conn).expect("Error al ejecutar la query");

    for post in posts_result {
        println!("{:?}", post);
    }


    // Select * from posts limit 1
    println!("Query con limites");
    let posts_result = posts.limit(1).load::<Post>(&conn).expect("Error al ejecutar la query");

    for post in posts_result {
        println!("{:?}", post);
    }

    // Select * from posts order by id limit 1
    println!("Query con limites ordenado por id");
    let posts_result = posts.order(id.desc()).limit(1).load::<Post>(&conn).expect("Error al ejecutar la query");

    for post in posts_result {
        println!("{:?}", post);
    }


    // Select title, body from posts limit 1
    println!("Query columnas especificas");
    let posts_result = posts.select((title, body)).limit(1).load::<PostSimplificado>(&conn).expect("Error al ejecutar la query");

    for post in posts_result {
        println!("{:?}", post);
    }


    // Select title, body from posts where id = 2 limit 1
    println!("Query con where");
    let posts_result = posts.filter(slug.eq("primer-post")).limit(1).load::<Post>(&conn).expect("Error al ejecutar la query");

    for post in posts_result {
        println!("{:?}", post);
    }


}
