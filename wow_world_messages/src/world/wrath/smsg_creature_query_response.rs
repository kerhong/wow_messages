use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/smsg_creature_query_response.wowm:74`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/smsg_creature_query_response.wowm#L74):
/// ```text
/// smsg SMSG_CREATURE_QUERY_RESPONSE = 0x0061 {
///     u32 creature_entry;
///     optional found {
///         CString name1;
///         CString name2;
///         CString name3;
///         CString name4;
///         CString sub_name;
///         CString description;
///         u32 type_flags;
///         u32 creature_type;
///         u32 creature_family;
///         u32 creature_rank;
///         u32 kill_credit1;
///         u32 kill_credit2;
///         u32[4] display_ids;
///         f32 health_multiplier;
///         f32 mana_multiplier;
///         u8 racial_leader;
///         u32[6] quest_items;
///         u32 movement_id;
///     }
/// }
/// ```
pub struct SMSG_CREATURE_QUERY_RESPONSE {
    pub creature_entry: u32,
    pub found: Option<SMSG_CREATURE_QUERY_RESPONSE_found>,
}

impl crate::Message for SMSG_CREATURE_QUERY_RESPONSE {
    const OPCODE: u32 = 0x0061;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // creature_entry: u32
        w.write_all(&self.creature_entry.to_le_bytes())?;

