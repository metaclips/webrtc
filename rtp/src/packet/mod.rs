use crate::header::*;
use std::fmt;
use util::Error;

mod packet_test;

// Packet represents an RTP Packet
// NOTE: Raw is populated by Marshal/Unmarshal and should not be modified
#[derive(Debug, Eq, PartialEq, Default)]
pub struct Packet {
    pub header: Header,
    pub payload: Vec<u8>,
    pub raw: Vec<u8>,
}

impl fmt::Display for Packet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out = "RTP PACKET:\n".to_string();

        out += format!("\tVersion: {}\n", self.header.version).as_str();
        out += format!("\tMarker: {}\n", self.header.marker).as_str();
        out += format!("\tPayload Type: {}\n", self.header.payload_type).as_str();
        out += format!("\tSequence Number: {}\n", self.header.sequence_number).as_str();
        out += format!("\tTimestamp: {}\n", self.header.timestamp).as_str();
        out += format!("\tSSRC: {} ({:x})\n", self.header.ssrc, self.header.ssrc).as_str();
        out += format!("\tPayload Length: {}\n", self.payload.len()).as_str();

        write!(f, "{}", out)
    }
}

impl Packet {
    pub fn new() -> Self {
        Packet::default()
    }

    // MarshalSize returns the size of the packet once marshaled.
    pub fn marshal_size(&self) -> usize {
        self.header.marshal_size() + self.payload.len()
    }

    // Unmarshal parses the passed byte slice and stores the result in the Header this method is called upon
    pub fn unmarshal(&self, reader: &mut [u8]) -> Result<(), Error> {
        todo!()
        // let header = Header::unmarshal(reader)?;

        // let mut payload = vec![];
        // reader.read_to_end(&mut payload)?;

        // Ok(Packet { header, payload })
    }

    // Marshal serializes the header and writes to the buffer.
    pub fn marshal(&self, mut buf: &mut [u8]) -> Result<Vec<u8>, Error> {
        todo!()
        // self.header.marshal(buf)?;
        // writer.write_all(&self.payload)?;

        Ok(writer.flush()?)
    }
}
