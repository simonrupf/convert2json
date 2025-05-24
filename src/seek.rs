#![cfg(any(feature = "plist", feature = "plist2json", feature = "plistq"))]
use std::io::{BufRead, Read, Result, Seek, SeekFrom};

// Wrapper for readers to add Seek trait
pub struct BufSeek<R: BufRead> {
    inner: R,
    buffer: Vec<u8>,
    position: usize,
    end_reached: bool,
}

impl<R: BufRead> BufSeek<R> {
    pub fn new(inner: R) -> Self {
        Self {
            inner,
            buffer: vec![],
            position: 0,
            end_reached: false,
        }
    }

    fn fill_to(&mut self, end: usize) -> Result<()> {
        while self.buffer.len() < end && !self.end_reached {
            let mut tmp = [0u8; 8 * 1024];
            let n = self.inner.read(&mut tmp)?;
            if n == 0 {
                self.end_reached = true;
                break;
            }
            self.buffer.extend_from_slice(&tmp[..n]);
        }
        Ok(())
    }
}

impl<R: BufRead> BufRead for BufSeek<R> {
    fn fill_buf(&mut self) -> Result<&[u8]> {
        self.inner.fill_buf()
    }
    fn consume(&mut self, amt: usize) {
        self.inner.consume(amt);
    }
}

impl<R: BufRead> Read for BufSeek<R> {
    fn read(&mut self, out: &mut [u8]) -> Result<usize> {
        let available = self.buffer.len().saturating_sub(self.position);
        if available > 0 {
            let to_read = available.min(out.len());
            let start = self.position;
            out[..to_read].copy_from_slice(&self.buffer[start..start + to_read]);
            self.position += to_read;
            return Ok(to_read);
        }
        if self.end_reached {
            return Ok(0);
        }
        let n = self.inner.read(out)?;
        if n == 0 {
            self.end_reached = true;
        } else {
            self.buffer.extend_from_slice(&out[..n]);
            self.position += n;
        }
        Ok(n)
    }
}

impl<R: BufRead> Seek for BufSeek<R> {
    fn seek(&mut self, how: SeekFrom) -> Result<u64> {
        let target = match how {
            SeekFrom::Start(n) => usize::try_from(n).unwrap_or(usize::MAX),
            SeekFrom::End(n) => {
                // buffer everything to find the end
                self.fill_to(usize::MAX)?;
                if n < 0 {
                    self.buffer
                        .len()
                        .saturating_sub(usize::try_from(-n).unwrap_or(usize::MAX))
                } else {
                    self.buffer
                        .len()
                        .saturating_add(usize::try_from(n).unwrap_or(usize::MAX))
                }
            }
            SeekFrom::Current(n) => {
                if n < 0 {
                    self.position
                        .saturating_sub(usize::try_from(-n).unwrap_or(usize::MAX))
                } else {
                    self.position
                        .saturating_add(usize::try_from(n).unwrap_or(usize::MAX))
                }
            }
        };
        if target > self.buffer.len() {
            self.fill_to(target)?;
        }
        self.position = target.min(self.buffer.len());
        Ok(self.position as u64)
    }
}
