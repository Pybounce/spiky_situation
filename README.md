# Bevy Multiplayer Platformer

## Alpha 1.0 Checklist

- [ ] Many full levels with checkpoints and difficulty
- [ ] Stage select UI
- [ ] New stage mechanics
- [ ] Stage Editor
- [ ] Player death juice
- [ ] Player death counter?
- [x] Basic art

## ToDo!

- [ ] Level select with actual selectable levels
- [ ] Nice art
- [ ] Audio
- [ ] Particles etc (also art)
- [ ] Add in level goal to editor
- [ ] Make actual levels
- [ ] Tweak movement controller
- [ ] Add moving objects to game + level editor
  - Pixel perfect may ruin everything
- [ ] Possibly add sub varient things so I have 2 dimentions to work with
- [ ] Editor middle mouse button copies
- [ ] Editor quick test
- [ ] Reset levels on death
- [ ] Triggered metal spike trap
  - In discord sub, make it so you step on them, 0.5 seconds later, spikes come up

## To Theory Craft

- Ground Types:
  - Similar to portal 2 gels
  - Ice ground: slippery
  - Bouncing ground: bouncy
  - Fire ground: Damages player over time
- Moustache/player appearance (floating hats, such as a crown, and googly eyes)
- Fancy stage loading (fancy animation for a stage being loaded/unloaded)
- A certain game with very low concurrent players has cool traps, but they are all triggered based on where the player is. This would make it hard to show in networking since players have their own stage, not a shared one

## Critical Bugs

- Player spawns a little above the ground and doesn't fall all the way down
  - This links to the player being able to jump before hitting the ground
    - Likely just need to lower the raycast padding
- Touching a spring doesn't reset players jump
  - This means jumping right before touching a spring, will turn off gravity and then launch the player
- Player can stand on phantom blocks without activating them
  - Only breaks this way when standing on the corner
  - This is because the player collider is the circle and is a bit smaller than player
  - It's this way to be forgiving on spikes though perhaps I should change the spike collider instead

## Bugs

- Editor Grid Negatives
  - When the mouse is on the tile left of the y axis, the grid position says it's at x = 0
  - Since the way it's calculated is (x / tile_size).trunc()
  - The x pos might be -14, then after that calc it should be -0.0, or 0.0 I guess?
  - This functionally just means you can place a tile at the edge of the grid (in bounds), by clicking out of bounds
- Cannot preload (with a stage load event) on build complete, will try building that stage immediately and fail
  - Need to test the build failed events at different points (1 stage in, 0 stages in etc)
  - Seems like it's not scrubbing the current stage
  - Also might not be taking the user back to stage select
  - NOTE this happens when preloading the next, non-existent stage, in the BuildComplete event
- Removing Groundable or Wallable component will not remove Grounded or TouchingWall component
  - Can add a system to check Changed<Groundedable> but for now, who cares?
  - Consider removing groundable and wallable all together?
  - Same thing with controllers Changed<JumpController>
  - NOTE: Changed isn't how you track removed components, that is something different
- Stupid texture bleeding comes back if the PC is trash (such as NU's laptop)
- Sawblade shooter projectiles don't despawn on stage exit

## Stage Mechanics

- [x] Key Blocks
  - Keys are placed around the stage
  - Collect the key to unlock the assigned locked blocks
  - Different keys will be assigned to different blocks in one stage
- [ ] Moving platforms
  - Horizontal or vertical
  - Moving platforms should also be able to contain spikes etc
    - The moving 'block' could just be a spike
    - Last project handled this by moving them all separately but in the same way
- [x] Phantom blocks
  - On touch (by anything?), they dissapear
- [ ] Ghost blocks
  - They look solid until the player is close
  - Once one in the group is revealed, all of the group is revealed
- [x] Interval blocks
  - They 'switch' on and off every x seconds
  - When on, they are just blocks, when off, they have dissapear
  - Can be triggered by timer or maybe even player events (button/switch?)
- [ ] Crushing blocks
  - A trigger subscriber that usually just stays on a timer
  - Can act as a door if it subscribes to a non timer trigger?
  - Will crush the player if they hit the bottom or top of it as well as touching ground/ceiling
- [x] Springs
- [ ] Teleporters
- [ ] Enemies that will act as springs when they die
- [ ] Block that produces spikes when the player steps on it, similar to crumbling block

## LevelReset

- Will likely need a component LevelResetter or similar that can reset the level when the player dies.

So we store an id with every stage object
They key objects contain a list of objectIds for their triggers

When creating the level, we go through the triggerables first and get a mapping <Id => EntityId>
Then we go through the rest and if they have triggers, we can set the correct entityId

## Steamworks Integration

- I could let the download system for stages use steamIds
- This would let players like stages, also using Steam as the auth
- Would not be portable to places other than steam but, can cross that bridge when we get there, since we will never get there

## Achievements

- Bounce off a single spring 10.000 times

## Background

- Have set weathers
  - Snow
  - Rain
  - Clear
- Have set landscapes
  - Mountains
  - Hills
  - Clear
- Weather and Landscape are both varients of the editor options
- The individual types are sub varients of those options
- Clicking with one will set it, and will randomise the seed
  - So if you want to set a new seed, just click again
- Do I refactory the tilemap stuff and then I can make the background fully tilemapped.
  - Then the clouds are just tiles etc

## Clean Editor Hotbar

- Only have the hotbar fade in when you change a varient
  - Then fade out after 0.5s of not changing varient
- It should appear where ever the cursor is

## Decorations

#### Grass

- [ ] Vines
- [ ] Bushes

#### Snow

- [ ] Snowman
- [ ] Icicles
- [ ] Some red tree that makes the colour pop

#### Desert

- [ ] Cacti

#### Caverns

- [ ] Gems (in SUB)
