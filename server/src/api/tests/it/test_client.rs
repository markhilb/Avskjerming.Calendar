use api::routes::{
    auth::{ChangePassword, Login},
    employee::{CreateEmployee, Employee},
    event::{CreateEvent, EventsQuery, UpdateEvent},
    team::{CreateTeam, Team},
};
use reqwest::{Client, Response};
use serde::Serialize;

pub struct TestClient {
    client: Client,
    address: String,
}

impl TestClient {
    pub fn new(address: String) -> Self {
        let client = Client::builder().cookie_store(true).build().unwrap();

        Self { client, address }
    }

    pub async fn get<T: AsRef<str>, S: Serialize>(&self, path: T, query: S) -> Response {
        let url = format!("{}/{}", self.address, path.as_ref());
        self.client.get(url).query(&query).send().await.unwrap()
    }

    pub async fn post<T: AsRef<str>, B: Serialize>(&self, path: T, body: B) -> Response {
        let url = format!("{}/{}", self.address, path.as_ref());
        self.client.post(url).json(&body).send().await.unwrap()
    }

    pub async fn put<T: AsRef<str>, B: Serialize>(&self, path: T, body: B) -> Response {
        let url = format!("{}/{}", self.address, path.as_ref());
        self.client.put(url).json(&body).send().await.unwrap()
    }

    pub async fn delete<T: AsRef<str>>(&self, path: T) -> Response {
        let url = format!("{}/{}", self.address, path.as_ref());
        self.client.delete(url).send().await.unwrap()
    }

    pub async fn login(&self, login: Login) -> Response {
        self.post("login", login).await
    }

    pub async fn logout(&self) -> Response {
        self.post("logout", ()).await
    }

    pub async fn logged_in(&self) -> Response {
        self.get("logged_in", ()).await
    }

    pub async fn change_password(&self, body: ChangePassword) -> Response {
        self.post("change_password", body).await
    }

    pub async fn get_employees(&self) -> Response {
        self.get("employees", ()).await
    }

    pub async fn create_employee(&self, employee: CreateEmployee) -> Response {
        self.post("employees", employee).await
    }

    pub async fn update_employee(&self, employee: Employee) -> Response {
        self.put("employees", employee).await
    }

    pub async fn delete_employee(&self, employee_id: i64) -> Response {
        self.delete(format!("employees/{employee_id}")).await
    }

    pub async fn get_teams(&self) -> Response {
        self.get("teams", ()).await
    }

    pub async fn create_team(&self, team: CreateTeam) -> Response {
        self.post("teams", team).await
    }

    pub async fn update_team(&self, team: Team) -> Response {
        self.put("teams", team).await
    }

    pub async fn delete_team(&self, team_id: i64) -> Response {
        self.delete(format!("teams/{team_id}")).await
    }

    pub async fn get_events(&self, query: EventsQuery) -> Response {
        self.get("events", query).await
    }

    pub async fn create_event(&self, event: CreateEvent) -> Response {
        self.post("events", event).await
    }

    pub async fn update_event(&self, event: UpdateEvent) -> Response {
        self.put("events", event).await
    }

    pub async fn delete_event(&self, event_id: i64) -> Response {
        self.delete(format!("events/{event_id}")).await
    }
}
