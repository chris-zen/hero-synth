use std::ops::{Index, IndexMut};
use std::marker::PhantomData;

pub trait AudioOutputBuffer<S>: Index<usize, Output=S> + IndexMut<usize, Output=S> {}

pub struct ProcessingArgs<'a, S, O> where O: AudioOutputBuffer<S> + 'a {
    pub num_frames: usize,
    pub audio_left: &'a mut O,
    pub audio_right: &'a mut O,
    phantom1: PhantomData<&'a O>,
    phantom2: PhantomData<S>
}

impl<'a, S, O> ProcessingArgs<'a, S, O> where O: AudioOutputBuffer<S, Output=S> + 'a {
    pub fn new(num_frames: usize, audio_left: &'a mut O, audio_right: &'a mut O) -> ProcessingArgs<'a, S, O> {
        ProcessingArgs {
            num_frames: num_frames,
            audio_left: audio_left,
            audio_right: audio_right,
            phantom1: PhantomData,
            phantom2: PhantomData
        }
    }
}

pub trait Processor<'a, S, O> where O: AudioOutputBuffer<S> {
    fn process(&mut self, args: ProcessingArgs<'a, S, O>);
}
