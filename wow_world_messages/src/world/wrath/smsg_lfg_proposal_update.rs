use std::convert::{TryFrom, TryInto};
use crate::world::wrath::LfgProposal;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/_need_sorting/smsg_lfg_proposal_update.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/smsg_lfg_proposal_update.wowm#L1):
/// ```text
/// smsg SMSG_LFG_PROPOSAL_UPDATE = 0x0361 {
///     u32 dungeon_id;
///     u8 proposal_state;
///     u32 proposal_id;
///     u32 encounters_finished_mask;
///     u8 silent;
///     u8 amount_of_proposals;
///     LfgProposal[amount_of_proposals] proposals;
/// }
/// ```
pub struct SMSG_LFG_PROPOSAL_UPDATE {
    pub dungeon_id: u32,
    pub proposal_state: u8,
    pub proposal_id: u32,
    pub encounters_finished_mask: u32,
    pub silent: u8,
    pub proposals: Vec<LfgProposal>,
}

impl crate::Message for SMSG_LFG_PROPOSAL_UPDATE {
    const OPCODE: u32 = 0x0361;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // dungeon_id: u32
        w.write_all(&self.dungeon_id.to_le_bytes())?;

        // proposal_state: u8
        w.write_all(&self.proposal_state.to_le_bytes())?;

        // proposal_id: u32
        w.write_all(&self.proposal_id.to_le_bytes())?;

        // encounters_finished_mask: u32
        w.write_all(&self.encounters_finished_mask.to_le_bytes())?;

        // silent: u8
        w.write_all(&self.silent.to_le_bytes())?;

        // amount_of_proposals: u8
        w.write_all(&(self.proposals.len() as u8).to_le_bytes())?;

        // proposals: LfgProposal[amount_of_proposals]
        for i in self.proposals.iter() {
            i.write_into_vec(w)?;
        }

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(15..=2319).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0361, size: body_size as u32 });
        }

        // dungeon_id: u32
        let dungeon_id = crate::util::read_u32_le(r)?;

        // proposal_state: u8
        let proposal_state = crate::util::read_u8_le(r)?;

        // proposal_id: u32
        let proposal_id = crate::util::read_u32_le(r)?;

        // encounters_finished_mask: u32
        let encounters_finished_mask = crate::util::read_u32_le(r)?;

        // silent: u8
        let silent = crate::util::read_u8_le(r)?;

        // amount_of_proposals: u8
        let amount_of_proposals = crate::util::read_u8_le(r)?;

        // proposals: LfgProposal[amount_of_proposals]
        let mut proposals = Vec::with_capacity(amount_of_proposals as usize);
        for i in 0..amount_of_proposals {
            proposals.push(LfgProposal::read(r)?);
        }

        Ok(Self {
            dungeon_id,
            proposal_state,
            proposal_id,
            encounters_finished_mask,
            silent,
            proposals,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::world::wrath::ServerMessage for SMSG_LFG_PROPOSAL_UPDATE {}

impl SMSG_LFG_PROPOSAL_UPDATE {
    pub(crate) fn size(&self) -> usize {
        4 // dungeon_id: u32
        + 1 // proposal_state: u8
        + 4 // proposal_id: u32
        + 4 // encounters_finished_mask: u32
        + 1 // silent: u8
        + 1 // amount_of_proposals: u8
        + self.proposals.len() * 9 // proposals: LfgProposal[amount_of_proposals]
    }
}
