use api::routes::{
    employee::CreateEmployee,
    event::{CreateEvent, EventsQuery, UpdateEvent},
    team::CreateTeam,
};
use chrono::{Duration, TimeZone, Utc};
use reqwest::StatusCode;

use super::helper::test;

#[tokio::test]
async fn test_cant_use_event_endpoints_if_not_logged_in() {
    test(|helper| async move {
        let create_event = CreateEvent {
            title: "event".into(),
            details: "details".into(),
            start: Utc::now(),
            end: Utc::now(),
            team_id: None,
            employee_ids: vec![],
        };
        let update_event = UpdateEvent {
            id: 1,
            title: "event".into(),
            details: "details".into(),
            start: Utc::now(),
            end: Utc::now(),
            team_id: None,
            employee_ids: vec![],
        };

        let response = helper.client.get_events(Default::default()).await;
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
        let response = helper.client.create_event(create_event).await;
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
        let response = helper.client.update_event(update_event).await;
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
        let response = helper.client.delete_event(1).await;
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    })
    .await;
}

#[tokio::test]
async fn test_create_and_get_events() {
    test(|helper| async move {
        helper.login().await;

        let create_employee1 = CreateEmployee {
            name: "employee1".into(),
            color: "black".into(),
        };
        let create_employee2 = CreateEmployee {
            name: "employee2".into(),
            color: "white".into(),
        };

        let employee1 = helper.create_employee(create_employee1.clone()).await;
        let employee2 = helper.create_employee(create_employee2.clone()).await;

        let create_team1 = CreateTeam {
            name: "team1".into(),
            primary_color: "black".into(),
            secondary_color: "white".into(),
        };
        let create_team2 = CreateTeam {
            name: "team2".into(),
            primary_color: "green".into(),
            secondary_color: "blue".into(),
        };

        let team1 = helper.create_team(create_team1.clone()).await;
        let team2 = helper.create_team(create_team2.clone()).await;

        let create_event1 = CreateEvent {
            title: "event1".into(),
            details: "details1".into(),
            start: Utc::now(),
            end: Utc::now(),
            team_id: None,
            employee_ids: vec![],
        };
        let create_event2 = CreateEvent {
            title: "event2".into(),
            details: "details2".into(),
            start: Utc::now(),
            end: Utc::now(),
            team_id: Some(team1.id),
            employee_ids: vec![employee1.id],
        };
        let create_event3 = CreateEvent {
            title: "event3".into(),
            details: "details3".into(),
            start: Utc::now(),
            end: Utc::now(),
            team_id: Some(team2.id),
            employee_ids: vec![employee1.id, employee2.id],
        };

        let expected = vec![
            helper.create_event(create_event1.clone()).await,
            helper.create_event(create_event2.clone()).await,
            helper.create_event(create_event3.clone()).await,
        ];

        let mut events = helper.get_events(Default::default()).await;

        events.sort_by_key(|e| e.id);

        assert_eq!(events, expected);
    })
    .await;
}

#[tokio::test]
async fn test_get_events_filters_by_range() {
    test(|helper| async move {
        helper.login().await;

        let create_event1 = CreateEvent {
            title: "event1".into(),
            details: "details1".into(),
            start: Utc.timestamp_opt(1000, 0).unwrap(),
            end: Utc.timestamp_opt(1000, 0).unwrap(),
            team_id: None,
            employee_ids: vec![],
        };
        let create_event2 = CreateEvent {
            title: "event2".into(),
            details: "details2".into(),
            start: Utc.timestamp_opt(2000, 0).unwrap(),
            end: Utc.timestamp_opt(2000, 0).unwrap(),
            team_id: None,
            employee_ids: vec![],
        };
        let create_event3 = CreateEvent {
            title: "event3".into(),
            details: "details3".into(),
            start: Utc.timestamp_opt(3000, 0).unwrap(),
            end: Utc.timestamp_opt(3000, 0).unwrap(),
            team_id: None,
            employee_ids: vec![],
        };

        helper.create_event(create_event1.clone()).await;
        let event2 = helper.create_event(create_event2.clone()).await;
        helper.create_event(create_event3.clone()).await;

        let query = EventsQuery {
            start: Some(create_event2.start - Duration::seconds(10)),
            end: Some(create_event2.end + Duration::seconds(10)),
        };

        let events = helper.get_events(query).await;
        assert_eq!(events.len(), 1);
        assert_eq!(events[0], event2);
    })
    .await;
}

