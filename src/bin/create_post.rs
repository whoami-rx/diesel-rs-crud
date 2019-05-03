extern crate diesel;
extern crate diesel_rs_crud;

use self::diesel_rs_crud::*;
use std::io::{stdin, Read};

fn main() {
    let connection = stablish_connection();

    println!("Enter the Title:");
    let mut title = String::new();
    stdin().read_line(&mut title).unwrap();
    let title = &title[..(title.len() - 1)]; // Drop the newline character
    println!(
        "\nWrite the body (Press {} when finished)\n", EOF
    );
    let mut body = String::new();
    stdin().read_to_string(&mut body).unwrap();

    let _ = create_post(&connection, title, &body);
    println!("\nSaved draft {}", title);
}

#[cfg(not(windows))]
const EOF: &str = "CTRL+D";

#[cfg(windows)]
const EOF: &str = "CTRL+Z";