use naia_derive::Replicate;
use naia_shared::Property;

use super::Protocol;

#[derive(Replicate, Clone)]
pub struct Auth {
    pub username: Property<String>,
    pub password: Property<String>,
}

impl Auth {
    pub fn new(username: &str, password: &str) -> Ref<Auth> {
        return Auth::new_complete(username.to_string(), password.to_string());
    }
}