#[tokio::test]
async fn test_update_event() {
    test(|helper| async move {
        helper.login().await;

        let create_employee1 = CreateEmployee {
            name: "employee1".into(),
            color: "black".into(),
        };
        let create_employee2 = CreateEmployee {
            name: "employee2".into(),
            color: "white".into(),
        };

        let employee1 = helper.create_employee(create_employee1.clone()).await;
        let employee2 = helper.create_employee(create_employee2.clone()).await;

        let create_team1 = CreateTeam {
            name: "team1".into(),
            primary_color: "black".into(),
            secondary_color: "white".into(),
        };
        let create_team2 = CreateTeam {
            name: "team2".into(),
            primary_color: "green".into(),
            secondary_color: "blue".into(),
        };

        let team1 = helper.create_team(create_team1.clone()).await;
        let team2 = helper.create_team(create_team2.clone()).await;

        let create_event = CreateEvent {
            title: "event".into(),
            details: "details".into(),
            start: Utc::now(),
            end: Utc::now(),
            team_id: Some(team1.id),
            employee_ids: vec![employee1.id],
        };

        let event = helper.create_event(create_event.clone()).await;

        let update_event = UpdateEvent {
            id: event.id,
            title: "new_event".into(),
            details: "new_details".into(),
            start: Utc::now(),
            end: Utc::now(),
            team_id: Some(team2.id),
            employee_ids: vec![employee2.id],
        };

        let event = helper.update_event(update_event.clone()).await;

        assert_eq!(event.to_update_event(), update_event);
    })
    .await;
}

#[tokio::test]
async fn test_delete_event() {
    test(|helper| async move {
        helper.login().await;

        let create_employee1 = CreateEmployee {
            name: "employee1".into(),
            color: "black".into(),
        };
        let create_employee2 = CreateEmployee {
            name: "employee2".into(),
            color: "white".into(),
        };

        let employee1 = helper.create_employee(create_employee1.clone()).await;
        let employee2 = helper.create_employee(create_employee2.clone()).await;

        let create_team1 = CreateTeam {
            name: "team1".into(),
            primary_color: "black".into(),
            secondary_color: "white".into(),
        };
        let create_team2 = CreateTeam {
            name: "team2".into(),
            primary_color: "green".into(),
            secondary_color: "blue".into(),
        };

        let team1 = helper.create_team(create_team1.clone()).await;
        let team2 = helper.create_team(create_team2.clone()).await;

        let create_event1 = CreateEvent {
            title: "event1".into(),
            details: "details1".into(),
            start: Utc::now(),
            end: Utc::now(),
            team_id: None,
            employee_ids: vec![],
        };
        let create_event2 = CreateEvent {
            title: "event2".into(),
            details: "details2".into(),
            start: Utc::now(),
            end: Utc::now(),
            team_id: Some(team1.id),
            employee_ids: vec![employee1.id],
        };
        let create_event3 = CreateEvent {
            title: "event3".into(),
            details: "details3".into(),
            start: Utc::now(),
            end: Utc::now(),
            team_id: Some(team2.id),
            employee_ids: vec![employee1.id, employee2.id],
        };

        let event1 = helper.create_event(create_event1.clone()).await;

        let expected = vec![
            helper.create_event(create_event2.clone()).await,
            helper.create_event(create_event3.clone()).await,
        ];

        helper.delete_event(event1.id).await;

        let mut events = helper.get_events(Default::default()).await;

        events.sort_by_key(|e| e.id);

        assert_eq!(events, expected);
    })
    .await;
}
