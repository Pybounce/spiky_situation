# Audio

**Footsteps**

- Have ogg constantly looping on player entity
- Control it's volume
- When touching ground, the current velocity (x) decides the volume
- Then (de)accelerate to that volume depending on the current volume, far away is faster change etc.
- Since the player will have multiple sounds, makes sense to spawn them as their own entity and have a PlayerSoundController component that links to them?

**Stupid Scope Creep**

- Ok so this is a little crazy but what about ray traced audio (or semi raytraced)
  - I can simply say that every ground/trap/physical block is a cube it can collide with
  - Would be SO COOL to have some lock blocks closing a room, then you unlock them and hear all the sound come in
  - Would also be amazing to run in a large room and then down a corridoor and here the footsteps begin to echo etc
  - Only issue is that, since you can see saws in a closed off room, the player may feel like they should be able to hear them too.
