// use serde::{de::DeserializeOwned, Serialize};

// #[cfg(target_os = "android")]
// use super::android::*;

// use std::{
//     cell::LazyCell,
//     collections::HashMap,
//     sync::{LazyLock, Mutex},
// };

// // Cache for secret values to avoid frequent JNI calls
// static SECRET_CACHE: LazyLock<Mutex<HashMap<String, String>>> =
//     LazyLock::new(|| Mutex::new(HashMap::new()));

// pub struct SecureStore;

// impl SecureStore {
//     // Store a secret value
//     pub fn set<T: Serialize>(key: &str, value: T) -> () {
//         let serialized = serde_json::to_string(&value).unwrap();
//         secure_store(key, serialized.as_str());

//         // Update cache
//         SECRET_CACHE
//             .lock()
//             .unwrap()
//             .insert(key.to_string(), serialized);
//     }

//     // Retrieve a secret value
//     pub fn get<T: DeserializeOwned>(key: &str) -> Option<T> {
//         // Check cache first
//         if let Some(value) = SECRET_CACHE.lock().unwrap().get(key) {
//             let deserialized: T = serde_json::from_str(value).unwrap();
//             return Some(deserialized);
//         }

//         // If not in cache, fetch from secure storage
//         let result = secure_retrieve(key);

//         // Update cache if value exists
//         if let Some(ref value) = result {
//             SECRET_CACHE
//                 .lock()
//                 .unwrap()
//                 .insert(key.to_string(), value.to_string());
//         }

//         result.map(|value| {
//             let deserialized: T = serde_json::from_str(&value).unwrap();
//             deserialized
//         })
//     }

//     // Delete a secret value
//     pub fn delete(key: &str) -> () {
//         // Store empty string to effectively delete
//         secure_store(key, "");

//         // Remove from cache
//         SECRET_CACHE.lock().unwrap().remove(key);
//     }

//     // Check if a key exists
//     pub fn exists(key: &str) -> bool {
//         // Check cache first
//         if SECRET_CACHE.lock().unwrap().contains_key(key) {
//             return true;
//         }

//         // Check secure storage
//         secure_retrieve(key).is_some()
//     }
// }
