use std::io::{Read, Seek, SeekFrom};
use std::time::Duration;

use simplemad; // decodes frames of mpP3

// decodes frames of mpP3
pub struct Mp3Decoder<R>
where
    R: Read,
{
    reader: simplemad::Decoder<R>, // information goes from the simplemad
    current_frame: simplemad::Frame,
    current_frame_channel: usize,
    current_frame_sample_pos: usize,
    current_time: u64,
}

// checks whether a stream of data is an mp3 file
fn is_mp3<R>(mut data: R) -> bool
where
    R: Read + Seek,
{
    let stream_pos = data.seek(SeekFrom::Current(0)).unwrap();
    let is_mp3 = simplemad::Decoder::decode(data.by_ref()).is_ok();
    data.seek(SeekFrom::Start(stream_pos)).unwrap();
    is_mp3
}

// decoding the next frame of an mp3 file
fn next_frame<R: Read>(decoder: &mut simplemad::Decoder<R>) -> simplemad::Frame {
    decoder
        .filter_map(|f| f.ok())
        .next()
        .unwrap_or_else(|| simplemad::Frame {
            bit_rate: 0,
            layer: Default::default(),
            mode: Default::default(),
            sample_rate: 44100,
            samples: vec![Vec::new()],
            position: Duration::from_secs(0),
            duration: Duration::from_secs(0),
        })
}
