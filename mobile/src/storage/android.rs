//! Android Secure Storage Implementation using EncryptedSharedPreferences.
//!
//! **Important:** Ensure you have added the necessary dependency to your Android
//! project's `app/build.gradle` file:
//!
//! dependencies {
//!     implementation("androidx.security:security-crypto:1.1.0-alpha06") // Or the latest stable version
//! }

use dioxus::mobile::wry::prelude::{dispatch, find_class};
use jni::objects::{JObject, JString, JValue};
use jni::JNIEnv;
use std::sync::mpsc;

#[derive(Debug)]
pub enum SecureStorageError {
    Jni(jni::errors::Error),
    RecvError(mpsc::RecvError),
}

impl From<jni::errors::Error> for SecureStorageError {
    fn from(err: jni::errors::Error) -> Self {
        SecureStorageError::Jni(err)
    }
}

impl From<mpsc::RecvError> for SecureStorageError {
    fn from(err: mpsc::RecvError) -> Self {
        SecureStorageError::RecvError(err)
    }
}

type Result<T> = std::result::Result<T, SecureStorageError>;

/// Securely stores a key-value pair using Android EncryptedSharedPreferences.
pub fn secure_store(key: &str, value: &str) -> Result<()> {
    let key_str = key.to_string();
    let value_str = value.to_string();

    let (tx, rx) = mpsc::channel::<Result<()>>();

    dispatch(move |env: &mut JNIEnv, activity: &JObject, _webview| {
        let result = (|| -> Result<()> {
            // Find Classes
            let master_key_class = find_class(
                env,
                activity,
                "androidx/security/crypto/MasterKey$Builder".to_string(),
            )?;
            let key_scheme_enum = find_class(
                env,
                activity,
                "androidx/security/crypto/MasterKey$KeyScheme".to_string(),
            )?;
            let encrypted_prefs_class = find_class(
                env,
                activity,
                "androidx/security/crypto/EncryptedSharedPreferences".to_string(),
            )?;
            let pref_key_scheme_enum = find_class(
                env,
                activity,
                "androidx/security/crypto/EncryptedSharedPreferences$PrefKeyEncryptionScheme"
                    .to_string(),
            )?;
            let pref_value_scheme_enum = find_class(
                env,
                activity,
                "androidx/security/crypto/EncryptedSharedPreferences$PrefValueEncryptionScheme"
                    .to_string(),
            )?;

            // Create Master Key Builder
            let master_key_builder = env.new_object(
                &master_key_class,
                "(Landroid/content/Context;)V",
                &[JValue::Object(activity)],
            )?;

            // Get MasterKey.KeyScheme.AES256_GCM enum value
            let key_scheme_aes256_gcm = env
                .get_static_field(
                    key_scheme_enum,
                    "AES256_GCM",
                    "Landroidx/security/crypto/MasterKey$KeyScheme;",
                )?
                .l()?;

            // Set key scheme
            let master_key_builder = env.call_method( // Re-assign builder
                &master_key_builder,
                "setKeyScheme",
                "(Landroidx/security/crypto/MasterKey$KeyScheme;)Landroidx/security/crypto/MasterKey$Builder;",
                &[JValue::Object(&key_scheme_aes256_gcm)],
            )?.l()?;

            // Build the MasterKey
            let master_key = env
                .call_method(
                    &master_key_builder,
                    "build",
                    "()Landroidx/security/crypto/MasterKey;",
                    &[],
                )?
                .l()?;

            // Get EncryptedSharedPreferences Settings
            let pref_file_name = env.new_string("secure_prefs")?;

            // Get PrefKeyEncryptionScheme.AES256_SIV enum value
            let pref_key_scheme_aes256_siv = env
                .get_static_field(
                    pref_key_scheme_enum,
                    "AES256_SIV",
                    "Landroidx/security/crypto/EncryptedSharedPreferences$PrefKeyEncryptionScheme;",
                )?
                .l()?;

            // Get PrefValueEncryptionScheme.AES256_GCM enum value
            let pref_value_scheme_aes256_gcm = env.get_static_field(
                pref_value_scheme_enum,
                "AES256_GCM",
                "Landroidx/security/crypto/EncryptedSharedPreferences$PrefValueEncryptionScheme;",
            )?.l()?;

            // Create EncryptedSharedPreferences
            let encrypted_prefs = env.call_static_method(
                &encrypted_prefs_class,
                "create",
                "(Landroid/content/Context;Ljava/lang/String;Landroidx/security/crypto/MasterKey;Landroidx/security/crypto/EncryptedSharedPreferences$PrefKeyEncryptionScheme;Landroidx/security/crypto/EncryptedSharedPreferences$PrefValueEncryptionScheme;)Landroid/content/SharedPreferences;",
                &[
                    JValue::Object(activity), // Use activity JObject
                    JValue::Object(&pref_file_name),
                    JValue::Object(&master_key),
                    JValue::Object(&pref_key_scheme_aes256_siv),
                    JValue::Object(&pref_value_scheme_aes256_gcm),
                ],
            )?.l()?;

            // Get the SharedPreferences Editor
            let editor = env
                .call_method(
                    &encrypted_prefs,
                    "edit",
                    "()Landroid/content/SharedPreferences$Editor;",
                    &[],
                )?
                .l()?;

            // Convert Rust strings to Java Strings
            let key_jstring = env.new_string(&key_str)?;
            let value_jstring = env.new_string(&value_str)?;

            // Put the key-value pair
            env.call_method(
                &editor,
                "putString",
                "(Ljava/lang/String;Ljava/lang/String;)Landroid/content/SharedPreferences$Editor;",
                &[JValue::Object(&key_jstring), JValue::Object(&value_jstring)], // Pass JStrings as JValues containing JObjects
            )?;

            // Apply the changes asynchronously
            env.call_method(&editor, "apply", "()V", &[])?;

            Ok(())
        })();

        // If sending fails, it means the receiver was dropped, ignore.
        let _ = tx.send(result);
    });

    rx.recv()?
}

