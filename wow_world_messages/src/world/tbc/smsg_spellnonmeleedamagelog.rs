use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::tbc::SpellSchool;
use crate::world::tbc::HitInfo;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_spellnonmeleedamagelog.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_spellnonmeleedamagelog.wowm#L1):
/// ```text
/// smsg SMSG_SPELLNONMELEEDAMAGELOG = 0x0250 {
///     PackedGuid target;
///     PackedGuid attacker;
///     u32 spell;
///     u32 damage;
///     SpellSchool school;
///     u32 absorbed_damage;
///     u32 resisted;
///     Bool periodic_log;
///     u8 unused;
///     u32 blocked;
///     HitInfo hit_info;
///     u8 extend_flag;
/// }
/// ```
pub struct SMSG_SPELLNONMELEEDAMAGELOG {
    pub target: Guid,
    pub attacker: Guid,
    pub spell: u32,
    pub damage: u32,
    pub school: SpellSchool,
    pub absorbed_damage: u32,
    /// cmangos/mangoszero/vmangos: sent as int32
    ///
    pub resisted: u32,
    /// cmangos/mangoszero/vmangos: if 1, then client show spell name (example: %s's ranged shot hit %s for %u school or %s suffers %u school damage from %s's spell_name
    ///
    pub periodic_log: bool,
    pub unused: u8,
    pub blocked: u32,
    pub hit_info: HitInfo,
    /// cmangos has some that might be correct `https://github.com/cmangos/mangos-classic/blob/524a39412dae7946d06e4b8f319f45b615075815/src/game/Entities/Unit.cpp#L5497`.
    ///
    pub extend_flag: u8,
}

impl crate::Message for SMSG_SPELLNONMELEEDAMAGELOG {
    const OPCODE: u32 = 0x0250;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // target: PackedGuid
        self.target.write_packed_guid_into_vec(w);

        // attacker: PackedGuid
        self.attacker.write_packed_guid_into_vec(w);

        // spell: u32
        w.write_all(&self.spell.to_le_bytes())?;

        // damage: u32
        w.write_all(&self.damage.to_le_bytes())?;

        // school: SpellSchool
        w.write_all(&(self.school.as_int() as u8).to_le_bytes())?;

        // absorbed_damage: u32
        w.write_all(&self.absorbed_damage.to_le_bytes())?;

        // resisted: u32
        w.write_all(&self.resisted.to_le_bytes())?;

        // periodic_log: Bool
        w.write_all(u8::from(self.periodic_log).to_le_bytes().as_slice())?;

        // unused: u8
        w.write_all(&self.unused.to_le_bytes())?;

        // blocked: u32
        w.write_all(&self.blocked.to_le_bytes())?;

        // hit_info: HitInfo
        w.write_all(&(self.hit_info.as_int() as u32).to_le_bytes())?;

        // extend_flag: u8
        w.write_all(&self.extend_flag.to_le_bytes())?;

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(32..=46).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0250, size: body_size as u32 });
        }

        // target: PackedGuid
        let target = Guid::read_packed(r)?;

        // attacker: PackedGuid
        let attacker = Guid::read_packed(r)?;

        // spell: u32
        let spell = crate::util::read_u32_le(r)?;

        // damage: u32
        let damage = crate::util::read_u32_le(r)?;

        // school: SpellSchool
        let school: SpellSchool = crate::util::read_u8_le(r)?.try_into()?;

        // absorbed_damage: u32
        let absorbed_damage = crate::util::read_u32_le(r)?;

        // resisted: u32
        let resisted = crate::util::read_u32_le(r)?;

        // periodic_log: Bool
        let periodic_log = crate::util::read_u8_le(r)? != 0;
        // unused: u8
        let unused = crate::util::read_u8_le(r)?;

        // blocked: u32
        let blocked = crate::util::read_u32_le(r)?;

        // hit_info: HitInfo
        let hit_info = HitInfo::new(crate::util::read_u32_le(r)?);

        // extend_flag: u8
        let extend_flag = crate::util::read_u8_le(r)?;

        Ok(Self {
            target,
            attacker,
            spell,
            damage,
            school,
            absorbed_damage,
            resisted,
            periodic_log,
            unused,
            blocked,
            hit_info,
            extend_flag,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::world::tbc::ServerMessage for SMSG_SPELLNONMELEEDAMAGELOG {}

impl SMSG_SPELLNONMELEEDAMAGELOG {
    pub(crate) fn size(&self) -> usize {
        self.target.size() // target: Guid
        + self.attacker.size() // attacker: Guid
        + 4 // spell: u32
        + 4 // damage: u32
        + 1 // school: SpellSchool
        + 4 // absorbed_damage: u32
        + 4 // resisted: u32
        + 1 // periodic_log: Bool
        + 1 // unused: u8
        + 4 // blocked: u32
        + 4 // hit_info: HitInfo
        + 1 // extend_flag: u8
    }
}
