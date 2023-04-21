use proto::{
    service::{goals_client::GoalsClient, *},
    wedding::Goal,
};
use tonic::{transport::Channel, Request};
use uuid::Uuid;

pub async fn create_goal(
    goal: Goal,
    client: &mut GoalsClient<Channel>,
) -> Result<Uuid, Box<dyn std::error::Error>> {
    let req = Request::new(CreateGoalRequest { goal: Some(goal) });

    let resp = client.create_goal(req).await?;
    let resp = resp.into_inner();

    match Uuid::parse_str(&resp.id) {
        Ok(uuid) => Ok(uuid),
        Err(e) => Err(Box::new(e)),
    }
}

pub async fn read_goal(
    id: String,
    client: &mut GoalsClient<Channel>,
) -> Result<Goal, Box<dyn std::error::Error>> {
    let req = Request::new(ReadGoalRequest { id });

    let resp = client.read_goal(req).await?;
    let resp = resp.into_inner();
    let goal = resp.goal;

    if let Some(goal) = goal {
        Ok(goal)
    } else {
        Err("Failed to read goal.".into())
    }
}

pub async fn read_multi_goals(
    client: &mut GoalsClient<Channel>,
) -> Result<Vec<Goal>, Box<dyn std::error::Error>> {
    let req = Request::new(ReadMultiGoalsRequest {});

    let resp = client.read_multi_goals(req).await?;
    let resp = resp.into_inner();

    Ok(resp.goals)
}

pub async fn update_goal(
    goal: Goal,
    client: &mut GoalsClient<Channel>,
) -> Result<Uuid, Box<dyn std::error::Error>> {
    let req = Request::new(UpdateGoalRequest { goal: Some(goal) });

    let resp = client.update_goal(req).await?;
    let resp = resp.into_inner();

    match Uuid::parse_str(&resp.id) {
        Ok(uuid) => Ok(uuid),
        Err(e) => Err(Box::new(e)),
    }
}

pub async fn delete_goal(
    id: String,
    client: &mut GoalsClient<Channel>,
) -> Result<Uuid, Box<dyn std::error::Error>> {
    let req = Request::new(DeleteGoalRequest { id });

    let resp = client.delete_goal(req).await?;
    let resp = resp.into_inner();

    match Uuid::parse_str(&resp.id) {
        Ok(uuid) => Ok(uuid),
        Err(e) => Err(Box::new(e)),
    }
}
