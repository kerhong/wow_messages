use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::shared::mail_item_tbc_wrath::MailItem;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/mail/cmsg_send_mail.wowm:32`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/mail/cmsg_send_mail.wowm#L32):
/// ```text
/// cmsg CMSG_SEND_MAIL = 0x0238 {
///     Guid mailbox;
///     CString receiver;
///     CString subject;
///     CString body;
///     u32 unknown1;
///     u32 unknown2;
///     u8 amount_of_items;
///     MailItem[amount_of_items] items;
///     u32 money;
///     u32 cash_on_delivery_amount;
///     u32 unknown3;
///     u32 unknown4;
/// }
/// ```
pub struct CMSG_SEND_MAIL {
    pub mailbox: Guid,
    pub receiver: String,
    pub subject: String,
    pub body: String,
    /// cmangos: stationery?
    ///
    pub unknown1: u32,
    /// cmangos: 0x00000000
    ///
    pub unknown2: u32,
    pub items: Vec<MailItem>,
    pub money: u32,
    pub cash_on_delivery_amount: u32,
    /// mangosone: const 0
    ///
    pub unknown3: u32,
    /// mangosone: const 0
    ///
    pub unknown4: u32,
}

impl crate::Message for CMSG_SEND_MAIL {
    const OPCODE: u32 = 0x0238;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // mailbox: Guid
        w.write_all(&self.mailbox.guid().to_le_bytes())?;

        // receiver: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.receiver.as_bytes().iter().rev().next(), Some(&0_u8), "String `receiver` must not be null-terminated.");
        w.write_all(self.receiver.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // subject: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.subject.as_bytes().iter().rev().next(), Some(&0_u8), "String `subject` must not be null-terminated.");
        w.write_all(self.subject.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // body: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.body.as_bytes().iter().rev().next(), Some(&0_u8), "String `body` must not be null-terminated.");
        w.write_all(self.body.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // unknown1: u32
        w.write_all(&self.unknown1.to_le_bytes())?;

        // unknown2: u32
        w.write_all(&self.unknown2.to_le_bytes())?;

        // amount_of_items: u8
        w.write_all(&(self.items.len() as u8).to_le_bytes())?;

        // items: MailItem[amount_of_items]
        for i in self.items.iter() {
            i.write_into_vec(w)?;
        }

        // money: u32
        w.write_all(&self.money.to_le_bytes())?;

        // cash_on_delivery_amount: u32
        w.write_all(&self.cash_on_delivery_amount.to_le_bytes())?;

        // unknown3: u32
        w.write_all(&self.unknown3.to_le_bytes())?;

        // unknown4: u32
        w.write_all(&self.unknown4.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // mailbox: Guid
        let mailbox = Guid::read(r)?;

        // receiver: CString
        let receiver = crate::util::read_c_string_to_vec(r)?;
        let receiver = String::from_utf8(receiver)?;

        // subject: CString
        let subject = crate::util::read_c_string_to_vec(r)?;
        let subject = String::from_utf8(subject)?;

        // body: CString
        let body = crate::util::read_c_string_to_vec(r)?;
        let body = String::from_utf8(body)?;

        // unknown1: u32
        let unknown1 = crate::util::read_u32_le(r)?;

        // unknown2: u32
        let unknown2 = crate::util::read_u32_le(r)?;

        // amount_of_items: u8
        let amount_of_items = crate::util::read_u8_le(r)?;

        // items: MailItem[amount_of_items]
        let mut items = Vec::with_capacity(amount_of_items as usize);
        for i in 0..amount_of_items {
            items.push(MailItem::read(r)?);
        }

        // money: u32
        let money = crate::util::read_u32_le(r)?;

        // cash_on_delivery_amount: u32
        let cash_on_delivery_amount = crate::util::read_u32_le(r)?;

        // unknown3: u32
        let unknown3 = crate::util::read_u32_le(r)?;

        // unknown4: u32
        let unknown4 = crate::util::read_u32_le(r)?;

        Ok(Self {
            mailbox,
            receiver,
            subject,
            body,
            unknown1,
            unknown2,
            items,
            money,
            cash_on_delivery_amount,
            unknown3,
            unknown4,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::world::tbc::ClientMessage for CMSG_SEND_MAIL {}

#[cfg(feature = "wrath")]
impl crate::world::wrath::ClientMessage for CMSG_SEND_MAIL {}

impl CMSG_SEND_MAIL {
    pub(crate) fn size(&self) -> usize {
        8 // mailbox: Guid
        + self.receiver.len() + 1 // receiver: CString
        + self.subject.len() + 1 // subject: CString
        + self.body.len() + 1 // body: CString
        + 4 // unknown1: u32
        + 4 // unknown2: u32
        + 1 // amount_of_items: u8
        + self.items.len() * 9 // items: MailItem[amount_of_items]
        + 4 // money: u32
        + 4 // cash_on_delivery_amount: u32
        + 4 // unknown3: u32
        + 4 // unknown4: u32
    }
}
