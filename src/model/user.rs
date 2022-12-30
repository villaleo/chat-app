use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct User {
    pub id: Uuid,
    pub name: String,
}

impl User {
    pub fn new(id: Uuid, name: &str) -> Self {
        Self { id, name: String::from(name) }
    }
}
