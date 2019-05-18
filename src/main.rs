#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate rand;

mod paste_id;
#[cfg(test)] mod tests;

use std::io;
use std::fs::File;
use std::path::Path;

use rocket::Data;
use rocket::response::content;

use paste_id::PasteID;

const ID_LENGTH: usize = 8;

#[post("/", data = "<paste>")]
fn upload(paste: Data) -> io::Result<String> {
    let id = PasteID::new(ID_LENGTH);
    let filename = format!("upload/{id}", id = id);
    let url = format!("{id}\n", id = id);

    paste.stream_to_file(Path::new(&filename))?;
    Ok(url)
}

#[get("/<id>")]
fn retrieve(id: PasteID) -> Option<content::Plain<File>> {
    let filename = format!("upload/{id}", id = id);
    File::open(&filename).map(|f| content::Plain(f)).ok()
}

#[get("/")]
fn index() -> &'static str {
    ""
}

#[get("/robots.txt")]
fn robots() -> &'static str {
    "User-agent: *
Disallow: /"
}

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![index, upload, retrieve,robots])
}

fn main() {
    rocket().launch();
}
