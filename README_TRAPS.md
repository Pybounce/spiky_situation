#### Stage Mechanics

**Ideas**

- [ ] Pressure Spikes
  - Spikes that rise x seconds after you step on them.
  - Could possibly use Triggers since they stay forever.
  - To reset, maybe have another component that can reset a triggerId after x seconds? maybe delayed event? who knows.
- [ ] Lasers
  - Lasers will have 2 parts, the backing ground tile and then the laser core
  - There will be laser varients that just increase the amount of lasers in the core (1, 2, 3, 4)
  - Then to make a rotating laser, just apply the rotator augment to it, simple.
- [ ] Teleporters
  - Anything that can be teleported is teleported when it collides

**Unsure Ideas**

- [ ] Homing Mine
  - Once a targettable is within x radius, it will fly towards them at increasing speed
  - This is actually too similar to an existing trap in that other platformer ffs
- [ ] Switch Blocks??
  - So top side you have a spring, bottom side spike or blade or nothing etc
  - Every x seconds, they switch (maybe it rotates then pauses etc)
- [ ] Explosive
  - What if the player can pick up an explosive and then place it down
  - When they pick it up the timer starts
  - Then can have blast doors etc
  - Kind of worse version of just leading a missile though damn
- [ ] Ghost objects
  - Only visible when in mid air OR maybe based on distance??
  - Could be an augment that is applied to many objects, same as mover
- [ ] Block Rotator
  - Single block that just rotates in increments of 90, with x seconds in between
  - Things will be attached to this block so one side can have a spring, the other spike/laser etc
  - Only issue is that rotation is usually done with animation, not just rotation - but if rotation works in general then this could as well not sure

**Lasers**

- Basic setup is to have a raycast find the start/end, and then slap a rect with that length, rotated correctly
- Can either use a tiled texture (which would be procedurally making the mesh since the uvs need to tile it, I think.), or make a shader
  - Possibly some middle ground where I define the mesh just as a normal quad, and tell it that the x repeats N times, so then the shader just takes the uv and divides uv.x by N??
- Ignore blood splats for now
- Collider will just take the same shape as the quad
- _Issue_: Not sure how the static vs moving laser will be, more on this below

**Static/Moving editor item textures**

- So the blade shooter static is a block, on the same layer as ground, but when it moves, that might be clunky, same with laser
- Rotating laser would also look clunky
- So do I separate the textures such that I have a _Core_ texture that is the thing that moves, and when the object has no _Mover_ on it, I just also add a _Backing_ texture??
  - Would the core have collision? probably not.
  - So with this you would end up with a split between moving textures and non moving ones
- Other platformers just have the tiles small enough so that a tile moving doesn't look strange.
- My game could probably get away with a moving tile, but then...how will it work if it moves into another ground tile etc, it would look strange.
