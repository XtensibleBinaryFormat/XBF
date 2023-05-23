use std::{
    io::{self, BufRead, Write},
    net::TcpStream,
};

pub struct LinesCodec {
    reader: io::BufReader<TcpStream>,
    writer: io::LineWriter<TcpStream>,
}

impl LinesCodec {
    pub fn new(stream: TcpStream) -> io::Result<Self> {
        let writer = io::LineWriter::new(stream.try_clone()?);
        let reader = io::BufReader::new(stream);

        Ok(Self { reader, writer })
    }

    pub fn send_message(&mut self, message: &str) -> io::Result<()> {
        self.writer.write(message.as_bytes())?;
        self.writer.write(b"\n")?;
        Ok(())
    }

    pub fn receive_message(&mut self) -> io::Result<String> {
        let mut message = String::new();
        self.reader.read_line(&mut message)?;
        message.pop();
        Ok(message)
    }
}
