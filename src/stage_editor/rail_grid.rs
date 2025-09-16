
use bevy::{platform::collections::HashMap, prelude::*};

use crate::common::math_ex::axis_aligned_intersect;

#[derive(Default, Debug)]
pub struct RailGrid {
    current_id: u32,
    rails: HashMap<u32, Rail>,
    dirty_rails: Vec<u32>   // would also mark new rails as dirty. A system should then go through the rails to get cells, then find editoritems based on the rail cell. Hmm...but if I remove a rail, then I'll need to check every railid on the editoritems so, yeah not sure.
    // alternate to dirty rails would be to just get all the points those rails use and mark those as dirty. should be fairly easy to get outer points, it's just getting the full list that is annoying, doable though.
}

impl RailGrid {
    pub fn from_rails(rails: &HashMap<u32, Vec<IVec2>>) -> Self {
        return Self {
            current_id: rails.iter().fold(0, |max_id, (&id, _)| max_id.max(id)) + 1,
            rails: HashMap::<u32, Rail>::from_iter(rails.iter().map(|(id, points)| (*id, Rail { points: points.clone() }))),
            dirty_rails: vec![],
        };
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Rail {
    points: Vec<IVec2>
}

pub enum RailMergeOrder {
    HH,
    HT,
    TH,
    TT
}

impl Rail {

    /// Removes any points that aren't on corners (likely as a result from merging another rail in)
    pub fn compress(&mut self) {
        let mut new_points: Vec<IVec2> = vec![self.points[0]];
        for window in self.points.windows(3) {
            let dir0 = (window[1] - window[0]).clamp(IVec2::new(-1, -1), IVec2::new(1, 1));
            let dir1 = (window[2] - window[1]).clamp(IVec2::new(-1, -1), IVec2::new(1, 1));

            if dir0 != dir1 {
                new_points.push(window[1]);
            }
        } 
        new_points.push(*self.points.last().unwrap());
        self.points = new_points;
    }

    /// Gets the total length of the rail, in number of cells covered
    pub fn length(&self) -> u32 {
        let mut len = IVec2::ZERO;
        for line in self.points.windows(2) {
            len += line[0].clamp(IVec2::new(-1, -1), IVec2::new(1, 1));
        }

        return (len.x + len.y) as u32;
    }

    pub fn try_new(start_cell: IVec2, end_cell: IVec2) -> Option<Self> {
        if start_cell == end_cell { return None; }
        if !(start_cell.x == end_cell.x || start_cell.y == end_cell.y) { return None; } //Only accepts straight lines now, no corners. 
        
        let points = (start_cell, start_cell + (IVec2::X * (end_cell - start_cell)), end_cell);
        
        return match points.0 == points.1 || points.1 == points.2 {
            true => Self { points: vec![points.0, points.2] },
            false => Self { points: vec![points.0, points.1, points.2] },
        }.into();
    }

    pub fn tail(&self) -> IVec2 {
        return self.points[0];
    }

    pub fn head(&self) -> IVec2 {
        return self.points[self.points.len() - 1];
    }

    pub fn is_head_or_tail(&self, cell: IVec2) -> bool {
        return self.is_head(cell) || self.is_tail(cell);
    }

    pub fn is_head(&self, cell: IVec2) -> bool {
        return self.head() == cell;
    }

    pub fn is_tail(&self, cell: IVec2) -> bool {
        return self.tail() == cell;
    }

    pub fn is_body(&self, cell: IVec2) -> bool {
        if self.is_head(cell) || self.is_tail(cell) { return false; }
        
        for rail_line in self.points.windows(2) {
            if axis_aligned_intersect(cell, cell, rail_line[0], rail_line[1]) { return true; }
        }

        return false;
    }

    pub fn is_on(&self, cell: IVec2) -> bool {
        return self.is_head(cell) || self.is_tail(cell) || self.is_body(cell);
    }

    /// If cell is on the rail, returns the amount of waypoints it has passed </br>
    /// So if you want the next waypoint of non-reversed, just add 1. </br>
    /// Reversed next waypoint will be this one.
    pub fn waypoint_index(&self, cell: IVec2) -> Option<usize> {
        for (window_index, rail_line) in self.points.windows(2).enumerate() {
            if axis_aligned_intersect(cell, cell, rail_line[0], rail_line[1]) { 
                return Some(window_index);
            }
        }
        return None;
    }

    pub fn try_merge(&mut self, rail: &mut Rail) -> bool {

        if let Some(merge_order) = self.can_merge(rail) {
            match merge_order {
                RailMergeOrder::HH => { rail.reverse(); },
                RailMergeOrder::HT => { },
                RailMergeOrder::TH => { self.reverse(); rail.reverse(); },
                RailMergeOrder::TT => { self.reverse(); },
            }
            rail.points.remove(0);
            
            self.points.append(&mut rail.points);

            match merge_order {
                RailMergeOrder::HH => (),
                RailMergeOrder::HT => (),
                RailMergeOrder::TH => self.reverse(),
                RailMergeOrder::TT => self.reverse(),
            };

            self.compress();
            return true;
        }

        return false;
    }

    pub fn can_merge(&self, rail: &Rail) -> Option<RailMergeOrder> {
        if self.head() == self.tail() { return None; }  // This rail is a loop
        if self.head() == rail.head() { return RailMergeOrder::HH.into(); }
        if self.head() == rail.tail() { return RailMergeOrder::HT.into(); }
        if self.tail() == rail.head() { return RailMergeOrder::TH.into(); }
        if self.tail() == rail.tail() { return RailMergeOrder::TT.into(); }
        return None;
    }

    pub fn reverse(&mut self) {
        self.points.reverse();
    }

    pub fn valid_with(&self, rail: &Rail) -> bool {
        for rail_line in rail.points.windows(2) {
            let diff = (rail_line[1] - rail_line[0]).clamp(IVec2::new(-1, -1), IVec2::new(1, 1));
            let mut current_cell = rail_line[0];

            if (rail.is_body(current_cell) && self.is_on(current_cell)) || self.is_body(current_cell) || (self.is_head_or_tail(current_cell) && self.can_merge(rail).is_none()) { return false; }
            while current_cell != rail_line[1] {
                current_cell += diff;
                if (rail.is_body(current_cell) && self.is_on(current_cell)) || self.is_body(current_cell) || (self.is_head_or_tail(current_cell) && self.can_merge(rail).is_none()) { return false; }
            }
        }

        return true;
    }


    pub fn iter_cells(&self) -> impl Iterator<Item = IVec2> + '_ {
        return self.points
            .windows(2).enumerate()
            .flat_map(|(wi, w)| {
                let start = w[0];
                let end = w[1];
                let step = (end - start).clamp(IVec2::new(-1, -1), IVec2::new(1, 1));
                let len = (end - start).abs().max_element();
                ((wi as i32).min(1)..=len).map(move |i| start + step * i)
        });
    }
    
    pub fn iter_points(&self) -> impl Iterator<Item = &IVec2> + '_ {
        return self.points.iter();
    }

}

