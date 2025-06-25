//! Android Secure Storage Implementation using EncryptedSharedPreferences.

use dioxus::mobile::wry::prelude::{dispatch, find_class};
use jni::objects::{JByteArray, JObject, JString, JValue};
use jni::JNIEnv;
use std::sync::mpsc;

#[derive(Debug)]
pub enum SecureStorageError {
    Jni(jni::errors::Error),
    RecvError(mpsc::RecvError),
    FromUtf8Error(std::string::FromUtf8Error),
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

impl From<std::string::FromUtf8Error> for SecureStorageError {
    fn from(err: std::string::FromUtf8Error) -> Self {
        SecureStorageError::FromUtf8Error(err)
    }
}

type Result<T> = std::result::Result<T, SecureStorageError>;

const KEYSTORE_ALIAS: &str = "jwt_encryption_key";
const PREFS_NAME: &str = "jwt_secure_prefs";
const AES_TRANSFORMATION: &str = "AES/GCM/NoPadding";
const GCM_TAG_LENGTH: i32 = 128;

/// Stores a value securely using regular SharedPreferences with AES encryption
pub fn secure_store(key: &str, value: &str) -> Result<()> {
    let key_str = key.to_string();
    let value_str = value.to_string();
    let (tx, rx) = mpsc::channel::<Result<()>>();

    dispatch(move |env: &mut JNIEnv, activity: &JObject, _webview| {
        let result = (|| -> Result<()> {
            // Encrypt the value
            let encrypted_data = encrypt_with_keystore_key(env, &value_str)?;

            // Get SharedPreferences
            let prefs_name = env.new_string(PREFS_NAME)?;
            let prefs = env
                .call_method(
                    activity,
                    "getSharedPreferences",
                    "(Ljava/lang/String;I)Landroid/content/SharedPreferences;",
                    &[
                        JValue::Object(&prefs_name),
                        JValue::Int(0), // MODE_PRIVATE
                    ],
                )?
                .l()?;

            // Get editor
            let editor = env
                .call_method(
                    &prefs,
                    "edit",
                    "()Landroid/content/SharedPreferences$Editor;",
                    &[],
                )?
                .l()?;

            // Put encrypted string
            let key_jstring = env.new_string(&key_str)?;
            let encrypted_data = env.new_string(&encrypted_data)?;
            env.call_method(
                &editor,
                "putString",
                "(Ljava/lang/String;Ljava/lang/String;)Landroid/content/SharedPreferences$Editor;",
                &[
                    JValue::Object(&key_jstring.into()),
                    JValue::Object(&encrypted_data),
                ],
            )?;

            // Commit changes
            env.call_method(&editor, "apply", "()V", &[])?;

            Ok(())
        })();

        let _ = tx.send(result);
    });

    rx.recv()?
}

/// Retrieves a value securely using regular SharedPreferences with AES decryption
pub fn secure_retrieve(key: &str) -> Result<Option<String>> {
    let key_str = key.to_string();
    let (tx, rx) = mpsc::channel::<Result<Option<String>>>();

    dispatch(move |env: &mut JNIEnv, activity: &JObject, _webview| {
        let result = (|| -> Result<Option<String>> {
            // Get SharedPreferences
            let prefs_name = env.new_string(PREFS_NAME)?;
            let prefs = env
                .call_method(
                    activity,
                    "getSharedPreferences",
                    "(Ljava/lang/String;I)Landroid/content/SharedPreferences;",
                    &[
                        JValue::Object(&prefs_name),
                        JValue::Int(0), // MODE_PRIVATE
                    ],
                )?
                .l()?;

            // Get encrypted value
            let key_jstring = env.new_string(&key_str)?;
            let encrypted_value = env
                .call_method(
                    &prefs,
                    "getString",
                    "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;",
                    &[
                        JValue::Object(&key_jstring.into()),
                        JValue::Object(&JObject::null()),
                    ],
                )?
                .l()?;

            if encrypted_value.is_null() {
                return Ok(None);
            }

            // Try to decrypt the value
            match decrypt_with_keystore_key(env, encrypted_value) {
                Ok(decrypted) => Ok(Some(decrypted)),
                Err(e) => {
                    // Check if the error is an AEADBadTagException
                    if let SecureStorageError::Jni(jni_err) = &e {
                        if is_aead_bad_tag_exception(env, jni_err) {
                            // Clear the shared preferences
                            let editor = env
                                .call_method(
                                    &prefs,
                                    "edit",
                                    "()Landroid/content/SharedPreferences$Editor;",
                                    &[],
                                )?
                                .l()?;

                            env.call_method(
                                &editor,
                                "clear",
                                "()Landroid/content/SharedPreferences$Editor;",
                                &[],
                            )?;
                            env.call_method(&editor, "apply", "()V", &[])?;

                            return Ok(None);
                        }
                    }
                    Err(e)
                }
            }
        })();

        let _ = tx.send(result);
    });

    rx.recv()?
}

/// Helper function to check if a JNI error is caused by AEADBadTagException
fn is_aead_bad_tag_exception(env: &mut JNIEnv, _jni_err: &jni::errors::Error) -> bool {
    // Check if there's a pending exception
    if let Ok(true) = env.exception_check() {
        if let Ok(exception) = env.exception_occurred() {
            // Clear the exception so we can continue
            let _ = env.exception_clear();

            // Check if it's an AEADBadTagException
            if let Ok(class) = env.find_class("javax/crypto/AEADBadTagException") {
                if let Ok(is_instance) = env.is_instance_of(&exception, class) {
                    return is_instance;
                }
            }
        }
    }
    false
}

/// Encrypts a string using the keystore-backed secret key
fn encrypt_with_keystore_key(env: &mut JNIEnv, plaintext: &str) -> Result<String> {
    let secret_key = {
        // Get Android Keystore
        let keystore_name = env.new_string("AndroidKeyStore")?;
        let keystore = env
            .call_static_method(
                "java/security/KeyStore",
                "getInstance",
                "(Ljava/lang/String;)Ljava/security/KeyStore;",
                &[JValue::Object(&keystore_name)],
            )?
            .l()?;

        // Load the keystore
        env.call_method(
            &keystore,
            "load",
            "(Ljava/security/KeyStore$LoadStoreParameter;)V",
            &[JValue::Object(&JObject::null())],
        )?;

        let key_alias = env.new_string(KEYSTORE_ALIAS)?;

        // Check if key exists
        let key_exists = env
            .call_method(
                &keystore,
                "containsAlias",
                "(Ljava/lang/String;)Z",
                &[JValue::Object(&key_alias.into())],
            )?
            .z()?;

        if key_exists {
            let keystore_name = env.new_string("AndroidKeyStore")?;
            let keystore = env
                .call_static_method(
                    "java/security/KeyStore",
                    "getInstance",
                    "(Ljava/lang/String;)Ljava/security/KeyStore;",
                    &[JValue::Object(&keystore_name)],
                )?
                .l()?;

            env.call_method(
                &keystore,
                "load",
                "(Ljava/security/KeyStore$LoadStoreParameter;)V",
                &[JValue::Object(&JObject::null())],
            )?;

            let key_alias = env.new_string(KEYSTORE_ALIAS)?;
            env.call_method(
                &keystore,
                "getKey",
                "(Ljava/lang/String;[C)Ljava/security/Key;",
                &[
                    JValue::Object(&key_alias.into()),
                    JValue::Object(&JObject::null()),
                ],
            )?
            .l()?
        } else {
            // Get KeyGenerator instance
            let keystore_name = env.new_string("AndroidKeyStore")?;
            let key_type = env.new_string("AES")?;
            let key_generator = env
                .call_static_method(
                    "javax/crypto/KeyGenerator",
                    "getInstance",
                    "(Ljava/lang/String;Ljava/lang/String;)Ljavax/crypto/KeyGenerator;",
                    &[JValue::Object(&key_type), JValue::Object(&keystore_name)],
                )?
                .l()?;

            // Create KeyGenParameterSpec.Builder
            let key_spec_builder_class =
                env.find_class("android/security/keystore/KeyGenParameterSpec$Builder")?;
            let purposes = 3; // KeyProperties.PURPOSE_ENCRYPT | KeyProperties.PURPOSE_DECRYPT

            let key_alias = env.new_string(KEYSTORE_ALIAS)?;
            let key_spec_builder = env.new_object(
                key_spec_builder_class,
                "(Ljava/lang/String;I)V",
                &[JValue::Object(&key_alias), JValue::Int(purposes)],
            )?;

            // Set encryption paddings
            let paddings = env.new_string("NoPadding")?;
            let padding_array =
                env.new_object_array(1, "java/lang/String", &JObject::from(paddings))?;

            env.call_method(
                &key_spec_builder,
                "setEncryptionPaddings",
                "([Ljava/lang/String;)Landroid/security/keystore/KeyGenParameterSpec$Builder;",
                &[JValue::Object(&padding_array.into())],
            )?;

            // Set block modes
            let modes = env.new_string("GCM")?;
            let modes_array = env.new_object_array(1, "java/lang/String", &JObject::from(modes))?;

            env.call_method(
                &key_spec_builder,
                "setBlockModes",
                "([Ljava/lang/String;)Landroid/security/keystore/KeyGenParameterSpec$Builder;",
                &[JValue::Object(&modes_array.into())],
            )?;

            // Build KeyGenParameterSpec
            let key_spec = env
                .call_method(
                    &key_spec_builder,
                    "build",
                    "()Landroid/security/keystore/KeyGenParameterSpec;",
                    &[],
                )?
                .l()?;

            // Initialize KeyGenerator
            env.call_method(
                &key_generator,
                "init",
                "(Ljava/security/spec/AlgorithmParameterSpec;)V",
                &[JValue::Object(&key_spec)],
            )?;

            // Generate key
            env.call_method(
                &key_generator,
                "generateKey",
                "()Ljavax/crypto/SecretKey;",
                &[],
            )?
            .l()?
        }
    };

    // Get Cipher instance
    let aes_transformation = env.new_string(AES_TRANSFORMATION)?;
    let cipher = env
        .call_static_method(
            "javax/crypto/Cipher",
            "getInstance",
            "(Ljava/lang/String;)Ljavax/crypto/Cipher;",
            &[JValue::Object(&aes_transformation)],
        )?
        .l()?;

    // Initialize cipher for encryption
    let encrypt_mode = 1; // Cipher.ENCRYPT_MODE
    env.call_method(
        &cipher,
        "init",
        "(ILjava/security/Key;)V",
        &[JValue::Int(encrypt_mode), JValue::Object(&secret_key)],
    )?;

    // Get IV
    let iv = env.call_method(&cipher, "getIV", "()[B", &[])?.l()?;

    // Encrypt
    let plaintext_bytes = env.byte_array_from_slice(plaintext.as_bytes())?;
    let ciphertext = env
        .call_method(
            &cipher,
            "doFinal",
            "([B)[B",
            &[JValue::Object(&plaintext_bytes.into())],
        )?
        .l()?;

    // Combine IV and ciphertext
    let iv_bytes = env.convert_byte_array(JByteArray::from(iv))?;
    let ciphertext_bytes = env.convert_byte_array(JByteArray::from(ciphertext))?;

    let mut combined = Vec::new();
    combined.extend_from_slice(&(iv_bytes.len() as u32).to_be_bytes());
    combined.extend_from_slice(&iv_bytes);
    combined.extend_from_slice(&ciphertext_bytes);

    // Encode to Base64
    let base64_class = env.find_class("android/util/Base64")?;
    let combined_array = env.byte_array_from_slice(&combined)?;
    let flags = 2; // Base64.NO_WRAP

    let encoded = env
        .call_static_method(
            base64_class,
            "encodeToString",
            "([BI)Ljava/lang/String;",
            &[JValue::Object(&combined_array.into()), JValue::Int(flags)],
        )?
        .l()?;

    let encoded_str: String = env.get_string(&JString::from(encoded))?.into();
    Ok(encoded_str)
}

/// Decrypts a string using the keystore-backed secret key
fn decrypt_with_keystore_key(env: &mut JNIEnv, encrypted_data: JObject) -> Result<String> {
    let secret_key = {
        // Get Android Keystore
        let keystore_name = env.new_string("AndroidKeyStore")?;
        let keystore = env
            .call_static_method(
                "java/security/KeyStore",
                "getInstance",
                "(Ljava/lang/String;)Ljava/security/KeyStore;",
                &[JValue::Object(&keystore_name)],
            )?
            .l()?;

        // Load the keystore
        env.call_method(
            &keystore,
            "load",
            "(Ljava/security/KeyStore$LoadStoreParameter;)V",
            &[JValue::Object(&JObject::null())],
        )?;

        let key_alias = env.new_string(KEYSTORE_ALIAS)?;

        // Check if key exists
        let key_exists = env
            .call_method(
                &keystore,
                "containsAlias",
                "(Ljava/lang/String;)Z",
                &[JValue::Object(&key_alias.into())],
            )?
            .z()?;

        if key_exists {
            let keystore_name = env.new_string("AndroidKeyStore")?;
            let keystore = env
                .call_static_method(
                    "java/security/KeyStore",
                    "getInstance",
                    "(Ljava/lang/String;)Ljava/security/KeyStore;",
                    &[JValue::Object(&keystore_name)],
                )?
                .l()?;

            env.call_method(
                &keystore,
                "load",
                "(Ljava/security/KeyStore$LoadStoreParameter;)V",
                &[JValue::Object(&JObject::null())],
            )?;

            let key_alias = env.new_string(KEYSTORE_ALIAS)?;
            env.call_method(
                &keystore,
                "getKey",
                "(Ljava/lang/String;[C)Ljava/security/Key;",
                &[
                    JValue::Object(&key_alias.into()),
                    JValue::Object(&JObject::null()),
                ],
            )?
            .l()?
        } else {
            // Get KeyGenerator instance
            let keystore_name = env.new_string("AndroidKeyStore")?;
            let key_type = env.new_string("AES")?;
            let key_generator = env
                .call_static_method(
                    "javax/crypto/KeyGenerator",
                    "getInstance",
                    "(Ljava/lang/String;Ljava/lang/String;)Ljavax/crypto/KeyGenerator;",
                    &[JValue::Object(&key_type), JValue::Object(&keystore_name)],
                )?
                .l()?;

            // Create KeyGenParameterSpec.Builder
            let key_spec_builder_class =
                env.find_class("android/security/keystore/KeyGenParameterSpec$Builder")?;
            let purposes = 3; // KeyProperties.PURPOSE_ENCRYPT | KeyProperties.PURPOSE_DECRYPT

            let key_alias = env.new_string(KEYSTORE_ALIAS)?;
            let key_spec_builder = env.new_object(
                key_spec_builder_class,
                "(Ljava/lang/String;I)V",
                &[JValue::Object(&key_alias), JValue::Int(purposes)],
            )?;

            // Set encryption paddings
            let paddings = env.new_string("NoPadding")?;
            let padding_array =
                env.new_object_array(1, "java/lang/String", &JObject::from(paddings))?;

            env.call_method(
                &key_spec_builder,
                "setEncryptionPaddings",
                "([Ljava/lang/String;)Landroid/security/keystore/KeyGenParameterSpec$Builder;",
                &[JValue::Object(&padding_array.into())],
            )?;

            // Set block modes
            let modes = env.new_string("GCM")?;
            let modes_array = env.new_object_array(1, "java/lang/String", &JObject::from(modes))?;

            env.call_method(
                &key_spec_builder,
                "setBlockModes",
                "([Ljava/lang/String;)Landroid/security/keystore/KeyGenParameterSpec$Builder;",
                &[JValue::Object(&modes_array.into())],
            )?;

            // Build KeyGenParameterSpec
            let key_spec = env
                .call_method(
                    &key_spec_builder,
                    "build",
                    "()Landroid/security/keystore/KeyGenParameterSpec;",
                    &[],
                )?
                .l()?;

            // Initialize KeyGenerator
            env.call_method(
                &key_generator,
                "init",
                "(Ljava/security/spec/AlgorithmParameterSpec;)V",
                &[JValue::Object(&key_spec)],
            )?;

            // Generate key
            env.call_method(
                &key_generator,
                "generateKey",
                "()Ljavax/crypto/SecretKey;",
                &[],
            )?
            .l()?
        }
    };

    // Decode from Base64
    let base64_class = env.find_class("android/util/Base64")?;
    let flags = 2; // Base64.NO_WRAP

    let decoded = env
        .call_static_method(
            base64_class,
            "decode",
            "(Ljava/lang/String;I)[B",
            &[JValue::Object(&encrypted_data), JValue::Int(flags)],
        )?
        .l()?;

    let combined_bytes = env.convert_byte_array(JByteArray::from(decoded))?;

    // Extract IV length, IV, and ciphertext
    let iv_len = u32::from_be_bytes([
        combined_bytes[0],
        combined_bytes[1],
        combined_bytes[2],
        combined_bytes[3],
    ]) as usize;

    let iv = &combined_bytes[4..4 + iv_len];
    let ciphertext = &combined_bytes[4 + iv_len..];

    // Get Cipher instance
    let aes_transformation = env.new_string(AES_TRANSFORMATION)?;
    let cipher = env
        .call_static_method(
            "javax/crypto/Cipher",
            "getInstance",
            "(Ljava/lang/String;)Ljavax/crypto/Cipher;",
            &[JValue::Object(&aes_transformation)],
        )?
        .l()?;

    // Create GCMParameterSpec
    let gcm_spec_class = env.find_class("javax/crypto/spec/GCMParameterSpec")?;
    let iv_array = env.byte_array_from_slice(iv)?;
    let gcm_spec = env.new_object(
        gcm_spec_class,
        "(I[B)V",
        &[
            JValue::Int(GCM_TAG_LENGTH),
            JValue::Object(&iv_array.into()),
        ],
    )?;

    // Initialize cipher for decryption
    let decrypt_mode = 2; // Cipher.DECRYPT_MODE
    env.call_method(
        &cipher,
        "init",
        "(ILjava/security/Key;Ljava/security/spec/AlgorithmParameterSpec;)V",
        &[
            JValue::Int(decrypt_mode),
            JValue::Object(&secret_key),
            JValue::Object(&gcm_spec),
        ],
    )?;

    // Decrypt - this is where AEADBadTagException can occur
    let ciphertext_array = env.byte_array_from_slice(ciphertext)?;

    // Check for exception after doFinal
    let plaintext_result = env.call_method(
        &cipher,
        "doFinal",
        "([B)[B",
        &[JValue::Object(&ciphertext_array.into())],
    );

    // If there's an exception, propagate it
    if let Err(e) = plaintext_result {
        return Err(SecureStorageError::Jni(e));
    }

    let plaintext = plaintext_result?.l()?;

    // Convert to string
    let plaintext_bytes = env.convert_byte_array(JByteArray::from(plaintext))?;
    Ok(String::from_utf8(plaintext_bytes)?)
}
