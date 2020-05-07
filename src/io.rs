use async_std::io::prelude::*;
use async_std::io::Write;
use futures::stream::StreamExt;

use std::marker::Unpin;
use super::hook::*;

pub struct Writer<W> {
    write: W
}

impl<W: Write + Unpin + Send + Sync> Writer<W> {
    pub fn new(w: W) -> Self {
	Self {
	    write: w
	}
    }
    
    pub async fn consume(self, mut recv: super::Receiver<TaskOutput>) -> Option<()> {
	let mut write = self.write;
	while let Some(task) = recv.next().await {
	    let fut = writeln!(write, "{}", task);
	    fut.await.ok()?;
	}
	Some(())
    }
}
