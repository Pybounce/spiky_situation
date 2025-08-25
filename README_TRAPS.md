#### Stage Mechanics

**Ideas**

- [ ] Teleporters
  - Anything that can be teleported is teleported when it collides
- [ ] Laser rotation
  - Additional of NS and NSEW laser varients
  - Separate the laser core from the block and only rotate it
  - Laser core will need many laser beams, probably want to have many lasers as children and keep it the current way it is
  - Will need to get the collision normal for the end particles

**Unsure Ideas**

- [ ] Homing Mine
  - Once a targettable is within x radius, it will fly towards them at increasing speed
  - This is actually too similar to an existing trap in that other platformer ffs
- [ ] Switch Blocks??
  - So top side you have a spring, bottom side spike or blade or nothing etc
  - Every x seconds, they switch (maybe it rotates then pauses etc)
- [ ] Explosive
  - Little explosive thing where player picks it up and it arms
  - Then they have to deposite it at a defuser or _something_ on time or they blow up
- [ ] Ghost objects
  - Only visible when in mid air OR maybe based on distance??
  - Could be an augment that is applied to many objects, same as mover
- [ ] Block Rotator
  - Single block that just rotates in increments of 90, with x seconds in between
  - Things will be attached to this block so one side can have a spring, the other spike/laser etc
  - Only issue is that rotation is usually done with animation, not just rotation - but if rotation works in general then this could as well not sure

**Static/Moving editor item textures**

- So the blade shooter static is a block, on the same layer as ground, but when it moves, that might be clunky, same with laser
- Rotating laser would also look clunky
- So do I separate the textures such that I have a _Core_ texture that is the thing that moves, and when the object has no _Mover_ on it, I just also add a _Backing_ texture??
  - Would the core have collision? probably not.
  - So with this you would end up with a split between moving textures and non moving ones
- Other platformers just have the tiles small enough so that a tile moving doesn't look strange.
- My game could probably get away with a moving tile, but then...how will it work if it moves into another ground tile etc, it would look strange.
