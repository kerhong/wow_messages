use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/_need_sorting/cmsg_set_title.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/cmsg_set_title.wowm#L1):
/// ```text
/// cmsg CMSG_SET_TITLE = 0x0374 {
///     u32 title;
/// }
/// ```
pub struct CMSG_SET_TITLE {
    pub title: u32,
}

impl crate::Message for CMSG_SET_TITLE {
    const OPCODE: u32 = 0x0374;

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // title: u32
        w.write_all(&self.title.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0374, size: body_size as u32 });
        }

        // title: u32
        let title = crate::util::read_u32_le(r)?;

        Ok(Self {
            title,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::world::tbc::ClientMessage for CMSG_SET_TITLE {}

#[cfg(feature = "wrath")]
impl crate::world::wrath::ClientMessage for CMSG_SET_TITLE {}
