#[cfg(test)]
mod tests {
    use std::future::Future;
    use std::pin::Pin;
    use std::ptr::eq;
    use std::sync::{Arc, Mutex};

    use argon2::{
        Argon2,
        password_hash::{PasswordHasher, SaltString, rand_core},
    };

    use crate::app::entities::user::User;
    use crate::app::repositories::mock_user_repository::MockUserRepository;
    use crate::app::repositories::user_repository::UserRepository;
    use crate::app::use_cases::create_user_session_use_case::{
        CreateUserSessionError, CreateUserSessionRequest, CreateUserSessionUseCase,
    };

    fn setup() {
        unsafe {
            std::env::set_var("JWT_SECRET", "test_secret_key_for_sessions");
        }
    }

    #[actix_web::test]
    async fn test_execute_with_valid_credentials_should_return_token() {
        // Arrange
        setup();
        let mock_repo = Arc::new(MockUserRepository::new());
        let email = "test@example.com".to_string();
        let password = "password123".to_string();

        let user = User::new(email.clone(), "testuser".to_string(), password.clone());
        mock_repo.add_user(user);

        let use_case = CreateUserSessionUseCase::new(mock_repo.clone());
        let request = CreateUserSessionRequest::new(email.clone(), password);

        // Act
        let result = use_case.execute(request).await;

        // Assert
        let response = result.expect("Expected a valid token response");
        assert!(!response.token.is_empty());
    }

    #[actix_web::test]
    async fn test_execute_with_invalid_password_should_return_error() {
        // Arrange
        setup();
        let mock_repo = Arc::new(MockUserRepository::new());
        let email = "test@example.com".to_string();
        let password = "password123".to_string();
        let user = User::new(email.clone(), "testuser".to_string(), password.clone());
        mock_repo.add_user(user);

        let use_case = CreateUserSessionUseCase::new(mock_repo.clone());
        let request = CreateUserSessionRequest::new(email.clone(), "wrong_password".to_string());

        // Act
        let result = use_case.execute(request).await;

        // Assert
        assert!(result.is_err());
        match result.err().unwrap() {
            CreateUserSessionError::InvalidCredentials(msg) => {
                assert_eq!(msg, "Invalid credentials");
            }
        }
    }

    #[actix_web::test]
    async fn test_execute_with_non_existent_user_should_return_error() {
        // Arrange
        setup();
        let mock_repo = Arc::new(MockUserRepository::new());
        let use_case = CreateUserSessionUseCase::new(mock_repo.clone());
        let request = CreateUserSessionRequest::new(
            "nonexistent@example.com".to_string(),
            "password".to_string(),
        );

        // Act
        let result = use_case.execute(request).await;

        // Assert
        assert!(result.is_err());
        match result.err().unwrap() {
            CreateUserSessionError::InvalidCredentials(msg) => {
                assert_eq!(msg, "Invalid credentials");
            }
        }
    }

    #[actix_web::test]
    async fn test_execute_with_user_without_id_should_return_error() {
        // Arrange
        setup();
        let mock_repo = Arc::new(MockUserRepository::new());
        let email = "test@example.com".to_string();
        let password = "password123".to_string();
        let mut user = User::new(email.clone(), "testuser".to_string(), password.clone());

        // Manually hash password but do not set an ID
        let salt = SaltString::generate(rand_core::OsRng);
        let password_hash = Argon2::default()
            .hash_password(user.password().as_bytes(), &salt)
            .expect("Failed to hash password")
            .to_string();
        user.set_password(password_hash);
        // user.set_id(Some(...)) is not called

        mock_repo.users.lock().unwrap().push(user);

        let use_case = CreateUserSessionUseCase::new(mock_repo.clone());
        let request = CreateUserSessionRequest::new(email.clone(), password.clone());

        // Act
        let result = use_case.execute(request).await;

        // Assert
        assert!(result.is_err());
        match result.err().unwrap() {
            CreateUserSessionError::InvalidCredentials(msg) => {
                assert_eq!(msg, "Invalid credentials");
            }
        }
    }
}
