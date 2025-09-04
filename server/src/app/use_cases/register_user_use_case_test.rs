#[cfg(test)]
mod tests {
    use crate::app::entities::user::User;
    use crate::app::repositories::user_repository::{UserProfile, UserRepository};
    use crate::app::use_cases::register_user_use_case::{RegisterUserRequest, RegisterUserUseCase};
    use argon2::Argon2;
    use argon2::password_hash::PasswordVerifier;
    use std::future::Future;
    use std::pin::Pin;
    use std::sync::{Arc, Mutex};

    struct MockUserRepository {
        // email, username, password_hash, file_key, mime_type
        register_called_with: Mutex<Option<(String, String, String, String, String)>>,
    }

    impl MockUserRepository {
        fn new() -> Self {
            Self {
                register_called_with: Mutex::new(None),
            }
        }
    }

    impl UserRepository for MockUserRepository {
        fn register<'a>(
            &'a self,
            user: User,
            file_key: String,
            mime_type: String,
        ) -> Pin<Box<dyn Future<Output = ()> + Send + 'a>> {
            let mut call_data = self.register_called_with.lock().unwrap();
            *call_data = Some((
                user.email().to_string(),
                user.username().to_string(),
                user.password().to_string(),
                file_key,
                mime_type,
            ));
            Box::pin(async {})
        }

        fn get_user_by_email<'a>(
            &'a self,
            email: String,
        ) -> Pin<Box<dyn Future<Output = Option<User>> + Send + 'a>> {
            todo!()
        }

        fn get_user_profile<'a>(
            &'a self,
            user_id: String,
        ) -> Pin<Box<dyn Future<Output = Option<UserProfile>> + Send>> {
            todo!()
        }
    }

    #[actix_web::test]
    async fn test_execute_should_hash_password_and_register_user() {
        // Arrange
        let mock_repo = Arc::new(MockUserRepository::new());
        let use_case = RegisterUserUseCase::new(mock_repo.clone());

        let email = "test@example.com".to_string();
        let username = "testuser".to_string();
        let password = "password123".to_string();
        let file_key = "test_file_key".to_string();
        let mime_type = "image/png".to_string();

        let request = RegisterUserRequest::new(
            email.clone(),
            username.clone(),
            password.clone(),
            file_key.clone(),
            mime_type.clone(),
        );

        // Act
        use_case.execute(request).await;

        // Assert
        let call_data_lock = mock_repo.register_called_with.lock().unwrap();
        let call_data = call_data_lock.as_ref();

        assert!(call_data.is_some(), "register was not called");

        if let Some((
            saved_email,
            saved_username,
            saved_password_hash,
            saved_file_key,
            saved_mime_type,
        )) = call_data
        {
            assert_eq!(saved_email, &email);
            assert_eq!(saved_username, &username);
            assert_ne!(saved_password_hash, &password);

            let is_valid = argon2::PasswordHash::new(saved_password_hash)
                .and_then(|hash| Argon2::default().verify_password(password.as_bytes(), &hash))
                .is_ok();

            assert!(is_valid, "password was not hashed correctly");

            assert_eq!(saved_file_key, &file_key);
            assert_eq!(saved_mime_type, &mime_type);
        }
    }
}
