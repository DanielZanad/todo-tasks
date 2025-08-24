#[cfg(test)]
mod tests {

    // This test needs to be run with `cargo test -- --test-threads=1`

    use crate::app::use_cases::get_signed_url_use_case::{
        GetSignedUrlError, GetSignedUrlUseCase, GetSignedUrlUseCaseRequest, IncomingPayload,
    };
    use mockito::Server;

    #[actix_web::test]
    async fn execute_should_return_signed_url_on_success() {
        // Arrange
        let mut server = Server::new_async().await;
        let url = server.url();
        unsafe {
            std::env::set_var("SIGNED_URL_API", &url);
        }

        let mock_response_body = IncomingPayload {
            url: "http://signed.url/for/test".to_string(),
        };

        let mock = server
            .mock("POST", "/uploads")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(serde_json::to_string(&mock_response_body).unwrap())
            .create_async()
            .await;

        let use_case = GetSignedUrlUseCase::new();
        let request =
            GetSignedUrlUseCaseRequest::new("test-key".to_string(), "image/png".to_string());

        // Act
        let result = use_case.execute(request).await;

        // Assert
        mock.assert_async().await;
        let response = result.expect("execute should not fail");
        assert_eq!(response.url, "http://signed.url/for/test");
    }

    #[actix_web::test]
    async fn execute_should_return_error_on_api_failure() {
        // Arrange
        let mut server = Server::new_async().await;
        let url = server.url();
        unsafe {
            std::env::set_var("SIGNED_URL_API", &url);
        }

        let mock = server
            .mock("POST", "/uploads")
            .with_status(500)
            .with_body("Internal Server Error")
            .create_async()
            .await;

        let use_case = GetSignedUrlUseCase::new();
        let request =
            GetSignedUrlUseCaseRequest::new("test-key".to_string(), "image/png".to_string());

        // Act
        let result = use_case.execute(request).await;

        // Assert
        mock.assert_async().await;
        match result {
            Err(GetSignedUrlError::ApiError { status, body }) => {
                assert_eq!(status, 500);
                assert_eq!(body, "Internal Server Error");
            }
            _ => panic!("Expected ApiError, got {:?}", result),
        }
    }
}
