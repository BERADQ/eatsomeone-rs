use bevy::input::keyboard::KeyCode;

pub const MICHELE_SIZE: (f32, f32) = (100., 100.);
pub const DAMIER_WIDTH: i32 = 4;
pub const DAMIER_HEIGHT: i32 = 6;
pub const WINDOW_WIDTH: f32 = MICHELE_SIZE.0 * DAMIER_WIDTH as f32;
pub const WINDOW_HEIGHT: f32 = MICHELE_SIZE.1 * DAMIER_HEIGHT as f32 + BOTTOM_BAR_HEIGHT;
pub const FALLING_SPEED: f32 = 1600.;
pub const BOTTOM_BAR_HEIGHT: f32 = 40.;
pub const KEY_MAP: [KeyCode; 4] = [KeyCode::D, KeyCode::F, KeyCode::J, KeyCode::K];
