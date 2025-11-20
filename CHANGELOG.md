# [unreleased]

Breaking changes:

- Upgrade Ruma to 0.14.0
  - All endpoints use `SinglePath` rather than `VersionHistory` as `PathBuilder`.
  - Bump MSRV to 1.88.

# 0.8.0

Breaking changes:

- The list_room response changes the fields `version`, `join_rules`, `guest_access` and
  `history_visibility` to be an option
- The list_room response changes the `join_rules` field to be `Option<SpaceRoomJoinRule>`
- `background_update::run::v1::JobName` is now non-exhaustive.
- `RoomSortOrder` and `RoomDirection` in `rooms::list_rooms::v1` are now non-
  exhaustive. Their `PartialOrd` and `Ord` implementations now use their string
  representation instead of the order in which they are defined in the enum.
- `UserDetails`, `ExternalId`, `CurrentUpdate`, `ExperimentalFeatures`,
  `RoomDetails` and `UserMinorDetails` are now non-exhaustive. To keep using
  them as if they were exhaustive, use the `ruma_unstable_exhaustive_types`
  compile-time `cfg` setting.
- Changed `users::create_or_modify::v2` third party id parsing by adding a new type without
  `validated_at` and `added_at` fields (which are not sent according to the API documentation)
- Upgrade ruma to 0.13.0

Improvement:

- The list_room response now includes the `room_type` field
- Add room_details api
- Add room_members api

# 0.7.0

* Upgrade to ruma 0.12.0
* Add room_membership api

# 0.6.0

* Update v1/server_version endpoint response data with optional python_version key
* Upgrade to ruma 0.9.4
* Add background updates endpoints

# 0.5.0

* Upgrade ruma dependency to 0.7.2
* Work around a Synapse issue affecting sqlite configurations

# 0.4.0

* Upgrade ruma dependency to 0.6.4

# 0.3.0

* Upgrade ruma dependency to 0.5.0

# 0.2.0

Improvements:

* Upgrade ruma dependency to 0.4.0
