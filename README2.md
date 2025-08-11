#### ToDo Next

- [ ] Camera Bounds
- [ ] Think about changing colour themes
  - Doing this every level could be jarring
  - Even though the levels will have a Stage Complete screen in between

#### Demo Checklist

- [ ] Main menu UI
  - [ ] Background
  - [ ] Animated buttons
    - On hover, have a slider come over that reveals "Continue" or something
  - [ ] Save info (ie deaths etc)
- [ ] Icons
  - [ ] Deaths icon (maybe a skull)
  - [ ] Lives icon (maybe a heart)
- [ ] Death Animation
  - [ ] Player death
  - [ ] Saw death
- [ ] Key animations
  - [ ] Key
  - [ ] Lock blocks
- [ ] Audio
  - [ ] Death
  - [ ] Button selection
  - [ ] Keys
  - [ ] Stage complete?
  - [ ] Backing track?
- [ ] Stage Complete screen
- [ ] Run Dead screen
- [ ] More stages! :D

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
  - Use bevy's new error handling and create a logger resource

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

#### Gamepad

- Have a system that checks if a gamepad is being used, and if so add a Gamepad component to an entity.
- Have a component for GamepadSelectedEntity that is moved around and marks the entity (usually a button etc), that the gamepad is currently selecting
- Resource for selected is bad since it breaks with multiple gamepads (Selected(gamepad_id)) is better.
- Could have GamepadSelectable marker, and then have some basic rules for traversing, such as getting the parent node and moving along/down etc etc - I don't fucking know

**VIRTUAL CURSOR**

- It's annoying yes, but there's VERY mininal UI
- Can add movement speed (ie how far it goes when you press) and then also movement delay
  - This would then let it work in a grid with (movement_speed = 2, delay = 0.5) or something like that.
- Using a virtual cursor fucking sucks for everything, like when you end a level, and there's a button to continue and one to leave
  - However, could just have a _PressAnyButtonToContinue_ or something.
  - Then the cursor is only used on the pause menu
- There will be more UI for things like settings, and uploading/downloading stages

**Position Based**

- Have a resource containing a graph of selectable nodes
- This graph is populated using triggers/observors
- The resource has a currently selected node (Option)
- If currently selected is None, then it just doesn't traverse
  - When the pause menu comes up, it sets the currently selected etc
- Optionally just have it be position based such that when you press a direction, it goes through all Selectables and checks there positions and works it out on the fly
  - Would be better since you can press more unique directions but it's whatever
- Bevy has some support for this kind of UI movement https://docs.rs/bevy/latest/bevy/input_focus/index.html

#### Multiple Colliders

- If I want ccd on the player then I need a secondary collider as a sensor for other stuff
- I could have the ccd collider be the parent object to the actual Player entity but it's messy
- I could use colliderOf(Entity) but it currently doesn't work with CollidingEntities, which I use a lot.

#### Splatter! :D

**Saving to back wall**

- Give each temporary decal a _Splat_ component
- Have a system that periodically runs through them:
  - Gets their Handle<Image> and Rect to find pixels
  - Gets their position + rotation
  - Updates any background canvases as it needs
  - Deletes the Splat entity since it should now be reflected in the background (if there's a delay then I can sort that whenever)

**Splat textures**

- Have a fairly large texture (1280x1280)
- Only create splats for _radial_, _up_, _up diagonal_
- These splats can then be rotated by 90deg to create 8 compass directions, without rotating the pixels themselves
- With a texture this size, should be able to create a lot of splats
- For now can just hardcode Rect -> SplatData (ie type: radial/up/diagonal, size)
- _Greyscale_ texture containing time. Since it's low resolution I can have theh large time be multiple seconds. This way I can add _dripping_

#### Editor

**Quality of Life**

- Zooming in and out moves you closer/further to/from your cursor location (like sprite editor)
- Hold to place

#### Camera Footage

**How**

- Can put the border lines in the shader
- Can have a resource with some config for the margins etc
  - Since the border outlines and the UI will need to be some amount from the screen edge
  - Then can have a system with Changed<Settings> or whatever

**What**

- Very subtle scan lines
- Vignette to simulate lens or screen glow
- Chromatic aberration
- Noise/static: Random flicker or grain
- Barrel distortion: Slight curvature at screen edges
