mod use_auth;
mod use_keyboard_open;

pub use crate::platform::{use_persistent, UsePersistent};
pub use use_auth::use_auth;
pub use use_keyboard_open::use_keyboard_open;

pub fn use_token() -> UsePersistent<Option<String>> {
    use_persistent::<Option<String>>("token", || None)
}
