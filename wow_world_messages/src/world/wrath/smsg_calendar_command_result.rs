use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/_need_sorting/smsg_calendar_command_result.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/smsg_calendar_command_result.wowm#L1):
/// ```text
/// smsg SMSG_CALENDAR_COMMAND_RESULT = 0x043D {
///     u32 unknown1;
///     u8 unknown2;
///     CString name;
///     u32 result;
/// }
/// ```
pub struct SMSG_CALENDAR_COMMAND_RESULT {
    /// All emus set to 0.
    ///
    pub unknown1: u32,
    /// All emus set to 0.
    ///
    pub unknown2: u8,
    pub name: String,
    pub result: u32,
}

impl crate::Message for SMSG_CALENDAR_COMMAND_RESULT {
    const OPCODE: u32 = 0x043d;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // unknown1: u32
        w.write_all(&self.unknown1.to_le_bytes())?;

        // unknown2: u8
        w.write_all(&self.unknown2.to_le_bytes())?;

        // name: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.name.as_bytes().iter().rev().next(), Some(&0_u8), "String `name` must not be null-terminated.");
        w.write_all(self.name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // result: u32
        w.write_all(&self.result.to_le_bytes())?;

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(10..=265).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x043D, size: body_size as u32 });
        }

        // unknown1: u32
        let unknown1 = crate::util::read_u32_le(r)?;

        // unknown2: u8
        let unknown2 = crate::util::read_u8_le(r)?;

        // name: CString
        let name = crate::util::read_c_string_to_vec(r)?;
        let name = String::from_utf8(name)?;

        // result: u32
        let result = crate::util::read_u32_le(r)?;

        Ok(Self {
            unknown1,
            unknown2,
            name,
            result,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::world::wrath::ServerMessage for SMSG_CALENDAR_COMMAND_RESULT {}

impl SMSG_CALENDAR_COMMAND_RESULT {
    pub(crate) fn size(&self) -> usize {
        4 // unknown1: u32
        + 1 // unknown2: u8
        + self.name.len() + 1 // name: CString
        + 4 // result: u32
    }
}
