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
- Can have mover varients that change speed
- How will I make things like springs move?
  - Would be good if they did, but needs to look right
  - Can they just move on their own or do they have to be connected to a moving platform

**Quality of life**

- [ ] Hold to place
- [ ] Better editor movement
  - Mouse3 or Space + click + drag should move the editor by that amount
- [ ] Zooming in and out moves you closer/further to/from your cursor location (like sprite editor)
- [ ] Copy key
  - Probably Q but will be settings to rebind
  - Sets your current item to the item your mouse is hovering over

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

- Something you can place on different stage objects, but not all
- Mover is an example of an augment
- Another example might be Ghost which makes it only visible based on distance or touching ground etc
