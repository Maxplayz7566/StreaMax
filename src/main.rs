#[macro_use] extern crate rocket;

#[get("/")]
async fn index() -> &'static str {
    "Hello from Rocket with Tokio!"
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    rocket::build()
        .mount("/", routes![index])
        .launch()
        .await
}