/// Retrieves a securely stored value using Android EncryptedSharedPreferences.
/// Returns Ok(None) if the key is not found.
pub fn secure_retrieve(key: &str) -> Result<Option<String>> {
    let key_str = key.to_string();
    let (tx, rx) = mpsc::channel::<Result<Option<String>>>();

    dispatch(move |env: &mut JNIEnv, activity: &JObject, _webview| {
        let result = (|| -> Result<Option<String>> {
            // Find Classes
            let master_key_class = find_class(
                env,
                activity,
                "androidx/security/crypto/MasterKey$Builder".to_string(),
            )?;
            let key_scheme_enum = find_class(
                env,
                activity,
                "androidx/security/crypto/MasterKey$KeyScheme".to_string(),
            )?;
            let encrypted_prefs_class = find_class(
                env,
                activity,
                "androidx/security/crypto/EncryptedSharedPreferences".to_string(),
            )?;
            let pref_key_scheme_enum = find_class(
                env,
                activity,
                "androidx/security/crypto/EncryptedSharedPreferences$PrefKeyEncryptionScheme"
                    .to_string(),
            )?;
            let pref_value_scheme_enum = find_class(
                env,
                activity,
                "androidx/security/crypto/EncryptedSharedPreferences$PrefValueEncryptionScheme"
                    .to_string(),
            )?;

            // Create Master Key Builder
            let master_key_builder = env.new_object(
                &master_key_class,
                "(Landroid/content/Context;)V",
                &[JValue::Object(activity)], // Pass JObject as JValue
            )?;

            // Get MasterKey.KeyScheme.AES256_GCM enum value
            let key_scheme_aes256_gcm = env
                .get_static_field(
                    key_scheme_enum,
                    "AES256_GCM",
                    "Landroidx/security/crypto/MasterKey$KeyScheme;",
                )?
                .l()?;

            // Set key scheme
            let master_key_builder = env.call_method( // Re-assign builder
                 &master_key_builder,
                 "setKeyScheme",
                 "(Landroidx/security/crypto/MasterKey$KeyScheme;)Landroidx/security/crypto/MasterKey$Builder;",
                 &[JValue::Object(&key_scheme_aes256_gcm)],
             )?.l()?;

            // Build the MasterKey
            let master_key = env
                .call_method(
                    &master_key_builder,
                    "build",
                    "()Landroidx/security/crypto/MasterKey;",
                    &[],
                )?
                .l()?;

            // Get EncryptedSharedPreferences Settings
            let pref_file_name = env.new_string("secure_prefs")?;

            // Get PrefKeyEncryptionScheme.AES256_SIV enum value
            let pref_key_scheme_aes256_siv = env
                .get_static_field(
                    pref_key_scheme_enum,
                    "AES256_SIV",
                    "Landroidx/security/crypto/EncryptedSharedPreferences$PrefKeyEncryptionScheme;",
                )?
                .l()?;

            // Get PrefValueEncryptionScheme.AES256_GCM enum value
            let pref_value_scheme_aes256_gcm = env.get_static_field(
                 pref_value_scheme_enum,
                 "AES256_GCM",
                 "Landroidx/security/crypto/EncryptedSharedPreferences$PrefValueEncryptionScheme;",
             )?.l()?;

            // Create EncryptedSharedPreferences
            let encrypted_prefs = env.call_static_method(
                &encrypted_prefs_class,
                "create",
                "(Landroid/content/Context;Ljava/lang/String;Landroidx/security/crypto/MasterKey;Landroidx/security/crypto/EncryptedSharedPreferences$PrefKeyEncryptionScheme;Landroidx/security/crypto/EncryptedSharedPreferences$PrefValueEncryptionScheme;)Landroid/content/SharedPreferences;",
                &[
                    JValue::Object(activity),
                    JValue::Object(&pref_file_name),
                    JValue::Object(&master_key),
                    JValue::Object(&pref_key_scheme_aes256_siv),
                    JValue::Object(&pref_value_scheme_aes256_gcm),
                ],
            )?.l()?; // Extract JObject (SharedPreferences)

            let key_jstring: JString = env.new_string(&key_str)?; // Borrow is fine
            let key_jobject: JObject = key_jstring.into(); // Convert JString to JObject for calls needing Objects

            // Check if key exists
            let contains_key = env
                .call_method(
                    &encrypted_prefs,
                    "contains",
                    "(Ljava/lang/String;)Z",
                    &[JValue::Object(&key_jobject)],
                )?
                .z()?;

            // Key not found
            if !contains_key {
                return Ok(None);
            }

            // Retrieve the value
            let default_value_jstring = env.new_string("")?;
            let default_value: JObject = default_value_jstring.into();

            let value_jobject = env
                .call_method(
                    // Borrow is fine
                    &encrypted_prefs,
                    "getString",
                    "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;",
                    &[JValue::Object(&key_jobject), JValue::Object(&default_value)],
                )?
                .l()?;

            if value_jobject.is_null() {
                println!(
                    "Warning: key '{}' exists but getString returned null.",
                    key_str
                );
                Ok(None)
            } else {
                // Convert Java String JObject to Rust String
                let value_jstring: JString = value_jobject.into();
                let value_rust_string = env
                    .get_string(&value_jstring)?
                    .to_string_lossy()
                    .into_owned();
                Ok(Some(value_rust_string))
            }
        })();

        let _ = tx.send(result);
    });

    rx.recv()?
}
