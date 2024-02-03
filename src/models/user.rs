use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "rocket::serde")]

pub struct BasicAuthStruct {
    pub username: String,
    pub password: String,
}

// impl BasicAuthStruct {
//     fn from_header(header: &str) -> Option<BasicAuthStruct> {
//         let split_vec = header.split_whitespace().collect::<Vec<_>>();
//     }
// }
