use crate::assets;
use crate::assets::font;
use crate::env::*;
use crate::GameStatus;
use bevy::prelude::*;
#[derive(Component)]
struct TopBar;
pub struct TopBarPlugin;
impl Plugin for TopBarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup).add_systems(Update, update);
    }
}
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: TOP_BAR_COLOR,
                    custom_size: Some(Vec2::new(WINDOW_WIDTH, TOP_BAR_HEIGHT)),
                    ..default()
                },
                transform: Transform {
                    translation: Vec3::new(
                        WINDOW_WIDTH / 2. - MICHELE_SIZE.0 / 2.,
                        MICHELE_SIZE.1 / 2. - TOP_BAR_HEIGHT / 2.,
                        0.5,
                    ),
                    ..default()
                },
                ..default()
            },
            TopBar,
        ))
        .with_children(|builder| {
            let font = asset_server.add(font::MAIN_FONT.clone());
            builder.spawn((
                Text2dBundle {
                    text: Text::from_section(
                        "123",
                        TextStyle {
                            font,
                            font_size: 30.,
                            color: Color::BLACK,
                        },
                    ),
                    transform: Transform {
                        translation: Vec3 {
                            x: 0.,
                            y: 0.,
                            z: 1.0,
                        },
                        ..default()
                    },
                    ..default()
                },
                TopBar,
            ));
        });
}
fn update(game_status: Res<GameStatus>, mut texts: Query<&mut Text, With<TopBar>>) {
    for mut text in &mut texts {
        text.sections[0].value = game_status.point.to_string()
    }
}
