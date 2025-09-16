#### Editor Refactor

**Current Issues**

- [ ] Moveable Clone
  - If the EditorItem contains all it's info then it also will contain the full vec path
  - So when I copy one, I wil need to clear that path manually, also I can't copy, must be clone, which is annoying
  - Could just impl Copy myself and leave that part out buttt it's risky.
- [ ] Moveable Tracks vs Augment
  - Tracks would allow me to draw one track and place many things on it
  - Augment would mean I have to redo the track for every item on it, every time.

**Must Haves**

- [ ] Change stage size (within size limits)
- [ ] Validation check (that can be used on other stages, just pass the stage in)
- [ ] Easy way to apply movement to any moveable
- [ ] Easy way to change speed of projectile firers or rotators?
- [ ] Test runs
  - Allow the player to spawn in
  - On completion of level OR KEYPRESS, it returns to the editor
  - It should not update any save file stuff

**Moveables**

- What if the mover was it's own editor item
- When you hover over a moveable (saw, moving platform, key etc), it gets some highlight
- First click starts mover edit
- All other clicks in unique cells mark that position as the next
- Clicks in the same cell twice in a row marks the end of the route
- Can have mover variants that change speed
- How will I make things like springs move?
  - Would be good if they did, but needs to look right
  - Can they just move on their own or do they have to be connected to a moving platform
- _Mover highlights_
  - When you select the mover _tool_, lines appear where everything will move
  - When you hover over a moveable that has a path, the path appears in another colour than the rest and it's z is brought all the way forward so you can see it clearly
  - COULD have the lines be offset so if 2 paths overlap, the lines are above/below each other but that's tricky so fuck that
- _Issue: Do things attach to a mover_
  - If theres a spike or a spring on top etc
  - If I child it, rapier will break everything, so will need to switch to avian

**Mover Implementation**

- EditorController holds enum for idk EditorUseable?
- EditorUseable is an enum { Item(EditorItem), Augment(EditorAugment) }
- Then EditorItem needs a method that says whether or not it accepts the augment
- Then we have separate systems dealing with each augment
  - So moveable will need something that tracks if we've selected an item to apply it ot
  - It needs to track all the positions
  - It needs to apply them and clear the currently selected item
- _maybe value augments are their own enum since they will all work the same way, unlike mover which requires specialised logic_

**Quality of life**

- [x] Hold to place
- [x] Better editor movement
  - Mouse3 or Space + click + drag should move the editor by that amount
- [ ] Zooming in and out moves you closer/further to/from your cursor location (like sprite editor)
- [x] Copy key
  - Probably Q but will be settings to rebind
  - Sets your current item to the item your mouse is hovering over
- [ ] Draggable areas
  - Click and drag will basically just apply whatever you have to each cell in the rect you have outlined
  - So if you're holding a spike, it will try to place it in each cell
  - If you're holding an augment it will try to apply it to all cells
  - Some things cannot use this feature, such as movement augment, since it's not a value or placeable
- [ ] Augments
  - When holding an augment, highlight editor items that can use it
  - When holding an augment, show debug info for editor items using it (maybe ONLY on hover of that item)
- [ ] Dynamic stage size
  - If you place a tile outside of the stage size, it extends to fit the tile (to a max)

**Offset Grid**

- _make more buildables before deciding as currently only saws use it_
- Instead of having half saws and such, could make an offset grid
- Holding KEY will cause the editor to snap to (-0.5, -0.5) of the current tile
- Can just have the buildables this applies to contain a bool for is_offset
- Will need to work with the Mover etc

**Grid Layers**

- _again, leave for later_
- Since the grid is a hashmap, just add a z-cord to the hash key
- Some enforcement needed for things that fire projectiles?
- But really it would just allow for some nicer designed levels decorative stuff

**Augments**

- Ok so what if I had different augments that can be applied to existing editor items (different ones depending on augment)
- _Value Augments_
  - These will very likely be _variants_
    - So movementSpeed augment will be a variant of the movement augment
  - Can have value augments that just increase or decrease the augment value from a range of 0-1
  - Instead of clicking the editor item to increase/decrease, make it so you change the augment value of the augment you're holding, and then click to apply that value
    - Otherwise doing mass changes will be a pain, since you'd have to go to each one to click and hold etc
- _Rotator_
  - Applying more of this will increase rotation cone
  - To change start direction you can rotate the editor item (but it only does 90deg turns)
  - If you make a full circle (ie value of 1), then the rotation doesn't turn around on itself but just goes around
  - When less than a full circle, rewinds and repeats
