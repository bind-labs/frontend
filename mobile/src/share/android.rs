//! Android Sharing implementation using Android Sharesheet.

use dioxus::mobile::wry::prelude::{dispatch, find_class};
use jni::objects::{JObject, JString, JValue};
use jni::JNIEnv;
use std::borrow::Borrow; // Needed for potentially borrowing JObject refs

pub fn share_link(link: String, title: String) -> Result<(), String> {
    // Java code that I am looking to implement
    // Intent sendIntent = new Intent();
    // sendIntent.setAction(Intent.ACTION_SEND);
    // sendIntent.putExtra(Intent.EXTRA_TEXT, "This is my text to send."); // Use link here
    // sendIntent.setType("text/plain");

    // // Use title for the chooser title
    // Intent shareIntent = Intent.createChooser(sendIntent, title);
    // startActivity(shareIntent);

    // Dispatch the JNI operations to the main Android thread
    dispatch(move |env: &mut JNIEnv, activity: &JObject, _webview| {
        // Use a closure to handle potential JNI errors cleanly
        let result: Result<(), jni::errors::Error> = (|| {
            // 1. Find the Intent class
            let intent_class = env.find_class("android/content/Intent")?;

            // 2. Create sendIntent = new Intent()
            let send_intent = env.new_object(&intent_class, "()V", &[])?;

            // 3. Get Intent.ACTION_SEND static field
            //    Signature "Ljava/lang/String;"
            let action_send_field =
                env.get_static_field(&intent_class, "ACTION_SEND", "Ljava/lang/String;")?;
            let action_send_jstring: JObject = action_send_field.l()?; // Get the JObject reference

            // 4. Call sendIntent.setAction(Intent.ACTION_SEND)
            //    Method signature "(Ljava/lang/String;)Landroid/content/Intent;"
            env.call_method(
                &send_intent,
                "setAction",
                "(Ljava/lang/String;)Landroid/content/Intent;",
                &[JValue::Object(&action_send_jstring)],
            )?
            .l()?; // We need to extract the result even if discarding, to check for exceptions

            // 5. Get Intent.EXTRA_TEXT static field
            //    Signature "Ljava/lang/String;"
            let extra_text_field =
                env.get_static_field(&intent_class, "EXTRA_TEXT", "Ljava/lang/String;")?;
            let extra_text_jstring: JObject = extra_text_field.l()?;

            // Create JString for the link content
            let link_jstring: JString = env.new_string(&link)?;

            // 6. Call sendIntent.putExtra(Intent.EXTRA_TEXT, link)
            //    Method signature "(Ljava/lang/String;Ljava/lang/String;)Landroid/content/Intent;"
            env.call_method(
                &send_intent,
                "putExtra",
                "(Ljava/lang/String;Ljava/lang/String;)Landroid/content/Intent;",
                &[
                    JValue::Object(&extra_text_jstring),
                    JValue::Object(link_jstring.borrow()),
                ], // Borrow JString as JObject
            )?
            .l()?;

            // 7. Create JString for the MIME type "text/plain"
            let mime_type_jstring: JString = env.new_string("text/plain")?;

            // 8. Call sendIntent.setType("text/plain")
            //    Method signature "(Ljava/lang/String;)Landroid/content/Intent;"
            env.call_method(
                &send_intent,
                "setType",
                "(Ljava/lang/String;)Landroid/content/Intent;",
                &[JValue::Object(mime_type_jstring.borrow())],
            )?
            .l()?;

            // Create JString for the chooser title
            let title_jstring: JString = env.new_string(&title)?;

            // 9. Create the chooser intent: Intent shareIntent = Intent.createChooser(sendIntent, title);
            //    Static method signature "(Landroid/content/Intent;Ljava/lang/CharSequence;)Landroid/content/Intent;"
            //    Note: JString implements CharSequence
            let share_intent_result = env.call_static_method(
                &intent_class, // Call on the class for static method
                "createChooser",
                "(Landroid/content/Intent;Ljava/lang/CharSequence;)Landroid/content/Intent;",
                &[
                    JValue::Object(&send_intent),
                    JValue::Object(title_jstring.borrow()),
                ],
            )?;
            let share_intent: JObject = share_intent_result.l()?; // Get the chooser Intent JObject

            // 10. Call activity.startActivity(shareIntent)
            //     Method signature "(Landroid/content/Intent;)V" (void return)
            env.call_method(
                activity, // The Activity object provided by dispatch
                "startActivity",
                "(Landroid/content/Intent;)V",
                &[JValue::Object(&share_intent)],
            )?
            .v()?; // Use .v() for void methods

            Ok(()) // Indicate success within the inner closure
        })(); // Execute the inner closure

        // Handle the result of the JNI operations
        if let Err(e) = result {
            // Log the error or handle it appropriately
            // Using log crate is common in Rust projects
            eprintln!("JNI Error creating share intent: {}", e);
            // Optionally, you could try to send this error back to the Rust part
            // of the app if needed, but dispatch doesn't directly support returning values.
        }
    });

    // TODO:return error
    Ok(())
}
