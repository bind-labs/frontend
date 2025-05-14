use std::sync::atomic::AtomicBool;

static HAS_SETUP: AtomicBool = AtomicBool::new(false);

#[cfg(target_os = "android")]
pub fn setup_platform() {
    use dioxus::mobile::wry::prelude::dispatch;

    if HAS_SETUP.load(std::sync::atomic::Ordering::SeqCst) {
        return;
    }

    // TODO: remove unwraps
    dispatch(|env, activity, _webview| {
        // Get the window
        let window = env
            .call_method(activity, "getWindow", "()Landroid/view/Window;", &[])
            .unwrap()
            .l()
            .unwrap();

        // Set window flags
        const FLAG_KEEP_SCREEN_ON: i32 = 128;
        env.call_method(&window, "addFlags", "(I)V", &[FLAG_KEEP_SCREEN_ON.into()])
            .unwrap();

        // Use dark icons for status bar
        let insets_controller = env
            .call_method(
                &window,
                "getInsetsController",
                "()Landroid/view/WindowInsetsController;",
                &[],
            )
            .unwrap()
            .l()
            .unwrap();

        if !insets_controller.is_null() {
            let appearance_flag = 0x00000008; // APPEARANCE_LIGHT_STATUS_BARS
            env.call_method(
                &insets_controller,
                "setSystemBarsAppearance",
                "(II)V",
                &[appearance_flag.into(), appearance_flag.into()],
            )
            .unwrap();
        }

        // Set status bar color
        let color = 0xFFF2EDE3u32 as i32; // ARGB
        env.call_method(&window, "setStatusBarColor", "(I)V", &[color.into()])
            .unwrap();

        // Set navigation bar color
        env.call_method(&window, "setNavigationBarColor", "(I)V", &[color.into()])
            .unwrap();
    });

    if !HAS_SETUP.load(std::sync::atomic::Ordering::SeqCst) {
        HAS_SETUP.store(true, std::sync::atomic::Ordering::SeqCst);
    }
}

#[cfg(target_os = "ios")]
pub fn setup_platform() {
    if !HAS_SETUP.load(std::sync::atomic::Ordering::SeqCst) {
        HAS_SETUP.store(true, std::sync::atomic::Ordering::SeqCst);
    }
}
