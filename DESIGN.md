# Ashes Design Document

Since this is a demo, the original scale of the commercial game has been dramatically scaled back to make this fit in a 30-day timeline. This demo is free, of course, as part of the jam. This is also the official demo that will be published to Steam when the full game page is setup and ready for use. (likely by November if anyone is interested in staying up-to-date) The following checklist / unordered list below is the list of curated features that should and will be implemented in teh final build of the demo.

```
- Scavenging in City Blocks
- Basic Resource Collection
- Combat via improvised weapons
- Crafting weapons/armour
- Permadeath
- Recipe-Based Crafting
- One-Room Safehouse
- Score ~ Days Survived
- Small explorable world
```

And that's it, the main commercial game has far more to keep track of, the demo will be updated periodically as development trudges through pre-alpha, alpha and finally before the open early access is closed and final release is prepared.

## Data Driven Development is Cool!

All the data for the game is written in plain YAML, since YAML is cool and there's a native Rust package for it. All the YAML is plain-text so you can easily modify it if you want to make things interesting. There are backups that will auto-revert the modified files if anything turns up broken (logic is cool!), this data is not editable in the web build without building it for yourself and playing locally. The save is also in YAML.
