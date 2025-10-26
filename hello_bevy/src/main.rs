use bevy::prelude::*;
use bevy::sprite::Sprite;

#[derive(Component)]
struct Dragon;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d::default());
    let dragon_image = asset_server.load("dragon.png");
    let mut sprite = Sprite::sized(Vec2::new(600.0, 300.0));
    sprite.image = dragon_image;

    commands.spawn(sprite).insert(Dragon);
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}
