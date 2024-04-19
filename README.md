# Death Must Die save repacker

[так же на Русском языке](doc/README_RU.md)

This is tool can unpack your savegame to json and pack it back.
Helps to save your time grinding gold or loot for some useful thing (like learning rust for example =)

# Usage
App has only one argument is filename.
Extension provided filename makes sense, for example ".sav" file 
will be unpacked as "json", but "json" will be packed back to "sav".

For the sake of security this app can't overwrite any existing file creating new one instead.

## Unpack .sav to .json
`dmdsave Slot_0.sav`

## Pack .json to .sav
`dmdsave result_00.json`

## Savegame paths
Game has been written on Unity, so here are paths for save file:

| OS      | path                                                                              |
|---------|-----------------------------------------------------------------------------------|
| MacOS   | `~/Library/Application Support/com.Realm-Archive.Death-Must-Die/Saves/Slot_0.sav` |
| Windows | `\Appdata\LocalLow\Realm Archive\Death Must Die\Saves\Slot_0.sav.`                |                

## Workflow
1. Find the file `Slot_0.sav` in your OS
2. Make a backup this file
3. Unpack it `dmdsave Slot_0.sav` and get `result_00.json` file
4. Make any changes in your favourite text editor.
5. Pack it back `dmdsave result_00.json` and get `result_00.sav` file
6. Rename it to `Slot_0.sav` manually and replace with this file you original `Slot_0.sav` file

## What can i change in .json file?
### Add some Gold
The most harmless game impression hack.
Open your `result_*.json` and just find the string `Gold`. You will find the string like this:
```json
\"Gold\":400}
```
Just the put amount of gold you want, pack and put back to saves with `Slot_0.sav` name.

### Modify items
Every item in game has [Affixes](doc/Affixes.md) you can modify. When you find the item in json file,
you will see the Affixes json array [], just put new json object {Code:?, Level:?} in it or edit original. Few samples:
#### Dash Cooldown
```json
{\\\"Code\\\":\\\"dcld%\\\",\\\"Levels\\\":10}
```
This affix gives you improved cooldown with level 10 (-100% Dash Cooldown). Seems nice for your new boots =)
#### Skill Summon (Skeleton)
```json
{\\\"Code\\\":\\\"ssum4\\\",\\\"Levels\\\":100}
```
Skeletal Army fights alongside you (400 skeletons)

#### Shard Pull Area
```json
{\\\"Code\\\":\\\"pul\\\",\\\"Levels\\\":200}
```
+6000 Shard pull Area

#### How to find item
Find equipped item by text `BoundToCharacterCode`.

Found item type by text `\\\"Type\\\"`. List of types:

| Type   | Position |
|--------|----------|
| Weapon | 0        |
| Head   | 1        |
| Torso  | 2        |
| Hands  | 3        |
| Waist  | 4        |
| Feet   | 5        |
| Ring   | 6        |
| Amulet | 7        |
| Relic  | 8        |
| Jewel  | 9        |

Have a nice play and have fun!
(tested on version v0.7.1)