#![windows_subsystem = "windows"]
use bevy::{
    prelude::*,
    render::texture::{CompressedImageFormats, ImageSampler, ImageType},
    window::WindowResolution,
};
use bevy_framepace::FramepacePlugin;
mod assets;
mod bottom_bar;
mod env;
mod michele;
mod topbar;
use assets::pic;
use bottom_bar::BottomBarPlugin;
use env::*;
use michele::{Michele, MichelePlugin};
use rand::prelude::*;
use topbar::TopBarPlugin;

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

#[derive(Resource, Debug)]
pub struct GameStatus {
    point: u64,
}

impl GameStatus {
    fn new() -> Self {
        GameStatus { point: 0 }
    }
}

fn main() {
    App::new()
        .insert_resource(Map::new())
        .insert_resource(GameStatus::new())
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
        .add_plugins(TopBarPlugin)
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
    mut micheles: Query<(&mut Michele, &mut Handle<Image>)>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut map: ResMut<Map>,
    mut game_status: ResMut<GameStatus>,
) {
    let after_img = asset_server.add(pic::CLICK_BEFORE.clone());
    let mut rng = rand::thread_rng();
    if key_input.just_pressed(KEY_MAP[*map.0.last().unwrap() as usize]) {
        let ix: u32 = rng.gen_range(0..=3);
        commands.spawn(Michele::new(&asset_server, ix, -1));
        let mut new_map = vec![ix];
        new_map.extend(&map.0[0..(DAMIER_HEIGHT as usize)]);
        map.0 = unsafe { *(new_map[0..].as_ptr() as *const [u32; DAMIER_HEIGHT as usize + 1]) };
        //map.0 = new_map[0..].try_into().unwrap();
        game_status.point += 1;
        for (mut m, mut s) in &mut micheles {
            if m.is_last() {
                *s = after_img.clone();
            }
            m.move_next();
        }
    } else if key_input.any_just_pressed(KEY_MAP) {
        if game_status.point != 0 {
            game_status.point -= 1;
        }
    }
}
