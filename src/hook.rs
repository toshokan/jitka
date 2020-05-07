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

    // pub async fn execute() -> TaskOutput {
    // 	unimplemented!();
    // }

    // pub fn stream(self) -> impl stream::Stream<Item = TaskOutput> {
    // 	use Kind::*;
    // 	match &self.kind {
    // 	    Interval {millis} => {
    // 		let interval = Duration::from_millis(*millis);
    // 		stream::unfold((), move |()| async {
    // 		    task::sleep(interval).await;
    // 		    let output = Self::execute().await;
    // 		    Some((output, ()))
    // 		})
    // 	    },
    // 	    _ => unimplemented!()
    // 	}
    // }
}

pub enum Kind {
    Interval {
	millis: u64
    },
    Changed {
	paths: Vec<String>
    },
    Queue {
	paths: Vec<String>
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
