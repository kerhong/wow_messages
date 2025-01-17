# SMSG_GUILD_ROSTER

## Client Version 1.12

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/guild/smsg_guild_roster.wowm:24`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild/smsg_guild_roster.wowm#L24).
```rust,ignore
smsg SMSG_GUILD_ROSTER = 0x008A {
    u32 amount_of_members;
    CString motd;
    CString guild_info;
    u32 amount_of_rights;
    u32[amount_of_rights] rights;
    GuildMember[amount_of_members] members;
}
```
### Header

SMSG have a header of 4 bytes.

#### SMSG Header

| Offset | Size / Endianness | Type   | Name   | Description |
| ------ | ----------------- | ------ | ------ | ----------- |
| 0x00   | 2 / Big           | uint16 | size   | Size of the rest of the message including the opcode field but not including the size field.|
| 0x02   | 2 / Little        | uint16 | opcode | Opcode that determines which fields the message contains.|

### Body

| Offset | Size / Endianness | Type | Name | Comment |
| ------ | ----------------- | ---- | ---- | ------- |
| 0x04 | 4 / Little | u32 | amount_of_members |  |
| 0x08 | - / - | CString | motd |  |
| - | - / - | CString | guild_info |  |
| - | 4 / Little | u32 | amount_of_rights |  |
| - | ? / - | u32[amount_of_rights] | rights |  |
| - | ? / - | [GuildMember](guildmember.md)[amount_of_members] | members |  |

## Client Version 2.4.3

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/guild/smsg_guild_roster.wowm:82`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild/smsg_guild_roster.wowm#L82).
```rust,ignore
smsg SMSG_GUILD_ROSTER = 0x008A {
    u32 amount_of_members;
    CString motd;
    CString guild_info;
    u32 amount_of_rights;
    GuildRights[amount_of_rights] rights;
    GuildMember[amount_of_members] members;
}
```
### Header

SMSG have a header of 4 bytes.

#### SMSG Header

| Offset | Size / Endianness | Type   | Name   | Description |
| ------ | ----------------- | ------ | ------ | ----------- |
| 0x00   | 2 / Big           | uint16 | size   | Size of the rest of the message including the opcode field but not including the size field.|
| 0x02   | 2 / Little        | uint16 | opcode | Opcode that determines which fields the message contains.|

### Body

| Offset | Size / Endianness | Type | Name | Comment |
| ------ | ----------------- | ---- | ---- | ------- |
| 0x04 | 4 / Little | u32 | amount_of_members |  |
| 0x08 | - / - | CString | motd |  |
| - | - / - | CString | guild_info |  |
| - | 4 / Little | u32 | amount_of_rights |  |
| - | ? / - | [GuildRights](guildrights.md)[amount_of_rights] | rights |  |
| - | ? / - | [GuildMember](guildmember.md)[amount_of_members] | members |  |

## Client Version 3.3.5

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/guild/smsg_guild_roster.wowm:82`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild/smsg_guild_roster.wowm#L82).
```rust,ignore
smsg SMSG_GUILD_ROSTER = 0x008A {
    u32 amount_of_members;
    CString motd;
    CString guild_info;
    u32 amount_of_rights;
    GuildRights[amount_of_rights] rights;
    GuildMember[amount_of_members] members;
}
```
### Header

SMSG have a header of 4 bytes.

#### SMSG Header

| Offset | Size / Endianness | Type   | Name   | Description |
| ------ | ----------------- | ------ | ------ | ----------- |
| 0x00   | 2 **OR** 3 / Big           | uint16 **OR** uint16+uint8 | size | Size of the rest of the message including the opcode field but not including the size field. Wrath server messages **can** be 3 bytes. If the first (most significant) size byte has `0x80` set, the header will be 3 bytes, otherwise it is 2.|
| -      | 2 / Little| uint16 | opcode | Opcode that determines which fields the message contains. |

### Body

| Offset | Size / Endianness | Type | Name | Comment |
| ------ | ----------------- | ---- | ---- | ------- |
| - | 4 / Little | u32 | amount_of_members |  |
| - | - / - | CString | motd |  |
| - | - / - | CString | guild_info |  |
| - | 4 / Little | u32 | amount_of_rights |  |
| - | ? / - | [GuildRights](guildrights.md)[amount_of_rights] | rights |  |
| - | ? / - | [GuildMember](guildmember.md)[amount_of_members] | members |  |

