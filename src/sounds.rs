use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink, source::Source};
use rodio::queue::SourcesQueueOutput;
use rodio::source::SamplesConverter;

pub struct Sound{
    _stream: OutputStream,
    sink: Sink
}

impl Sound{
    pub fn new() ->Self{
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        Sound{
            _stream,
            sink:Sink::try_new(&stream_handle).unwrap(),
        }
    }

    pub fn play_noise(&self){
        let file = BufReader::new(File::open("src/assets/Chip8Emu_smw2_coin.wav").unwrap());
        let source = Decoder::new(file).unwrap();
        if self.sink.empty() {
            self.sink.append(source);
        }
    }
}