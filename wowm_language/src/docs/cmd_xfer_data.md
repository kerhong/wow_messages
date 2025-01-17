# CMD_XFER_DATA

## Protocol Version 3

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/login/cmd_xfer.wowm:10`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/login/cmd_xfer.wowm#L10).
```rust,ignore
slogin CMD_XFER_DATA = 0x31 {
    u16 size;
    u8[size] data;
}
```
### Header

Login messages have a header of 1 byte with an opcode. Some messages also have a size field but this is not considered part of the header.

#### Login Header

| Offset | Size / Endianness | Type   | Name   | Description |
| ------ | ----------------- | ------ | ------ | ----------- |
| 0x00   | 1 / -             | uint8  | opcode | Opcode that determines which fields the message contains.|

### Body

| Offset | Size / Endianness | Type | Name | Comment |
| ------ | ----------------- | ---- | ---- | ------- |
| 0x01 | 2 / Little | u16 | size |  |
| 0x03 | ? / - | u8[size] | data |  |

### Examples

#### Example 1

```c
49, // opcode (49)
1, 0, // size: u16
255, // data: u8[size]
```
#### Example 2

```c
49, // opcode (49)
0, 0, // size: u16
// data: u8[size]
```
