use std::convert::{TryFrom, TryInto};
use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/_need_sorting/smsg_modify_cooldown.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/smsg_modify_cooldown.wowm#L1):
/// ```text
/// smsg SMSG_MODIFY_COOLDOWN = 0x0491 {
///     u32 spell;
///     Guid player;
///     i32 cooldown_in_milliseconds;
/// }
/// ```
pub struct SMSG_MODIFY_COOLDOWN {
    pub spell: u32,
    pub player: Guid,
    pub cooldown_in_milliseconds: i32,
}

impl crate::Message for SMSG_MODIFY_COOLDOWN {
    const OPCODE: u32 = 0x0491;

    fn size_without_header(&self) -> u32 {
        16
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // spell: u32
        w.write_all(&self.spell.to_le_bytes())?;

        // player: Guid
        w.write_all(&self.player.guid().to_le_bytes())?;

        // cooldown_in_milliseconds: i32
        w.write_all(&self.cooldown_in_milliseconds.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 16 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0491, size: body_size as u32 });
        }

        // spell: u32
        let spell = crate::util::read_u32_le(r)?;

        // player: Guid
        let player = Guid::read(r)?;

        // cooldown_in_milliseconds: i32
        let cooldown_in_milliseconds = crate::util::read_i32_le(r)?;

        Ok(Self {
            spell,
            player,
            cooldown_in_milliseconds,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::world::wrath::ServerMessage for SMSG_MODIFY_COOLDOWN {}
