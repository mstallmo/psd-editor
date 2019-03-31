#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[get("/")]
fn hello_world() -> &'static str {
    "Hello, World!"
}

fn how_does() {
    println!("Some Rust look on stream?");
}

fn main() {
    how_does(); 

    rocket::ignite().mount("/", routes![hello_world]).launch();
}
