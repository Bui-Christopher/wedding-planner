use proto::{
    methods::{goals_client::GoalsClient, *},
    objects::Goal,
};
use tonic::{transport::Channel, Request};
use uuid::Uuid;

pub async fn create_goal(goal: Goal, channel: Channel) -> Result<Uuid, Box<dyn std::error::Error>> {
    let mut client = GoalsClient::new(channel);
    let req = Request::new(CreateGoalRequest { goal: Some(goal) });
    let resp = client.create_goal(req).await?.into_inner();

    match Uuid::parse_str(&resp.id) {
        Ok(uuid) => Ok(uuid),
        Err(e) => Err(Box::new(e)),
    }
}

pub async fn read_goal(id: String, channel: Channel) -> Result<Goal, Box<dyn std::error::Error>> {
    let mut client = GoalsClient::new(channel);
    let req = Request::new(ReadGoalRequest { id });
    let resp = client.read_goal(req).await?.into_inner();

    if let Some(goal) = resp.goal {
        Ok(goal)
    } else {
        Err("Failed to read goal.".into())
    }
}

pub async fn read_multi_goals(channel: Channel) -> Result<Vec<Goal>, Box<dyn std::error::Error>> {
    let mut client = GoalsClient::new(channel);
    let req = Request::new(ReadMultiGoalsRequest {});
    let resp = client.read_multi_goals(req).await?.into_inner();

    Ok(resp.goals)
}

pub async fn update_goal(goal: Goal, channel: Channel) -> Result<Uuid, Box<dyn std::error::Error>> {
    let mut client = GoalsClient::new(channel);
    let req = Request::new(UpdateGoalRequest { goal: Some(goal) });
    let resp = client.update_goal(req).await?.into_inner();

    match Uuid::parse_str(&resp.id) {
        Ok(uuid) => Ok(uuid),
        Err(e) => Err(Box::new(e)),
    }
}

pub async fn delete_goal(id: String, channel: Channel) -> Result<Uuid, Box<dyn std::error::Error>> {
    let mut client = GoalsClient::new(channel);
    let req = Request::new(DeleteGoalRequest { id });
    let resp = client.delete_goal(req).await?.into_inner();

    match Uuid::parse_str(&resp.id) {
        Ok(uuid) => Ok(uuid),
        Err(e) => Err(Box::new(e)),
    }
}
