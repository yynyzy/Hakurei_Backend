use rocket::{
    catch, catchers, get, post, routes,
    serde::json::{serde_json::json, Value},
};

// struct BasicAuthStruct {
//     username: String,
//     password: String,
// }

// impl BasicAuthStruct {
//     fn from_header(header: &str) -> Option<BasicAuthStruct> {
//         let split_vec = header.split_whitespace().collect::<Vec<_>>();
//     }
// }

#[post("/login", data = "<a>")]
async fn login_user(a: String) -> Value {
    json!("login success")
}

#[catch(404)]
async fn not_found_url() -> Value {
    json!("url not found")
}
#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    rocket::build()
        .mount("/user", routes![login_user])
        .register("/", catchers![not_found_url])
        .launch()
        .await?;
    Ok(())
}
