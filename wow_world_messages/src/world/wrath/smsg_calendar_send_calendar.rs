use std::convert::{TryFrom, TryInto};
use crate::DateTime;
use crate::world::wrath::SendCalendarEvent;
use crate::world::wrath::SendCalendarInstance;
use crate::world::wrath::SendCalendarInvite;
use crate::world::wrath::SendCalendarResetTime;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/_need_sorting/smsg_calendar_send_calendar.wowm:55`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/smsg_calendar_send_calendar.wowm#L55):
/// ```text
/// smsg SMSG_CALENDAR_SEND_CALENDAR = 0x0436 {
///     u32 amount_of_invites;
///     SendCalendarInvite[amount_of_invites] invites;
///     u32 amount_of_events;
///     SendCalendarEvent[amount_of_events] events;
///     u32 current_time;
///     DateTime zone_time;
///     u32 amount_of_instances;
///     SendCalendarInstance[amount_of_instances] instances;
///     u32 relative_time;
///     u32 amount_of_reset_times;
///     SendCalendarResetTime[amount_of_reset_times] reset_times;
///     u32 amount_of_holidays;
/// }
/// ```
pub struct SMSG_CALENDAR_SEND_CALENDAR {
    pub invites: Vec<SendCalendarInvite>,
    pub events: Vec<SendCalendarEvent>,
    pub current_time: u32,
    pub zone_time: DateTime,
    pub instances: Vec<SendCalendarInstance>,
    pub relative_time: u32,
    pub reset_times: Vec<SendCalendarResetTime>,
    pub amount_of_holidays: u32,
}

impl crate::Message for SMSG_CALENDAR_SEND_CALENDAR {
    const OPCODE: u32 = 0x0436;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // amount_of_invites: u32
        w.write_all(&(self.invites.len() as u32).to_le_bytes())?;

        // invites: SendCalendarInvite[amount_of_invites]
        for i in self.invites.iter() {
            i.write_into_vec(w)?;
        }

        // amount_of_events: u32
        w.write_all(&(self.events.len() as u32).to_le_bytes())?;

        // events: SendCalendarEvent[amount_of_events]
        for i in self.events.iter() {
            i.write_into_vec(w)?;
        }

        // current_time: u32
        w.write_all(&self.current_time.to_le_bytes())?;

        // zone_time: DateTime
        w.write_all(&self.zone_time.as_int().to_le_bytes())?;

        // amount_of_instances: u32
        w.write_all(&(self.instances.len() as u32).to_le_bytes())?;

        // instances: SendCalendarInstance[amount_of_instances]
        for i in self.instances.iter() {
            i.write_into_vec(w)?;
        }

        // relative_time: u32
        w.write_all(&self.relative_time.to_le_bytes())?;

        // amount_of_reset_times: u32
        w.write_all(&(self.reset_times.len() as u32).to_le_bytes())?;

        // reset_times: SendCalendarResetTime[amount_of_reset_times]
        for i in self.reset_times.iter() {
            i.write_into_vec(w)?;
        }

        // amount_of_holidays: u32
        w.write_all(&self.amount_of_holidays.to_le_bytes())?;

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(32..=4294967294).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0436, size: body_size as u32 });
        }

        // amount_of_invites: u32
        let amount_of_invites = crate::util::read_u32_le(r)?;

        // invites: SendCalendarInvite[amount_of_invites]
        let mut invites = Vec::with_capacity(amount_of_invites as usize);
        for i in 0..amount_of_invites {
            invites.push(SendCalendarInvite::read(r)?);
        }

        // amount_of_events: u32
        let amount_of_events = crate::util::read_u32_le(r)?;

        // events: SendCalendarEvent[amount_of_events]
        let mut events = Vec::with_capacity(amount_of_events as usize);
        for i in 0..amount_of_events {
            events.push(SendCalendarEvent::read(r)?);
        }

        // current_time: u32
        let current_time = crate::util::read_u32_le(r)?;

        // zone_time: DateTime
        let zone_time: DateTime = crate::util::read_u32_le(r)?.try_into()?;
        // amount_of_instances: u32
        let amount_of_instances = crate::util::read_u32_le(r)?;

        // instances: SendCalendarInstance[amount_of_instances]
        let mut instances = Vec::with_capacity(amount_of_instances as usize);
        for i in 0..amount_of_instances {
            instances.push(SendCalendarInstance::read(r)?);
        }

        // relative_time: u32
        let relative_time = crate::util::read_u32_le(r)?;

        // amount_of_reset_times: u32
        let amount_of_reset_times = crate::util::read_u32_le(r)?;

        // reset_times: SendCalendarResetTime[amount_of_reset_times]
        let mut reset_times = Vec::with_capacity(amount_of_reset_times as usize);
        for i in 0..amount_of_reset_times {
            reset_times.push(SendCalendarResetTime::read(r)?);
        }

        // amount_of_holidays: u32
        let amount_of_holidays = crate::util::read_u32_le(r)?;

        Ok(Self {
            invites,
            events,
            current_time,
            zone_time,
            instances,
            relative_time,
            reset_times,
            amount_of_holidays,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::world::wrath::ServerMessage for SMSG_CALENDAR_SEND_CALENDAR {}

impl SMSG_CALENDAR_SEND_CALENDAR {
    pub(crate) fn size(&self) -> usize {
        4 // amount_of_invites: u32
        + self.invites.iter().fold(0, |acc, x| acc + x.size()) // invites: SendCalendarInvite[amount_of_invites]
        + 4 // amount_of_events: u32
        + self.events.iter().fold(0, |acc, x| acc + x.size()) // events: SendCalendarEvent[amount_of_events]
        + 4 // current_time: u32
        + 4 // zone_time: DateTime
        + 4 // amount_of_instances: u32
        + self.instances.len() * 20 // instances: SendCalendarInstance[amount_of_instances]
        + 4 // relative_time: u32
        + 4 // amount_of_reset_times: u32
        + self.reset_times.len() * 12 // reset_times: SendCalendarResetTime[amount_of_reset_times]
        + 4 // amount_of_holidays: u32
    }
}
