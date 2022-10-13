# ComplaintStatus

## Client Version 3.3.5

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/social/smsg_feature_system_status.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/smsg_feature_system_status.wowm#L3).

```rust,ignore
enum ComplaintStatus : u8 {
    DISABLED = 0;
    ENABLED_WITHOUT_AUTO_IGNORE = 1;
    ENABLED_WITH_AUTO_IGNORE = 2;
}
```
### Type
The basic type is `u8`, a 1 byte (8 bit) integer.
### Enumerators
| Enumerator | Value  | Description | Comment |
| --------- | -------- | ----------- | ------- |
| `DISABLED` | 0 (0x00) |  |  |
| `ENABLED_WITHOUT_AUTO_IGNORE` | 1 (0x01) |  |  |
| `ENABLED_WITH_AUTO_IGNORE` | 2 (0x02) |  |  |

Used in:
* [SMSG_FEATURE_SYSTEM_STATUS](smsg_feature_system_status.md)
