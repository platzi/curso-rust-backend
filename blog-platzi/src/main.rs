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

    let post_update = diesel::update(posts.filter(id.eq(3))).set((body.eq("Este es el post que hemos editado otra vez"), title.eq("Mi tercer blogpost"))).get_result::<Post>(&conn).expect("Error en el update");


    // Select * from posts
    println!("Query sin limites");
    let posts_result = posts.load::<Post>(&conn).expect("Error al ejecutar la query");

    for post in posts_result {
        println!("{:?}", post);
    }

}
