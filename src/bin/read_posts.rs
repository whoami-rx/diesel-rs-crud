extern crate diesel_rs_crud;
extern crate diesel;

use self::diesel_rs_crud::*;
use self::models::*;
use self::diesel::prelude::*;

fn main() {
  use diesel_rs_crud::schema::posts::dsl::*;

  let connection = stablish_connection();
  let results = posts.filter(published.eq(true))
    .limit(5)
    .load::<Post>(&connection)
    .expect("Error loading posts");

  println!("Displaying {} posts", results.len());
  for post in results {
    println!("{}", post.title);
    println!("*************\n");
    println!("{}", post.body);
  }
}