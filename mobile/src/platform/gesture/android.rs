use jni::{objects::JClass, JNIEnv};
use std::sync::mpsc;
use std::sync::OnceLock;

// Channel for sending back press events
static BACK_PRESS_SENDER: OnceLock<mpsc::Sender<()>> = OnceLock::new();

// Initialize the back press listener with a channel
pub fn init_back_press_listener() -> mpsc::Receiver<()> {
    let (tx, rx) = mpsc::channel();
    BACK_PRESS_SENDER
        .set(tx)
        .expect("Back press listener already initialized");
    rx
}

// JNI function that gets called from Kotlin via `MainActivity.onBackPressed_native()`
#[no_mangle]
pub extern "C" fn Java_dev_dioxus_main_MainActivity_onBackPressed_1native(
    _env: JNIEnv,
    _class: JClass,
) {
    if let Some(sender) = BACK_PRESS_SENDER.get() {
        let _ = sender.send(());
    }
}
