/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/common.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/common.wowm#L1):
/// ```text
/// flag SpellCastTargetFlags : u16 {
///     SELF = 0x00000000;
///     UNUSED1 = 0x00000001;
///     UNIT = 0x00000002;
///     UNUSED2 = 0x00000004;
///     UNUSED3 = 0x00000008;
///     ITEM = 0x00000010;
///     SOURCE_LOCATION = 0x00000020;
///     DEST_LOCATION = 0x00000040;
///     OBJECT_UNK = 0x00000080;
///     UNIT_UNK = 0x00000100;
///     PVP_CORPSE = 0x00000200;
///     UNIT_CORPSE = 0x00000400;
///     GAMEOBJECT = 0x00000800;
///     TRADE_ITEM = 0x00001000;
///     STRING = 0x00002000;
///     UNK1 = 0x00004000;
///     CORPSE = 0x00008000;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct SpellCastTargetFlags {
    inner: u16,
}

impl SpellCastTargetFlags {
    pub const fn new(inner: u16) -> Self {
        Self { inner }
    }

    pub const SELF: u16 = 0x00;
    pub const UNUSED1: u16 = 0x01;
    pub const UNIT: u16 = 0x02;
    pub const UNUSED2: u16 = 0x04;
    pub const UNUSED3: u16 = 0x08;
    pub const ITEM: u16 = 0x10;
    pub const SOURCE_LOCATION: u16 = 0x20;
    pub const DEST_LOCATION: u16 = 0x40;
    pub const OBJECT_UNK: u16 = 0x80;
    pub const UNIT_UNK: u16 = 0x100;
    pub const PVP_CORPSE: u16 = 0x200;
    pub const UNIT_CORPSE: u16 = 0x400;
    pub const GAMEOBJECT: u16 = 0x800;
    pub const TRADE_ITEM: u16 = 0x1000;
    pub const STRING: u16 = 0x2000;
    pub const UNK1: u16 = 0x4000;
    pub const CORPSE: u16 = 0x8000;

    pub const fn empty() -> Self {
        Self { inner: 0 }
    }

    pub const fn is_empty(&self) -> bool {
        self.inner == 0
    }

    pub const fn all() -> Self {
        Self {
            inner: Self::SELF
                | Self::UNUSED1
                | Self::UNIT
                | Self::UNUSED2
                | Self::UNUSED3
                | Self::ITEM
                | Self::SOURCE_LOCATION
                | Self::DEST_LOCATION
                | Self::OBJECT_UNK
                | Self::UNIT_UNK
                | Self::PVP_CORPSE
                | Self::UNIT_CORPSE
                | Self::GAMEOBJECT
                | Self::TRADE_ITEM
                | Self::STRING
                | Self::UNK1
                | Self::CORPSE
        }
    }

    pub const fn is_UNUSED1(&self) -> bool {
        (self.inner & Self::UNUSED1) != 0
    }

    /// not used in any spells (can be set dynamically)
    ///
    pub const fn new_UNUSED1() -> Self {
        Self { inner: Self::UNUSED1 }
    }

    pub fn set_UNUSED1(&mut self) -> Self {
        self.inner |= Self::UNUSED1;
        *self
    }

    pub fn clear_UNUSED1(&mut self) -> Self {
        self.inner &= Self::UNUSED1.reverse_bits();
        *self
    }

    pub const fn is_UNIT(&self) -> bool {
        (self.inner & Self::UNIT) != 0
    }

    pub const fn new_UNIT() -> Self {
        Self { inner: Self::UNIT }
    }

    pub fn set_UNIT(&mut self) -> Self {
        self.inner |= Self::UNIT;
        *self
    }

    pub fn clear_UNIT(&mut self) -> Self {
        self.inner &= Self::UNIT.reverse_bits();
        *self
    }

    pub const fn is_UNUSED2(&self) -> bool {
        (self.inner & Self::UNUSED2) != 0
    }

    /// not used in any spells (can be set dynamically)
    ///
    pub const fn new_UNUSED2() -> Self {
        Self { inner: Self::UNUSED2 }
    }

    pub fn set_UNUSED2(&mut self) -> Self {
        self.inner |= Self::UNUSED2;
        *self
    }

    pub fn clear_UNUSED2(&mut self) -> Self {
        self.inner &= Self::UNUSED2.reverse_bits();
        *self
    }

