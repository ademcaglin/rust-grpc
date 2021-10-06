use services::user_service::users::user_service_server::UserServiceServer;
use services::user_service::UserServiceImpl;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:5010".parse()?;
    let user_service = UserServiceImpl::default();
    let svc = UserServiceServer::new(user_service);
    Server::builder().add_service(svc).serve(addr).await?;

    Ok(())
}

pub mod services;
pub mod conn;
