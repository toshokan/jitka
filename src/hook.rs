use std::fmt;
use async_std::{task, fs::File, io::BufReader};
use std::time::Duration;

use futures::stream;


pub struct Hook {
    pub tag: String,
    pub kind: Kind,
    pub renderer: Option<String>,
    pub separator: String
}


impl Hook {
    pub async fn be(&self) -> TaskOutput {
	let body = match &self.renderer {
	    Some(r) => {
		let o = std::process::Command::new(r).output().unwrap().stdout;
		String::from_utf8_lossy(&o).to_string()
	    },
	    _ => "".to_string()
	};
	TaskOutput {
	    tag: self.tag.clone(),
	    body,
	    separator: self.separator.clone()
	}
    }

    pub async fn stream(self) -> Option<super::TaskOutputStream> {
    	use Kind::*;
    	let stream = match &self.kind {
    	    Interval {millis} => {
		let interval = Duration::from_millis(*millis);
    		let s = stream::unfold((self, interval), |(s, i)| async move {
    		    task::sleep(i).await;
    		    let output = s.be().await;
    		    Some((output, (s, i)))
    		});
		futures::stream::StreamExt::boxed(s)
    	    },
	    Queue { path } => {
		use async_std::prelude::*;
		let path = path.clone();
		let line = String::new();
		let s = stream::unfold((self, path, line), |(s, p, mut l)| async move {
		    let file = File::open(&p).await.ok()?;
		    BufReader::new(file).read_line(&mut l).await.ok()?;
		    eprintln!("Got line {:?}", l);
		    let output = s.be().await;
		    Some((output, (s, p, l)))
		});
		futures::stream::StreamExt::boxed(s)
	    }
	    
    	    _ => unimplemented!()
    	};
	Some(stream)
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
