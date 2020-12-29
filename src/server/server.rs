use tonic::{transport::Server, Request, Response, Status};

pub mod runinwsl_proto {
    tonic::include_proto!("runinwsl");
}

/* Proto generated traits */
use runinwsl_proto::run_in_wsl_server::{RunInWsl, RunInWslServer};

/* Generated message structs for sending and reciving apps */
use runinwsl_proto::{FileReply, FileRequest};

/* We use this to parse the address */
use crate::ServerOptions;

use std::sync::Arc;

use crate::lib::{finder, models, parser};

#[derive(Debug, Default)]
pub struct File {}

use std::path::Path;

use tokio::sync::mpsc;

#[tonic::async_trait]
impl RunInWsl for File {
    type GetFileTypesStream = mpsc::Receiver<Result<FileReply, Status>>;

    async fn get_file_types(
        &self,
        request: Request<FileRequest>,
    ) -> Result<Response<Self::GetFileTypesStream>, Status> {
        /* Convert to Path, so we can extract the extension */
        let req_path = request.into_inner();

        /* The Path should always be valid, because we are expecting a request from Windows */
        if let Some(path) = Path::new(&req_path.file_path).extension() {
            let path_string = path.to_str().unwrap();

            if let Some(guess_type) = mime_guess::from_ext(&path_string).first_raw() {
                if let Some(capable_apps) = finder::get_capable_at_location(&guess_type) {
                    if let Some(app_stream) = parser::parse_entries(capable_apps) {
                        let app_stream = Arc::new(models::load(app_stream));

                        let (mut tx, rx) = mpsc::channel(4);
                        let applications = app_stream.clone();

                        tokio::spawn(async move {
                            for app in &applications[..] {
                                tx.send(Ok(app.clone())).await.unwrap();
                            }
                        });

                        Ok(Response::new(rx))
                    } else {
                        Err(tonic::Status::new(
                            tonic::Code::Internal,
                            "Critical internal error at parsing",
                        ))
                    }
                } else {
                    Err(tonic::Status::new(
                        tonic::Code::NotFound,
                        "No application can open the mimetype",
                    ))
                }
            } else {
                Err(tonic::Status::new(
                    tonic::Code::NotFound,
                    "File has no mimetype",
                ))
            }
        } else {
            Err(tonic::Status::new(
                tonic::Code::NotFound,
                "File has no extension.",
            ))
        }
    }
}

pub async fn start_server(opts: ServerOptions) -> Result<(), Box<dyn std::error::Error>> {
    let addr = opts.server_listen_addr.parse().unwrap();

    let runner_server = File::default();

    println!("Remote RunInWSLServer is listening on {}", addr);

    Server::builder()
    .add_service(RunInWslServer::new(runner_server))
    .serve(addr)
    .await?;

    Ok(())
}