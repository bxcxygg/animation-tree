use std::time::Duration;

use bevy::prelude::*;
use keyframe_animate::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(KeyframeAnimationPlugin)
        .add_startup_system(spawn)
        .run();
}

fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut textures: ResMut<Assets<TextureAtlas>>,
) {
    // Don't forget the camera ;-)
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // The animation API uses the `Name` component to target entities
    let coin = Name::new("coin");

    // Create an animation
    let mut animation = KeyframeAnimationClip::default();
    // Here we use an index-range (from 0 to 4) where each frame has the same
    // duration
    let duration = Duration::from_millis(100).as_secs_f32();
    animation.add_curve_to_path(
        KeyframeEntityPath {
            parts: vec![coin.clone()],
        },
        KeyframeVariableCurve {
            keyframe_timestamps: vec![0.0, duration, duration * 2., 3. * duration, 4. * duration],
            keyframes: Keyframe::index(vec![0, 1, 2, 3, 4]),
            options: Some(vec!["index".to_string()]),
        },
    );

    // Create the animation player, and set it to repeat
    let mut player = KeyframeAnimationPlayer::new(animation);
    player.repeat();

    commands
        // Spawn a bevy sprite-sheet
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: textures.add(TextureAtlas::from_grid(
                asset_server.load("coin.png"),
                Vec2::new(16.0, 16.0),
                5,
                1,
            )),
            transform: Transform::from_scale(Vec3::splat(10.0)),
            ..Default::default()
        })
        // Add the Name component, and the animation player
        .insert_bundle((coin, player));
}
