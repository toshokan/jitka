use super::hook::*;
use futures::{
    sink::SinkExt,
    stream::StreamExt,
    stream::FuturesUnordered
};

pub struct Scheduler {
    sink: super::Sender<TaskOutput>
}

impl Scheduler {
    pub fn output_to(sink: super::Sender<TaskOutput>) -> Self {
	Self { sink }
    }
    
    pub async fn schedule(&mut self, mut tasks: Vec<Hook>) -> Option<()> {
	let mut stream = FuturesUnordered::new();
	tasks.drain(..).for_each(|t| {
	    stream.push(t.schedule())
	});

	while let Some(t) = stream.next().await {
	    self.sink.send(
		TaskOutput {
		    tag: t.tag.clone(),
		    separator: " <> ".to_string(),
		    body: "aha!".to_string()
		}
	    ).await.ok()?;
	    stream.push(t.schedule())
	}
	
	Some(())
    }
}