    pub const fn is_UNUSED3(&self) -> bool {
        (self.inner & Self::UNUSED3) != 0
    }

    /// not used in any spells (can be set dynamically)
    ///
    pub const fn new_UNUSED3() -> Self {
        Self { inner: Self::UNUSED3 }
    }

    pub fn set_UNUSED3(&mut self) -> Self {
        self.inner |= Self::UNUSED3;
        *self
    }

    pub fn clear_UNUSED3(&mut self) -> Self {
        self.inner &= Self::UNUSED3.reverse_bits();
        *self
    }

    pub const fn is_ITEM(&self) -> bool {
        (self.inner & Self::ITEM) != 0
    }

    pub const fn new_ITEM() -> Self {
        Self { inner: Self::ITEM }
    }

    pub fn set_ITEM(&mut self) -> Self {
        self.inner |= Self::ITEM;
        *self
    }

    pub fn clear_ITEM(&mut self) -> Self {
        self.inner &= Self::ITEM.reverse_bits();
        *self
    }

    pub const fn is_SOURCE_LOCATION(&self) -> bool {
        (self.inner & Self::SOURCE_LOCATION) != 0
    }

    pub const fn new_SOURCE_LOCATION() -> Self {
        Self { inner: Self::SOURCE_LOCATION }
    }

    pub fn set_SOURCE_LOCATION(&mut self) -> Self {
        self.inner |= Self::SOURCE_LOCATION;
        *self
    }

    pub fn clear_SOURCE_LOCATION(&mut self) -> Self {
        self.inner &= Self::SOURCE_LOCATION.reverse_bits();
        *self
    }

    pub const fn is_DEST_LOCATION(&self) -> bool {
        (self.inner & Self::DEST_LOCATION) != 0
    }

    pub const fn new_DEST_LOCATION() -> Self {
        Self { inner: Self::DEST_LOCATION }
    }

    pub fn set_DEST_LOCATION(&mut self) -> Self {
        self.inner |= Self::DEST_LOCATION;
        *self
    }

    pub fn clear_DEST_LOCATION(&mut self) -> Self {
        self.inner &= Self::DEST_LOCATION.reverse_bits();
        *self
    }

    pub const fn is_OBJECT_UNK(&self) -> bool {
        (self.inner & Self::OBJECT_UNK) != 0
    }

    /// used in 7 spells only
    ///
    pub const fn new_OBJECT_UNK() -> Self {
        Self { inner: Self::OBJECT_UNK }
    }

    pub fn set_OBJECT_UNK(&mut self) -> Self {
        self.inner |= Self::OBJECT_UNK;
        *self
    }

    pub fn clear_OBJECT_UNK(&mut self) -> Self {
        self.inner &= Self::OBJECT_UNK.reverse_bits();
        *self
    }

    pub const fn is_UNIT_UNK(&self) -> bool {
        (self.inner & Self::UNIT_UNK) != 0
    }

    /// looks like self target (389 spells)
    ///
    pub const fn new_UNIT_UNK() -> Self {
        Self { inner: Self::UNIT_UNK }
    }

    pub fn set_UNIT_UNK(&mut self) -> Self {
        self.inner |= Self::UNIT_UNK;
        *self
    }

    pub fn clear_UNIT_UNK(&mut self) -> Self {
        self.inner &= Self::UNIT_UNK.reverse_bits();
        *self
    }

    pub const fn is_PVP_CORPSE(&self) -> bool {
        (self.inner & Self::PVP_CORPSE) != 0
    }

    pub const fn new_PVP_CORPSE() -> Self {
        Self { inner: Self::PVP_CORPSE }
    }

    pub fn set_PVP_CORPSE(&mut self) -> Self {
        self.inner |= Self::PVP_CORPSE;
        *self
    }

    pub fn clear_PVP_CORPSE(&mut self) -> Self {
        self.inner &= Self::PVP_CORPSE.reverse_bits();
        *self
    }

    pub const fn is_UNIT_CORPSE(&self) -> bool {
        (self.inner & Self::UNIT_CORPSE) != 0
    }

    /// 10 spells (gathering professions)
    ///
    pub const fn new_UNIT_CORPSE() -> Self {
        Self { inner: Self::UNIT_CORPSE }
    }

