#[macro_use]
extern crate log;

use futures::prelude::*;

use raffineria::cli::Cli;
use raffineria::config;
use raffineria::graph::runner::GraphRunner;

struct GraphRunnerCli;

impl Cli for GraphRunnerCli {
    fn name() -> &'static str {
        "graph-runner"
    }
    fn use_dotenv(&self) -> bool {
        true
    }
    fn run(self) -> Result<(), failure::Error> {
        let graph_spec = config::yaml::read(std::io::stdin())?;
        config::yaml::write(std::io::stdout(), graph_spec.clone())?;

        let runner = GraphRunner::top_level(graph_spec)
            .into_future()
            .map(|_| ())
            .map_err(|err| error!("Top-Runner failure: {:?}", err));

        tokio::run(runner);

        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    raffineria::util::run(GraphRunnerCli)
}
