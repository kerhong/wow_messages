use std::convert::{TryFrom, TryInto};
use crate::world::wrath::MovementInfo;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/_need_sorting/msg_move_unroot.wowm:8`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/msg_move_unroot.wowm#L8):
/// ```text
/// smsg MSG_MOVE_UNROOT_Server = 0x00ED {
///     MovementInfo info;
/// }
/// ```
pub struct MSG_MOVE_UNROOT_Server {
    pub info: MovementInfo,
}

impl crate::Message for MSG_MOVE_UNROOT_Server {
    const OPCODE: u32 = 0x00ed;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // info: MovementInfo
        self.info.write_into_vec(w)?;

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(30..=84).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x00ED, size: body_size as u32 });
        }

        // info: MovementInfo
        let info = MovementInfo::read(r)?;

        Ok(Self {
            info,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::world::wrath::ServerMessage for MSG_MOVE_UNROOT_Server {}

impl MSG_MOVE_UNROOT_Server {
    pub(crate) fn size(&self) -> usize {
        self.info.size() // info: MovementInfo
    }
}
