use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Condvar, Mutex}; // sync can be safely shared with multiple threads
use std::thread;
use crossbeam::sync::SegQueue; // lock-free queue, can be used by multiple threads without a lock
use pulse_simple::Playback;
use super::mp3::Mp3Decoder;
use self::Action::*;

const BUFFER_SIZE: usize = 1000;
const DEFAULT_RATE: u32 = 44100;

enum Action {
    Load(PathBuf),
    Stop,
}

#[derive(Clone)]
struct EventLoop {
    queue: Arc<SegQueue<Action>>,
    playing: Arc<Mutex<bool>>,
}

// create the queue and the Boolean wrapped in a Mutex
impl EventLoop {
	fn new() -> Self {
		EventLoop {
			queue: Arc::new(SegQueue::new()),
			playing: Arc::new(Mutex::new(false)),
		}
	}
}
