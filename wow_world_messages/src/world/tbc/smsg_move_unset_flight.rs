use std::convert::{TryFrom, TryInto};
use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/_need_sorting/smsg_move_unset_flight.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/smsg_move_unset_flight.wowm#L1):
/// ```text
/// smsg SMSG_MOVE_UNSET_FLIGHT = 0x033F {
///     Guid guid;
///     u32 counter;
/// }
/// ```
pub struct SMSG_MOVE_UNSET_FLIGHT {
    pub guid: Guid,
    pub counter: u32,
}

impl crate::Message for SMSG_MOVE_UNSET_FLIGHT {
    const OPCODE: u32 = 0x033f;

    fn size_without_header(&self) -> u32 {
        12
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // counter: u32
        w.write_all(&self.counter.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 12 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x033F, size: body_size as u32 });
        }

        // guid: Guid
        let guid = Guid::read(r)?;

        // counter: u32
        let counter = crate::util::read_u32_le(r)?;

        Ok(Self {
            guid,
            counter,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::world::tbc::ServerMessage for SMSG_MOVE_UNSET_FLIGHT {}
