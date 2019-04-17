#[macro_use]
extern crate log;

use futures::prelude::*;
use raffineria::cli::Cli;

struct SinkIgnore;

impl Sink for SinkIgnore {
    type SinkItem = i32;
    type SinkError = failure::Error;

    fn start_send(&mut self, item: Self::SinkItem) -> StartSend<Self::SinkItem, Self::SinkError> {
        debug!("received: {:?}", item);
        Ok(AsyncSink::Ready)
    }

    fn poll_complete(&mut self) -> Poll<(), Self::SinkError> {
        Ok(Async::Ready(()))
    }
}

impl Cli for SinkIgnore {
    fn name() -> &'static str {
        "sink-ignore"
    }
    fn use_dotenv(&self) -> bool {
        true
    }
    fn run(self) -> Result<(), failure::Error> {
        info!("SinkIgnore::run(...)");
        let ports = raffineria::os_process::Ports::stdio();

        let schema = raffineria::protocol::Schema::Int;
        let sink = raffineria::os_process::std::SinkStage::from_sink(self, &schema);
        let runner = raffineria::os_process::Stage::into_future(sink, ports);

        raffineria::run(runner.map_err(|reason| error!("{:?}", reason)));

        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    raffineria::util::run(SinkIgnore)
}
