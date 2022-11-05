#![allow(dead_code)]

use beat_up_rust::config::Config;
use bevy::prelude::*;

struct GuiLaneConst {
    width: f32,
    height: f32,
}

struct GuiLandingConst {
    width: f32,
    height: f32,
}

struct BeatupTexture {
    img_path: String,
    width: f32,
    height: f32,
    scale: f32,
    position: Vec2,
}

impl Default for BeatupTexture {
    fn default() -> Self {
        BeatupTexture {
            img_path: "".to_string(),
            width: 0.0,
            height: 0.0,
            scale: 2.0,
            position: Vec2::new(0.0, 0.0),
        }
    }
}

fn build_lane_l_sprite(
    lane_const: &Res<GuiLaneConst>,
    asset_server: &Res<AssetServer>,
) -> SpriteBundle {
    let margin_top = 60.0;
    let width = lane_const.width;
    let space = 150.0;

    let _x = width * 3.0 - space;
    let position = Vec2::new(0.0 - _x, 0.0 - margin_top);
    let texture_config = BeatupTexture {
        width,
        position,
        img_path: "img/laneL.png".to_string(),
        ..default()
    };
    return build_sprite(texture_config, asset_server);
}

fn build_lane_r_sprite(
    lane_const: &Res<GuiLaneConst>,
    landing_const: &Res<GuiLandingConst>,
    asset_server: &Res<AssetServer>,
) -> SpriteBundle {
    let margin_top = 60.0;
    let width = lane_const.width;
    let space = 150.0;

    let _x = space + lane_const.width + 1.5 * landing_const.width + 24.0;
    let position = Vec2::new(_x, 0.0 - margin_top);
    let texture_config = BeatupTexture {
        width,
        position,
        img_path: "img/laneR.png".to_string(),
        ..default()
    };
    return build_sprite(texture_config, asset_server);
}

fn build_landing_l_sprite(
    landing_const: &Res<GuiLandingConst>,
    asset_server: &Res<AssetServer>,
) -> SpriteBundle {
    let margin_top = 60.0;
    let width = landing_const.width;
    let space = 150.0;

    let _x = width * 3.0 - space;
    let position = Vec2::new(0.0 - _x, 0.0 - margin_top);
    let texture_config = BeatupTexture {
        width,
        position,
        img_path: "img/landingL.png".to_string(),
        ..default()
    };
    return build_sprite(texture_config, asset_server);
}

fn build_landing_r_sprite(
    landing_const: &Res<GuiLandingConst>,
    asset_server: &Res<AssetServer>,
) -> SpriteBundle {
    let margin_top = 60.0;
    let width = landing_const.width;

    let space = 150.0;
    let _x = space;

    let position = Vec2::new(0.0 + _x + width - 40.0, 0.0 - margin_top);
    let texture_config = BeatupTexture {
        width,
        position,
        img_path: "img/landingR.png".to_string(),
        ..default()
    };
    return build_sprite(texture_config, asset_server);
}
fn build_sprite(texture_config: BeatupTexture, asset_server: &Res<AssetServer>) -> SpriteBundle {
    let texture: Handle<Image> = asset_server.load(&texture_config.img_path);
    let pos = texture_config.position;
    let transform = Transform {
        translation: Vec3::new(pos.x, pos.y, 0.0),
        scale: Vec3::new(texture_config.scale, texture_config.scale, 0.),
        ..default()
    };

    SpriteBundle {
        texture,
        transform,
        ..default()
    }
}

fn build_table_l(window: &mut Window, asset_server: &Res<AssetServer>) -> SpriteBundle {
    let margin_top = 60.0;
    let width = 128.0;

    let window_w = window.width();

    let _x = window_w / 2.0; // - 36.0;
    let position = Vec2::new(0.0 - _x, 0.0 - margin_top);
    let texture_config = BeatupTexture {
        width,
        position,
        img_path: "img/tableL.png".to_string(),
        ..default()
    };
    return build_sprite(texture_config, asset_server);
}
fn build_table_r(window: &mut Window, asset_server: &Res<AssetServer>) -> SpriteBundle {
    let margin_top = 60.0;
    let width = 128.0;

    let window_w = window.width();

    let _x = (window_w / 2.0) + 18.0;
    let position = Vec2::new(0.0 + _x, 0.0 - margin_top);
    let texture_config = BeatupTexture {
        width,
        position,
        img_path: "img/tableR.png".to_string(),
        ..default()
    };
    return build_sprite(texture_config, asset_server);
}

fn setup(
    mut commands: Commands,
    mut windows: ResMut<Windows>,
    asset_server: Res<AssetServer>,
    lane_const: Res<GuiLaneConst>,
    landing_const: Res<GuiLandingConst>,
) {
    let window = windows.get_primary_mut().unwrap();
    commands.spawn_bundle(Camera2dBundle::default());
    commands.spawn_bundle(build_table_l(window, &asset_server));
    commands.spawn_bundle(build_lane_l_sprite(&lane_const, &asset_server));
    commands.spawn_bundle(build_landing_l_sprite(&landing_const, &asset_server));
    commands.spawn_bundle(build_landing_r_sprite(&landing_const, &asset_server));
    commands.spawn_bundle(build_lane_r_sprite(
        &lane_const,
        &landing_const,
        &asset_server,
    ));
    commands.spawn_bundle(build_table_r(window, &asset_server));
    let transform = Transform {
        translation: Vec3::new(0.0 - 200.0 - 150.0, 128.0, 0.0),
        scale: Vec3::new(2.0, 2.0, 0.),
        ..default()
    };
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("img/lane_1.png"),
        transform,
        ..default()
    });
}

fn main() {
    let config = Config::default();
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(GuiLaneConst {
            width: 256.0,
            height: 64.0,
        })
        .insert_resource(GuiLandingConst {
            width: 128.0,
            height: 64.0,
        })
        .insert_resource(WindowDescriptor {
            title: "Beat up".to_string(),
            width: config.width as f32,
            height: config.height as f32,
            resizable: false,
            transparent: true,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .run();
}
