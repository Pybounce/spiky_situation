#### ToDo Next

- [ ] Camera Bounds
- [ ] Think about changing colour themes
  - Doing this every level could be jarring
  - Even though the levels will have a Stage Complete screen in between

#### Collectables

- Components for MoveOnCollide and SoundOnCollide and FlashOnCollide
- These can all be used so collecting keys and hearts etc all do something when collected
- KillOnCollide(delay) is a good one too
- Can have a Resource for DelayedInserter that takes the method I
  Insert(Entity, T (Component, Copy), Duration)
- This resource would keep track of the time passed and insert them (tryinsert?)
- This way I can have delayed death and animation and flashing etc etc etc

###### Insert/Remove OnCollide

- Since this is common, things being animated/destroyed/whatever on collide, could have a resource for it
- Maybe too much?

###### Animation Insert/Remove

- Like could have the animatorInserter take in components to insert/remove at timestamps during an animation
- Then a system will look for <(Animator, AnimatorInserter)> and see if it should add/remove stuff that way
- Can be used for destroying it, adding/removing collision for things like the phantom blocks
- Potentially used for projectile firing!!!
  - WAIT THIS IS GREAT
  - Then can have an animation constantly playing, and at the end, fire a projectile by adding a component or SOMETHING idk
  - Wouldn't want to constantly add/remove since it would change topology too much
  - Could have ProjectileShooter, and then another component named Shoot that is SparseStorage
- HOWEVER: If I have 2 components on an entity with the same Timer stuff and I tick them both, then really, it should be fine. It should stay in sync and even if not, stages are short lived

#### ToDo

- Stage data layout
  - Directory based? ie chapter = ['stages/stage_1', 'stages/stage_2', 'stages/custom/my_stages/spike_death']
  - Uses a lot more bytes
  - Simple to load since you have the filepath
- Stage sharing
  - Could use steam workshop but it would only ever work for steam. But still come on...
- Stage loading
  - Ideally stage loading would be done from anywhere at any time with an event
- Stage resetting
  - Will need to reset the stage when the player dies (keys etc)
- Backgrounds
  - Backgrounds likely consist of a repeating pattern slightly moving, with colours
  - So perhaps just have a 128x128 texture for the pattern, then a list of those to pick from?
- Colours
  - Figure out the size of the colour pallete
  - Make the shader
- Deaths scoreboard
  - Will likely rely on knowing how chapters are going to work?
  - Also relies on knowing how multiplayer will work (ie playerIds)

#### Bugs

- Dashing into a wall forces you into it slightly
- Dashing when just above ground causes you to hover slightly
  - Since the raycast doesn't then correctly set you (LIKELY WOULD SOLVE THE ABOVE BUG TOO)
- Saw projectiles break when hitting keys.
  - Probably need to add some more collision layers etc
- Checkpoints do nothing
  - I changed it so when you die the whole level is rebuilt, so now checkpoints don't do a thing.

#### Gamemodes

- [ ] Life Based
  - Given a certain amount of lives at the start
  - See how many stages you can beat before you lose them all
  - **IDEA** --> If you beat the par time on a stage, you get a life
  - _QoL_ --> Timer only starts when you leave spawn tile? / Or can reset the map without loss of life if you're within 5 tiles of spawn

#### Refactors

- [ ] Gamemodes
- [ ] Save system
- [ ] Error handling

#### Art

GENERAL

- Potentially need to increase size to 32x32 for better animations and such?
- Pixel art
- White flashing when collecting/dying etc

KEY + LOCKS

- Key moves up and down hovering
- Collecting could have it rise up flashing white, then exlpode?
- Locks would need to break and then the block explodes or something?
- The lock could also explode so it says within the same tile - unlocks then explode
- Locks unlock at the same time?? Or slowly over time?
