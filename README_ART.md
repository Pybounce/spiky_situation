#### Art

**General**

- Dilapidated egyption temple theme

**Background**

- [ ] Embers flying across
  - Very small amount to add ambience, like snow.
- [ ] Possible backwall
  - Have a fairly simple back wall that is just a colour
  - Then add in the ability to have a broken brick backwall to give structure to floating platforms etc
- [ ] Torches
  - Can be placed on the backwall to provide lighting than flickers like fire

**Stretch Goals**

- [ ] Reflections
  - Sprites are given a reflective map which contains how far up the reflection is taken from
  - Then I can have the player and objects reflected on things like gold ground or whatever
  - Maybe even spikes/saws too!
- [ ] Lighting!
  - Can easily just add a warm transparent texture over the player and over torches and call it a day
  - Maybe look in to normal maps for sprites

**Dynamic Environment**

- [ ] Skulls and rocks littering the ground
  - Can be pushed by other objects and moved
  - Landing sends out a shockwave that causes things to jump up a little
    - Basically just the inverse y velocity of the landing
    - Not all things will have this since we don't want a knock on effect
  - When things hit a sawblade they break into dust or whatever
- [ ] Vines hang from the ceiling
  - When things push them they move like a chain
  - Saw blades/lasers should cut the vine, destroying the bottom half?

**Player Animations**

- Jump
- Fall
- Run
- Wall grip
- Wall jump
- Dance

_Player States_

- On wall (up/down)
- On ground
- Air up
- Air down
- Idle

```rs
pub struct StateAnimations {
  enter: Vec<Rect>,
  constant: Vec<Rect>
}

pub struct AnimationController {
  current_state: AnimationState,
  doing_enter: bool,
  state_animations: HashMap<state_id: u32, StateAnimations>
}

// then I need an animationstatemask trait that can convert an enum to a u32 or something.
// then we have an AnimationState(u32)


// I COULD even have the SpriteAnimator keep track of the index, and then grab the rect from the AnimationController, instead of copying the Vec over every time.

pub trait AnimationStateSet { ... }
pub enum PlayerAnimationStates { ... }  // this will impl AnimationStateSet

pub struct AnimationState(u32);
impl AnimationState {
  pub set_state(&mut self, state: AnimationStateSet) {
    self.0 = state.as_u32();
  }
}

pub fn system(query: Query<(), Changed<AnimationState>>)
```
