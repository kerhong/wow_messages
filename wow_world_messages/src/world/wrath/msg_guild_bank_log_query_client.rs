use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/_need_sorting/msg_guild_bank_log_query.wowm:16`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/msg_guild_bank_log_query.wowm#L16):
/// ```text
/// cmsg MSG_GUILD_BANK_LOG_QUERY_Client = 0x03EE {
///     u8 slot;
/// }
/// ```
pub struct MSG_GUILD_BANK_LOG_QUERY_Client {
    pub slot: u8,
}

impl crate::Message for MSG_GUILD_BANK_LOG_QUERY_Client {
    const OPCODE: u32 = 0x03ee;

    fn size_without_header(&self) -> u32 {
        1
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // slot: u8
        w.write_all(&self.slot.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 1 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x03EE, size: body_size as u32 });
        }

        // slot: u8
        let slot = crate::util::read_u8_le(r)?;

        Ok(Self {
            slot,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::world::wrath::ClientMessage for MSG_GUILD_BANK_LOG_QUERY_Client {}
