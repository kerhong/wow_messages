use std::convert::{TryFrom, TryInto};
use crate::world::wrath::ChatRestrictionType;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/chat/smsg_chat_restricted.wowm:28`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/chat/smsg_chat_restricted.wowm#L28):
/// ```text
/// smsg SMSG_CHAT_RESTRICTED = 0x02FD {
///     ChatRestrictionType restriction;
/// }
/// ```
pub struct SMSG_CHAT_RESTRICTED {
    pub restriction: ChatRestrictionType,
}

impl crate::Message for SMSG_CHAT_RESTRICTED {
    const OPCODE: u32 = 0x02fd;

    fn size_without_header(&self) -> u32 {
        1
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // restriction: ChatRestrictionType
        w.write_all(&(self.restriction.as_int() as u8).to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 1 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // restriction: ChatRestrictionType
        let restriction: ChatRestrictionType = crate::util::read_u8_le(r)?.try_into()?;

        Ok(Self {
            restriction,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::world::wrath::ServerMessage for SMSG_CHAT_RESTRICTED {}
