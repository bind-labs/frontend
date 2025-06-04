use dioxus::{
    hooks::use_signal,
    signals::{Readable, Signal, Writable},
};
use serde::{de::DeserializeOwned, Serialize};

#[cfg(target_os = "android")]
mod android;
#[cfg(target_os = "ios")]
mod ios;
mod store;

pub use store::SecureStore;
