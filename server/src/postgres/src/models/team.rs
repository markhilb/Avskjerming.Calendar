use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Team {
    pub team_id: i64,
    pub name: String,
    pub primary_color: String,
    pub secondary_color: String,
    pub disabled: bool,
}

#[derive(Debug, Clone)]
pub struct CreateTeam {
    pub name: String,
    pub primary_color: String,
    pub secondary_color: String,
}
