use crate::env::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct Michele {
    next: bool,
    pub now_iy: i32,
    pub delta_y: f32,
    beyond: bool,
}
impl Michele {
    pub fn new(asset_server: &AssetServer, ix: u32, iy: i32) -> (SpriteBundle, Self) {
        (
            SpriteBundle {
                texture: asset_server.load("pic/ClickBefore.png"),
                transform: Transform {
                    translation: Vec3::new(
                        ix as f32 * MICHELE_SIZE.0,
                        iy as f32 * -MICHELE_SIZE.1,
                        0.,
                    ),
                    ..default()
                },
                ..default()
            },
            Michele {
                next: false,
                now_iy: iy,
                delta_y: 0.,
                beyond: false,
            },
        )
    }
    pub fn move_next(&mut self) {
        self.next = true;
        self.now_iy += 1;
    }
    pub fn stop(&mut self) {
        self.next = false;
        self.delta_y = 0.;
    }
    pub fn is_move(&mut self) -> bool {
        self.next
    }
    pub fn is_beyond(&mut self) -> bool {
        self.beyond = self.now_iy >= DAMIER_HEIGHT;
        self.now_iy >= DAMIER_HEIGHT
    }
    pub fn is_last(&self) -> bool {
        self.now_iy >= DAMIER_HEIGHT - 1
    }
}
pub struct MichelePlugin;
impl Plugin for MichelePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_michele);
    }
}
fn move_michele(
    mut commands: Commands,
    time: Res<Time>,
    mut sprite_position: Query<(&mut Michele, &mut Transform, Entity)>,
) {
    for (mut m, mut transform, e) in &mut sprite_position {
        if m.is_move() {
            transform.translation.y -= FALLING_SPEED * time.delta_seconds();
            m.delta_y += FALLING_SPEED * time.delta_seconds();
            if m.delta_y >= MICHELE_SIZE.1 {
                m.stop();
                transform.translation.y = m.now_iy as f32 * -100.;
                if m.is_beyond() {
                    commands.entity(e).despawn_recursive();
                }
            }
        }
    }
}
