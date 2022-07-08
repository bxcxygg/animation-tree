use bevy::prelude::*;
use keyframe_animate::prelude::*;

#[derive(Component, Default)]
struct Custom(f32);

impl Lerp<Custom> for Custom {
    fn lerp(&self, other: &Self, scalar: f32, _: &Custom, _: &Option<Vec<String>>) -> Self {
        Custom(interpolation::lerp(&self.0, &other.0, &scalar))
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(KeyframeAnimationPlugin)
        .add_startup_system(spawn)
        .add_system(keyframe_animation_player::<Custom>)
        .add_system(check_value)
        .run();
}

fn spawn(mut commands: Commands) {
    // The animation API uses the `Name` component to target entities
    let custom = Name::new("custom");

    // Create an animation
    let mut animation = KeyframeAnimationClip::default();
    // Here we use an index-range (from 0 to 4) where each frame has the same
    // duration
    animation.add_curve_to_path(
        KeyframeEntityPath {
            parts: vec![custom.clone()],
        },
        KeyframeVariableCurve {
            keyframe_timestamps: vec![0.0, 1.0, 2.0, 3.0],
            keyframes: vec![
                Keyframe(Custom(0.0)),
                Keyframe(Custom(1.0)),
                Keyframe(Custom(2.0)),
                Keyframe(Custom(3.0)),
            ],
            options: None,
        },
    );

    // Create the animation player, and set it to repeat
    let mut player = KeyframeAnimationPlayer::new(animation);
    player.repeat();

    commands
        // Spawn a bevy sprite-sheet
        .spawn()
        // Add the Name component, and the animation player
        .insert_bundle((player, custom, Custom(0.0)));
}

fn check_value(mut query: Query<&Custom>) {
    for custom in query.iter_mut() {
        println!("got {:?}", custom.0);
    }
}
