use proto::{
    methods::{goals_server::Goals, *},
    objects::Goal,
};
use scylla::transport::session::Session;
use tonic::{Request, Response, Status};

use crate::database;

pub struct GoalService {
    db_session: Session,
}

impl GoalService {
    pub(super) fn new(db_session: Session) -> Self {
        Self { db_session }
    }
}

pub fn init_goal_server(db_session: Session) -> goals_server::GoalsServer<GoalService> {
    goals_server::GoalsServer::new(GoalService::new(db_session))
}

#[tonic::async_trait]
impl Goals for GoalService {
    async fn create_goal(
        &self,
        req: Request<CreateGoalRequest>,
    ) -> Result<Response<CreateGoalResponse>, Status> {
        let CreateGoalRequest { goal } = req.into_inner();
        let goal = safely_extract(goal)?;
        let id = goal.id.clone();

        database::insert_goal(&self.db_session, goal).await?;
        let resp = CreateGoalResponse { id };
        Ok(Response::new(resp))
    }

    async fn read_goal(
        &self,
        req: Request<ReadGoalRequest>,
    ) -> Result<Response<ReadGoalResponse>, Status> {
        let ReadGoalRequest { id } = req.into_inner();

        let goal = database::read_goal(&self.db_session, id).await?;
        let resp = ReadGoalResponse { goal: Some(goal) };
        Ok(Response::new(resp))
    }

    async fn read_multi_goals(
        &self,
        req: Request<ReadMultiGoalsRequest>,
    ) -> Result<Response<ReadMultiGoalsResponse>, Status> {
        let ReadMultiGoalsRequest {} = req.into_inner();

        let goals = database::read_multi_goals(&self.db_session).await?;
        let resp = ReadMultiGoalsResponse { goals };
        Ok(Response::new(resp))
    }

    async fn update_goal(
        &self,
        req: Request<UpdateGoalRequest>,
    ) -> Result<Response<UpdateGoalResponse>, Status> {
        let UpdateGoalRequest { goal } = req.into_inner();
        let goal = safely_extract(goal)?;
        let id = goal.id.clone();

        database::insert_goal(&self.db_session, goal).await?;
        let resp = UpdateGoalResponse { id };
        Ok(Response::new(resp))
    }

    async fn delete_goal(
        &self,
        req: Request<DeleteGoalRequest>,
    ) -> Result<Response<DeleteGoalResponse>, Status> {
        let DeleteGoalRequest { id } = req.into_inner();

        database::delete_goal(&self.db_session, id.clone()).await?;
        let resp = DeleteGoalResponse { id };
        Ok(Response::new(resp))
    }
}

fn safely_extract(goal: Option<Goal>) -> Result<Goal, Status> {
    match goal {
        Some(goal) => Ok(goal),
        _ => Err(Status::invalid_argument("Not a valid Goal Object.")),
    }
}
