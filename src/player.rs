use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Condvar, Mutex}; // sync can be safely shared with multiple threads
use std::thread;
use crossbeam::sync::SegQueue; // lock-free queue, can be used by multiple threads without a lock
use pulse_simple::Playback;
// use gtk::{ApplicationWindow, FileChooserAction, FileChooserDialog, FileFilter};
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
    // load: Arc<SegQueue<Action>>,
    // load: Arc<crate::player::Action>,
}

pub struct Player {
	app_state: Arc<Mutex<super::State>>,
	event_loop: EventLoop,
    // pub load: EventLoop,
}

// create the queue and the Boolean wrapped in a Mutex
impl EventLoop {
	fn new() -> Self {
		EventLoop {
			queue: Arc::new(SegQueue::new()),
			playing: Arc::new(Mutex::new(false)),
            // load: Arc::new(SegQueue::new()),
		}
	}
}

// player constructor
impl Player {
// method for self.player.load(&path);
    pub(crate) fn load(&self, path: &String) {}

	pub(crate) fn new(app_state: Arc<Mutex<super::State>>) -> Self {
		let event_loop = EventLoop::new();
        // let load = EventLoop::new();
		{
			let app_state = app_state.clone();
			let event_loop = event_loop.clone();
            // let load = load.clone();
			thread::spawn(move || {
                let mut buffer = [[0; 2]; BUFFER_SIZE];
                let mut playback = Playback::new("MP3", "MP3 Playback", None, DEFAULT_RATE);
                let mut source = None;
                loop {
                    if let Some(action) = event_loop.queue.try_pop() {
                        match action {
                            Load(path) => {
                                let file = File::open(path).unwrap();
                                source = Some(Mp3Decoder::new(BufReader::new(file)).unwrap());
                                let rate = source.as_ref().map(|source|
            source.samples_rate()).unwrap_or(DEFAULT_RATE);
                                playback = Playback::new("MP3", "MP3 Playback", None, rate);
                                // app_state.lock().unwrap().stopped = false; // first use of Mutex
                                let mut guard = app_state.lock().unwrap(); // first use of Mutex
                                guard.stopped = false; // first use of Mutex
                            },
                            Stop => {}, // handle it later
                        }
                    }
                    else if *event_loop.playing.lock().unwrap() {
                        let mut written = false;
                        if let Some(ref mut source) = source {
                            let size = iter_to_buffer(source, &mut buffer);
                            if size > 0 {
                                playback.write(&buffer[..size]);
                                written = true;
                            }
                        }
                        if !written {
                            app_state.lock().unwrap().stopped = true;
                            *event_loop.playing.lock().unwrap() = false;
                            source = None;
                        }
                    }
                }
			});
		}
		Player {
			app_state,
			event_loop,
            // load,
		}
	}
}

// take the value from decoder, write to buffer
fn iter_to_buffer<I: Iterator<Item=i16>>(iter: &mut I, buffer: &mut [[i16;
    2]; BUFFER_SIZE]) -> usize {
    let mut iter = iter.take(BUFFER_SIZE);
    let mut index = 0;
    while let Some(sample1) = iter.next() {
        if let Some(sample2) = iter.next() {
            buffer[index][0] = sample1;
            buffer[index][1] = sample2;
        }
        index += 1;
    }
    index
}