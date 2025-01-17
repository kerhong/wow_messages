# SMSG_SHOWTAXINODES

## Client Version 1.12, Client Version 2, Client Version 3

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/movement/smsg/smsg_showtaxinodes.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/smsg/smsg_showtaxinodes.wowm#L3).
```rust,ignore
smsg SMSG_SHOWTAXINODES = 0x01A9 {
    u32 unknown1;
    Guid guid;
    u32 nearest_node;
    u32[-] nodes;
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
| - | 4 / Little | u32 | unknown1 | Set to 1 in mangoszero |
| - | 8 / Little | [Guid](../types/packed-guid.md) | guid |  |
| - | 4 / Little | u32 | nearest_node |  |
| - | ? / - | u32[-] | nodes |  |

