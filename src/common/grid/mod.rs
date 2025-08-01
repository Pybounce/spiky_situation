
use std::collections::{HashMap, VecDeque};

use bevy::prelude::*;

use crate::stage::stage_builder::stage_creator::TILE_SIZE;

use super::limited_stack::LimitedStack;

pub trait GridObject: Sized + Clone {
    fn draw(&self, grid: &mut Grid<Self>, commands: &mut Commands, pos: Vec3) -> Entity;
}

#[derive(Component)]
pub struct Grid<T: GridObject> {
    //TODO: Remove public access
    pub size: IVec2,
    pub contents: HashMap<IVec2, T>,
    pub event_groups: LimitedStack<GridEventGroup<T>>

}

impl<T: GridObject> Grid<T> {
    pub fn new(size: IVec2) -> Self {
        Self {
            size,
            contents: HashMap::new(),
            event_groups: LimitedStack::new(50)
        }
    }

    pub fn insert(&mut self, grid_object: T, grid_pos: IVec2) {
        let mut event_group = GridEventGroup::<T>::default();
        if let Some(old_grid_object) = self.contents.insert(grid_pos, grid_object.clone()) {
            event_group.events.push_front(GridEvent::Remove { grid_pos, grid_object: old_grid_object});
        }
        event_group.events.push_front(GridEvent::Remove { grid_pos, grid_object: grid_object});
        
        self.event_groups.push(event_group);
    }

    pub fn remove(&mut self, grid_pos: IVec2) {
        let mut event_group = GridEventGroup::<T>::default();
        if let Some(old_grid_object) = self.contents.remove(&grid_pos) {
            event_group.events.push_front(GridEvent::Remove { grid_pos, grid_object: old_grid_object});
        }
        
        self.event_groups.push(event_group);
    }


}

//------------------

// Data and Visual Grid Generics

// Have a system Query<Grid<T>, GridVisualisor<T>> where T : GridObject + VisualGridObject
// Then I can deal with visuals that way
// And also have a system Query<Grid<T>> where T : GridObject
// Dealing with only data, ignorinhg visuals



//------------------
// Issue with events
// You create 3 events fine
// Then you press undo and it moves the pointer to the second newest event
// Press undo again, moves pointer to 3rd newest
// Then create a new event, it would wipe any event above the pointer out
// This loss of data makes it hard for external places to make their way to most recent

pub struct GridEventGroup<T: GridObject> {
    pub events: VecDeque<GridEvent<T>>,
}

impl<T: GridObject> Default for GridEventGroup<T> {
    fn default() -> Self {
        Self { events: Default::default() }
    }
}

pub enum GridEvent<T: GridObject> {
    Insert { grid_pos: IVec2 },
    Remove { grid_pos: IVec2, grid_object: T },
    Resize { old_size: IVec2, new_size: IVec2 },
}


//--------------------------------------------------------//

// POSSIBLE COOKING
// What if the Grid component only held data, that's it
// Then the GridView component, displayed any data the Grid component was holding
// No idea how this works but fuck I like it
// If the Grid holds some events with an incrementing id, the GridView can keep track of which ones it has processed
// However, will the GridView use the events, given that the gridData will be out of date from the specific it looks at (unless it maintains it's own griddata?)
// The GridView can use events to work out what tiles are dirty, and then only ever display the most recent
// The GridView will need to read things like resize events too

//--------------------------------------------------------//


// what the fuck am I doing
// Does the grid just track entities and data, and completely replace them? even with an atlas change? it's just
//  BOOP and it deletes and spawns?
// HOW DOES THIS WORK FOR FUCK SAKE??


//Ok so what if the GridObject trait was implemented by an enum
// Then it could take in commands
// It could have an insert function (since remove will just delete the entity, can be done anywhere)
// The insert function takes in the data of it, and returns an entity
// So I raise events with the GridObject in them, then it runs through and calls insert(gridObject)
//      It takes the returned entity and inserts it into the hashmap, along with the gridObject
//      The hashmap becomes HashMap<GridPos, (Entity, GridObject)>
// As for the ground, the insert function could also take in the current HashMap, where it could match the gridobjects and work out the correct atlas
// I'm not 100% sure how it would then start creating events for the surrounding ground tiles etc
// Perhaps the insert function could also add events to the event group to happen after?


//--------------------------------------------------------//


// have a tile component
// have a tileTexture component
// in the tile component, have an entity pointing to the grid that owns the tile

// have a grid component

// each tile component will contain a get_tile_type() func and a same_type_as(TileType) to do equality checks MAYBE
// grid will have a get getneighbourbitmask(grid_pos) - which returns the 8 neighbours surrounding that position
    // 1 if they are the same type, 0 otherwise



//--------------------------------------------------------//

pub struct GridAtlases<T: GridObject> {
    pub texture_handles: HashMap<T, Handle<Image>>,
}

impl<T: GridObject> GridAtlases<T> {

}
#[derive(Clone)]
pub enum EditorGridObject {
    Ground { atlas_handle: Handle<Image>, rect: Rect },
    Saw,
    Spring
}


impl GridObject for EditorGridObject {
    fn draw(&self, grid: &mut Grid<Self>, commands: &mut Commands, pos: Vec3) -> Entity {
        match self {
            EditorGridObject::Ground { atlas_handle, rect } => {
                let a = grid.contents.get(&IVec2::default());
                match a {
                    Some(x) => {
                        match x {
                            EditorGridObject::Ground { atlas_handle, rect } => todo!(),
                            EditorGridObject::Saw => todo!(),
                            EditorGridObject::Spring => todo!(),
                        }
                    },
                    None => todo!(),
                }
                commands.spawn(SpriteBundle {
                    transform: Transform {
                        translation: pos, 
                        ..default()
                    },
                    texture: atlas_handle.clone(),
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                        rect: Some(rect.clone()),
                        ..default()
                    },
                    ..default()
                }).id()
            },
            EditorGridObject::Saw => todo!(),
            EditorGridObject::Spring => todo!(),
        }
    }
}