use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/_need_sorting/cmsg_search_lfg_join.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/cmsg_search_lfg_join.wowm#L1):
/// ```text
/// cmsg CMSG_SEARCH_LFG_JOIN = 0x035E {
///     u32 dungeon_id;
/// }
/// ```
pub struct CMSG_SEARCH_LFG_JOIN {
    pub dungeon_id: u32,
}

impl crate::Message for CMSG_SEARCH_LFG_JOIN {
    const OPCODE: u32 = 0x035e;

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // dungeon_id: u32
        w.write_all(&self.dungeon_id.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x035E, size: body_size as u32 });
        }

        // dungeon_id: u32
        let dungeon_id = crate::util::read_u32_le(r)?;

        Ok(Self {
            dungeon_id,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::world::wrath::ClientMessage for CMSG_SEARCH_LFG_JOIN {}