impl RailGrid {
    pub fn iter_rails(&self) -> impl Iterator<Item = (&u32, &Rail)> {
        return self.rails.iter();
    }
    pub fn try_add_from_rail(&mut self, mut new_rail: Rail) -> bool {
        if self.valid_rail(&new_rail) == false { return false; }

        let mut mergable_rail_ids: Vec<u32> = vec![];

        for (rail_id, rail) in self.rails.iter_mut() {
            if rail.can_merge(&new_rail).is_some() {
                mergable_rail_ids.push(*rail_id);
            }
        }

        if mergable_rail_ids.len() > 0 {
            let Some(rail) = self.rails.get_mut(&mergable_rail_ids[0]) else { return false };
            rail.try_merge(&mut new_rail);

            for rail_id in mergable_rail_ids.iter().skip(1) {
                let [rail0, rail1] = self.rails.get_many_mut([&mergable_rail_ids[0], rail_id]);
                // always merge into rail0
                let Some(rail0) = rail0 else { return false };
                let Some(rail1) = rail1 else { return false };

                rail0.try_merge(rail1);    // if this fails then we're boned because I'm not doing rollback right now.
                self.rails.remove(rail_id);
            }

        }
        else {
            // no merging, just add the rail
            self.rails.insert(self.current_id, new_rail);
            self.current_id += 1;
        }

        return true;
    }
    /// Favours horizontal-first, to do vertical-first, just swap start and end cells
    pub fn try_add_from_cells(&mut self, start_cell: IVec2, end_cell: IVec2) -> bool {

        let Some(new_rail) = Rail::try_new(start_cell, end_cell) else { return false; };
        return self.try_add_from_rail(new_rail);
    }

        
    pub fn try_remove_cell(&mut self, cell: IVec2) -> bool {
        let Some((rail_id, _)) = self.is_on(cell) else { return false };
        
        if let Some(rail) = self.rails.get_mut(&rail_id) {

            let mut rail_cells: Vec<IVec2> = rail.iter_cells().collect();
            let index = rail_cells.iter().position(|x| *x == cell).unwrap();

            let mut right_cells = rail_cells.split_off(index);
            right_cells.remove(0);

            if rail_cells.len() > 1 {
                rail.points = rail_cells;
                rail.compress();
            } else {
                self.rails.remove(&rail_id);
            }

            if right_cells.len() > 1 {
                let mut right_rail = Rail::try_new(IVec2::new(0, 0), IVec2::new(0, 1)).unwrap();
                right_rail.points = right_cells;
                right_rail.compress();
                self.try_add_from_rail(right_rail);
            }

            return true;
        }
        return false;
    }

    fn valid_rail(&self, rail: &Rail) -> bool {

        for existing_rail in self.rails.values() {
            if existing_rail.valid_with(&rail) == false { return false; }
        }

        return true;
    }
    fn is_head(&self, cell: IVec2) -> Option<u32> {
        for (rail_id, rail) in self.rails.iter() {
            if rail.is_head(cell) { return Some(*rail_id); }
        }
        return None;
    }
    fn is_tail(&self, cell: IVec2) -> Option<u32> {
        for (rail_id, rail) in self.rails.iter() {
            if rail.is_tail(cell) { return Some(*rail_id); }
        }
        return None;
    }
    fn is_body(&self, cell: IVec2) -> Option<u32> {
         for (rail_id, rail) in self.rails.iter() {
            if rail.is_body(cell) { return Some(*rail_id); }
        }
        return None;
    }

    pub fn is_on(&self, cell: IVec2) -> Option<(u32, usize)> {
        for (rail_id, rail) in self.rails.iter() {
            if let Some(waypoint_index) = rail.waypoint_index(cell) 
            { 
                return Some((*rail_id, waypoint_index)); 
            }
        }
        return None;
    }

}

#[cfg(test)]
mod tests {
    use super::*;


}

