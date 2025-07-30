// loading/unloading stages right now is stupid
// for one, I can't preload multiple stages
// 2, I can't reload a stage since when deleting old, it finds the diff between stageobjects of stageId A and B, and they're the same if it's just reloading the stage
// also would be cool to have a curtain on reload, and even a wait amount so I can have some animation?
// wait amount would be good for curtain also, wait until curtain is drawn, then reload etc

// possible fix is to have TearDownStageEvent that removes all stages, then another event for ReplaceStageEvent, this will call teardown and then call build/load??

// Should not ever really have 2 stages at the same time, so maybe BuildStage is the external event that will unload, and then build another on top

#### ToDo Next

- Stage data and loading layout
  - Effects so many things including game architecture and stage editor
- Get singleplayer working before doing multiplayer

#### Stage Data

- Directory style
- Any levels under "custom" should be found on the workshop under workshop/playertag/levelname
- Chapters just consist of pointers to levels, some are premade for you but you can make your own.

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
