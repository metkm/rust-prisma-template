mod prisma;

use prisma::new_client;
use prisma::{user, team};

#[tokio::main]
async fn main() {
    let client = new_client().await.expect("Can't create prisma client");

    println!("Creating user");
    client
        .user()
        .create("Sibyl".to_string(), vec![])
        .exec()
        .await
        .expect("error creating user");

    println!("Creating team");
    let created_team = client
        .team()
        .create("hello_team".to_string(), vec![])
        .exec()
        .await
        .expect("error while creating database");
    println!("Created team");

    println!("Creating user");
    client
        .user()
        .update(
            user::id::equals(1),
            vec![
                user::team_id::set(Some(created_team.id))
            ]
        )
        .exec()
        .await
        .expect("error while updating user");
    println!("Created user");

    println!("Querying team players");
    let team = client
        .team()
        .find_unique(team::id::equals(created_team.id)) // find the team we are looking for
        .with(team::players::fetch(vec![]))
        .exec()
        .await
        .expect("error while querying users with team");

    let Some(team) = team else {
        return;
    };

    let Some(players) = team.players else {
        return;
    };

    println!("Found players..");
    for player in players {
        println!("Username -> {}", player.display_name);
    }
}
