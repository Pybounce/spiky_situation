#### ToDo Next

- [ ] Save system
  - Will need to be able to save/load stages from assets and also from custom.
  - Will need to get player save files from somewhere other than assets too.

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
