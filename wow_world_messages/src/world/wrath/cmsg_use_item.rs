use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::wrath::MovementInfo;
use crate::world::wrath::SpellCastTargets;
use crate::world::wrath::ClientCastFlags;
use crate::world::wrath::ClientMovementData;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/cmsg_use_item.wowm:37`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/cmsg_use_item.wowm#L37):
/// ```text
/// cmsg CMSG_USE_ITEM = 0x00AB {
///     u8 bag_index;
///     u8 bag_slot;
///     u8 spell_index;
///     u8 cast_count;
///     u32 spell;
///     Guid item;
///     u32 glyph_index;
///     ClientCastFlags cast_flags;
///     if (cast_flags == EXTRA) {
///         f32 elevation;
///         f32 speed;
///         ClientMovementData movement_data;
///         if (movement_data == PRESENT) {
///             u32 opcode;
///             PackedGuid guid;
///             MovementInfo info;
///         }
///     }
///     SpellCastTargets targets;
/// }
/// ```
pub struct CMSG_USE_ITEM {
    pub bag_index: u8,
    pub bag_slot: u8,
    pub spell_index: u8,
    /// mangosone: next cast if exists (single or not)
    ///
    pub cast_count: u8,
    pub spell: u32,
    pub item: Guid,
    pub glyph_index: u32,
    pub cast_flags: CMSG_USE_ITEM_ClientCastFlags,
    pub targets: SpellCastTargets,
}

impl crate::Message for CMSG_USE_ITEM {
    const OPCODE: u32 = 0x00ab;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // bag_index: u8
        w.write_all(&self.bag_index.to_le_bytes())?;

        // bag_slot: u8
        w.write_all(&self.bag_slot.to_le_bytes())?;

        // spell_index: u8
        w.write_all(&self.spell_index.to_le_bytes())?;

        // cast_count: u8
        w.write_all(&self.cast_count.to_le_bytes())?;

        // spell: u32
        w.write_all(&self.spell.to_le_bytes())?;

        // item: Guid
        w.write_all(&self.item.guid().to_le_bytes())?;

        // glyph_index: u32
        w.write_all(&self.glyph_index.to_le_bytes())?;

        // cast_flags: ClientCastFlags
        w.write_all(&(self.cast_flags.as_int() as u8).to_le_bytes())?;

        match &self.cast_flags {
            CMSG_USE_ITEM_ClientCastFlags::None => {}
            CMSG_USE_ITEM_ClientCastFlags::Extra {
                elevation,
                movement_data,
                speed,
            } => {
                // elevation: f32
                w.write_all(&elevation.to_le_bytes())?;

                // speed: f32
                w.write_all(&speed.to_le_bytes())?;

                // movement_data: ClientMovementData
                w.write_all(&(movement_data.as_int() as u8).to_le_bytes())?;

                match &movement_data {
                    CMSG_USE_ITEM_ClientMovementData::NotPresent => {}
                    CMSG_USE_ITEM_ClientMovementData::Present {
                        guid,
                        info,
                        opcode,
                    } => {
                        // opcode: u32
                        w.write_all(&opcode.to_le_bytes())?;

                        // guid: PackedGuid
                        guid.write_packed_guid_into_vec(w);

                        // info: MovementInfo
                        info.write_into_vec(w)?;

                    }
                }

            }
        }

