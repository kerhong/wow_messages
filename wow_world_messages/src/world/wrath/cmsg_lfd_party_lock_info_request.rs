use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/_need_sorting/cmsg_lfd_party_lock_info_request.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/cmsg_lfd_party_lock_info_request.wowm#L1):
/// ```text
/// cmsg CMSG_LFD_PARTY_LOCK_INFO_REQUEST = 0x0371 {
/// }
/// ```
pub struct CMSG_LFD_PARTY_LOCK_INFO_REQUEST {
}

impl crate::Message for CMSG_LFD_PARTY_LOCK_INFO_REQUEST {
    const OPCODE: u32 = 0x0371;

    fn size_without_header(&self) -> u32 {
        0
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 0 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0371, size: body_size as u32 });
        }

        Ok(Self {
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::world::wrath::ClientMessage for CMSG_LFD_PARTY_LOCK_INFO_REQUEST {}
