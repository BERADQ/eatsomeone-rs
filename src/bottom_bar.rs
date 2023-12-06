use crate::env::*;
use bevy::prelude::*;

const HEIGHTLIGHT_COLOR: Color = Color::rgb(0.4, 0.4, 0.4);
const DEFAULT_COLOR: Color = Color::rgb(0.25, 0.25, 0.25);

pub struct BottomBarPlugin;
impl Plugin for BottomBarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup).add_systems(Update, update);
    }
}
#[derive(Component)]
enum BarKey {
    D,
    F,
    J,
    K,
}
impl BarKey {
    fn all() -> [Self; 4] {
        [Self::D, Self::F, Self::J, Self::K]
    }
    fn key_code(&self) -> KeyCode {
        match self {
            &Self::D => KeyCode::D,
            &Self::F => KeyCode::F,
            &Self::J => KeyCode::J,
            &Self::K => KeyCode::K,
        }
    }
}
fn setup(mut commands: Commands) {
    let all = BarKey::all();
    let mut i: usize = 0;
    for bk in all {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: DEFAULT_COLOR,
                    custom_size: Some(Vec2::new(MICHELE_SIZE.0, BOTTOM_BAR_HEIGHT)),
                    ..default()
                },
                transform: Transform {
                    translation: Vec3::new(
                        i as f32 * MICHELE_SIZE.0,
                        -WINDOW_HEIGHT + MICHELE_SIZE.1 / 2. + BOTTOM_BAR_HEIGHT / 2.,
                        0.1,
                    ),
                    ..default()
                },
                ..default()
            },
            bk,
        ));
        i += 1;
    }
}
fn update(mut key_bars: Query<(&mut Sprite, &BarKey)>, key_input: Res<Input<KeyCode>>) {
    for (mut s, key) in &mut key_bars {
        if key_input.just_pressed(key.key_code()) {
            s.color = HEIGHTLIGHT_COLOR;
        }
        if key_input.just_released(key.key_code()) {
            s.color = DEFAULT_COLOR;
        }
    }
}
