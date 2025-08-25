#### Editor Refactor

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

**Quality of life**

- [x] Hold to place
- [ ] Better editor movement
  - Mouse3 or Space + click + drag should move the editor by that amount
- [ ] Zooming in and out moves you closer/further to/from your cursor location (like sprite editor)
- [ ] Copy key
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
