use async_std::prelude::*;
use async_std::task;
use std::fmt;
use futures::channel::mpsc;
use futures::stream::FuturesUnordered;
use futures::sink::SinkExt;

struct Interval {
    tag: String,
    millis: u32,
    times: u32,
    max: u32
}

struct TaskOutput {
    tag: String,
    body: String
}

impl fmt::Display for TaskOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
	write!(f, "{} <msg> {}", self.tag, self.body)
    }
}

async fn start() -> Option<()> {
    let (send, recv) = mpsc::unbounded();
    let output = task::spawn(output(recv));

    let tasks = read_tasks();
    do_tasks(tasks, send.clone()).await;
    
    drop(send);
    output.await?;
    Some(())
}


fn read_tasks() -> Vec<Interval> {
    vec![]
}

async fn sched(i: Interval) -> Interval {
    task::sleep(std::time::Duration::from_millis(i.millis.into())).await;
    i
}

async fn do_tasks(mut tasks: Vec<Interval>, mut sink: mpsc::UnboundedSender<TaskOutput>) -> Option<()> {
    let mut stream = FuturesUnordered::new();
    tasks.drain(..).for_each(|t| {
	stream.push(sched(t))
    });

    while let Some(mut t) = stream.next().await {
	sink.send(
	    TaskOutput {
		tag: t.tag.clone(),
		body: format!("times: {}", t.times)
	    }
	).await.ok()?;
	if t.times < t.max {
	    t.times += 1;
	    stream.push(sched(t))
	}
    }
    
    Some(())
}

async fn output(mut receiver: mpsc::UnboundedReceiver<TaskOutput>) -> Option<()> {
    while let Some(task) = receiver.next().await {
	println!("{}", task);
    }
    Some(())
}


#[async_std::main]
async fn main() {
    start().await;
}
