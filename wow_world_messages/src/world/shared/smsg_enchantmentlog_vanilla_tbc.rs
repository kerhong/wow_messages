use std::convert::{TryFrom, TryInto};
use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// cmangos and vmangos/mangoszero disagree about packed and extra u8
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/smsg_enchantmentlog.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/smsg_enchantmentlog.wowm#L1):
/// ```text
/// smsg SMSG_ENCHANTMENTLOG = 0x01D7 {
///     Guid target_guid;
///     Guid caster_guid;
///     u32 item;
///     u32 spell;
///     Bool show_affiliation;
/// }
/// ```
pub struct SMSG_ENCHANTMENTLOG {
    pub target_guid: Guid,
    /// vmangos: message says enchant has faded if empty
    ///
    pub caster_guid: Guid,
    pub item: u32,
    pub spell: u32,
    /// vmangos: Only used if `caster_guid` is not 0.
    ///
    pub show_affiliation: bool,
}

impl crate::Message for SMSG_ENCHANTMENTLOG {
    const OPCODE: u32 = 0x01d7;

    fn size_without_header(&self) -> u32 {
        25
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // target_guid: Guid
        w.write_all(&self.target_guid.guid().to_le_bytes())?;

        // caster_guid: Guid
        w.write_all(&self.caster_guid.guid().to_le_bytes())?;

        // item: u32
        w.write_all(&self.item.to_le_bytes())?;

        // spell: u32
        w.write_all(&self.spell.to_le_bytes())?;

        // show_affiliation: Bool
        w.write_all(u8::from(self.show_affiliation).to_le_bytes().as_slice())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 25 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x01D7, size: body_size as u32 });
        }

        // target_guid: Guid
        let target_guid = Guid::read(r)?;

        // caster_guid: Guid
        let caster_guid = Guid::read(r)?;

        // item: u32
        let item = crate::util::read_u32_le(r)?;

        // spell: u32
        let spell = crate::util::read_u32_le(r)?;

        // show_affiliation: Bool
        let show_affiliation = crate::util::read_u8_le(r)? != 0;
        Ok(Self {
            target_guid,
            caster_guid,
            item,
            spell,
            show_affiliation,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ServerMessage for SMSG_ENCHANTMENTLOG {}

#[cfg(feature = "tbc")]
impl crate::world::tbc::ServerMessage for SMSG_ENCHANTMENTLOG {}
