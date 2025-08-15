# bcsfe_rs

A rust port of <https://github.com/fieryhenry/BCSFE-Python> with a GUI.

Note: this project is quite early on in development, so there will be many features missing.

WARNING: there is no ban prevention measures yet, so editing catfood, rare tickets, platinum tickets
and legend tickets could lead to your account being banned.

## Features

Features which are marked as ~strike-through~ are completed.

### Save Management

#### Loading a Save
~- Load from file~
- ~Load from transfer codes~
- ~Pull from adb / waydroid~
- Load from root storage
- Load from json

#### Writing a Save

- ~Write to file~
- ~Upload to server and get transfer codes~
- ~Push to device with adb / waydroid~
- ~Write to root storage~
- Upload tracked items to the server
- Write to json

### Edits

#### Account

- Unban account
- Change Inquiry Code
- Change Country Code
- Change Game Version

#### Items

- ~Catfood~
- ~XP~
- ~Normal Tickets~
- ~Rare Tickets~
- Rare Ticket Trade
- ~Platinum Tickets~
- Platinum Shards
- ~Legend Tickets~
- ~Leadership~
- ~NP~
- Battle Items
- Catseyes
- Catfruit
- Talent Orbs
- Catamins
- Scheme Items
- Labyrinth Medals
- Event Tickets
- Treasure Chests

#### Stages / Maps

- ~Main Story Clear All~
- Main Story Clear Specific Stages
- Main Story Treasures
- Stories of Legend
- Uncanny Legend
- Normal Event Stages
- Collab Event Stages
- Gauntlets
- Collab Gauntlets
- Tower Stages
- Catamin Stages
- Enigma Stages
- Clear Tutorial
- Challenge Battle
- Dojo
- Aku Realm
- Timed Scores
- Outbreaks / Zombie Stages
- Behemoth Stages
- Legend Quest
- Zero Legends


#### Cats

- Unlock
- Upgrade
- True Form
- Ultra Form
- Talents
- Cat Guide

#### Gamatoto / Ototo

- Engineers
- Base Materials
- Gamatoto Level
- Gamatoto Helpers
- Ototo Cat Cannon
- Cat Shrine


#### Gacha
- Rare Gacha Seed
- Normal Gacha Seed
- Event Gacha Seed

#### Misc

- Special Skills / Base Abilities
- Enemy Guide
- Unlocked Equip Slots
- Restart Pack
- Play Time
- User Rank Rewards
- Gold Pass
- Meow Medals
- Missions


## Build from source

```sh
git clone https://codeberg.org/fieryhenry/bcsfe_rs.git
cd bcsfe_rs
cargo run --features gui
```

## Contributing

See [CONTRIBUTING](./docs/CONTRIBUTING.md)


## License

bcsfe_rs is licensed under the GNU GPLv3 which can be read [here](https://www.gnu.org/licenses/gpl-3.0.en.html)
