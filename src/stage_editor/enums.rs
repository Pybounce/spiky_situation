
#[derive(Default, Copy, Clone, Debug)]
#[repr(u8)]
pub enum EditorItem {
    #[default]
    Ground = 0,
    Spike { rotation: f32 }= 1,
    Spawn = 2,
    Spring { rotation: f32 }= 3,
    PhantomBlock = 4,
    HalfSaw { rotation: f32 }= 5,
    Key { variant: KeyVariant } = 6,
    LockBlock { variant: LockBlockVariant } = 7,
    IntervalBlock { variant: IntervalBlockVariant } = 8,
    SawShooter { rotation: f32 } = 9,
    Goal = 10,
    TerrainTheme { variant: TerrainThemeVarient } = 11
}

impl EditorItem {
    pub fn cycle_next(&self) -> Self {
        match self {
            EditorItem::Ground => EditorItem::Key { variant: KeyVariant::One },
            EditorItem::Key { .. } => EditorItem::Spike { rotation: 0.0 },
            EditorItem::Spike { .. } => EditorItem::Spawn,
            EditorItem::Spawn => EditorItem::Spring { rotation: 0.0 },
            EditorItem::Spring { .. } => EditorItem::PhantomBlock,
            EditorItem::PhantomBlock => EditorItem::HalfSaw { rotation: 0.0 },
            EditorItem::HalfSaw { .. } => EditorItem::LockBlock { variant: LockBlockVariant::One },
            EditorItem::LockBlock { .. } => EditorItem::IntervalBlock { variant: IntervalBlockVariant::On },
            EditorItem::IntervalBlock { .. } => EditorItem::SawShooter { rotation: 0.0 },
            EditorItem::SawShooter { .. } => EditorItem::Goal,
            EditorItem::Goal => EditorItem::TerrainTheme { variant: TerrainThemeVarient::Grass },
            EditorItem::TerrainTheme { .. } => EditorItem::Ground,
        }
    }
    pub fn cycle_prev(&self) -> Self {
        match self {
            EditorItem::SawShooter { .. } => EditorItem::IntervalBlock { variant: IntervalBlockVariant::On },
            EditorItem::Ground => EditorItem::TerrainTheme { variant: TerrainThemeVarient::Grass },
            EditorItem::IntervalBlock { .. } => EditorItem::LockBlock { variant: LockBlockVariant::One },
            EditorItem::LockBlock { .. } => EditorItem::HalfSaw { rotation: 0.0 },
            EditorItem::HalfSaw { .. } => EditorItem::PhantomBlock,
            EditorItem::PhantomBlock => EditorItem::Spring { rotation: 0.0 },
            EditorItem::Spring { .. } => EditorItem::Spawn,
            EditorItem::Spawn => EditorItem::Spike { rotation: 0.0 },
            EditorItem::Spike { .. } => EditorItem::Key { variant: KeyVariant::One },
            EditorItem::Key { .. } => EditorItem::Ground,
            EditorItem::Goal { .. } => EditorItem::SawShooter { rotation: 0.0 },
            EditorItem::TerrainTheme { .. } => EditorItem::Goal,
        }
    }
    pub fn cycle_next_variant(&self) -> Self {
        match self {
            EditorItem::Ground => EditorItem::Ground,
            EditorItem::Key { variant } => EditorItem::Key { variant: variant.cycle_next() },
            EditorItem::Spike { rotation } => EditorItem::Spike { rotation: *rotation },
            EditorItem::Spawn => EditorItem::Spawn,
            EditorItem::Spring { rotation } => EditorItem::Spring { rotation: *rotation },
            EditorItem::PhantomBlock => EditorItem::PhantomBlock,
            EditorItem::HalfSaw { rotation } => EditorItem::HalfSaw { rotation: *rotation },
            EditorItem::LockBlock { variant } => EditorItem::LockBlock { variant: variant.cycle_next() },
            EditorItem::IntervalBlock { variant } => EditorItem::IntervalBlock { variant: variant.cycle_next() },
            EditorItem::SawShooter { rotation } => EditorItem::SawShooter { rotation: *rotation },
            EditorItem::Goal => EditorItem::Goal,
            EditorItem::TerrainTheme { variant } => EditorItem::TerrainTheme { variant: variant.cycle_next() },
        }
    }
    pub fn cycle_prev_variant(&self) -> Self {
        match self {
            EditorItem::Ground => EditorItem::Ground,
            EditorItem::LockBlock { variant } => EditorItem::LockBlock { variant: variant.cycle_prev() },
            EditorItem::HalfSaw { rotation } => EditorItem::HalfSaw { rotation: *rotation },
            EditorItem::PhantomBlock => EditorItem::PhantomBlock,
            EditorItem::Spring { rotation } => EditorItem::Spring { rotation: *rotation },
            EditorItem::Spawn => EditorItem::Spawn,
            EditorItem::Spike { rotation } => EditorItem::Spike { rotation: *rotation },
            EditorItem::Key { variant } => EditorItem::Key { variant: variant.cycle_prev() },
            EditorItem::IntervalBlock { variant } => EditorItem::IntervalBlock { variant: variant.cycle_prev() },
            EditorItem::SawShooter { rotation } => EditorItem::SawShooter { rotation: *rotation },
            EditorItem::Goal => EditorItem::Goal,
            EditorItem::TerrainTheme { variant } => EditorItem::TerrainTheme { variant: variant.cycle_prev() },
        }
    }