        // optional found
        if let Some(v) = &self.found {
            // name1: CString
            // TODO: Guard against strings that are already null-terminated
            assert_ne!(v.name1.as_bytes().iter().rev().next(), Some(&0_u8), "String `name1` must not be null-terminated.");
            w.write_all(v.name1.as_bytes())?;
            // Null terminator
            w.write_all(&[0])?;

            // name2: CString
            // TODO: Guard against strings that are already null-terminated
            assert_ne!(v.name2.as_bytes().iter().rev().next(), Some(&0_u8), "String `name2` must not be null-terminated.");
            w.write_all(v.name2.as_bytes())?;
            // Null terminator
            w.write_all(&[0])?;

            // name3: CString
            // TODO: Guard against strings that are already null-terminated
            assert_ne!(v.name3.as_bytes().iter().rev().next(), Some(&0_u8), "String `name3` must not be null-terminated.");
            w.write_all(v.name3.as_bytes())?;
            // Null terminator
            w.write_all(&[0])?;

            // name4: CString
            // TODO: Guard against strings that are already null-terminated
            assert_ne!(v.name4.as_bytes().iter().rev().next(), Some(&0_u8), "String `name4` must not be null-terminated.");
            w.write_all(v.name4.as_bytes())?;
            // Null terminator
            w.write_all(&[0])?;

            // sub_name: CString
            // TODO: Guard against strings that are already null-terminated
            assert_ne!(v.sub_name.as_bytes().iter().rev().next(), Some(&0_u8), "String `sub_name` must not be null-terminated.");
            w.write_all(v.sub_name.as_bytes())?;
            // Null terminator
            w.write_all(&[0])?;

            // description: CString
            // TODO: Guard against strings that are already null-terminated
            assert_ne!(v.description.as_bytes().iter().rev().next(), Some(&0_u8), "String `description` must not be null-terminated.");
            w.write_all(v.description.as_bytes())?;
            // Null terminator
            w.write_all(&[0])?;

            // type_flags: u32
            w.write_all(&v.type_flags.to_le_bytes())?;

            // creature_type: u32
            w.write_all(&v.creature_type.to_le_bytes())?;

            // creature_family: u32
            w.write_all(&v.creature_family.to_le_bytes())?;

            // creature_rank: u32
            w.write_all(&v.creature_rank.to_le_bytes())?;

            // kill_credit1: u32
            w.write_all(&v.kill_credit1.to_le_bytes())?;

            // kill_credit2: u32
            w.write_all(&v.kill_credit2.to_le_bytes())?;

            // display_ids: u32[4]
            for i in v.display_ids.iter() {
                w.write_all(&i.to_le_bytes())?;
            }

            // health_multiplier: f32
            w.write_all(&v.health_multiplier.to_le_bytes())?;

            // mana_multiplier: f32
            w.write_all(&v.mana_multiplier.to_le_bytes())?;

            // racial_leader: u8
            w.write_all(&v.racial_leader.to_le_bytes())?;

            // quest_items: u32[6]
            for i in v.quest_items.iter() {
                w.write_all(&i.to_le_bytes())?;
            }

            // movement_id: u32
            w.write_all(&v.movement_id.to_le_bytes())?;

        }

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(4..=1617).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0061, size: body_size as u32 });
        }

        // creature_entry: u32
        let creature_entry = crate::util::read_u32_le(r)?;

        // optional found
        let current_size = {
            4 // creature_entry: u32
        };
        let found = if current_size < body_size as usize {
            // name1: CString
            let name1 = crate::util::read_c_string_to_vec(r)?;
            let name1 = String::from_utf8(name1)?;

            // name2: CString
            let name2 = crate::util::read_c_string_to_vec(r)?;
            let name2 = String::from_utf8(name2)?;

            // name3: CString
            let name3 = crate::util::read_c_string_to_vec(r)?;
            let name3 = String::from_utf8(name3)?;

            // name4: CString
            let name4 = crate::util::read_c_string_to_vec(r)?;
            let name4 = String::from_utf8(name4)?;

            // sub_name: CString
            let sub_name = crate::util::read_c_string_to_vec(r)?;
            let sub_name = String::from_utf8(sub_name)?;

            // description: CString
            let description = crate::util::read_c_string_to_vec(r)?;
            let description = String::from_utf8(description)?;

            // type_flags: u32
            let type_flags = crate::util::read_u32_le(r)?;

            // creature_type: u32
            let creature_type = crate::util::read_u32_le(r)?;

            // creature_family: u32
            let creature_family = crate::util::read_u32_le(r)?;

            // creature_rank: u32
            let creature_rank = crate::util::read_u32_le(r)?;

            // kill_credit1: u32
            let kill_credit1 = crate::util::read_u32_le(r)?;

            // kill_credit2: u32
            let kill_credit2 = crate::util::read_u32_le(r)?;

            // display_ids: u32[4]
            let mut display_ids = [u32::default(); 4];
            for i in display_ids.iter_mut() {
                *i = crate::util::read_u32_le(r)?;
            }

            // health_multiplier: f32
            let health_multiplier = crate::util::read_f32_le(r)?;
            // mana_multiplier: f32
            let mana_multiplier = crate::util::read_f32_le(r)?;
            // racial_leader: u8
            let racial_leader = crate::util::read_u8_le(r)?;

            // quest_items: u32[6]
            let mut quest_items = [u32::default(); 6];
            for i in quest_items.iter_mut() {
                *i = crate::util::read_u32_le(r)?;
            }

            // movement_id: u32
            let movement_id = crate::util::read_u32_le(r)?;

            Some(SMSG_CREATURE_QUERY_RESPONSE_found {
                name1,
                name2,
                name3,
                name4,
                sub_name,
                description,
                type_flags,
                creature_type,
                creature_family,
                creature_rank,
                kill_credit1,
                kill_credit2,
                display_ids,
                health_multiplier,
                mana_multiplier,
                racial_leader,
                quest_items,
                movement_id,
            })
        } else {
            None
        };

        Ok(Self {
            creature_entry,
            found,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::world::wrath::ServerMessage for SMSG_CREATURE_QUERY_RESPONSE {}

impl SMSG_CREATURE_QUERY_RESPONSE {
    pub(crate) fn size(&self) -> usize {
        4 // creature_entry: u32
        + if let Some(found) = &self.found {
            found.name1.len() + 1 // name1: CString
            + found.name2.len() + 1 // name2: CString
            + found.name3.len() + 1 // name3: CString
            + found.name4.len() + 1 // name4: CString
            + found.sub_name.len() + 1 // sub_name: CString
            + found.description.len() + 1 // description: CString
            + 4 // type_flags: u32
            + 4 // creature_type: u32
            + 4 // creature_family: u32
            + 4 // creature_rank: u32
            + 4 // kill_credit1: u32
            + 4 // kill_credit2: u32
            + 4 * core::mem::size_of::<u32>() // display_ids: u32[4]
            + 4 // health_multiplier: f32
            + 4 // mana_multiplier: f32
            + 1 // racial_leader: u8
            + 6 * core::mem::size_of::<u32>() // quest_items: u32[6]
            + 4 // movement_id: u32
        } else {
            0
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct SMSG_CREATURE_QUERY_RESPONSE_found {
    pub name1: String,
    pub name2: String,
    pub name3: String,
    pub name4: String,
    pub sub_name: String,
    pub description: String,
    pub type_flags: u32,
    pub creature_type: u32,
    pub creature_family: u32,
    pub creature_rank: u32,
    pub kill_credit1: u32,
    pub kill_credit2: u32,
    pub display_ids: [u32; 4],
    pub health_multiplier: f32,
    pub mana_multiplier: f32,
    pub racial_leader: u8,
    pub quest_items: [u32; 6],
    pub movement_id: u32,
}

impl SMSG_CREATURE_QUERY_RESPONSE_found {
    pub(crate) fn size(&self) -> usize {
        self.name1.len() + 1 // name1: CString
        + self.name2.len() + 1 // name2: CString
        + self.name3.len() + 1 // name3: CString
        + self.name4.len() + 1 // name4: CString
        + self.sub_name.len() + 1 // sub_name: CString
        + self.description.len() + 1 // description: CString
        + 4 // type_flags: u32
        + 4 // creature_type: u32
        + 4 // creature_family: u32
        + 4 // creature_rank: u32
        + 4 // kill_credit1: u32
        + 4 // kill_credit2: u32
        + 4 * core::mem::size_of::<u32>() // display_ids: u32[4]
        + 4 // health_multiplier: f32
        + 4 // mana_multiplier: f32
        + 1 // racial_leader: u8
        + 6 * core::mem::size_of::<u32>() // quest_items: u32[6]
        + 4 // movement_id: u32
    }

}
