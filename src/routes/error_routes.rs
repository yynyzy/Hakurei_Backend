use rocket::{
    catch,
    serde::json::{serde_json::json, Value},
};

#[catch(404)]
pub async fn not_found_url() -> Value {
    json!("url not found")
}
