use dioxus::{mobile::window, prelude::*};

// TODO: clean up event listener
pub fn use_keyboard_open() -> Signal<bool> {
    let mut keyboard_open = use_signal(|| false);

    use_future(move || async move {
        let window = &window().window;

        let mut eval = document::eval(
            r#"
            dioxus.send(window.innerHeight * window.devicePixelRatio);
            window.addEventListener('resize', function() {
                dioxus.send(window.innerHeight * window.devicePixelRatio);
            });
            "#,
        );

        loop {
            let window_size = window.inner_size().height as f32;
            let viewport_size = eval.recv::<f32>().await.unwrap();

            // TODO: base it on the viewport size diff, rather than percent
            // Need devicePixelRatio to be accurate
            keyboard_open.set((viewport_size / window_size) < 0.75);
        }
    });

    keyboard_open
}
