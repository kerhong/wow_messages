use std::convert::{TryFrom, TryInto};
use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/_need_sorting/smsg_lfg_role_check_update.wowm:14`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/smsg_lfg_role_check_update.wowm#L14):
/// ```text
/// struct LfgRole {
///     Guid guid;
///     Bool ready;
///     u32 roles;
///     u8 level;
/// }
/// ```
pub struct LfgRole {
    pub guid: Guid,
    pub ready: bool,
    pub roles: u32,
    pub level: u8,
}

impl LfgRole {
    pub(crate) fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // ready: Bool
        w.write_all(u8::from(self.ready).to_le_bytes().as_slice())?;

        // roles: u32
        w.write_all(&self.roles.to_le_bytes())?;

        // level: u8
        w.write_all(&self.level.to_le_bytes())?;

        Ok(())
    }
}

impl LfgRole {
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, crate::errors::ParseError> {
        // guid: Guid
        let guid = Guid::read(r)?;

        // ready: Bool
        let ready = crate::util::read_u8_le(r)? != 0;
        // roles: u32
        let roles = crate::util::read_u32_le(r)?;

        // level: u8
        let level = crate::util::read_u8_le(r)?;

        Ok(Self {
            guid,
            ready,
            roles,
            level,
        })
    }

}
