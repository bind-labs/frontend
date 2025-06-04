use std::sync::LazyLock;

use dioxus::prelude::*;
use serde::{de::DeserializeOwned, Serialize};

use crate::api::ApiClient;
use crate::platform::SecureStore;

mod use_keyboard_open;

pub use use_keyboard_open::use_keyboard_open;

static API: LazyLock<ApiClient> = LazyLock::new(|| ApiClient::new("https://api.bind.sh"));
static TOKEN: GlobalSignal<Option<String>> =
    Signal::global(|| SecureStore::get("token").unwrap_or(None));

pub fn use_token() -> Signal<Option<String>> {
    let token = use_signal(|| TOKEN.read().clone());

    use_effect(move || {
        SecureStore::set("token", &token());
        *TOKEN.write() = token();
    });

    token
}

pub fn use_api() -> &'static ApiClient {
    use_effect(move || {
        API.set_token(TOKEN());
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
