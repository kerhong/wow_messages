use std::convert::{TryFrom, TryInto};
use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/resurrect/smsg_spirit_healer_confirm.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/resurrect/smsg_spirit_healer_confirm.wowm#L3):
/// ```text
/// smsg SMSG_SPIRIT_HEALER_CONFIRM = 0x0222 {
///     Guid guid;
/// }
/// ```
pub struct SMSG_SPIRIT_HEALER_CONFIRM {
    pub guid: Guid,
}

impl crate::Message for SMSG_SPIRIT_HEALER_CONFIRM {
    const OPCODE: u32 = 0x0222;

    fn size_without_header(&self) -> u32 {
        8
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 8 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0222, size: body_size as u32 });
        }

        // guid: Guid
        let guid = Guid::read(r)?;

        Ok(Self {
            guid,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ServerMessage for SMSG_SPIRIT_HEALER_CONFIRM {}

#[cfg(feature = "tbc")]
impl crate::world::tbc::ServerMessage for SMSG_SPIRIT_HEALER_CONFIRM {}

#[cfg(feature = "wrath")]
impl crate::world::wrath::ServerMessage for SMSG_SPIRIT_HEALER_CONFIRM {}
