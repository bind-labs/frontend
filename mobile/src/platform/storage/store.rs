use serde::{de::DeserializeOwned, Serialize};

#[cfg(target_os = "android")]
use super::android::*;
#[cfg(target_os = "ios")]
use super::ios::*;

use std::{
    cell::LazyCell,
    collections::HashMap,
    sync::{LazyLock, Mutex},
};

// Cache for secret values to avoid frequent JNI calls
static SECRET_CACHE: LazyLock<Mutex<HashMap<String, String>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

pub struct SecureStore;

impl SecureStore {
    // Store a secret value
    pub fn set<T: Serialize>(key: &str, value: T) -> () {
        // TODO: check if the value is already set to the same value
        let serialized = serde_json::to_string(&value).unwrap();
        secure_store(key, serialized.as_str());

        // Update cache
        SECRET_CACHE
            .lock()
            .unwrap()
            .insert(key.to_string(), serialized);
    }

    // Retrieve a secret value
    pub fn get<T: DeserializeOwned>(key: &str) -> Option<T> {
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

        result.map(|value| {
            let deserialized: T = serde_json::from_str(&value).unwrap();
            deserialized
        })
    }
}
