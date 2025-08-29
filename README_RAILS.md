#### Rails! :D

**Placement of Rails**

- A RailTool(Option<Vec2>) will be added
- OnClick:
  - Is None:
    - Set vec2 to Some(current_cell)
  - Is Some:
    - current_cell == vec2:
      - Set vec2 to None
    - current_cell != vec2:
      - Try_place_segment from (vec2 -> current_cell)
      - Set vec2 to None
- try_place_segment(start: vec2, end: vec2):
  - Check data structure to see if any ends == start or end
  - If they do, extend that rail
  - Otherwise, create a new rail
- OnDelete:
  - Find any rails that use this cell
  - Split them into 2 rails (Or if it's a looped rail, just change the head/tail)
- As for the mover component, will just need a bool for ping_pong
  - If ping_pong == false, then on completion, just set it's translation back to the start (t == 0)

**Ground Stickables ---- awful name**

- Basically some objects are required to be placed on static ground
- These objects are:
  - Spring
  - Spike
  - Halfsaw
  - Pressure Spikes
- Static ground is ground that is never not ground...
  - So it includes:
    - Ground
    - Laser Block
    - Saw Shooter BLock
    - Platforms (when they're added)
  - So it does NOT include:
    - KeyBlocks
    - PhantomBlocks
    - IntervalBlocks
- So when a ground-stickable block is placed, it should be the child of the ground block it's on
- This way they can be placed on moving ground blocks too! - Even platforms -_NOTE_ If I am messing around with hierarchies, I will need to add an ExtendedColliderOf marker component for my new physics stuff.
