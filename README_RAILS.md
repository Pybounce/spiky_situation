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
- _Start by_ creating the data structure to hold the rails, with the correct API, maybe even some tests
  - Will need a cell_index HashMap<IVec2, track_id>

**Moveables**

- Can simply have Moveable(track_id: u32)
- So when you place it, you check to see if there's a track there, if so, assign the track_id

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
- This way they can be placed on moving ground blocks too! - Even platforms
- _NOTE_ If I am messing around with hierarchies, I will need to add an ExtendedColliderOf marker component for my new physics stuff.
- _Question_ How will I assign the stickable.
  - Moveables will be given a track_id
  - Stickables could also just be given Moveable(track_id) I suppose
  - Not sure when it would happen, ie when I create the stage or when I save the stage asset.
    - When I create the stage would be fairly easy
    - Check below me to see if an object exists, is it a moveable and static ground, if so, take it's track_id

**Graph-Based Rails**

- Have a RailGraph:
  - nodes: Vec<(pos: IVec2, in_edge: Option<u32>, out_edge: Option<u32>)>
  - edges: Vec(usize, usize)
- When you click and drag, it ensures the 2 points are on one axis
- Validity:
  - If either node exists (node being the start/end), does it already have 2 connections
    - If yes then we cannot add another currently.
- Upon passing validation...
- Create nodes that need to be created
- Add edge between the 2 nodes.

**Edge-Based Graph??**

- RailGraph:
  - edges: HashMap<u32, (start: IVec2, end: IVec2, in_edge: u32, out_edge: u32)>
- So here, the start and end are always axis aligned
- The added benefit to having no nodes is that the position between edges can be whatever the fuck I want.
  - So if a rail goes through a teleporter, then it simply enters another edge that is connected to it, but in a totally different position
- Only teleporter drawback is moving teleporters
