use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::wrath::MovementInfo;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/_need_sorting/cmsg_move_set_can_transition_between_swim_and_fly_ack.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/cmsg_move_set_can_transition_between_swim_and_fly_ack.wowm#L1):
/// ```text
/// cmsg CMSG_MOVE_SET_CAN_TRANSITION_BETWEEN_SWIM_AND_FLY_ACK = 0x0340 {
///     PackedGuid guid;
///     u32 unknown1;
///     MovementInfo info;
///     u32 unknown2;
/// }
/// ```
pub struct CMSG_MOVE_SET_CAN_TRANSITION_BETWEEN_SWIM_AND_FLY_ACK {
    pub guid: Guid,
    pub unknown1: u32,
    pub info: MovementInfo,
    pub unknown2: u32,
}

impl crate::Message for CMSG_MOVE_SET_CAN_TRANSITION_BETWEEN_SWIM_AND_FLY_ACK {
    const OPCODE: u32 = 0x0340;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // guid: PackedGuid
        self.guid.write_packed_guid_into_vec(w);

        // unknown1: u32
        w.write_all(&self.unknown1.to_le_bytes())?;

        // info: MovementInfo
        self.info.write_into_vec(w)?;

        // unknown2: u32
        w.write_all(&self.unknown2.to_le_bytes())?;

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(40..=101).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0340, size: body_size as u32 });
        }

        // guid: PackedGuid
        let guid = Guid::read_packed(r)?;

        // unknown1: u32
        let unknown1 = crate::util::read_u32_le(r)?;

        // info: MovementInfo
        let info = MovementInfo::read(r)?;

        // unknown2: u32
        let unknown2 = crate::util::read_u32_le(r)?;

        Ok(Self {
            guid,
            unknown1,
            info,
            unknown2,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::world::wrath::ClientMessage for CMSG_MOVE_SET_CAN_TRANSITION_BETWEEN_SWIM_AND_FLY_ACK {}

impl CMSG_MOVE_SET_CAN_TRANSITION_BETWEEN_SWIM_AND_FLY_ACK {
    pub(crate) fn size(&self) -> usize {
        self.guid.size() // guid: Guid
        + 4 // unknown1: u32
        + self.info.size() // info: MovementInfo
        + 4 // unknown2: u32
    }
}
