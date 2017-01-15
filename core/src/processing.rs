use std::ops::{Index, IndexMut};
use std::marker::PhantomData;

pub trait AudioInputBuffer: Index<usize> {}

pub trait AudioOutputBuffer: AudioInputBuffer + IndexMut<usize> {}

pub struct ProcessingArgs<'a, S, O>
    where O: AudioOutputBuffer<Output=S> + 'a {

    pub num_frames: usize,
    // pub audio_in_left: &'a mut I,
    // pub audio_in_right: &'a mut I,
    pub audio_out_left: &'a mut O,
    pub audio_out_right: &'a mut O,
    phantom1: PhantomData<&'a O>
}

impl<'a, S, O> ProcessingArgs<'a, S, O>
    where O: AudioOutputBuffer<Output=S> + 'a {

    pub fn new(num_frames: usize,
            //    audio_in_left: &'a mut I,
            //    audio_in_right: &'a mut I,
               audio_out_left: &'a mut O,
               audio_out_right: &'a mut O) -> ProcessingArgs<'a, S, O> {

        ProcessingArgs {
            num_frames: num_frames,
            // audio_in_left: audio_in_left,
            // audio_in_right: audio_in_right,
            audio_out_left: audio_out_left,
            audio_out_right: audio_out_right,
            phantom1: PhantomData
        }
    }
}

pub trait Processor<'a, S, O>
    where O: AudioOutputBuffer<Output=S> {

    fn process(&mut self, args: ProcessingArgs<'a, S, O>);
}
