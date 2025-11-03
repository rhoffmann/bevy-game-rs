use bevy::prelude::*;
use bevy_egui::{EguiContexts, EguiPlugin, EguiPrimaryContextPass, egui};
use my_library::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, Default, States)]
enum GamePhase {
    #[default]
    Player,
    Cpu,
}

const DIE_SIZE: f32 = 64.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin::default())
        .add_plugins(RandomPlugin)
        .add_systems(Startup, setup)
        .init_state::<GamePhase>()
        .add_systems(EguiPrimaryContextPass, display_score)
        .add_systems(
            EguiPrimaryContextPass,
            player.run_if(in_state(GamePhase::Player)),
        )
        .add_systems(Update, cpu.run_if(in_state(GamePhase::Cpu)))
        .run();
}

#[derive(Resource)]
struct GameAssets {
    image: Handle<Image>,
    layout: Handle<TextureAtlasLayout>,
}

#[derive(Clone, Copy, Resource)]
struct Scores {
    player: usize,
    cpu: usize,
}

#[derive(Component)]
struct HandDie;

#[derive(Resource)]
struct HandTimer(Timer);

fn setup(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn(Camera2d::default());

    let texture = asset_server.load("dice_cubes_sd_bit.png");
    // Use from_grid with 16px padding and 16px offset to match dice layout
    let layout = TextureAtlasLayout::from_grid(
        UVec2::splat(DIE_SIZE as u32), // cell size
        3,                             // columns
        2,                             // rows
        None,                          // Some(UVec2::new(8, 0)),        // padding (x, y)
        Some(UVec2::new(32, 0)),       // offset (x, y)
    );
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    commands.insert_resource(GameAssets {
        image: texture,
        layout: texture_atlas_layout,
    });

    commands.insert_resource(Scores { cpu: 0, player: 0 });

    commands.insert_resource(HandTimer(Timer::from_seconds(0.5, TimerMode::Repeating)));
}

fn display_score(scores: Res<Scores>, mut egui_context: EguiContexts) {
    if let Ok(ctx) = egui_context.ctx_mut() {
        egui::Window::new("Total Scores").show(ctx, |ui| {
            ui.label(&format!("Player: {}", scores.player));
            ui.label(&format!("CPU: {}", scores.cpu));
        });
    }
}

fn spawn_die(
    hand_query: &Query<(Entity, &Sprite), With<HandDie>>,
    commands: &mut Commands,
    assets: &GameAssets,
    new_roll: usize,
    color: Color,
) {
    let rolled_die = hand_query.iter().count() as f32 * DIE_SIZE;
    let mut sprite = Sprite::from_atlas_image(
        assets.image.clone(),
        TextureAtlas {
            layout: assets.layout.clone(),
            index: new_roll - 1,
        },
    );

    sprite.color = color;

    commands.spawn((
        sprite,
        Transform::from_xyz(rolled_die - 400.0, 60.0, 1.0),
        HandDie,
    ));
}

fn clear_die(hand_query: &Query<(Entity, &Sprite), With<HandDie>>, commands: &mut Commands) {
    hand_query
        .iter()
        .for_each(|(entity, _)| commands.entity(entity).despawn());
}

fn player(
    hand_query: Query<(Entity, &Sprite), With<HandDie>>,
    mut commands: Commands,
    assets: Res<GameAssets>,
    rng: Res<RandomNumberGenerator>,
    mut scores: ResMut<Scores>,
    mut state: ResMut<NextState<GamePhase>>,
    mut egui_context: EguiContexts,
) {
    if let Ok(ctx) = egui_context.ctx_mut() {
        egui::Window::new("Play Options").show(ctx, |ui| {
            let hand_score: usize = hand_query
                .iter()
                .map(|(_, ts)| ts.texture_atlas.as_ref().unwrap().index + 1)
                .sum();

            ui.label(&format!("Score for this hand: {hand_score}"));

            if ui.button("Roll Dice").clicked() {
                let new_roll = rng.range(1..=6);
                if new_roll == 1 {
                    // End turn!
                    clear_die(&hand_query, &mut commands);
                    state.set(GamePhase::Cpu);
                } else {
                    spawn_die(&hand_query, &mut commands, &assets, new_roll, Color::WHITE);
                }
            }

            if ui.button("Pass - Keep Hand Score").clicked() {
                let total_hand: usize = hand_query
                    .iter()
                    .map(|(_, ts)| ts.texture_atlas.as_ref().unwrap().index + 1)
                    .sum();

                scores.player += total_hand;
                clear_die(&hand_query, &mut commands);
                state.set(GamePhase::Cpu);
            }
        });
    }
}

#[allow(clippy::too_many_arguments)]
fn cpu(
    hand_query: Query<(Entity, &Sprite), With<HandDie>>,
    mut state: ResMut<NextState<GamePhase>>,
    mut scores: ResMut<Scores>,
    rng: Res<RandomNumberGenerator>,
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut timer: ResMut<HandTimer>,
    time: Res<Time>,
) {
    timer.0.tick(time.delta());

    if timer.0.just_finished() {
        let total_hand: usize = hand_query
            .iter()
            .map(|(_, ts)| ts.texture_atlas.as_ref().unwrap().index + 1)
            .sum();
        if total_hand < 20 && scores.cpu + total_hand < 100 {
            let new_roll = rng.range(1..=6);

            if new_roll == 1 {
                clear_die(&hand_query, &mut commands);
                state.set(GamePhase::Player);
            } else {
                spawn_die(
                    &hand_query,
                    &mut commands,
                    &assets,
                    new_roll,
                    Color::Srgba(Srgba::new(0.0, 0.0, 1.0, 1.0)),
                );
            }
        } else {
            scores.cpu += total_hand;
            state.set(GamePhase::Player);
            hand_query
                .iter()
                .for_each(|(entity, _)| commands.entity(entity).despawn());
        }
    }
}
