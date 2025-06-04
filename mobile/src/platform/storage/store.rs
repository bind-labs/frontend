use dioxus::prelude::spawn;
use serde::{de::DeserializeOwned, Serialize};

#[cfg(target_os = "android")]
use super::android::*;
#[cfg(target_os = "ios")]
use super::ios::*;

use std::{
    cell::LazyCell,
    collections::HashMap,
    sync::{LazyLock, Mutex},
    time::Instant,
};

// Cache for secret values to avoid frequent JNI calls
static SECRET_CACHE: LazyLock<Mutex<HashMap<String, String>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

pub struct SecureStore;

impl SecureStore {
    // Store a secret value
    pub fn set<T: Serialize>(key: &str, value: T) {
        let start = Instant::now();

        let key = key.to_string();
        let serialized = serde_json::to_string(&value).unwrap();
        // TODO: check if the value is already set to the same value
        secure_store(&key, serialized.as_str());

        let elapsed = start.elapsed().as_millis();
        tracing::info!("SecureStore::set({}) took {}ms", key, elapsed);

        // Update cache
        SECRET_CACHE.lock().unwrap().insert(key, serialized);
    }

    // Retrieve a secret value
    pub fn get<T: DeserializeOwned>(key: &str) -> Option<T> {
        let elapsed = Instant::now();
        // Check cache first
        if let Some(value) = SECRET_CACHE.lock().unwrap().get(key) {
            let deserialized: T = serde_json::from_str(value).unwrap();
            return Some(deserialized);
        }

        // If not in cache, fetch from secure storage
        let result = secure_retrieve(key).unwrap_or(None);

        // Update cache if value exists
        if let Some(ref value) = result {
            SECRET_CACHE
                .lock()
                .unwrap()
                .insert(key.to_string(), value.to_string());
        }

        let elapsed = elapsed.elapsed().as_millis();
        tracing::info!("SecureStore::get({}) took {}ms", key, elapsed);
        result.map(|value| {
            let deserialized: T = serde_json::from_str(&value).unwrap();
            deserialized
        })
    }
}
