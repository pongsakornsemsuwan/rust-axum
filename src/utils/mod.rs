use uuid::Uuid;

pub mod db_connection;

pub fn str_to_uuid(string: String) -> Uuid {
    Uuid::parse_str(string.as_str()).unwrap()
}