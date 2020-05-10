use std::fmt;
use async_std::task;
use std::time::Duration;

use futures::stream;

pub struct Hook {
    pub tag: String,
    pub kind: Kind,
    pub renderer: Option<String>,
    pub separator: String
}


impl Hook {
    pub async fn schedule(self) -> Self {
	use Kind::*;
	match &self.kind {
	    Interval {millis} => {
		task::sleep(Duration::from_millis(*millis)).await;
	    },
	    _ => ()
	}
	
	self
    }

    pub async fn be(&self) -> TaskOutput {
	TaskOutput {
	    tag: "be".to_string(),
	    body: "be".to_string(),
	    separator: "<>".to_string(),
	}
    }

    pub async fn stream(self) -> Option<Box<dyn stream::Stream<Item = TaskOutput>>> {
    	use Kind::*;
    	match &self.kind {
    	    Interval {millis} => {
		let interval = Duration::from_millis(*millis);
    		let s = stream::unfold((self, interval), |(s, i)| async move {
    		    task::sleep(i).await;
    		    let output = s.be().await;
    		    Some((output, (s, i)))
    		});
		Some(Box::new(s))
    	    },
	    Queue {path} => {
		use async_std::prelude::*;
	    	let file = async_std::fs::File::open(path).await.ok()?;
		let reader = async_std::io::BufReader::new(file);
		let lines = reader.lines();
		let s = stream::unfold((self, lines), |(s, mut l)| async move {
		    let _line = l.next().await;
		    let output = s.be().await;
		    Some((output, (s, l)))
		});
		Some(Box::new(s))
	    }
	    
    	    _ => unimplemented!()
    	}
    }
}

pub enum Kind {
    Interval {
	millis: u64
    },
    Changed {
	path: String
    },
    Queue {
	path: String
    }
}


pub struct TaskOutput {
    pub tag: String,
    pub body: String,
    pub separator: String,
}

impl fmt::Display for TaskOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
	write!(f, "{}{}{}", self.tag, self.separator, self.body)
    }
}
