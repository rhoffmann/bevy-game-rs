use bevy::prelude::*;
use bevy::sprite::Sprite;

#[derive(Component)]
struct Dragon;

fn movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut dragon_query: Query<&mut Transform, With<Dragon>>,
) {
    let mut translate_delta = Vec2::ZERO;
    let mut scale_delta = Vec2::ZERO;

    // Pattern match on individual key presses
    for key in keyboard.get_pressed() {
        match key {
            KeyCode::ArrowLeft | KeyCode::KeyA => translate_delta.x -= 1.0,
            KeyCode::ArrowRight | KeyCode::KeyD => translate_delta.x += 1.0,
            KeyCode::ArrowUp | KeyCode::KeyW => translate_delta.y += 1.0,
            KeyCode::ArrowDown | KeyCode::KeyS => translate_delta.y -= 1.0,
            KeyCode::Space => scale_delta += 0.05,
            KeyCode::KeyQ => scale_delta -= 0.05,
            _ => {}
        }
    }

    dragon_query.iter_mut().for_each(|mut transform| {
        transform.translation += translate_delta.extend(0.0);
        transform.scale += scale_delta.extend(0.0);
    })
}

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
        .add_systems(Update, movement)
        .run();
}
