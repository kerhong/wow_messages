use std::convert::{TryFrom, TryInto};
use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/_need_sorting/cmsg_player_vehicle_enter.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/cmsg_player_vehicle_enter.wowm#L1):
/// ```text
/// cmsg CMSG_PLAYER_VEHICLE_ENTER = 0x04A8 {
///     Guid vehicle;
/// }
/// ```
pub struct CMSG_PLAYER_VEHICLE_ENTER {
    pub vehicle: Guid,
}

impl crate::Message for CMSG_PLAYER_VEHICLE_ENTER {
    const OPCODE: u32 = 0x04a8;

    fn size_without_header(&self) -> u32 {
        8
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // vehicle: Guid
        w.write_all(&self.vehicle.guid().to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 8 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x04A8, size: body_size as u32 });
        }

        // vehicle: Guid
        let vehicle = Guid::read(r)?;

        Ok(Self {
            vehicle,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::world::wrath::ClientMessage for CMSG_PLAYER_VEHICLE_ENTER {}
