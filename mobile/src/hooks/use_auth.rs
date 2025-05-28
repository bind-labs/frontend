use dioxus::prelude::*;

use crate::{
    storage::{use_persistent, UsePersistent},
    AuthContext,
};

pub fn use_auth() -> Signal<AuthContext> {
    let auth_token: UsePersistent<Option<String>> = use_persistent("auth_token", || None);
    todo!("implement a use auth hook")
}
