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
- [ ] Baked and Dynamic
  - For the most part, the lighting won't change
  - Therefore you could just bake the nice raytraced light with static occluders, and then add simple lighting for dynamic ones
- [ ] Light Frame Budget
  - Create another lightmap and switch between the 2
  - Instead of doing all rays, split them across 4 frames and do first quater, then second etc
    - _Reducing race conditions_ could be achieved here by splitting the rays into equal directions still. So instead of doing the first quater, you do 1 every 4.
    - Only issue might be that the memory access becomes more sparse (hence why it also reduces race conditions)
    - _Reducing flickering_ would happen if we added yet another lightmap
      - 1st lightmap is the new target lightmap
      - 2nd lightmap is the old lightmap (we will lerp from old to new)
      - 3rd lightmap is the one under construction
      - Biggest issue here (aside from memory), is that if we take 4 frames to generate a lightmap, then the user wouldn't see it for 8 since we take another 4 to lerp to it
- [ ] Reducing occluder resolution
  - Occluder maps don't need great resolution, could probably reduce to 800x800

**Occluder Optimisations**

- [ ] Only load in nearby occluders
  - This would mean knowing the max distance a light can travel
  - Then do a distance check between the camera and the occluder (+ occluder radius)
  - Only send over occluders that are inside that range.
- [ ] Dynamic vs Static occluders
  - Most occluders will not move
- [ ] Merging occluders
  - Can merge static square/rect occluders but is fairly annoying to do.
- [ ] Multi-frame lighting
  - Lighting doesn't need to be done every frame and can likely be split over multiple frames
  - Would need all occluders in before we can start ray tracing
  - Maybe take 4 frames for occluders, 2 for ray tracing etc
- [ ] Switch back to processing occluders on cpu
  - Most processing is gpu bound now
  - Could just process the occluders in a parallel thread on the cpu instead
- [ ] _Baked vs Realtime_
  - Can have a setting that does realtime and another setting that just does baked
  - Baked will only include static occluders (maybe also trigger when key blocks are activated?)
    - Key blocks complicate things since I might make them activate one by one and that might be too much of a performance hit not sure
  - Maybe I should just do this...hmm
- [ ] Spatial Partitioning
  - Can partition the occluders so sdf calcs are far lower per cell.

**Blur + ContructionLightmap**

- Can have 2 lightmaps, the output and the construction
- Takes x frames to build the construction
- On the final pass once it's built, the blur shader will perform 2 passes and write the blurred output to the output lightmap
- Output can then be used wherever

**Emission**

- Can probably make each cell in the emissionmap 16x16 (so total just 100x100)
  - Since emission light is very basic and just needs to go across the room
  - Then that is applied to lighting and we blur
  - Otherwise the amount of passes would be far too many given that it's 1 pass per 1 cell movement
    - So even with 16x16 cells, we would probably need 8 passes
- Can emission be done separately? (maybe)
  - It can be updated over multiple frames, same as the occluders.
  - _however_ since it's cells are larger, differences will be more apparent

**Guassian Blur**

- Can do horizontal/vertical separately by having 2 lightmaps
- Original/output lightmap, intermidiary lightmap
- Take in original and apply horizontal blur, write to intermediary
- Take in intermediary and apply vertical blur, write to output/original again
- Since horizontal and vertical should be the same, when we write to intermediary, we can probably just rotate it by 90 and store it at a rotation
  - Then when we apply vertical, we actually just apply horizontally but to this other buffer (so just 2 passes of the same shader with different buffers)
  - Once again the output lightmap will be rotated 90 (ie flipped)
  - We could therefore read it flipped to ensure the output orientation matches input
