use std::{io::{Read, Result, Write}};

// the PhantomData instances in this file are just to stop compiler complaints
// about missing generics; feel free to remove them

pub struct ReadStats<R>(R, usize, usize);

impl<R: Read> ReadStats<R> {
    pub fn new(wrapped: R) -> ReadStats<R> {
        ReadStats(wrapped, 0, 0)
    }

    pub fn get_ref(&self) -> &R {
        &self.0
    }

    pub fn bytes_through(&self) -> usize {
        self.1
    }

    pub fn reads(&self) -> usize {
        self.2
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.0.read(buf).inspect(|c| {
            self.1 += c;
            self.2 += 1;
        })
    }
}

pub struct WriteStats<W>(W, usize, usize);

impl<W: Write> WriteStats<W> {
    pub fn new(wrapped: W) -> WriteStats<W> {
        WriteStats(wrapped, 0, 0)
    }

    pub fn get_ref(&self) -> &W {
        &self.0
    }

    pub fn bytes_through(&self) -> usize {
        self.1
    }

    pub fn writes(&self) -> usize {
        self.2
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.0.write(buf).inspect(|c| {
            self.1 += c;
            self.2 += 1;
        })
    }

    fn flush(&mut self) -> Result<()> {
        self.0.flush()
    }
}
