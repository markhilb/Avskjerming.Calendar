use api::routes::employee::{CreateEmployee, Employee};
use reqwest::StatusCode;

use super::helper::test;

#[tokio::test]
async fn test_cant_use_employee_endpoints_if_not_logged_in() {
    test(|helper| async move {
        let create_employee = CreateEmployee {
            name: "employee".into(),
            color: "black".into(),
        };
        let update_employee = Employee {
            id: 1,
            name: "employee".into(),
            color: "black".into(),
            disabled: false,
        };

        let response = helper.client.get_employees().await;
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
        let response = helper.client.create_employee(create_employee).await;
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
        let response = helper.client.update_employee(update_employee).await;
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
        let response = helper.client.delete_employee(1).await;
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    })
    .await;
}

#[tokio::test]
async fn test_create_and_get_employees() {
    test(|helper| async move {
        helper.login().await;

        let employee1 = CreateEmployee {
            name: "employee1".into(),
            color: "black".into(),
        };
        let employee2 = CreateEmployee {
            name: "employee2".into(),
            color: "white".into(),
        };
        let employee3 = CreateEmployee {
            name: "employee3".into(),
            color: "red".into(),
        };

        let expected = vec![
            helper.create_employee(employee1.clone()).await,
            helper.create_employee(employee2.clone()).await,
            helper.create_employee(employee3.clone()).await,
        ];

        let mut employees = helper.get_employees().await;

        employees.sort_by_key(|e| e.id);

        assert_eq!(employees, expected);
    })
    .await;
}

#[tokio::test]
async fn test_update_employee() {
    test(|helper| async move {
        helper.login().await;

        let create_employee = CreateEmployee {
            name: "employee".into(),
            color: "black".into(),
        };

        let mut update_employee = helper.create_employee(create_employee.clone()).await;
        update_employee.name = "updated_name".into();
        update_employee.color = "white".into();

        let employee = helper.update_employee(update_employee.clone()).await;

        assert_eq!(employee, update_employee);
    })
    .await;
}

#[tokio::test]
async fn test_delete_employee() {
    test(|helper| async move {
        helper.login().await;

        let create_employee = CreateEmployee {
            name: "employee".into(),
            color: "black".into(),
        };

        let employee = helper.create_employee(create_employee.clone()).await;

        let deleted_employee = helper.delete_employee(employee.id).await;
        assert!(deleted_employee.disabled);
    })
    .await;
}
