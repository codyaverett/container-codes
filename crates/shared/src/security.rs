use crate::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKey {
    pub id: String,
    pub name: String,
    pub key_hash: String,
    pub permissions: Vec<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
    pub last_used: Option<chrono::DateTime<chrono::Utc>>,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtClaims {
    pub sub: String,
    pub iat: i64,
    pub exp: i64,
    pub permissions: Vec<String>,
    pub session_id: String,
}

#[derive(Debug, Clone)]
pub struct SecurityContext {
    pub user_id: Option<String>,
    pub session_id: Option<String>,
    pub permissions: Vec<String>,
    pub api_key_id: Option<String>,
    pub request_id: String,
}

impl SecurityContext {
    pub fn new() -> Self {
        Self {
            user_id: None,
            session_id: None,
            permissions: vec![],
            api_key_id: None,
            request_id: Uuid::new_v4().to_string(),
        }
    }

    pub fn with_api_key(api_key: &ApiKey) -> Self {
        Self {
            user_id: None,
            session_id: None,
            permissions: api_key.permissions.clone(),
            api_key_id: Some(api_key.id.clone()),
            request_id: Uuid::new_v4().to_string(),
        }
    }

    pub fn with_jwt(claims: &JwtClaims) -> Self {
        Self {
            user_id: Some(claims.sub.clone()),
            session_id: Some(claims.session_id.clone()),
            permissions: claims.permissions.clone(),
            api_key_id: None,
            request_id: Uuid::new_v4().to_string(),
        }
    }

    pub fn has_permission(&self, permission: &str) -> bool {
        self.permissions.contains(&permission.to_string()) || 
        self.permissions.contains(&"admin".to_string())
    }

    pub fn require_permission(&self, permission: &str) -> Result<()> {
        if self.has_permission(permission) {
            Ok(())
        } else {
            Err(crate::Error::auth(format!(
                "Permission '{}' required",
                permission
            )))
        }
    }
}

impl Default for SecurityContext {
    fn default() -> Self {
        Self::new()
    }
}

pub fn hash_api_key(key: &str) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(key.as_bytes());
    format!("{:x}", hasher.finalize())
}

pub fn generate_api_key() -> String {
    format!("cc_{}", Uuid::new_v4().simple())
}

pub fn validate_password(password: &str) -> Result<()> {
    if password.len() < 8 {
        return Err(crate::Error::validation(
            "Password must be at least 8 characters long",
        ));
    }

    let has_uppercase = password.chars().any(|c| c.is_uppercase());
    let has_lowercase = password.chars().any(|c| c.is_lowercase());
    let has_digit = password.chars().any(|c| c.is_ascii_digit());
    let has_special = password.chars().any(|c| "!@#$%^&*()_+-=[]{}|;:,.<>?".contains(c));

    if !has_uppercase {
        return Err(crate::Error::validation(
            "Password must contain at least one uppercase letter",
        ));
    }

    if !has_lowercase {
        return Err(crate::Error::validation(
            "Password must contain at least one lowercase letter",
        ));
    }

    if !has_digit {
        return Err(crate::Error::validation(
            "Password must contain at least one digit",
        ));
    }

    if !has_special {
        return Err(crate::Error::validation(
            "Password must contain at least one special character",
        ));
    }

    Ok(())
}

pub fn sanitize_filename(filename: &str) -> String {
    filename
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '.' || *c == '_' || *c == '-')
        .collect()
}

pub fn validate_container_name(name: &str) -> Result<()> {
    if name.is_empty() {
        return Err(crate::Error::validation("Container name cannot be empty"));
    }

    if name.len() > 63 {
        return Err(crate::Error::validation(
            "Container name cannot be longer than 63 characters",
        ));
    }

    if !name
        .chars()
        .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
    {
        return Err(crate::Error::validation(
            "Container name can only contain alphanumeric characters, hyphens, and underscores",
        ));
    }

    if name.starts_with('-') || name.ends_with('-') {
        return Err(crate::Error::validation(
            "Container name cannot start or end with a hyphen",
        ));
    }

    Ok(())
}

pub fn validate_image_name(image: &str) -> Result<()> {
    if image.is_empty() {
        return Err(crate::Error::validation("Image name cannot be empty"));
    }

    // Basic validation - in production you might want more comprehensive validation
    if image.contains("..") || image.contains("//") {
        return Err(crate::Error::validation("Invalid image name format"));
    }

    Ok(())
}

pub fn validate_environment_variables(env: &HashMap<String, String>) -> Result<()> {
    for (key, value) in env {
        if key.is_empty() {
            return Err(crate::Error::validation(
                "Environment variable name cannot be empty",
            ));
        }

        if !key
            .chars()
            .all(|c| c.is_alphanumeric() || c == '_')
        {
            return Err(crate::Error::validation(
                "Environment variable names can only contain alphanumeric characters and underscores",
            ));
        }

        if key.starts_with(char::is_numeric) {
            return Err(crate::Error::validation(
                "Environment variable names cannot start with a number",
            ));
        }

        // Check for potentially sensitive values
        let sensitive_patterns = [
            "password", "secret", "key", "token", "credential", "private",
        ];
        
        if sensitive_patterns.iter().any(|pattern| {
            key.to_lowercase().contains(pattern) || value.to_lowercase().contains(pattern)
        }) {
            tracing::warn!(
                env_var = key,
                "Potentially sensitive environment variable detected"
            );
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_password() {
        assert!(validate_password("Password123!").is_ok());
        assert!(validate_password("short").is_err());
        assert!(validate_password("nouppercase123!").is_err());
        assert!(validate_password("NOLOWERCASE123!").is_err());
        assert!(validate_password("NoDigits!").is_err());
        assert!(validate_password("NoSpecial123").is_err());
    }

    #[test]
    fn test_validate_container_name() {
        assert!(validate_container_name("my-container").is_ok());
        assert!(validate_container_name("my_container_123").is_ok());
        assert!(validate_container_name("").is_err());
        assert!(validate_container_name("-invalid").is_err());
        assert!(validate_container_name("invalid-").is_err());
        assert!(validate_container_name("invalid@name").is_err());
    }

    #[test]
    fn test_sanitize_filename() {
        assert_eq!(sanitize_filename("test file.txt"), "testfile.txt");
        assert_eq!(sanitize_filename("file@#$%.pdf"), "file.pdf");
        assert_eq!(sanitize_filename("my_file-2.txt"), "my_file-2.txt");
    }
}