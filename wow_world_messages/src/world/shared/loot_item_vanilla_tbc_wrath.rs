use std::convert::{TryFrom, TryInto};
use crate::world::shared::loot_slot_type_vanilla_tbc_wrath::LootSlotType;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/loot/smsg_loot_response.wowm:42`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/loot/smsg_loot_response.wowm#L42):
/// ```text
/// struct LootItem {
///     u8 index;
///     u32 item;
///     LootSlotType ty;
/// }
/// ```
pub struct LootItem {
    pub index: u8,
    pub item: u32,
    pub ty: LootSlotType,
}

impl LootItem {
    pub(crate) fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // index: u8
        w.write_all(&self.index.to_le_bytes())?;

        // item: u32
        w.write_all(&self.item.to_le_bytes())?;

        // ty: LootSlotType
        w.write_all(&(self.ty.as_int() as u8).to_le_bytes())?;

        Ok(())
    }
}

impl LootItem {
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, crate::errors::ParseError> {
        // index: u8
        let index = crate::util::read_u8_le(r)?;

        // item: u32
        let item = crate::util::read_u32_le(r)?;

        // ty: LootSlotType
        let ty: LootSlotType = crate::util::read_u8_le(r)?.try_into()?;

        Ok(Self {
            index,
            item,
            ty,
        })
    }

}
