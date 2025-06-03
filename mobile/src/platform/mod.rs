use dioxus::prelude::*;
use std::sync::atomic::AtomicBool;

static HAS_SETUP: AtomicBool = AtomicBool::new(false);

mod decor;
mod email;
mod share;
mod storage;

pub use email::open_email;
pub use share::share_feed_item;
pub use storage::{use_persistent, UsePersistent};

pub fn use_platform_setup() {
    use_effect(|| {
        if HAS_SETUP.load(std::sync::atomic::Ordering::SeqCst) {
            return;
        }
        HAS_SETUP.store(true, std::sync::atomic::Ordering::SeqCst);

        decor::setup_decor();
    });
}
