use uuid::Uuid;

pub struct UUID;

impl UUID {
    pub fn generate_uuid() -> String {
        Uuid::new_v4().to_string()
    }
}
