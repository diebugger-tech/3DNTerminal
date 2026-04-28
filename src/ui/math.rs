//! UI utility functions and 3D mathematics.

use cosmic::iced::{Point, Rectangle};

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

/// Maps a screen point to normalized (0..1, 0..1) coordinates within a quad.
/// This is a simple bilinear approximation for hit-testing on the hologram surface.
pub fn project_onto_quad(p: Point, quad: &[Point; 4]) -> Option<(f32, f32)> {
    if !is_point_in_quad(p, quad) {
        return None;
    }

    // Solve for (u, v) such that p = (1-u)(1-v)Q0 + u(1-v)Q1 + uvQ2 + (1-u)vQ3
    // For simplicity in this terminal, we'll use a distance-based weight or 
    // a simple triangle decomposition for now.
    
    // Better: Just use the quad bounds for a rough estimate if it's almost rectangular,
    // or implement a proper inverse bilinear map.
    // For the "Cyberpunk" feel, we can just use the relative position between the edges.
    
    let u_top = (p.x - quad[0].x) / (quad[1].x - quad[0].x).max(1.0);
    let u_bottom = (p.x - quad[3].x) / (quad[2].x - quad[3].x).max(1.0);
    let v_left = (p.y - quad[0].y) / (quad[3].y - quad[0].y).max(1.0);
    let v_right = (p.y - quad[1].y) / (quad[2].y - quad[1].y).max(1.0);
    
    let u = (u_top + u_bottom) / 2.0;
    let v = (v_left + v_right) / 2.0;
    
    Some((u.clamp(0.0, 1.0), v.clamp(0.0, 1.0)))
}

/// Smoothly interpolates between two rectangles.
pub fn lerp_rect(a: Rectangle, b: Rectangle, t: f32) -> Rectangle {
    Rectangle {
        x: a.x + (b.x - a.x) * t,
        y: a.y + (b.y - a.y) * t,
        width: a.width + (b.width - a.width) * t,
        height: a.height + (b.height - a.height) * t,
    }
}
