use my_lib::hello_world::greeter_server::Greeter;
use my_lib::hello_world::{HelloReply, HelloRequest};
use tonic::{Request, Response, Result};
use tracing::{info, instrument};

#[derive(Debug, Default)]
pub struct Service;

#[tonic::async_trait]
impl Greeter for Service {
    #[instrument]
    async fn say_hello(&self, request: Request<HelloRequest>) -> Result<Response<HelloReply>> {
        info!("Got a request from {:?}", request.remote_addr());

        Ok(Response::new(HelloReply {
            message: format!("Hello {}!", request.into_inner().name),
        }))
    }
}
