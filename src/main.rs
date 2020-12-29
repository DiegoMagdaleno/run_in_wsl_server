extern crate mime_guess;
mod lib {
    pub mod parser;
    pub mod finder;
    pub mod models;
}

#[derive(Debug)]
struct FileService;

pub mod runinwsl {
    tonic::include_proto!("runinwsl");
}

use runinwsl::file_builder_server::{FileBuilder, FileBuilderServer};
use runinwsl::{FileReply, FileRequest};

use futures_core::Stream;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::mpsc;
use tonic::{Request, Response, Status};
use std::path::Path;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "riwserver")]
struct ApplicationArguments {
    #[structopt(flatten)]
    pub subcommand: SubCommand,
}

#[derive(Debug, StructOpt)]
pub enum SubCommand {
    #[structopt(name = "server")]
    StartServer(ServerOptions)
}

#[derive(Debug, StructOpt)]
pub struct ServerOptions {
    #[structopt(long, default_value = "127.0.0.1:42069")]
    pub server_listen_addr: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = ApplicationArguments::from_args();

    match args.subcommand {
        SubCommand::StartServer(opts) => {
            println!("Started the server at {:?}", opts.server_listen_addr);
        }
    }

    Ok(())
}