use actix_web::web;
use postgres::PostgresAdapter;
use serde::{Deserialize, Serialize};
use tracing::{event, instrument, Level};
use utoipa::ToSchema;

use crate::{error::ApiError, response::Response};

#[utoipa::path(
    get,
    path = "/employees",
    responses(
        (status = 200, description = "all employees", body = [Employee]),
        (status = 500, description = "an internal server error occured"),
    )
)]
#[instrument(skip(db))]
pub async fn get_employees(
    db: web::Data<PostgresAdapter>,
) -> Result<Response<Vec<Employee>>, ApiError> {
    Ok(Response::new(
        db.employees()
            .await
            .map_err(|e| {
                event!(Level::ERROR, "failed to get employees, err: {:?}", e);
                ApiError::InternalServerError
            })?
            .into_iter()
            .map(Employee::from)
            .collect(),
    ))
}

#[utoipa::path(
    post,
    path = "/employees",
    request_body(
        content = CreateEmployee,
        content_type = "application/json",
        description = "the employee to create",
    ),
    responses(
        (status = 200, description = "employee successfully created", body = i64),
        (status = 500, description = "an internal server error occured"),
    )
)]
#[instrument(skip(db))]
pub async fn create_employee(
    db: web::Data<PostgresAdapter>,
    employee: web::Json<CreateEmployee>,
) -> Result<Response<i64>, ApiError> {
    db.create_employee(employee.into_inner().into())
        .await
        .map_err(|e| {
            event!(Level::ERROR, "failed to create employee, err: {:?}", e);
            ApiError::InternalServerError
        })
        .map(Response::new)
}

#[utoipa::path(
    put,
    path = "/employees",
    request_body(
        content = Employee,
        content_type = "application/json",
        description = "the updated employee",
    ),
    responses(
        (status = 200, description = "employee successfully updated"),
        (status = 500, description = "an internal server error occured"),
    )
)]
#[instrument(skip(db))]
pub async fn update_employee(
    db: web::Data<PostgresAdapter>,
    employee: web::Json<Employee>,
) -> Result<Response<()>, ApiError> {
    db.update_employee(employee.into_inner().into())
        .await
        .map_err(|e| {
            event!(Level::ERROR, "failed to update employee, err: {:?}", e);
            ApiError::InternalServerError
        })
        .map(|_| Response::new(()))
}

#[utoipa::path(
    delete,
    path = "/employees/{id}",
    responses(
        (status = 200, description = "employee successfully deleted"),
        (status = 500, description = "an internal server error occured"),
    )
)]
#[instrument(skip(db))]
pub async fn delete_employee(
    db: web::Data<PostgresAdapter>,
    id: web::Path<i64>,
) -> Result<Response<()>, ApiError> {
    db.delete_employee(id.into_inner())
        .await
        .map_err(|e| {
            event!(Level::ERROR, "failed to delete employee, err: {:?}", e);
            ApiError::InternalServerError
        })
        .map(|_| Response::new(()))
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Employee {
    pub id: i64,
    pub name: String,
    pub color: String,
    pub disabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateEmployee {
    pub name: String,
    pub color: String,
}

impl From<postgres::Employee> for Employee {
    fn from(v: postgres::Employee) -> Self {
        Self {
            id: v.employee_id,
            name: v.name,
            color: v.color,
            disabled: v.disabled,
        }
    }
}

impl From<Employee> for postgres::Employee {
    fn from(v: Employee) -> Self {
        Self {
            employee_id: v.id,
            name: v.name,
            color: v.color,
            disabled: v.disabled,
        }
    }
}

impl From<CreateEmployee> for postgres::CreateEmployee {
    fn from(v: CreateEmployee) -> Self {
        Self {
            name: v.name,
            color: v.color,
        }
    }
}
