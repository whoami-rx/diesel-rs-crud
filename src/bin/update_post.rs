extern crate diesel;
extern crate diesel_rs_crud;

use self::diesel::prelude::*;
use self::diesel_rs_crud::*;
use std::env::args;

fn main() {
    use diesel_rs_crud::schema::posts::dsl::{posts, published};

    let id = args()
        .nth(1)
        .expect("publish_post requires a post id")
        .parse::<i32>()
        .expect("Invalid ID");
    let connection = stablish_connection();

    let _ = diesel::update(posts.find(id))
        .set(published.eq(true))
        .execute(&connection)
        .unwrap_or_else(|_| panic!("Unable to find post {}", id));

    let post: models::Post = posts
        .find(id)
        .first(&connection)
        .unwrap_or_else(|_| panic!("Unable to find post {}", id));

    println!("Published post {}", post.title);
}