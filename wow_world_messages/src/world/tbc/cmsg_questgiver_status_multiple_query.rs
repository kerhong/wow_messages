use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/_need_sorting/cmsg_questgiver_status_multiple_query.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/cmsg_questgiver_status_multiple_query.wowm#L1):
/// ```text
/// cmsg CMSG_QUESTGIVER_STATUS_MULTIPLE_QUERY = 0x0416 {
/// }
/// ```
pub struct CMSG_QUESTGIVER_STATUS_MULTIPLE_QUERY {
}

impl crate::Message for CMSG_QUESTGIVER_STATUS_MULTIPLE_QUERY {
    const OPCODE: u32 = 0x0416;

    fn size_without_header(&self) -> u32 {
        0
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 0 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0416, size: body_size as u32 });
        }

        Ok(Self {
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::world::tbc::ClientMessage for CMSG_QUESTGIVER_STATUS_MULTIPLE_QUERY {}
