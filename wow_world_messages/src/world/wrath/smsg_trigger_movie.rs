use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/_need_sorting/smsg_trigger_movie.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/smsg_trigger_movie.wowm#L1):
/// ```text
/// smsg SMSG_TRIGGER_MOVIE = 0x0464 {
///     u32 movie_id;
/// }
/// ```
pub struct SMSG_TRIGGER_MOVIE {
    pub movie_id: u32,
}

impl crate::Message for SMSG_TRIGGER_MOVIE {
    const OPCODE: u32 = 0x0464;

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // movie_id: u32
        w.write_all(&self.movie_id.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0464, size: body_size as u32 });
        }

        // movie_id: u32
        let movie_id = crate::util::read_u32_le(r)?;

        Ok(Self {
            movie_id,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::world::wrath::ServerMessage for SMSG_TRIGGER_MOVIE {}
