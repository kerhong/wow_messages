use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::wrath::MovementInfo;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/_need_sorting/msg_move_set_flight_speed.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/msg_move_set_flight_speed.wowm#L1):
/// ```text
/// smsg MSG_MOVE_SET_FLIGHT_SPEED_Server = 0x037E {
///     PackedGuid player;
///     MovementInfo info;
///     f32 new_speed;
/// }
/// ```
pub struct MSG_MOVE_SET_FLIGHT_SPEED_Server {
    pub player: Guid,
    pub info: MovementInfo,
    pub new_speed: f32,
}

impl crate::Message for MSG_MOVE_SET_FLIGHT_SPEED_Server {
    const OPCODE: u32 = 0x037e;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // player: PackedGuid
        self.player.write_packed_guid_into_vec(w);

        // info: MovementInfo
        self.info.write_into_vec(w)?;

        // new_speed: f32
        w.write_all(&self.new_speed.to_le_bytes())?;

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(36..=97).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x037E, size: body_size as u32 });
        }

        // player: PackedGuid
        let player = Guid::read_packed(r)?;

        // info: MovementInfo
        let info = MovementInfo::read(r)?;

        // new_speed: f32
        let new_speed = crate::util::read_f32_le(r)?;
        Ok(Self {
            player,
            info,
            new_speed,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::world::wrath::ServerMessage for MSG_MOVE_SET_FLIGHT_SPEED_Server {}

impl MSG_MOVE_SET_FLIGHT_SPEED_Server {
    pub(crate) fn size(&self) -> usize {
        self.player.size() // player: Guid
        + self.info.size() // info: MovementInfo
        + 4 // new_speed: f32
    }
}
