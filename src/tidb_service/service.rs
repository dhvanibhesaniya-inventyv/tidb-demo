use tonic::{Request, Response, Status};

use crate::tidb::{self, CheckConnectionRequest, CheckConnectionResponse};
use crate::utils::tidb::{check_connection, execute_query};

use tidb::tidb_server::Tidb;
use tidb::{ExecuteQueryRequest, ExecuteQueryResponse};

#[derive(Debug, Default)]
pub struct TidbService {}

#[tonic::async_trait]
impl Tidb for TidbService {
    async fn execute_query(
        &self,
        request: Request<ExecuteQueryRequest>,
    ) -> Result<Response<ExecuteQueryResponse>, Status> {
        //converting Request to ExecuteQueryRequest object
        let req_data = request.into_inner();
        // send bad request response in case of query & database name not provided
        if req_data.query.len() > 0 && req_data.db_name.len() > 0 {
            let result =
                execute_query(req_data.query.to_owned(), req_data.db_name.to_owned()).await;
            match result {
                Ok(result) => {
                    let reply = ExecuteQueryResponse {
                        code: 200,
                        msg: result.0,
                        data: result.1,
                    };
                    return Ok(Response::new(reply));
                }
                Err(error) => {
                    let reply = ExecuteQueryResponse {
                        code: 400,
                        msg: "error".to_string(),
                        data: error,
                    };
                    return Ok(Response::new(reply));
                }
            }
        } else {
            let reply = ExecuteQueryResponse {
                code: 500,
                msg: "error".to_string(),
                data: "Bad Request".to_string(),
            };
            return Ok(Response::new(reply));
        }
    }
    async fn check_connection(
        &self,
        request: Request<CheckConnectionRequest>,
    ) -> Result<Response<CheckConnectionResponse>, Status> {
        //converting Request to ExecuteQueryRequest object
        let req_data = request.into_inner();
        if req_data.db_name.len() > 0 {
            let res = check_connection(req_data.db_name.to_owned()).await;
            match res {
                Ok(_) => {
                    return Ok(Response::new(CheckConnectionResponse {
                        code: 200,
                        msg: format!("Connection with db {} is healthy", req_data.db_name),
                    }));
                }
                Err(error) => {
                    let reply = CheckConnectionResponse {
                        code: 400,
                        msg: format!(
                            "Error in Connection with db {} Error:-{}",
                            req_data.db_name, error
                        ),
                    };
                    return Ok(Response::new(reply));
                }
            }
        }
        let reply = CheckConnectionResponse {
            code: 500,
            msg: "Bad Request".to_string(),
        };
        return Ok(Response::new(reply));
    }
}
