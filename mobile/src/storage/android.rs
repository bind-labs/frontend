// use dioxus::mobile::wry::prelude::dispatch;
// use std::sync::mpsc;

// // Function to securely store a key-value pair
// pub fn secure_store(key: &str, value: &str) -> () {
//     let key_str = key.to_string();
//     let value_str = value.to_string();

//     let (tx, rx) = mpsc::channel();

//     dispatch(move |env, activity, _webview| {
//         let result = (|| {
//             // Get the context
//             let context = activity.clone();

//             // Get the security library classes
//             let master_key_class = env
//                 .find_class("androidx/security/crypto/MasterKey$Builder")
//                 .unwrap();
//             let encrypted_prefs_class = env
//                 .find_class("androidx/security/crypto/EncryptedSharedPreferences")
//                 .unwrap();

//             // Create master key
//             let master_key_builder = env
//                 .new_object(
//                     &master_key_class,
//                     "(Landroid/content/Context;)V",
//                     &[context.into()],
//                 )
//                 .unwrap();

//             // Set key parameters
//             env.call_method(
//                 &master_key_builder,
//                 "setKeyScheme",
//                 "(I)Landroidx/security/crypto/MasterKey$Builder;",
//                 &[3i32.into()], // MasterKey.KeyScheme.AES256_GCM
//             )
//             .unwrap();

//             let master_key = env
//                 .call_method(
//                     &master_key_builder,
//                     "build",
//                     "()Landroidx/security/crypto/MasterKey;",
//                     &[],
//                 )
//                 .unwrap()
//                 .l()
//                 .unwrap();

//             // Get file name for preferences
//             let pref_file = env.new_string("secure_prefs").unwrap();

//             // Get encryption schemes
//             let aes256_scheme = env
//                 .get_static_field(
//                     "androidx/security/crypto/EncryptedSharedPreferences$PrefKeyEncryptionScheme",
//                     "AES256_SIV",
//                     "Landroidx/security/crypto/EncryptedSharedPreferences$PrefKeyEncryptionScheme;",
//                 )
//                 .unwrap()
//                 .l()
//                 .unwrap();

//             let aes256_value_scheme = env.get_static_field(
//                   "androidx/security/crypto/EncryptedSharedPreferences$PrefValueEncryptionScheme",
//                   "AES256_GCM",
//                   "Landroidx/security/crypto/EncryptedSharedPreferences$PrefValueEncryptionScheme;"
//               ).unwrap().l().unwrap();

//             // Create encrypted preferences
//             let encrypted_prefs = env.call_static_method(
//                   &encrypted_prefs_class,
//                   "create",
//                   "(Landroid/content/Context;Ljava/lang/String;Landroidx/security/crypto/MasterKey;Landroidx/security/crypto/EncryptedSharedPreferences$PrefKeyEncryptionScheme;Landroidx/security/crypto
//   /EncryptedSharedPreferences$PrefValueEncryptionScheme;)Landroid/content/SharedPreferences;",
//                   &[
//                       context.into(),
//                       (&pref_file).into(),
//                       (&master_key).into(),
//                       (&aes256_scheme).into(),
//                       (&aes256_value_scheme).into()
//                   ]
//               ).unwrap().l().unwrap();

//             // Edit preferences
//             let editor = env
//                 .call_method(
//                     &encrypted_prefs,
//                     "edit",
//                     "()Landroid/content/SharedPreferences$Editor;",
//                     &[],
//                 )
//                 .unwrap()
//                 .l()
//                 .unwrap();

//             // Put the value
//             let key_jstring = env.new_string(&key_str).unwrap();
//             let value_jstring = env.new_string(&value_str).unwrap();

//             env.call_method(
//                 &editor,
//                 "putString",
//                 "(Ljava/lang/String;Ljava/lang/String;)Landroid/content/SharedPreferences$Editor;",
//                 &[(&key_jstring).into(), (&value_jstring).into()],
//             )
//             .unwrap();

