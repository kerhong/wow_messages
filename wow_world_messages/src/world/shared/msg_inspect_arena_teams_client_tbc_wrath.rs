use std::convert::{TryFrom, TryInto};
use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/_need_sorting/msg_inspect_arena_teams.wowm:14`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/msg_inspect_arena_teams.wowm#L14):
/// ```text
/// cmsg MSG_INSPECT_ARENA_TEAMS_Client = 0x0377 {
///     Guid player;
/// }
/// ```
pub struct MSG_INSPECT_ARENA_TEAMS_Client {
    pub player: Guid,
}

impl crate::Message for MSG_INSPECT_ARENA_TEAMS_Client {
    const OPCODE: u32 = 0x0377;

    fn size_without_header(&self) -> u32 {
        8
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // player: Guid
        w.write_all(&self.player.guid().to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 8 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0377, size: body_size as u32 });
        }

        // player: Guid
        let player = Guid::read(r)?;

        Ok(Self {
            player,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::world::tbc::ClientMessage for MSG_INSPECT_ARENA_TEAMS_Client {}

#[cfg(feature = "wrath")]
impl crate::world::wrath::ClientMessage for MSG_INSPECT_ARENA_TEAMS_Client {}
