use std::io::{Read, Seek, SeekFrom};
use std::time::Duration;

use simplemad; // decodes frames of mpP3

pub struct Mp3Decoder<R>
// decodes frames of mpP3
where
    R: Read,
{
    reader: simplemad::Decoder<R>, // information goes from the simplemad
    current_frame: simplemad::Frame,
    current_frame_channel: usize,
    current_frame_sample_pos: usize,
    current_time: u64,
}
