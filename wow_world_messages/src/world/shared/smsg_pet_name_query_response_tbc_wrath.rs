use std::convert::{TryFrom, TryInto};
use crate::world::shared::pet_query_disabled_names_tbc_wrath::PetQueryDisabledNames;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/smsg_pet_name_query_response.wowm:30`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/smsg_pet_name_query_response.wowm#L30):
/// ```text
/// smsg SMSG_PET_NAME_QUERY_RESPONSE = 0x0053 {
///     u32 pet_number;
///     CString name;
///     u32 pet_name_timestamp;
///     PetQueryDisabledNames names;
///     if (names == PRESENT) {
///         CString[5] declined_names;
///     }
/// }
/// ```
pub struct SMSG_PET_NAME_QUERY_RESPONSE {
    pub pet_number: u32,
    pub name: String,
    pub pet_name_timestamp: u32,
    pub names: SMSG_PET_NAME_QUERY_RESPONSE_PetQueryDisabledNames,
}

impl crate::Message for SMSG_PET_NAME_QUERY_RESPONSE {
    const OPCODE: u32 = 0x0053;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // pet_number: u32
        w.write_all(&self.pet_number.to_le_bytes())?;

        // name: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.name.as_bytes().iter().rev().next(), Some(&0_u8), "String `name` must not be null-terminated.");
        w.write_all(self.name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // pet_name_timestamp: u32
        w.write_all(&self.pet_name_timestamp.to_le_bytes())?;

        // names: PetQueryDisabledNames
        w.write_all(&(self.names.as_int() as u8).to_le_bytes())?;

        match &self.names {
            SMSG_PET_NAME_QUERY_RESPONSE_PetQueryDisabledNames::Present {
                declined_names,
            } => {
                // declined_names: CString[5]
                for i in declined_names.iter() {
                    w.write_all(i.as_bytes())?;
                    w.write_all(&[0])?;
                }

            }
            SMSG_PET_NAME_QUERY_RESPONSE_PetQueryDisabledNames::NotPresent => {}
        }

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(10..=1545).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0053, size: body_size as u32 });
        }

        // pet_number: u32
        let pet_number = crate::util::read_u32_le(r)?;

        // name: CString
        let name = crate::util::read_c_string_to_vec(r)?;
        let name = String::from_utf8(name)?;

        // pet_name_timestamp: u32
        let pet_name_timestamp = crate::util::read_u32_le(r)?;

        // names: PetQueryDisabledNames
        let names: PetQueryDisabledNames = crate::util::read_u8_le(r)?.try_into()?;

        let names_if = match names {
            PetQueryDisabledNames::Present => {
                // declined_names: CString[5]
                let mut declined_names = Vec::with_capacity(5);
                for i in 0..5 {
                    let s = crate::util::read_c_string_to_vec(r)?;
                    declined_names.push(String::from_utf8(s)?);
                }
                let declined_names = declined_names.try_into().unwrap();

                SMSG_PET_NAME_QUERY_RESPONSE_PetQueryDisabledNames::Present {
                    declined_names,
                }
            }
            PetQueryDisabledNames::NotPresent => SMSG_PET_NAME_QUERY_RESPONSE_PetQueryDisabledNames::NotPresent,
        };

        Ok(Self {
            pet_number,
            name,
            pet_name_timestamp,
            names: names_if,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::world::tbc::ServerMessage for SMSG_PET_NAME_QUERY_RESPONSE {}

#[cfg(feature = "wrath")]
impl crate::world::wrath::ServerMessage for SMSG_PET_NAME_QUERY_RESPONSE {}

impl SMSG_PET_NAME_QUERY_RESPONSE {
    pub(crate) fn size(&self) -> usize {
        4 // pet_number: u32
        + self.name.len() + 1 // name: CString
        + 4 // pet_name_timestamp: u32
        + self.names.size() // names: SMSG_PET_NAME_QUERY_RESPONSE_PetQueryDisabledNames
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SMSG_PET_NAME_QUERY_RESPONSE_PetQueryDisabledNames {
    Present {
        declined_names: [String; 5],
    },
    NotPresent,
}

impl Default for SMSG_PET_NAME_QUERY_RESPONSE_PetQueryDisabledNames {
    fn default() -> Self {
        // First enumerator without any fields
        Self::NotPresent
    }
}

impl SMSG_PET_NAME_QUERY_RESPONSE_PetQueryDisabledNames {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::Present { .. } => 1,
            Self::NotPresent => 0,
        }
    }

}

impl SMSG_PET_NAME_QUERY_RESPONSE_PetQueryDisabledNames {
    pub(crate) fn size(&self) -> usize {
        match self {
            Self::Present {
                declined_names,
            } => {
                1
                + declined_names.iter().fold(0, |acc, x| acc + x.len() + 1) // declined_names: CString[5]
            }
            Self::NotPresent => {
                1
            }
        }
    }
}
