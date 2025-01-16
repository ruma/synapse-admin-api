# [unreleased]

Breaking changes:

* The list_room response changes the fields `version`, `join_rules`, `guest_access` and `history_visibility` to be an option
* The list_room response changes the `join_rules` field to be `Option<SpaceRoomJoinRule>`

Improvement:

* The list_room response now includes the `room_type` field
* Add room_details api

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
