use uuid::Uuid;

pub fn get_new_uuid() -> String {
    Uuid::new_v4().to_string()
}
