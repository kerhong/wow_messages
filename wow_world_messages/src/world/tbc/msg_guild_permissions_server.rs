use std::convert::{TryFrom, TryInto};
use crate::world::tbc::BankTab;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/_need_sorting/msg_guild_permissions.wowm:16`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/msg_guild_permissions.wowm#L16):
/// ```text
/// smsg MSG_GUILD_PERMISSIONS_Server = 0x03FC {
///     u32 id;
///     u32 rights;
///     u32 gold_limit_per_day;
///     u8 purchased_bank_tabs;
///     BankTab[6] bank_tabs;
/// }
/// ```
pub struct MSG_GUILD_PERMISSIONS_Server {
    pub id: u32,
    pub rights: u32,
    pub gold_limit_per_day: u32,
    pub purchased_bank_tabs: u8,
    pub bank_tabs: [BankTab; 6],
}

impl crate::Message for MSG_GUILD_PERMISSIONS_Server {
    const OPCODE: u32 = 0x03fc;

    fn size_without_header(&self) -> u32 {
        61
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // id: u32
        w.write_all(&self.id.to_le_bytes())?;

        // rights: u32
        w.write_all(&self.rights.to_le_bytes())?;

        // gold_limit_per_day: u32
        w.write_all(&self.gold_limit_per_day.to_le_bytes())?;

        // purchased_bank_tabs: u8
        w.write_all(&self.purchased_bank_tabs.to_le_bytes())?;

        // bank_tabs: BankTab[6]
        for i in self.bank_tabs.iter() {
            i.write_into_vec(w)?;
        }

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 61 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x03FC, size: body_size as u32 });
        }

        // id: u32
        let id = crate::util::read_u32_le(r)?;

        // rights: u32
        let rights = crate::util::read_u32_le(r)?;

        // gold_limit_per_day: u32
        let gold_limit_per_day = crate::util::read_u32_le(r)?;

        // purchased_bank_tabs: u8
        let purchased_bank_tabs = crate::util::read_u8_le(r)?;

        // bank_tabs: BankTab[6]
        let mut bank_tabs = [BankTab::default(); 6];
        for i in bank_tabs.iter_mut() {
            *i = BankTab::read(r)?;
        }

        Ok(Self {
            id,
            rights,
            gold_limit_per_day,
            purchased_bank_tabs,
            bank_tabs,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::world::tbc::ServerMessage for MSG_GUILD_PERMISSIONS_Server {}