    pub fn try_rotate(&mut self) -> bool {

        match self {
            EditorItem::Ground => return false,
            EditorItem::Spike { rotation } => rotate_quater_bounded(rotation),
            EditorItem::Spawn => return false,
            EditorItem::Spring { rotation } => rotate_quater_bounded(rotation),
            EditorItem::PhantomBlock => return false,
            EditorItem::HalfSaw { rotation } => rotate_quater_bounded(rotation),
            EditorItem::Key { .. } => return false,
            EditorItem::LockBlock { .. } => return false,
            EditorItem::IntervalBlock { .. } => return false,
            EditorItem::SawShooter { rotation } => rotate_quater_bounded(rotation),
            EditorItem::Goal => return false,
            EditorItem::TerrainTheme { .. } => return false,
        };
        return true;
    }
    pub fn get_rotation(&self) -> f32 {
        match self {
            EditorItem::Ground => 0.0,
            EditorItem::Spike { rotation } => *rotation,
            EditorItem::Spawn => 0.0,
            EditorItem::Spring { rotation } => *rotation,
            EditorItem::PhantomBlock => 0.0,
            EditorItem::HalfSaw { rotation } => *rotation,
            EditorItem::Key { .. } => 0.0,
            EditorItem::LockBlock { .. } => 0.0,
            EditorItem::IntervalBlock { .. } => 0.0,
            EditorItem::SawShooter { rotation } => *rotation,
            EditorItem::Goal => 0.0,
            EditorItem::TerrainTheme { .. } => 0.0,
        }
    }
}

fn rotate_quater_bounded(r: &mut f32) {
    *r -= std::f32::consts::FRAC_PI_2;
    if *r <= 0.0 {
        *r = std::f32::consts::PI * 2.0;
    }
}


#[derive(Default, Copy, Clone, Debug)]
pub enum TerrainThemeVarient {
    #[default]
    Grass,
    Snow,
    Sand,
}

impl TerrainThemeVarient {
    pub fn cycle_next(&self) -> Self {
        match self {
            TerrainThemeVarient::Grass => TerrainThemeVarient::Snow,
            TerrainThemeVarient::Snow => TerrainThemeVarient::Sand,
            TerrainThemeVarient::Sand => TerrainThemeVarient::Grass
        }
    }
    pub fn cycle_prev(&self) -> Self {
        match self {
            TerrainThemeVarient::Grass => TerrainThemeVarient::Sand,
            TerrainThemeVarient::Snow => TerrainThemeVarient::Grass,
            TerrainThemeVarient::Sand => TerrainThemeVarient::Snow
        }
    }
}



#[derive(Default, Copy, Clone, Debug)]
pub enum KeyVariant {
    #[default]
    One,
    Two,
    Three,
}

impl KeyVariant {
    pub fn cycle_next(&self) -> Self {
        match self {
            KeyVariant::One => KeyVariant::Two,
            KeyVariant::Two => KeyVariant::Three,
            KeyVariant::Three => KeyVariant::One,
        }
    }
    pub fn cycle_prev(&self) -> Self {
        match self {
            KeyVariant::One => KeyVariant::Three,
            KeyVariant::Three => KeyVariant::Two,
            KeyVariant::Two => KeyVariant::One,
        }
    }
}

#[derive(Default, Copy, Clone, Debug)]
pub enum LockBlockVariant {
    #[default]
    One,
    Two,
    Three,
}

impl LockBlockVariant {
    pub fn cycle_next(&self) -> Self {
        match self {
            LockBlockVariant::One => LockBlockVariant::Two,
            LockBlockVariant::Two => LockBlockVariant::Three,
            LockBlockVariant::Three => LockBlockVariant::One,
        }
    }
    pub fn cycle_prev(&self) -> Self {
        match self {
            LockBlockVariant::One => LockBlockVariant::Three,
            LockBlockVariant::Three => LockBlockVariant::Two,
            LockBlockVariant::Two => LockBlockVariant::One,
        }
    }
}

#[derive(Default, Copy, Clone, Debug)]
pub enum IntervalBlockVariant {
    #[default]
    On,
    Off,
}

impl IntervalBlockVariant {
    pub fn cycle_next(&self) -> Self {
        match self {
            IntervalBlockVariant::On => IntervalBlockVariant::Off,
            IntervalBlockVariant::Off => IntervalBlockVariant::On,
        }
    }
    pub fn cycle_prev(&self) -> Self {
        match self {
            IntervalBlockVariant::On => IntervalBlockVariant::Off,
            IntervalBlockVariant::Off => IntervalBlockVariant::On,
        }
    }
}