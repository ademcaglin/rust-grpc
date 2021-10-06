use crate::conn::establish_connection;
use tonic::{Request, Response, Status};
pub mod users {
    tonic::include_proto!("users");
}
use users::user_service_server::UserService;
use users::{UserDetailsResult, UserIdInput};

#[derive(Debug, Default)]
pub struct UserServiceImpl {}

#[tonic::async_trait]
impl UserService for UserServiceImpl {
    async fn get_user(
        &self,
        request: Request<UserIdInput>,
    ) -> Result<Response<UserDetailsResult>, Status> {
        println!("Got a request: {:?}", request);
        let UserIdInput { id } = &request.into_inner();
        let client = establish_connection().await.unwrap();
        let rows = &client
            .query("SELECT * FROM users WHERE id = $1", &[&id])
            .await
            .unwrap();
        let row = rows.get(0).unwrap();
        let reply = UserDetailsResult {
            id: *id,
            name: row.get(1),
            email: row.get(2),
            deleted: row.get(3),
        };

        Ok(Response::new(reply))
    }
}
