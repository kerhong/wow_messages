use std::convert::{TryFrom, TryInto};
use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/pet/cmsg_pet_rename.wowm:8`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/pet/cmsg_pet_rename.wowm#L8):
/// ```text
/// cmsg CMSG_PET_RENAME = 0x0177 {
///     Guid pet_guid;
///     CString name;
///     Bool declined;
/// }
/// ```
pub struct CMSG_PET_RENAME {
    pub pet_guid: Guid,
    pub name: String,
    pub declined: bool,
}

impl crate::Message for CMSG_PET_RENAME {
    const OPCODE: u32 = 0x0177;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // pet_guid: Guid
        w.write_all(&self.pet_guid.guid().to_le_bytes())?;

        // name: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.name.as_bytes().iter().rev().next(), Some(&0_u8), "String `name` must not be null-terminated.");
        w.write_all(self.name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // declined: Bool
        w.write_all(u8::from(self.declined).to_le_bytes().as_slice())?;

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(10..=265).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0177, size: body_size as u32 });
        }

        // pet_guid: Guid
        let pet_guid = Guid::read(r)?;

        // name: CString
        let name = crate::util::read_c_string_to_vec(r)?;
        let name = String::from_utf8(name)?;

        // declined: Bool
        let declined = crate::util::read_u8_le(r)? != 0;
        Ok(Self {
            pet_guid,
            name,
            declined,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::world::tbc::ClientMessage for CMSG_PET_RENAME {}

#[cfg(feature = "wrath")]
impl crate::world::wrath::ClientMessage for CMSG_PET_RENAME {}

impl CMSG_PET_RENAME {
    pub(crate) fn size(&self) -> usize {
        8 // pet_guid: Guid
        + self.name.len() + 1 // name: CString
        + 1 // declined: Bool
    }
}
