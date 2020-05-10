use super::hook::*;
use futures::{
    sink::SinkExt,
    stream::StreamExt,
};

pub struct Scheduler {
    sink: super::Sender<TaskOutput>
}

impl Scheduler {
    pub fn output_to(sink: super::Sender<TaskOutput>) -> Self {
	Self { sink }
    }
    
    pub async fn schedule(&mut self, tasks: Vec<Hook>) -> Option<()> {
	let streams = {
	    let mut streams = vec![];
	    for task in tasks {
		if let Some(stream) = task.stream().await {
		    streams.push(stream);
		}
	    }
	    streams
	};
	let mut stream = futures::stream::select_all(streams);
	while let Some(t) = stream.next().await {
	    self.sink.send(t).await.ok()?
	}
	
	Some(())
    }
}
