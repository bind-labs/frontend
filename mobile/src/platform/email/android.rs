use std::sync::mpsc;

pub use dioxus::mobile::wry::prelude::{dispatch, find_class};
use jni::{
    errors::Error,
    objects::{JObject, JValue},
    JNIEnv,
};

pub fn open_email() -> Result<(), String> {
    let (tx, rx) = mpsc::channel::<Result<(), Error>>();

    dispatch(move |env: &mut JNIEnv, activity: &JObject, _webview| {
        let result = (|| -> Result<(), Error> {
            // Create an Intent with ACTION_MAIN
            let intent_class = env.find_class("android/content/Intent")?;
            let intent = env.new_object(&intent_class, "()V", &[])?;

            // Set the action to ACTION_MAIN
            let action_main = env
                .get_static_field(&intent_class, "ACTION_MAIN", "Ljava/lang/String;")?
                .l()?;

            env.call_method(
                &intent,
                "setAction",
                "(Ljava/lang/String;)Landroid/content/Intent;",
                &[JValue::Object(&action_main)],
            )?;

            // Add category CATEGORY_APP_EMAIL
            let category_email = env.new_string("android.intent.category.APP_EMAIL")?;
            env.call_method(
                &intent,
                "addCategory",
                "(Ljava/lang/String;)Landroid/content/Intent;",
                &[JValue::Object(&JObject::from(category_email))],
            )?;

            // Add FLAG_ACTIVITY_NEW_TASK
            let flag_new_task = 0x10000000i32; // FLAG_ACTIVITY_NEW_TASK
            env.call_method(
                &intent,
                "addFlags",
                "(I)Landroid/content/Intent;",
                &[JValue::Int(flag_new_task)],
            )?;

            // Start the activity
            env.call_method(
                activity,
                "startActivity",
                "(Landroid/content/Intent;)V",
                &[JValue::Object(&intent)],
            )?;

            Ok(())
        })();

        tx.send(result);
    });

    let recv_value = rx.recv();
    match recv_value {
        Ok(Ok(_)) => Ok(()),
        Ok(Err(err)) => {
            tracing::error!("JNI failed while opening email: {err}");
            Err(err.to_string())
        }
        Err(err) => {
            tracing::error!("Failed while getting result from open_email: {err}");
            Err(err.to_string())
        }
    }
}
