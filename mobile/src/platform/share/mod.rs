#[cfg(target_os = "android")]
mod android;
#[cfg(target_os = "ios")]
mod ios;

pub fn share_feed_item(link: String, title: String) -> Result<(), String> {
    #[cfg(target_os = "android")]
    android::share_link(link, title);
    #[cfg(target_os = "ios")]
    ios::share_link(link, title);
    Ok(())
}