- _Move Speed_
  - Applying this will change the translation speed of the editor item
- _Rotation Speed_
  - Applying this will change the rotation speed of the editor item
- _Mover_
  - Wouldn't be a value augment
  - More info on it above.
- _Speed_
  - Ok so this is a generic speed augment that will apply differently to different things
  - For example with blade shooters, it would increase the firerate, but for interval blocks it would decrease time between switches. Maybe for phantom blocks and pressure spikes it decreases there time too.
  - **Issue** is, some things have multiple factors
    - Blade shooter would have projectile speed and fire rate
    - Could do what that chicken one does where the stage has an overall projectile speed resource, instead of per entity

**Things that I do not know how to do**

- How do I define different behaviours for if you're holding editor items or augments etc
  - So if you're holding an editor item it should place it, but an augment it should apply it to the editor item in that cell
- How do I work out what can and cannot be applied as a bulk rect
  - Probably will be solved by the above

**Augment Implementation**

```rs
pub enum EditorTool {
  ValueAugment((augment: Augment, value: f32, min_val: f32, max_val: f32))
}

// augment is simply a marker, they all have the same data

pub enum Augment {
  MoveSpeed,
  FireRate,
  ProjectileSpeed,
  Rotation??
}
```

Having the value split from the Augment enum will let me do things like

```rs
pub fn value_augment_ui() {
   ... blah ...
   if let ValueAugment((_augment, val)) = controller.tool {
      augment_progress_ui.value = val;
   }
  // Instead of...
     if let ValueAugment(augment) = controller.tool {
      let val = match augment {
        MoveSpeed(v) => v,
        ProjectileSpeed(v) => v
      };
      augment_progress_ui.value = val;
   }
   // Since they should ALL have a value
   // When we switch or change value, can then apply min/max vals also.
}
```

_Current Issue_

- will need to have banding levels for the value
  - So movement speed should only have Slow, Medium, Fast etc (or just 1, 2, 3)

_Controls_

- Currently can just be 3 to switch to value augments
- A/D to switch between augment type
- W/S to increase/decrease augment value

**Selection/Move Tool**

- Highlight an area by dragging
- Should be able to:
  - Move area
  - Copy/Paste/Cut area
  - Undo area movements
  - Edit only selected area
- _The edit tool should simply update the selected area in the controller, not itself_
- Can then click the area and drag to move all things inside it
- For rails, this would disconnect them if the selection splits through the middle
- Can copy, cut and paste also
- _Issue_: How the fuck do I even implement this
  - I need to drag stuff, but check it can go there before placing it
  - But if they let go, it should just stay in the incorrect place and let them drag again right.
  - Placing when it can?
  - Maybe I invent blueprints for this, since I will effectively need copy paste functionality also

**Selection Tool Idea 1**

```rs
pub enum EditorTool {
  ...
  SelectionTool(area: Option<Rect>, offset: IVec2)
}
```

- When I select an area it populates the area rect
- When I drag to move, it updates the offset
- When I end the drag, no change.
- When I end the selection, it then moves the original area over the area + offset
- When I COPY, it needs to save that template somewhere.
- When I PASTE, it should end the current selection, but then where does it get the template and origin from to apply offset, since it no longer MOVES, and instead only places.

**Selection Tool Idea 2**

```rs
pub enum EditorTool {
  ...
  SelectionTool(area: Option<Template>, cell: IVec2)
}
pub struct Template(Vec<EditorItem>, Vec<Rail>)
```

- When I select an area, it copies all that area data into a template and then deletes the original area. It also sets the cell to the cell of the original area.
- When I end selection, it overwrites anything in that area with the template
  - _What if it cannot overwrite_: Then it simply displays an error message and does not allow the player to end selection. They can undo selection which places the template back where it was originally?
- If I COPY, it simply just stores the template in some other COPY variable
- If I PASTE, it places the current selection if one exists, and then sets the current template to the copied one, and the cell to the original cell.
  - _What if it cannot place current selection_: It doesn't let you paste?
- _Possible solution to the placement rules_: Have it so they are only enforced when you try to publish, not save or edit. So you can break as many placement rules as you like during edit but not publish
  - _HOWEVER_ This does not work if you try to place out of bounds. Or maybe it does? Maybe I let them place out of bounds but not publish.

**Selection Tool Idea 3**

- When I select an area it just saves the Rect
- When I drag to move, it deletes the old area, and then overwrites the new area with the copied entities
- When I end the selection, it just deletes the Rect
- When I copy an area
