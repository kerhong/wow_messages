use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::wrath::QuestGiverStatus;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/_need_sorting/smsg_questgiver_status_multiple.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/smsg_questgiver_status_multiple.wowm#L1):
/// ```text
/// struct QuestGiverStatusReport {
///     Guid npc;
///     QuestGiverStatus dialog_status;
/// }
/// ```
pub struct QuestGiverStatusReport {
    pub npc: Guid,
    pub dialog_status: QuestGiverStatus,
}

impl QuestGiverStatusReport {
    pub(crate) fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // npc: Guid
        w.write_all(&self.npc.guid().to_le_bytes())?;

        // dialog_status: QuestGiverStatus
        w.write_all(&(self.dialog_status.as_int() as u8).to_le_bytes())?;

        Ok(())
    }
}

impl QuestGiverStatusReport {
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, crate::errors::ParseError> {
        // npc: Guid
        let npc = Guid::read(r)?;

        // dialog_status: QuestGiverStatus
        let dialog_status: QuestGiverStatus = crate::util::read_u8_le(r)?.try_into()?;

        Ok(Self {
            npc,
            dialog_status,
        })
    }

}
