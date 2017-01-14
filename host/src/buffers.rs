use std::ops::{Index, IndexMut};

use hero_core::processing::AudioOutputBuffer;

pub struct DeinterlacedOutputBuffer {
    buffer: *mut f32
}

impl DeinterlacedOutputBuffer {

    pub fn from_left_channel(buffer: &mut [f32]) -> DeinterlacedOutputBuffer {
        DeinterlacedOutputBuffer {
            buffer: buffer as *mut _ as *mut f32
        }
    }

    pub fn from_right_channel(buffer: &mut [f32]) -> DeinterlacedOutputBuffer {
        let buffer = buffer as *mut _ as *mut f32;
        DeinterlacedOutputBuffer {
            buffer: unsafe { buffer.offset(1) }
        }
    }
}

impl Index<usize> for DeinterlacedOutputBuffer {
    type Output = f32;

    fn index<'b>(&'b self, index: usize) -> &f32 {
        unsafe { &*self.buffer.offset((index * 2) as isize) }
    }
}

impl IndexMut<usize> for DeinterlacedOutputBuffer {

    fn index_mut<'b>(&'b mut self, index: usize) -> &mut f32 {
        unsafe { &mut *self.buffer.offset((index * 2) as isize) }
    }
}

impl AudioOutputBuffer<f32> for DeinterlacedOutputBuffer {}

pub struct DeinterlacedOutputBuffers {
    pub left: DeinterlacedOutputBuffer,
    pub right: DeinterlacedOutputBuffer
}

impl DeinterlacedOutputBuffers {
    pub fn from(buffer: &mut [f32]) -> DeinterlacedOutputBuffers {
        DeinterlacedOutputBuffers {
            left: DeinterlacedOutputBuffer::from_left_channel(buffer),
            right: DeinterlacedOutputBuffer::from_right_channel(buffer)
        }
    }
}
