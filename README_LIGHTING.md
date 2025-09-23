# Lighting

**General**

- [ ] Multiple coloured lights
  - Diamond (or whatever is at the end) should be a coloured light
  - Torches will be yellow/orange
  - Player could be a light colour
  - Fireball projectiles could be blue or purple flame?
  - Water maybe has a cyan light?
  - Lava maybe a deep red light
  - Can have other collectibles such as coins that glow gold or emeralds that glow green

### Compute Ray Tracing

**General**

- Send over a buffer of (DynGround, StatGround, NoGround)
- Send over a buffer of lights (position, intensity, colour, falloff)
- Have the compute shader write to a texture with the colour/intensity etc
- Have a post processing effect apply that texture to the screen

**Occluders**

- Can have StaticOccluder and DynamicOccluder
- The buffer for ground/notground we send over actually has (StaticGround, Ground, NotGround)
- On init, we iterate every occluder and fill it in
- On update, we first remove all the Ground. Then we iterate all the dynamic occluders and mark them as ground.

**Screen Space Updates?**

- Could possibly send over a rect for the screenspace of the whole map
- Could then not raytrace lights that are too far
- Could then also only update the part of the texture that the screen covers
- To be clear, we would still be raytracing using all occluders outside of screenspace

_Better Data Input_

- Instead of sending over buffers full of each pixel and if it's ground, just send over SDFs and grid pos
- Then we're still writing to a large texture for light data, but the input data/texture is very simple
- _Is this actually better?_
  - Yes it sends less but realistically we will only be sending diffs
  - So still less but not nearly as bad as it was.
  - Also harder to tell if the grid pos a ray is on is a wall since it's not indexed via the buffer

**RTL Optimisations**

- [ ] Occlusion Diffs
  - Could split Dynamic and Static Occluders such that static ones go in the grid buffer, but dynamic ones go in a separate, far smaller buffer
    - This smaller buffer could just contain OccluderRect(w, h, x, y) and therefore raytracing will be less performant but updating the buffer is fast
  - Good idea to check if I can update parts of the buffer at a time and if so maybe can just keep them all in one not sure
