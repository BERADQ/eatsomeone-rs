#![windows_subsystem = "windows"]
use bevy::{prelude::*, window::WindowResolution};
use bevy_framepace::FramepacePlugin;
mod bottom_bar;
mod env;
mod michele;
use bottom_bar::BottomBarPlugin;
use env::*;
use michele::{Michele, MichelePlugin};
use rand::prelude::*;

#[derive(Resource)]
struct Map([u32; DAMIER_HEIGHT as usize + 1]);

impl Map {
    fn new() -> Self {
        let mut map: [u32; DAMIER_HEIGHT as usize + 1] = [0; DAMIER_HEIGHT as usize + 1];
        let mut rng = rand::thread_rng();
        for i in 0..(DAMIER_HEIGHT + 1) {
            map[i as usize] = rng.gen_range(0..=3);
        }
        Map(map)
    }
}

fn main() {
    App::new()
        .insert_resource(Map::new())
        .insert_resource(ClearColor(Color::WHITE))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "吃掉米雪儿".to_string(),
                resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(FramepacePlugin)
        .add_plugins(MichelePlugin)
        .add_plugins(BottomBarPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, event_hit)
        .run()
}

fn setup(mut commands: Commands, map: ResMut<Map>, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle {
        transform: Transform {
            translation: Vec3::new(
                WINDOW_WIDTH / 2. - MICHELE_SIZE.0 / 2.,
                MICHELE_SIZE.1 / 2. - WINDOW_HEIGHT / 2.,
                100.0,
            ),
            ..default()
        },
        ..default()
    });
    for (iy, ix) in map.0.iter().enumerate() {
        commands.spawn(Michele::new(&asset_server, *ix as u32, iy as i32 - 1));
    }
}

fn event_hit(
    key_input: Res<Input<KeyCode>>,
    mut micheles: Query<&mut Michele>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut map: ResMut<Map>,
) {
    if key_input.just_pressed(KEY_MAP[*map.0.last().unwrap() as usize]) {
        let mut rng = rand::thread_rng();
        let ix: u32 = rng.gen_range(0..=3);
        commands.spawn(Michele::new(&asset_server, ix, -1));
        let mut new_map = vec![ix];
        let old_map = Vec::from(&map.0[0..(DAMIER_HEIGHT as usize)]);
        new_map.extend(old_map);
        map.0 = unsafe { *(new_map[0..].as_ptr() as *const [u32; DAMIER_HEIGHT as usize + 1]) };

        for mut m in &mut micheles {
            m.move_next();
        }
    }
}
