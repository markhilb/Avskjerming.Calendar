use api::routes::team::{CreateTeam, Team};
use reqwest::StatusCode;

use super::helper::test;

#[tokio::test]
async fn test_cant_use_team_endpoints_if_not_logged_in() {
    test(|helper| async move {
        let create_team = CreateTeam {
            name: "team".into(),
            primary_color: "black".into(),
            secondary_color: "white".into(),
        };
        let update_team = Team {
            id: 1,
            name: "team".into(),
            primary_color: "black".into(),
            secondary_color: "white".into(),
            disabled: false,
        };

        let response = helper.client.get_teams().await;
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
        let response = helper.client.create_team(create_team).await;
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
        let response = helper.client.update_team(update_team).await;
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
        let response = helper.client.delete_team(1).await;
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    })
    .await;
}

#[tokio::test]
async fn test_create_and_get_teams() {
    test(|helper| async move {
        helper.login().await;

        let team1 = CreateTeam {
            name: "team1".into(),
            primary_color: "black".into(),
            secondary_color: "white".into(),
        };
        let team2 = CreateTeam {
            name: "team2".into(),
            primary_color: "green".into(),
            secondary_color: "blue".into(),
        };
        let team3 = CreateTeam {
            name: "team3".into(),
            primary_color: "red".into(),
            secondary_color: "yellow".into(),
        };

        let expected = vec![
            helper.create_team(team1.clone()).await,
            helper.create_team(team2.clone()).await,
            helper.create_team(team3.clone()).await,
        ];

        let mut teams = helper.get_teams().await;

        teams.sort_by_key(|e| e.id);

        assert_eq!(teams, expected);
    })
    .await;
}

#[tokio::test]
async fn test_update_team() {
    test(|helper| async move {
        helper.login().await;

        let create_team = CreateTeam {
            name: "team".into(),
            primary_color: "black".into(),
            secondary_color: "white".into(),
        };

        let mut update_team = helper.create_team(create_team.clone()).await;
        update_team.name = "updated_name".into();
        update_team.primary_color = "pink".into();
        update_team.secondary_color = "purple".into();

        let team = helper.update_team(update_team.clone()).await;
        assert_eq!(team, update_team);
    })
    .await;
}

#[tokio::test]
async fn test_delete_team() {
    test(|helper| async move {
        helper.login().await;

        let create_team = CreateTeam {
            name: "team".into(),
            primary_color: "black".into(),
            secondary_color: "white".into(),
        };

        let team = helper.create_team(create_team.clone()).await;

        let deleted_team = helper.delete_team(team.id).await;

        assert!(deleted_team.disabled);
    })
    .await;
}
