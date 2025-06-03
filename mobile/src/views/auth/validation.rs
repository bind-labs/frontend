use std::sync::LazyLock;

static EMAIL_REGEX: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r"^\S+@\S+\.\S+$").unwrap());

static USERNAME_REGEX: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r"^[a-zA-Z0-9_\.-]{2,48}$").unwrap());

pub fn validate_email(email: &str) -> Result<(), String> {
    if email.is_empty() {
        return Err("Email cannot be empty".to_string());
    }

    if !EMAIL_REGEX.is_match(email) {
        return Err("Email is invalid".to_string());
    }

    Ok(())
}

pub fn validate_username(username: &str) -> Result<(), String> {
    if username.is_empty() {
        return Err("Username cannot be empty".to_string());
    }
    if username.len() < 2 {
        return Err("Username must be at least 2 characters long".to_string());
    }
    if username.len() > 48 {
        return Err("Username must be at most 48 characters long".to_string());
    }

    if !USERNAME_REGEX.is_match(username) {
        return Err(
            "Username must contain only letters, numbers, dots, dashes, and underscores"
                .to_string(),
        );
    }

    Ok(())
}

pub fn validate_password(password: &str) -> Result<(), String> {
    if password.is_empty() {
        return Err("Password cannot be empty".to_string());
    }
    if password.len() < 12 {
        return Err("Password must be at least 12 characters long".to_string());
    }
    if password.len() > 128 {
        return Err("Password must be at most 128 characters long".to_string());
    }

    // Ensure it contains at least one uppercase and one lowercase letter
    if !password.chars().any(|c| c.is_ascii_uppercase())
        || !password.chars().any(|c| c.is_ascii_lowercase())
    {
        return Err(
            "Password must contain at least one uppercase and one lowercase letter".to_string(),
        );
    }

    Ok(())
}
