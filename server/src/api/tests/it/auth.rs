use api::routes::auth::{ChangePassword, Login};
use reqwest::StatusCode;

use super::helper::test;

#[tokio::test]
async fn test_login_and_is_logged_in() {
    test(|helper| async move {
        let response = helper.client.logged_in().await;
        assert_eq!(response.status(), StatusCode::OK);

        let logged_in: bool = response.json().await.unwrap();
        assert!(!logged_in);

        let response = helper
            .client
            .login(Login {
                password: "password".into(),
            })
            .await;
        assert_eq!(response.status(), StatusCode::OK);

        let login: bool = response.json().await.unwrap();
        assert!(login);

        let response = helper.client.logged_in().await;
        assert_eq!(response.status(), StatusCode::OK);

        let logged_in: bool = response.json().await.unwrap();
        assert!(logged_in);
    })
    .await;
}

#[tokio::test]
async fn test_logout() {
    test(|helper| async move {
        let response = helper
            .client
            .login(Login {
                password: "password".into(),
            })
            .await;
        assert_eq!(response.status(), StatusCode::OK);

        let login: bool = response.json().await.unwrap();
        assert!(login);

        let response = helper.client.logout().await;
        assert_eq!(response.status(), StatusCode::OK);

        let response = helper.client.logged_in().await;
        assert_eq!(response.status(), StatusCode::OK);

        let logged_in: bool = response.json().await.unwrap();
        assert!(!logged_in);
    })
    .await;
}

#[tokio::test]
async fn test_change_password() {
    test(|helper| async move {
        let response = helper
            .client
            .login(Login {
                password: "password".into(),
            })
            .await;
        assert_eq!(response.status(), StatusCode::OK);

        let response = helper
            .client
            .change_password(ChangePassword {
                old_password: "password".into(),
                new_password: "new_pass".into(),
            })
            .await;
        assert_eq!(response.status(), StatusCode::OK);

        let response = helper
            .client
            .login(Login {
                password: "new_pass".into(),
            })
            .await;
        assert_eq!(response.status(), StatusCode::OK);

        let login: bool = response.json().await.unwrap();
        assert!(login);
    })
    .await;
}

#[tokio::test]
async fn test_cant_change_password_if_not_logged_in() {
    test(|helper| async move {
        let response = helper
            .client
            .change_password(ChangePassword {
                old_password: "password".into(),
                new_password: "new_pass".into(),
            })
            .await;
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    })
    .await;
}
