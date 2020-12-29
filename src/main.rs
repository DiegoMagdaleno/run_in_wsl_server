extern crate mime_guess;
use structopt::StructOpt;
mod lib;
mod server;



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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = ApplicationArguments::from_args();

    match args.subcommand {
        SubCommand::StartServer(opts) => {
            println!("Started the server at {:?}", opts.server_listen_addr);
            server::server::start_server(opts).await?;
        }
    }

    Ok(())
}