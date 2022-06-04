use std::io::{Read, Seek, SeekFrom};
use std::time::Duration;

use simplemad; // decodes frames of mpP3

use super::to_millis;

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

// constructor of mp3 decoder
// it checks whether the stream contains mp3 data
// not - error
// yes - creates a Decoder from simplemad crate
// next read first frame and get it's time in milliseconds
impl<R> Mp3Decoder<R>
where
    R: Read + Seek,
{
    pub fn new(mut data: R) -> Result<Mp3Decoder<R>, R> {
        if !is_mp3(data.by_ref()) {
            return Err(data);
        }
        let mut reader = simplemad::Decoder::decode(data).unwrap();

        let current_frame = next_frame(&mut reader);
        let current_time = to_millis(current_frame.duration);

        Ok(Mp3Decoder {
            reader,
            current_frame,
            current_frame_channel: 0,
            current_frame_sample_pos: 0,
            current_time,
        })
    }

    // get current time
    pub fn current_time(&self) -> u64 {
        self.current_time
    }
    // get current rate
    pub fn samples_rate(&self) -> u32 {
        self.current_frame.sample_rate
    }

    // method to compute the duration of a song
    pub fn compute_duration(mut data: R) -> Option<Duration> {
        if !is_mp3(data.by_ref()) {
            return None;
        }
        let decoder = simplemad::Decoder::decode_headers(data).unwrap();
        Some(
            decoder
                .filter_map(|frame| match frame {
                    Ok(frame) => Some(frame.duration),
                    Err(_) => None,
                })
                .sum(),
        )
    }
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
