//! The Timster's Audio Works panel: the look the Library, Player, Equaliser
//! and Analyser share. Drone Box's panel language (Roland Jupiter-style
//! keys, red displays, the four-colour stripe), drawn with egui's painter.
//!
//! - [`theme`]: Black and Silver panels, key and light colours, the fonts
//! - [`widgets`]: keys, lit keys, knobs, faders, the LED display, readouts,
//!   meters, section frames and the stripe
//! - [`lists`]: wells, scrolling lists, chips and status lights
//! - [`frame`]: the panel background and fit-to-window scaling

pub mod frame;
pub mod lists;
pub mod theme;
pub mod widgets;
