#[macro_use]
extern crate log;

use futures::prelude::*;
use futures::stream;
use raffineria::cli::Cli;

struct SourceSeq;

impl Cli for SourceSeq {
    fn name() -> &'static str {
        "source-seq"
    }
    fn use_dotenv(&self) -> bool {
        true
    }
    fn run(self) -> Result<(), failure::Error> {
        info!("SourceSeq::run(...)");
        let ports = raffineria::os_process::Ports::stdio();

        let schema = raffineria::protocol::Schema::Int;
        let source = raffineria::os_process::std::SourceStage::from_stream(
            stream::iter_ok::<_, failure::Error>(0..1000000),
            &schema,
        );
        let runner = raffineria::os_process::Stage::into_future(source, ports);

        raffineria::run(runner.map_err(|reason| error!("{:?}", reason)));

        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    raffineria::util::run(SourceSeq)
}