//             // Apply changes
//             env.call_method(&editor, "apply", "()V", &[]).unwrap();
//         })();

//         // Send result back
//         tx.send(()).unwrap();
//     });

//     rx.recv().unwrap()
// }

// // Function to retrieve a securely stored value
// pub fn secure_retrieve(key: &str) -> Option<String> {
//     let key_str = key.to_string();
//     let (tx, rx) = mpsc::channel();

//     dispatch(move |env, activity, _webview| {
//         let result = (|| {
//             // Get the context
//             let context = activity.clone();

//             // Get the security library classes
//             let master_key_class = env
//                 .find_class("androidx/security/crypto/MasterKey$Builder")
//                 .unwrap();
//             let encrypted_prefs_class = env
//                 .find_class("androidx/security/crypto/EncryptedSharedPreferences")
//                 .unwrap();

//             // Create master key
//             let master_key_builder = env
//                 .new_object(
//                     &master_key_class,
//                     "(Landroid/content/Context;)V",
//                     &[context.into()],
//                 )
//                 .unwrap();

//             let master_key = env
//                 .call_method(
//                     &master_key_builder,
//                     "build",
//                     "()Landroidx/security/crypto/MasterKey;",
//                     &[],
//                 )
//                 .unwrap()
//                 .l()
//                 .unwrap();

//             // Get file name for preferences
//             let pref_file = env.new_string("secure_prefs").unwrap();

//             // Get encryption schemes
//             let aes256_scheme = env
//                 .get_static_field(
//                     "androidx/security/crypto/EncryptedSharedPreferences$PrefKeyEncryptionScheme",
//                     "AES256_SIV",
//                     "Landroidx/security/crypto/EncryptedSharedPreferences$PrefKeyEncryptionScheme;",
//                 )
//                 .unwrap()
//                 .l()
//                 .unwrap();

//             let aes256_value_scheme = env.get_static_field(
//                   "androidx/security/crypto/EncryptedSharedPreferences$PrefValueEncryptionScheme",
//                   "AES256_GCM",
//                   "Landroidx/security/crypto/EncryptedSharedPreferences$PrefValueEncryptionScheme;"
//               ).unwrap().l().unwrap();

//             // Create encrypted preferences
//             let encrypted_prefs = env.call_static_method(
//                   &encrypted_prefs_class,
//                   "create",
//                   "(Landroid/content/Context;Ljava/lang/String;Landroidx/security/crypto/MasterKey;Landroidx/security/crypto/EncryptedSharedPreferences$PrefKeyEncryptionScheme;Landroidx/security/crypto
//   /EncryptedSharedPreferences$PrefValueEncryptionScheme;)Landroid/content/SharedPreferences;",
//                   &[
//                       context.into(),
//                       (&pref_file).into(),
//                       (&master_key).into(),
//                       (&aes256_scheme).into(),
//                       (&aes256_value_scheme).into()
//                   ]
//               ).unwrap().l().unwrap();

//             // Get the value
//             let key_jstring = env.new_string(&key_str).unwrap();
//             let default_value = env.new_string("").unwrap();

//             let value = env
//                 .call_method(
//                     &encrypted_prefs,
//                     "getString",
//                     "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;",
//                     &[(&key_jstring).into(), (&default_value).into()],
//                 )
//                 .unwrap()
//                 .l()
//                 .unwrap();

//             // Convert to Rust String
//             if value.is_null() {
//                 None
//             } else {
//                 let value_str = env
//                     .get_string((&value).into())
//                     .unwrap()
//                     .to_string_lossy()
//                     .to_string();
//                 if value_str.is_empty() {
//                     None
//                 } else {
//                     Some(value_str)
//                 }
//             }
//         })();

//         // Send result back
//         tx.send(result).unwrap();
//     });

//     rx.recv().unwrap()
// }
