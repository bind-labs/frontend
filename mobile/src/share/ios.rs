use dioxus::document;

pub fn share_link(link: String, title: String) -> Result<(), String> {
    document::eval(&format!(
        r#"await navigator.share({{
            url: {link},
            title: {title}
        }})"#
    ));
    Ok(())
}
