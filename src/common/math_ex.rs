use bevy::math::IVec2;




pub fn axis_aligned_intersect(a1: IVec2, a2: IVec2, b1: IVec2, b2: IVec2) -> bool {
    let (a_min, a_max) = (a1.min(a2), a1.max(a2));
    let (b_min, b_max) = (b1.min(b2), b1.max(b2));
    let x_overlap = a_min.x <= b_max.x && b_min.x <= a_max.x;
    let y_overlap = a_min.y <= b_max.y && b_min.y <= a_max.y;

    x_overlap && y_overlap
}