mod android;

pub fn share_feed_item(link: String, title: String) -> Result<(), String> {
    if cfg!(target_os = "android") {
        android::share_link(link, title)
    } else {
        Ok(())
    }
}
