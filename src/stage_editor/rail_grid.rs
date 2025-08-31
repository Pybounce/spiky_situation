
use bevy::{platform::collections::HashMap, prelude::*};

use crate::common::math_ex::axis_aligned_intersect;

#[derive(Default, Debug)]
pub struct RailGrid {
    rails: HashMap<u32, Rail>,
    dirty_rails: Vec<u32>   // would also mark new rails as dirty. A system should then go through the rails to get cells, then find editoritems based on the rail cell. Hmm...but if I remove a rail, then I'll need to check every railid on the editoritems so, yeah not sure.
    // alternate to dirty rails would be to just get all the points those rails use and mark those as dirty. should be fairly easy to get outer points, it's just getting the full list that is annoying, doable though.
}

#[derive(PartialEq, Debug)]
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

    pub fn length(&self) -> u32 {
        let mut len = 0;
        for line in self.points.windows(2) {
            len += line[0].as_vec2().distance(line[1].as_vec2()) as u32;
        }

        return len;
    }

    pub fn try_new(start_cell: IVec2, end_cell: IVec2) -> Option<Self> {
        if start_cell == end_cell { return None; }        
        let points = (start_cell, start_cell + (IVec2::X * (end_cell - start_cell)), end_cell);
        return match points.0 == points.1 {
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

            return true;
        }

        return false;
    }

    pub fn can_merge(&self, rail: &Rail) -> Option<RailMergeOrder> {
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

            if (rail.is_body(current_cell) && self.is_on(current_cell)) || self.is_body(current_cell) { return false; }
            while current_cell != rail_line[1] {
                current_cell += diff;
                if (rail.is_body(current_cell) && self.is_on(current_cell)) || self.is_body(current_cell) { return false; }
            }
        }

        return true;
    }

}

impl RailGrid {
    /// Favours horizontal-first, to do vertical-first, just swap start and end cells
    pub fn try_add_rail(&mut self, start_cell: IVec2, end_cell: IVec2) -> bool {

        let Some(mut new_rail) = Rail::try_new(start_cell, end_cell) else { return false; };
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
        return true;
    }

        
    pub fn remove_cell(cell: IVec2) {
        // Find what rail contains the cell
        // if None, return

        // is it a start/end of the rail
        // If yes --> remove it
        // if no --> Split the rail into 2 rails
        todo!()
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

    fn is_on(&self, cell: IVec2) -> Option<u32> {
        for (rail_id, rail) in self.rails.iter() {
            if rail.is_on(cell) { return Some(*rail_id); }
        }
        return None;
    }

}

#[cfg(test)]
mod tests {
    use super::*;


}

