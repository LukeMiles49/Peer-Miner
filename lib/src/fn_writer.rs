use std::io::{
	self,
	Write,
};

pub struct FnWriter<F: FnMut(&[u8]) + Send> {
	handler: F,
}

impl<F: FnMut(&[u8]) + Send> FnWriter<F> {
	pub fn new(handler: F) -> Self {
		Self {
			handler,
		}
	}
}

impl<F: FnMut(&[u8]) + Send> Write for FnWriter<F> {
	fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
		let len = buf.len();
		(self.handler)(buf);
		Ok(len)
	}
	
	fn flush(&mut self) -> io::Result<()> {
		Result::Ok(())
	}
}
