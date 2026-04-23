//! UI utility functions and 3D mathematics.

use cosmic::iced::Point;

/// Easing Helper for smooth animations.
/// Approximates a cubic bezier curve (0.16, 1, 0.3, 1).
pub fn cubic_bezier(t: f32) -> f32 {
    let p1 = 1.0; // Control Point 1 (y)
    let p2 = 0.3; // Control Point 2 (y)
    let t2 = t * t;
    let t3 = t2 * t;
    (1.0 - t3) * 0.0 + 3.0 * (1.0 - t2) * t * p1 + 3.0 * (1.0 - t) * t2 * p2 + t3 * 1.0
}

/// Checks if a 2D point is inside a 2D convex quadrilateral.
pub fn is_point_in_quad(p: Point, quad: &[Point; 4]) -> bool {
    let cross = |a: Point, b: Point, c: Point| -> f32 {
        (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x)
    };
    let c1 = cross(quad[0], quad[1], p) >= 0.0;
    let c2 = cross(quad[1], quad[2], p) >= 0.0;
    let c3 = cross(quad[2], quad[3], p) >= 0.0;
    let c4 = cross(quad[3], quad[0], p) >= 0.0;
    (c1 && c2 && c3 && c4) || (!c1 && !c2 && !c3 && !c4)
}