    pub fn set_UNIT_CORPSE(&mut self) -> Self {
        self.inner |= Self::UNIT_CORPSE;
        *self
    }

    pub fn clear_UNIT_CORPSE(&mut self) -> Self {
        self.inner &= Self::UNIT_CORPSE.reverse_bits();
        *self
    }

    pub const fn is_GAMEOBJECT(&self) -> bool {
        (self.inner & Self::GAMEOBJECT) != 0
    }

    /// pguid, 0 spells
    ///
    pub const fn new_GAMEOBJECT() -> Self {
        Self { inner: Self::GAMEOBJECT }
    }

    pub fn set_GAMEOBJECT(&mut self) -> Self {
        self.inner |= Self::GAMEOBJECT;
        *self
    }

    pub fn clear_GAMEOBJECT(&mut self) -> Self {
        self.inner &= Self::GAMEOBJECT.reverse_bits();
        *self
    }

    pub const fn is_TRADE_ITEM(&self) -> bool {
        (self.inner & Self::TRADE_ITEM) != 0
    }

    /// pguid, 0 spells
    ///
    pub const fn new_TRADE_ITEM() -> Self {
        Self { inner: Self::TRADE_ITEM }
    }

    pub fn set_TRADE_ITEM(&mut self) -> Self {
        self.inner |= Self::TRADE_ITEM;
        *self
    }

    pub fn clear_TRADE_ITEM(&mut self) -> Self {
        self.inner &= Self::TRADE_ITEM.reverse_bits();
        *self
    }

    pub const fn is_STRING(&self) -> bool {
        (self.inner & Self::STRING) != 0
    }

    /// string, 0 spells
    ///
    pub const fn new_STRING() -> Self {
        Self { inner: Self::STRING }
    }

    pub fn set_STRING(&mut self) -> Self {
        self.inner |= Self::STRING;
        *self
    }

    pub fn clear_STRING(&mut self) -> Self {
        self.inner &= Self::STRING.reverse_bits();
        *self
    }

    pub const fn is_UNK1(&self) -> bool {
        (self.inner & Self::UNK1) != 0
    }

    /// 199 spells, opening object/lock
    ///
    pub const fn new_UNK1() -> Self {
        Self { inner: Self::UNK1 }
    }

    pub fn set_UNK1(&mut self) -> Self {
        self.inner |= Self::UNK1;
        *self
    }

    pub fn clear_UNK1(&mut self) -> Self {
        self.inner &= Self::UNK1.reverse_bits();
        *self
    }

    pub const fn is_CORPSE(&self) -> bool {
        (self.inner & Self::CORPSE) != 0
    }

    /// pguid, resurrection spells
    ///
    pub const fn new_CORPSE() -> Self {
        Self { inner: Self::CORPSE }
    }

    pub fn set_CORPSE(&mut self) -> Self {
        self.inner |= Self::CORPSE;
        *self
    }

    pub fn clear_CORPSE(&mut self) -> Self {
        self.inner &= Self::CORPSE.reverse_bits();
        *self
    }

    pub const fn as_int(&self) -> u16 {
        self.inner
    }

}

impl std::fmt::UpperHex for SpellCastTargetFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::UpperHex::fmt(&self.inner, f)
    }
}

impl std::fmt::LowerHex for SpellCastTargetFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::LowerHex::fmt(&self.inner, f)
    }
}

impl std::fmt::Octal for SpellCastTargetFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Octal::fmt(&self.inner, f)
    }
}

impl std::fmt::Binary for SpellCastTargetFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Binary::fmt(&self.inner, f)
    }
}

impl std::ops::BitAnd for SpellCastTargetFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitand(rhs.inner), }
    }
}

impl std::ops::BitAndAssign for SpellCastTargetFlags {
    fn bitand_assign(&mut self, rhs: Self) {
        self.inner.bitand_assign(rhs.inner)
    }
}

impl std::ops::BitOr for SpellCastTargetFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitor(rhs.inner), }
    }
}

impl std::ops::BitOrAssign for SpellCastTargetFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.inner.bitor_assign(rhs.inner)
    }
}

impl std::ops::BitXor for SpellCastTargetFlags {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitxor(rhs.inner), }
    }
}

impl std::ops::BitXorAssign for SpellCastTargetFlags {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.inner.bitxor_assign(rhs.inner)
    }
}
