use std::sync::LazyLock;

use dioxus::prelude::*;
use serde::{de::DeserializeOwned, Serialize};

use crate::api::ApiClient;
use crate::platform::SecureStore;

mod use_keyboard_open;

pub use use_keyboard_open::use_keyboard_open;

static API: LazyLock<ApiClient> = LazyLock::new(|| ApiClient::new("https://api.bind.sh"));

pub fn use_token() -> Signal<Option<String>> {
    use_persistent::<Option<String>>("token", || None)
}

pub fn use_api() -> &'static ApiClient {
    let token = use_token();
    use_effect(move || {
        API.set_token(token());
    });

    &*API
}

pub fn use_persistent<T: Serialize + DeserializeOwned + Default + 'static>(
    key: &str,
    init: impl FnOnce() -> T,
) -> Signal<T> {
    let state = use_signal(|| SecureStore::get(key).unwrap_or_else(init));

    // TODO: what the actual fu
    let key_clone = key.to_string();
    use_effect(move || {
        let value = state.read();
        SecureStore::set(&key_clone, &*value);
    });

    state
}
