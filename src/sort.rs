use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::thread::{spawn, JoinHandle};

pub fn start_thread_sort(
    from_read: Receiver<String>,
    to_write: Sender<String>,
    reverse: bool,
) -> JoinHandle<()> {
    spawn(move || {
        let mut buffer = Vec::new();
        for l in from_read {
            buffer.push(l);
        }
        buffer.sort();
        if reverse {
            buffer.reverse();
        }
        for l in buffer {
            if to_write.send(l).is_err() {
                println!("error sending to write");
                return;
            }
        }
    })
}
