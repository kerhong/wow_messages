use std::convert::{TryFrom, TryInto};
use crate::world::wrath::GmTicketEscalationStatus;
use crate::world::wrath::GmTicketStatus;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/gamemaster/smsg_gmticket_getticket.wowm:32`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/gamemaster/smsg_gmticket_getticket.wowm#L32):
/// ```text
/// smsg SMSG_GMTICKET_GETTICKET = 0x0212 {
///     GmTicketStatus status;
///     if (status == HAS_TEXT) {
///         u32 id;
///         CString text;
///         Bool need_more_help;
///         f32 days_since_ticket_creation;
///         f32 days_since_oldest_ticket_creation;
///         f32 days_since_last_updated;
///         GmTicketEscalationStatus escalation_status;
///         Bool read_by_gm;
///     }
/// }
/// ```
pub struct SMSG_GMTICKET_GETTICKET {
    pub status: SMSG_GMTICKET_GETTICKET_GmTicketStatus,
}

impl crate::Message for SMSG_GMTICKET_GETTICKET {
    const OPCODE: u32 = 0x0212;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // status: GmTicketStatus
        w.write_all(&(self.status.as_int() as u32).to_le_bytes())?;

        match &self.status {
            SMSG_GMTICKET_GETTICKET_GmTicketStatus::DbError => {}
            SMSG_GMTICKET_GETTICKET_GmTicketStatus::HasText {
                days_since_last_updated,
                days_since_oldest_ticket_creation,
                days_since_ticket_creation,
                escalation_status,
                id,
                need_more_help,
                read_by_gm,
                text,
            } => {
                // id: u32
                w.write_all(&id.to_le_bytes())?;

                // text: CString
                // TODO: Guard against strings that are already null-terminated
                assert_ne!(text.as_bytes().iter().rev().next(), Some(&0_u8), "String `text` must not be null-terminated.");
                w.write_all(text.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // need_more_help: Bool
                w.write_all(u8::from(*need_more_help).to_le_bytes().as_slice())?;

                // days_since_ticket_creation: f32
                w.write_all(&days_since_ticket_creation.to_le_bytes())?;

                // days_since_oldest_ticket_creation: f32
                w.write_all(&days_since_oldest_ticket_creation.to_le_bytes())?;

                // days_since_last_updated: f32
                w.write_all(&days_since_last_updated.to_le_bytes())?;

                // escalation_status: GmTicketEscalationStatus
                w.write_all(&(escalation_status.as_int() as u8).to_le_bytes())?;

                // read_by_gm: Bool
                w.write_all(u8::from(*read_by_gm).to_le_bytes().as_slice())?;

            }
            SMSG_GMTICKET_GETTICKET_GmTicketStatus::Default => {}
        }

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(4..=279).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0212, size: body_size as u32 });
        }

        // status: GmTicketStatus
        let status: GmTicketStatus = crate::util::read_u32_le(r)?.try_into()?;

        let status_if = match status {
            GmTicketStatus::DbError => SMSG_GMTICKET_GETTICKET_GmTicketStatus::DbError,
            GmTicketStatus::HasText => {
                // id: u32
                let id = crate::util::read_u32_le(r)?;

                // text: CString
                let text = crate::util::read_c_string_to_vec(r)?;
                let text = String::from_utf8(text)?;

                // need_more_help: Bool
                let need_more_help = crate::util::read_u8_le(r)? != 0;
                // days_since_ticket_creation: f32
                let days_since_ticket_creation = crate::util::read_f32_le(r)?;
                // days_since_oldest_ticket_creation: f32
                let days_since_oldest_ticket_creation = crate::util::read_f32_le(r)?;
                // days_since_last_updated: f32
                let days_since_last_updated = crate::util::read_f32_le(r)?;
                // escalation_status: GmTicketEscalationStatus
                let escalation_status: GmTicketEscalationStatus = crate::util::read_u8_le(r)?.try_into()?;

                // read_by_gm: Bool
                let read_by_gm = crate::util::read_u8_le(r)? != 0;
                SMSG_GMTICKET_GETTICKET_GmTicketStatus::HasText {
                    days_since_last_updated,
                    days_since_oldest_ticket_creation,
                    days_since_ticket_creation,
                    escalation_status,
                    id,
                    need_more_help,
                    read_by_gm,
                    text,
                }
            }
            GmTicketStatus::Default => SMSG_GMTICKET_GETTICKET_GmTicketStatus::Default,
        };

        Ok(Self {
            status: status_if,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::world::wrath::ServerMessage for SMSG_GMTICKET_GETTICKET {}

impl SMSG_GMTICKET_GETTICKET {
    pub(crate) fn size(&self) -> usize {
        self.status.size() // status: SMSG_GMTICKET_GETTICKET_GmTicketStatus
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SMSG_GMTICKET_GETTICKET_GmTicketStatus {
    DbError,
    HasText {
        days_since_last_updated: f32,
        days_since_oldest_ticket_creation: f32,
        days_since_ticket_creation: f32,
        escalation_status: GmTicketEscalationStatus,
        id: u32,
        need_more_help: bool,
        read_by_gm: bool,
        text: String,
    },
    Default,
}

impl Default for SMSG_GMTICKET_GETTICKET_GmTicketStatus {
    fn default() -> Self {
        // First enumerator without any fields
        Self::DbError
    }
}

impl SMSG_GMTICKET_GETTICKET_GmTicketStatus {
    pub(crate) const fn as_int(&self) -> u32 {
        match self {
            Self::DbError => 0,
            Self::HasText { .. } => 6,
            Self::Default => 10,
        }
    }

}

impl SMSG_GMTICKET_GETTICKET_GmTicketStatus {
    pub(crate) fn size(&self) -> usize {
        match self {
            Self::DbError => {
                4
            }
            Self::HasText {
                days_since_last_updated,
                days_since_oldest_ticket_creation,
                days_since_ticket_creation,
                escalation_status,
                id,
                need_more_help,
                read_by_gm,
                text,
            } => {
                4
                + 4 // days_since_last_updated: f32
                + 4 // days_since_oldest_ticket_creation: f32
                + 4 // days_since_ticket_creation: f32
                + 1 // escalation_status: GmTicketEscalationStatus
                + 4 // id: u32
                + 1 // need_more_help: Bool
                + 1 // read_by_gm: Bool
                + text.len() + 1 // text: CString
            }
            Self::Default => {
                4
            }
        }
    }
}
