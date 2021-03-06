use futures::channel::mpsc;
use async_std::{io, task};

use super::io::Writer;
use super::scheduler::Scheduler;

#[derive(Default)]
pub struct Server{}

impl Server {
    pub async fn start(&self) -> Option<()> {
	let (send, recv) = mpsc::unbounded();
	let output = task::spawn(async {
	    let writer = Writer::new(io::stdout());
	    writer.consume(recv).await
	});


	{
	    let mut scheduler = Scheduler::output_to(send.clone());
	    scheduler.schedule(vec![
		super::hook::Hook {
		    tag: "test".to_string(),
		    kind: super::hook::Kind::Interval{millis: 500},
		    renderer: Some("date".to_string()),
		    separator: "<>".to_string()
		},
		super::hook::Hook {
		    tag: "file".to_string(),
		    kind: super::hook::Kind::Queue{path: "testfifo".to_string()},
		    renderer: Some("hostname".to_string()),
		    separator: "|".to_string()
		}
	    ]).await;
	} // drop scheduler
	
	drop(send);
	output.await;
	Some(())
    }
}
