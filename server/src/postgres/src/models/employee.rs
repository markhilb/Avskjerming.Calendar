use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Employee {
    pub employee_id: i64,
    pub name: String,
    pub color: String,
    pub disabled: bool,
}

#[derive(Debug, Clone)]
pub struct CreateEmployee {
    pub name: String,
    pub color: String,
}
