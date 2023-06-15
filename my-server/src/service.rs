use my_lib::adder::adder_server::Adder;
use my_lib::adder::{AddNumbersRequest, AddNumbersResponse};
use tonic::{Request, Response, Result};
use tracing::instrument;

#[derive(Debug, Default)]
pub struct Service;

#[tonic::async_trait]
impl Adder for Service {
    #[instrument]
    async fn add_numbers(
        &self,
        request: Request<AddNumbersRequest>,
    ) -> Result<Response<AddNumbersResponse>> {
        let operands = request.into_inner();
        Ok(Response::new(AddNumbersResponse {
            sum: operands.a + operands.b,
        }))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn basics() {}
}
