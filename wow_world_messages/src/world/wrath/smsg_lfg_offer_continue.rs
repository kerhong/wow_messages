use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/_need_sorting/smsg_lfg_offer_continue.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/smsg_lfg_offer_continue.wowm#L1):
/// ```text
/// smsg SMSG_LFG_OFFER_CONTINUE = 0x0293 {
///     u32 dungeon_entry;
/// }
/// ```
pub struct SMSG_LFG_OFFER_CONTINUE {
    pub dungeon_entry: u32,
}

impl crate::Message for SMSG_LFG_OFFER_CONTINUE {
    const OPCODE: u32 = 0x0293;

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // dungeon_entry: u32
        w.write_all(&self.dungeon_entry.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0293, size: body_size as u32 });
        }

        // dungeon_entry: u32
        let dungeon_entry = crate::util::read_u32_le(r)?;

        Ok(Self {
            dungeon_entry,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::world::wrath::ServerMessage for SMSG_LFG_OFFER_CONTINUE {}
