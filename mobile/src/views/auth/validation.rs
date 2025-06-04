use std::sync::LazyLock;

use zxcvbn::Score;

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

pub fn validate_password(email: &str, username: &str, password: &str) -> Result<(), String> {
    if password.is_empty() {
        return Err("Password cannot be empty".to_string());
    }
    if password.len() < 8 || password.len() > 128 {
        return Err("Password must be between 8 and 128 characters long".to_string());
    }

    let password_estimate = zxcvbn::zxcvbn(&password, &[&username, &email]);
    if password_estimate.score() <= Score::Two {
        if let Some(feedback) = password_estimate.feedback() {
            return Err(format!(
                "{}. {}",
                feedback
                    .warning()
                    .map(|x| x.to_string())
                    .unwrap_or("Password is too weak".to_string()),
                feedback
                    .suggestions()
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(". ")
            ));
        } else {
            return Err("Password is too weak, try adding more letters, numbers and symbols. Avoid common words and phrases.".to_string());
        }
    }

    Ok(())
}