        // targets: SpellCastTargets
        self.targets.write_into_vec(w)?;

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(25..=429).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x00AB, size: body_size as u32 });
        }

        // bag_index: u8
        let bag_index = crate::util::read_u8_le(r)?;

        // bag_slot: u8
        let bag_slot = crate::util::read_u8_le(r)?;

        // spell_index: u8
        let spell_index = crate::util::read_u8_le(r)?;

        // cast_count: u8
        let cast_count = crate::util::read_u8_le(r)?;

        // spell: u32
        let spell = crate::util::read_u32_le(r)?;

        // item: Guid
        let item = Guid::read(r)?;

        // glyph_index: u32
        let glyph_index = crate::util::read_u32_le(r)?;

        // cast_flags: ClientCastFlags
        let cast_flags: ClientCastFlags = crate::util::read_u8_le(r)?.try_into()?;

        let cast_flags_if = match cast_flags {
            ClientCastFlags::None => CMSG_USE_ITEM_ClientCastFlags::None,
            ClientCastFlags::Extra => {
                // elevation: f32
                let elevation = crate::util::read_f32_le(r)?;
                // speed: f32
                let speed = crate::util::read_f32_le(r)?;
                // movement_data: ClientMovementData
                let movement_data: ClientMovementData = crate::util::read_u8_le(r)?.try_into()?;

                let movement_data_if = match movement_data {
                    ClientMovementData::NotPresent => CMSG_USE_ITEM_ClientMovementData::NotPresent,
                    ClientMovementData::Present => {
                        // opcode: u32
                        let opcode = crate::util::read_u32_le(r)?;

                        // guid: PackedGuid
                        let guid = Guid::read_packed(r)?;

                        // info: MovementInfo
                        let info = MovementInfo::read(r)?;

                        CMSG_USE_ITEM_ClientMovementData::Present {
                            guid,
                            info,
                            opcode,
                        }
                    }
                };

                CMSG_USE_ITEM_ClientCastFlags::Extra {
                    elevation,
                    movement_data: movement_data_if,
                    speed,
                }
            }
        };

        // targets: SpellCastTargets
        let targets = SpellCastTargets::read(r)?;

        Ok(Self {
            bag_index,
            bag_slot,
            spell_index,
            cast_count,
            spell,
            item,
            glyph_index,
            cast_flags: cast_flags_if,
            targets,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::world::wrath::ClientMessage for CMSG_USE_ITEM {}

impl CMSG_USE_ITEM {
    pub(crate) fn size(&self) -> usize {
        1 // bag_index: u8
        + 1 // bag_slot: u8
        + 1 // spell_index: u8
        + 1 // cast_count: u8
        + 4 // spell: u32
        + 8 // item: Guid
        + 4 // glyph_index: u32
        + self.cast_flags.size() // cast_flags: CMSG_USE_ITEM_ClientCastFlags
        + self.targets.size() // targets: SpellCastTargets
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CMSG_USE_ITEM_ClientMovementData {
    NotPresent,
    Present {
        guid: Guid,
        info: MovementInfo,
        opcode: u32,
    },
}

impl Default for CMSG_USE_ITEM_ClientMovementData {
    fn default() -> Self {
        // First enumerator without any fields
        Self::NotPresent
    }
}

impl CMSG_USE_ITEM_ClientMovementData {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::NotPresent => 0,
            Self::Present { .. } => 1,
        }
    }

}

impl CMSG_USE_ITEM_ClientMovementData {
    pub(crate) fn size(&self) -> usize {
        match self {
            Self::NotPresent => {
                1
            }
            Self::Present {
                guid,
                info,
                opcode,
            } => {
                1
                + guid.size() // guid: Guid
                + info.size() // info: MovementInfo
                + 4 // opcode: u32
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CMSG_USE_ITEM_ClientCastFlags {
    None,
    Extra {
        elevation: f32,
        movement_data: CMSG_USE_ITEM_ClientMovementData,
        speed: f32,
    },
}

impl Default for CMSG_USE_ITEM_ClientCastFlags {
    fn default() -> Self {
        // First enumerator without any fields
        Self::None
    }
}

impl CMSG_USE_ITEM_ClientCastFlags {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::None => 0,
            Self::Extra { .. } => 2,
        }
    }

}

impl CMSG_USE_ITEM_ClientCastFlags {
    pub(crate) fn size(&self) -> usize {
        match self {
            Self::None => {
                1
            }
            Self::Extra {
                elevation,
                movement_data,
                speed,
            } => {
                1
                + 4 // elevation: f32
                + movement_data.size() // movement_data: CMSG_USE_ITEM_ClientMovementData
                + 4 // speed: f32
            }
        }
    }
}
