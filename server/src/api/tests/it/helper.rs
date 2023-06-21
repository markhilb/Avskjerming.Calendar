use api::{
    app::App,
    routes::{
        auth::Login,
        employee::{CreateEmployee, Employee},
        event::{CreateEvent, Event, EventsQuery, UpdateEvent},
        team::{CreateTeam, Team},
    },
    settings::{ApiSettings, Environment, LogLevel, Settings},
};
use dockertest::{
    waitfor::{MessageSource, MessageWait},
    Composition, DockerTest, Image, StaticManagementPolicy,
};
use futures_util::Future;
use postgres::{PsqlSettings, TestDb};
use rand::random;
use reqwest::StatusCode;

use super::test_client::TestClient;

const DATABASE_PASSWORD: &str = "test123";

pub struct TestHelper {
    pub client: TestClient,
    pub db: TestDb,
}

impl TestHelper {
    pub async fn new(settings: &Settings) -> Self {
        let db = TestDb::new(&settings.postgres).await.unwrap();
        let app = App::new(settings).await;

        let address = format!("http://127.0.0.1:{}", app.port());

        tokio::spawn(async { app.run().await.unwrap() });

        Self {
            client: TestClient::new(address),
            db,
        }
    }

    pub async fn login(&self) {
        let response = self
            .client
            .login(Login {
                password: "password".into(),
            })
            .await;
        assert_eq!(response.status(), StatusCode::OK);

        let login: bool = response.json().await.unwrap();
        assert!(login);
    }

    pub async fn get_employees(&self) -> Vec<Employee> {
        let response = self.client.get_employees().await;
        assert_eq!(response.status(), StatusCode::OK);
        response.json().await.unwrap()
    }

    pub async fn get_employee(&self, id: i64) -> Employee {
        let employees = self.get_employees().await;
        employees.into_iter().find(|e| e.id == id).unwrap()
    }

    pub async fn create_employee(&self, employee: CreateEmployee) -> Employee {
        let response = self.client.create_employee(employee).await;
        assert_eq!(response.status(), StatusCode::OK);

        let id = response.json::<i64>().await.unwrap();
        self.get_employee(id).await
    }

    pub async fn update_employee(&self, employee: Employee) -> Employee {
        let id = employee.id;

        let response = self.client.update_employee(employee).await;
        assert_eq!(response.status(), StatusCode::OK);

        self.get_employee(id).await
    }

    pub async fn delete_employee(&self, id: i64) -> Employee {
        let response = self.client.delete_employee(id).await;
        assert_eq!(response.status(), StatusCode::OK);

        self.get_employee(id).await
    }

    pub async fn get_teams(&self) -> Vec<Team> {
        let response = self.client.get_teams().await;
        assert_eq!(response.status(), StatusCode::OK);
        response.json().await.unwrap()
    }

    pub async fn get_team(&self, id: i64) -> Team {
        let teams = self.get_teams().await;
        teams.into_iter().find(|e| e.id == id).unwrap()
    }

    pub async fn create_team(&self, team: CreateTeam) -> Team {
        let response = self.client.create_team(team).await;
        assert_eq!(response.status(), StatusCode::OK);

        let id = response.json::<i64>().await.unwrap();
        self.get_team(id).await
    }

    pub async fn update_team(&self, team: Team) -> Team {
        let id = team.id;

        let response = self.client.update_team(team).await;
        assert_eq!(response.status(), StatusCode::OK);

        self.get_team(id).await
    }

    pub async fn delete_team(&self, id: i64) -> Team {
        let response = self.client.delete_team(id).await;
        assert_eq!(response.status(), StatusCode::OK);

        self.get_team(id).await
    }

    pub async fn get_events(&self, query: EventsQuery) -> Vec<Event> {
        let response = self.client.get_events(query).await;
        assert_eq!(response.status(), StatusCode::OK);
        response.json().await.unwrap()
    }

    pub async fn get_event(&self, id: i64) -> Event {
        let events = self.get_events(Default::default()).await;
        events.into_iter().find(|e| e.id == id).unwrap()
    }

    pub async fn create_event(&self, event: CreateEvent) -> Event {
        let response = self.client.create_event(event).await;
        assert_eq!(response.status(), StatusCode::OK);

        let id = response.json::<i64>().await.unwrap();
        self.get_event(id).await
    }

    pub async fn update_event(&self, event: UpdateEvent) -> Event {
        let id = event.id;

        let response = self.client.update_event(event).await;
        assert_eq!(response.status(), StatusCode::OK);

        self.get_event(id).await
    }

    pub async fn delete_event(&self, id: i64) {
        let response = self.client.delete_event(id).await;
        assert_eq!(response.status(), StatusCode::OK);
    }
}

pub async fn test<T, F>(test: T)
where
    T: FnOnce(TestHelper) -> F + Send + 'static,
    F: Future<Output = ()> + Send,
{
    let mut docker_test = DockerTest::new(); //.with_default_source(Source::DockerHub);

    let mut composition =
        postgres_composition(DATABASE_PASSWORD, "postgres-test", "calendar/postgres-test")
            .with_log_options(None);

    composition.static_container(StaticManagementPolicy::Dynamic);

    docker_test.add_composition(composition);

    let db_name = random::<u32>().to_string();

    docker_test
        .run_async(|ops| async move {
            let handle = ops.handle("postgres-test");

            let mut db_settings = PsqlSettings {
                ip: handle.ip().to_string(),
                port: 5432,
                db_name: Some("template1".to_string()),
                username: "postgres".to_string(),
                password: DATABASE_PASSWORD.to_string(),
                max_connections: 1,
            };

            let test_db = TestDb::new(&db_settings).await.unwrap();
            test_db.create_db(&db_name).await.unwrap();

            db_settings.db_name = Some(db_name.clone());

            let settings = Settings {
                log_level: LogLevel::Debug,
                environment: Environment::Test,
                api: ApiSettings {
                    ip: "127.0.0.1".to_string(),
                    port: 0,
                    num_workers: Some(1),
                    secret_key: Some("0123456789_0123456789_0123456789".to_string()),
                },
                postgres: db_settings,
            };

            let helper = TestHelper::new(&settings).await;

            test(helper).await;

            test_db.drop_db(&db_name).await.unwrap();
        })
        .await;
}

pub fn postgres_composition(password: &str, container_name: &str, image: &str) -> Composition {
    let image = Image::with_repository(image);

    let mut composition = Composition::with_image(image)
        .with_container_name(container_name)
        .with_wait_for(Box::new(MessageWait {
            message: "PostgreSQL init process complete; ready for start up.".to_string(),
            source: MessageSource::Stdout,
            timeout: 60,
        }))
        .with_cmd(vec![
            "postgres".to_string(),
            "-c".to_string(),
            "log_statement=all".to_string(),
            "-c".to_string(),
            "max_connections=500".to_string(),
            "-c".to_string(),
            "shared_buffers=200MB".to_string(),
        ]);

    composition.env("POSTGRES_PASSWORD", password);

    composition
}
