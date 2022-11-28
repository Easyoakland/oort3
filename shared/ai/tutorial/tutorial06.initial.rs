// Tutorial 06
// Destroy the enemy ships. Use your radar to find them.
// Hint: Press 'g' in-game to show where your radar is looking.
// Hint: Use the set_radar_heading() function to keep your radar pointed at a
// target, or to search for a new one.
use oort_api::prelude::*;

pub struct Ship {}

impl Ship {
    pub fn new() -> Ship {
        Ship {}
    }

    pub fn tick(&mut self) {
        set_radar_heading(radar_heading() + TAU / 6.0);
        if let Some(contact) = scan() {
            accelerate(0.1 * (contact.position - position()));
            fire(0);
        }
    }
}
