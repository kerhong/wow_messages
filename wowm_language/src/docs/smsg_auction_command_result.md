# SMSG_AUCTION_COMMAND_RESULT

## Client Version 1.12

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/unimplemented/vanilla/smsg_auction_command_result.wowm:45`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/unimplemented/vanilla/smsg_auction_command_result.wowm#L45).
```rust,ignore
smsg SMSG_AUCTION_COMMAND_RESULT = 0x025B {
    unimplemented
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

The body for this message has not been implemented yet.
