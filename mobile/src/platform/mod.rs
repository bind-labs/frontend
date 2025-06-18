use dioxus::prelude::*;
use std::sync::atomic::AtomicBool;

mod decor;
mod email;
mod gesture;
mod share;
mod storage;

pub use email::open_email;
pub use gesture::init_back_press_listener;
pub use share::share_feed_item;
pub use storage::SecureStore;

static HAS_SETUP: AtomicBool = AtomicBool::new(false);

pub fn use_platform_setup() {
    use_effect(|| {
        if HAS_SETUP.load(std::sync::atomic::Ordering::SeqCst) {
            return;
        }
        HAS_SETUP.store(true, std::sync::atomic::Ordering::SeqCst);

        decor::setup_decor();
    });
}
