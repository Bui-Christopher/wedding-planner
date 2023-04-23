use proto::{
    methods::{goals_server::Goals, *},
    objects::Goal,
};
use tonic::{Request, Response, Status};

#[derive(Default)]
pub struct GoalService {}

#[tonic::async_trait]
impl Goals for GoalService {
    async fn create_goal(
        &self,
        req: Request<CreateGoalRequest>,
    ) -> Result<Response<CreateGoalResponse>, Status> {
        let CreateGoalRequest { goal } = req.into_inner();
        let goal = safely_extract(goal)?;
        let resp = CreateGoalResponse { id: goal.id };
        Ok(Response::new(resp))
    }

    async fn read_goal(
        &self,
        req: Request<ReadGoalRequest>,
    ) -> Result<Response<ReadGoalResponse>, Status> {
        let ReadGoalRequest { id: _id } = req.into_inner();
        let goal = Goal::default();
        let resp = ReadGoalResponse { goal: Some(goal) };
        Ok(Response::new(resp))
    }

    async fn read_multi_goals(
        &self,
        req: Request<ReadMultiGoalsRequest>,
    ) -> Result<Response<ReadMultiGoalsResponse>, Status> {
        let ReadMultiGoalsRequest {} = req.into_inner();
        let goals = vec![];
        let resp = ReadMultiGoalsResponse { goals };
        Ok(Response::new(resp))
    }

    async fn update_goal(
        &self,
        req: Request<UpdateGoalRequest>,
    ) -> Result<Response<UpdateGoalResponse>, Status> {
        let UpdateGoalRequest { goal } = req.into_inner();
        let goal = safely_extract(goal)?;
        let resp = UpdateGoalResponse { id: goal.id };
        Ok(Response::new(resp))
    }

    async fn delete_goal(
        &self,
        req: Request<DeleteGoalRequest>,
    ) -> Result<Response<DeleteGoalResponse>, Status> {
        let DeleteGoalRequest { id } = req.into_inner();
        let resp = DeleteGoalResponse { id };
        Ok(Response::new(resp))
    }
}

fn safely_extract(goal: Option<Goal>) -> Result<Goal, Status> {
    match goal {
        Some(goal) => Ok(goal),
        _ => Err(Status::invalid_argument("Not a valid Goal Object."))
    }
}
