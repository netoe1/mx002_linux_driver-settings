// netoe-mod:
// Pen pressure sensitivity configuration
//
// PEN_THRESHOLD:
//   Controls how close the pen must be to the tablet surface before
//   pressure processing starts.
//
//   This value defines the proximity cutoff. When the pen is hovering
//   close to the surface but not actually touching it, the raw pressure
//   value remains within this threshold range and is treated as zero.
//
//   Lower values:
//     - Pen activates earlier (more sensitive proximity detection)
//     - May cause unintended touches when hovering
//
//   Higher values:
//     - Pen must be closer / slightly pressing to activate
//     - Reduces false touch events
//
//   Typical values:
//     - 300–500 : Very sensitive, light hover detection
//     - 600–800 : Balanced (default range)
//     - 900+    : Requires firm contact to activate
//
//
// PEN_STRENGTH_SCALING:
//   Controls how much physical force is required for the touch/pressure
//   to be considered strong.
//
//   This value scales the normalized pressure after the proximity
//   threshold has been passed. It directly affects how fast the pressure
//   value increases once the pen is touching the surface.
//
//   Lower values:
//     - Requires more physical force for strong pressure
//     - Produces smoother, softer pressure curves
//
//   Higher values:
//     - Strong pressure is reached with less physical force
//     - Makes strokes feel heavier and more aggressive
//
//   Typical values:
//     - 1       : Very soft pressure response
//     - 2–3     : Balanced pressure curve (recommended)
//     - 4–5     : Very sensitive, strong pressure with light force
//
// These parameters work together to define the pen feel.
// PEN_THRESHOLD determines *when* the pen starts touching,
// PEN_STRENGTH_SCALING determines *how hard* the pen presses.


// Putting defaults in a file, that you can change later, based in a pattern.

use std::sync::atomic::{AtomicI32, Ordering};

// Default values
pub const PEN_THRESHOLD_STD: i32 = 600;
pub const PEN_STRENGTH_STD: i32 = 2;

// Global mutable configuration (thread-safe)
pub static PEN_THRESHOLD: AtomicI32 = AtomicI32::new(PEN_THRESHOLD_STD);
pub static PEN_STRENGTH_SCALING: AtomicI32 = AtomicI32::new(PEN_STRENGTH_STD);

// Helper setters (clean API)
pub fn set_pen_threshold(value: i32) {
    PEN_THRESHOLD.store(value, Ordering::Relaxed);
}

pub fn set_pen_strength(value: i32) {
    PEN_STRENGTH_SCALING.store(value, Ordering::Relaxed);
}