# Vector3d

## Client Version 1, Client Version 2, Client Version 3

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/vector.wowm:4`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/vector.wowm#L4).
```rust,ignore
struct Vector3d {
    f32 x;
    f32 y;
    f32 z;
}
```
### Body

| Offset | Size / Endianness | Type | Name | Comment |
| ------ | ----------------- | ---- | ---- | ------- |
| 0x00 | 4 / Little | f32 | x |  |
| 0x04 | 4 / Little | f32 | y |  |
| 0x08 | 4 / Little | f32 | z |  |


Used in:
* [CMSG_GMTICKET_CREATE](cmsg_gmticket_create.md)
* [CMSG_GM_REPORT_LAG](cmsg_gm_report_lag.md)
* [CMSG_MOVE_SET_RAW_POSITION](cmsg_move_set_raw_position.md)
* [CMSG_UPDATE_MISSILE_TRAJECTORY](cmsg_update_missile_trajectory.md)
* [CMSG_UPDATE_PROJECTILE_POSITION](cmsg_update_projectile_position.md)
* [CMSG_WORLD_TELEPORT](cmsg_world_teleport.md)
* [Character](character.md)
* [MSG_CORPSE_QUERY_Server](msg_corpse_query_server.md)
* [MSG_MOVE_TELEPORT_CHEAT_Server](msg_move_teleport_cheat_server.md)
* [MonsterMove](monstermove.md)
* [MovementBlock](movementblock.md)
* [MovementInfo](movementinfo.md)
* [SMSG_BINDPOINTUPDATE](smsg_bindpointupdate.md)
* [SMSG_DEATH_RELEASE_LOC](smsg_death_release_loc.md)
* [SMSG_LOGIN_VERIFY_WORLD](smsg_login_verify_world.md)
* [SMSG_MONSTER_MOVE](smsg_monster_move.md)
* [SMSG_MONSTER_MOVE_TRANSPORT](smsg_monster_move_transport.md)
* [SMSG_NEW_WORLD](smsg_new_world.md)
* [SMSG_PET_DISMISS_SOUND](smsg_pet_dismiss_sound.md)
* [SMSG_SET_PROJECTILE_POSITION](smsg_set_projectile_position.md)
* [SpellCastTargets](spellcasttargets.md)
* [TransportInfo](transportinfo.md)

