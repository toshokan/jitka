use futures::channel::mpsc;

mod hook;
mod io;
mod scheduler;
pub mod server;

type Receiver<T> = mpsc::UnboundedReceiver<T>;
type Sender<T> = mpsc::UnboundedSender<T>;
