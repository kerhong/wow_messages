# MoneyLogItem

## Client Version 2.4.3, Client Version 3

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/_need_sorting/msg_guild_bank_log_query.wowm:22`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/msg_guild_bank_log_query.wowm#L22).
```rust,ignore
struct MoneyLogItem {
    u8 action;
    Guid player;
    u32 entry;
    u32 timestamp;
}
```
### Body

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| 0x00 | 1 / - | u8 | action |  |  |
| 0x01 | 8 / Little | [Guid](../spec/packed-guid.md) | player |  |  |
| 0x09 | 4 / Little | u32 | entry |  |  |
| 0x0D | 4 / Little | u32 | timestamp |  |  |
