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

fn spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Don't forget the camera ;-)
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // The animation API uses the `Name` component to target entities
    let player_name = Name::new("player");

    // Create an animation
    let mut animation = KeyframeAnimationClip::default();
    // Here we use an index-range (from 0 to 5) where each frame has the same
    // duration
    let duration = Duration::from_millis(100).as_secs_f32();
    animation.add_curve_to_path(
        KeyframeEntityPath {
            parts: vec![player_name.clone()],
        },
        KeyframeVariableCurve {
            keyframe_timestamps: vec![
                0.0,
                duration,
                duration * 2.,
                3. * duration,
                4. * duration,
                5. * duration,
            ],
            keyframes: Keyframe::images(vec![
                asset_server.load("APimg[4].png"),
                asset_server.load("APimg[5].png"),
                asset_server.load("APimg[6].png"),
                asset_server.load("APimg[7].png"),
                asset_server.load("APimg[8].png"),
                asset_server.load("APimg[9].png"),
            ]),
            options: None,
        },
    );

    // Create the animation player, and set it to repeat
    let mut player = KeyframeAnimationPlayer::new(animation);
    player.repeat();

    commands
        // Spawn a bevy sprite-sheet
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("APimg[4].png"),
            transform: Transform::from_scale(Vec3::splat(1.0)),
            ..Default::default()
        })
        // Add the Name component, and the animation player
        .insert_bundle((player, player_name));
}
