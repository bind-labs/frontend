mod android;

pub fn open_email() -> Result<(), String> {
    android::open_email()
}
