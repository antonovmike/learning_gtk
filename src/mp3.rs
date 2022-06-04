use std::io::{Read, Seek, SeekFrom};
use std::time::Duration;

use simplemad;

pub struct Mp3Decoder<R> where R: Read {
	reader: simplemad::Decoder<R>,
	current_frame: simplemad::Frame,
	current_frame_channel: usize,
	current_frame_sample_pos: usize,
	current_time: u64,
}
