mod implemented;
mod lerp;

pub mod prelude {
    pub use crate::{lerp::Lerp, *};
}

use std::ops::Deref;

use bevy::{hierarchy::HierarchySystem, prelude::*, transform::TransformSystem, utils::HashMap};

use crate::lerp::Lerp;

/// Wrapper around a type that can be eased.
pub struct Keyframe<T>(pub T);

impl<T> Lerp<T> for Keyframe<T>
where
    T: Lerp<T>,
{
    fn lerp(&self, other: &Self, scalar: f32, target: &T, options: &Option<Vec<String>>) -> Self {
        Keyframe(self.0.lerp(&other.0, scalar, target, options))
    }
}

/// Describes how an attribute of a [`Entity`] should be animated.
///
/// `keyframe_timestamps` and `keyframes` should have the same length.
pub struct KeyframeVariableCurve<T> {
    /// Timestamp for each of the keyframes.
    pub keyframe_timestamps: Vec<f32>,
    /// List of the keyframes.
    pub keyframes: Vec<Keyframe<T>>,

    pub options: Option<Vec<String>>,
}

/// Path to an entity, with [`Name`]s. Each entity in a path must have a name.
#[derive(Clone, Debug, Hash, PartialEq, Eq, Default)]
pub struct KeyframeEntityPath {
    /// Parts of the path
    pub parts: Vec<Name>,
}

#[derive(Default, Component)]
pub struct KeyframeAnimationClip<T> {
    curves: HashMap<KeyframeEntityPath, Vec<KeyframeVariableCurve<T>>>,
    duration: f32,
}

impl<T> KeyframeAnimationClip<T> {
    #[inline]
    /// Hashmap of the [`VariableCurve`]s per [`EntityPath`].
    pub fn curves(&self) -> &HashMap<KeyframeEntityPath, Vec<KeyframeVariableCurve<T>>> {
        &self.curves
    }

    /// Duration of the clip, represented in seconds
    #[inline]
    pub fn duration(&self) -> f32 {
        self.duration
    }

    /// Add a [`VariableCurve`] to an [`EntityPath`].
    pub fn add_curve_to_path(&mut self, path: KeyframeEntityPath, curve: KeyframeVariableCurve<T>) {
        // Update the duration of the animation by this curve duration if it's longer
        self.duration = self
            .duration
            .max(*curve.keyframe_timestamps.last().unwrap_or(&0.0));
        self.curves.entry(path).or_default().push(curve);
    }
}

#[derive(Component)]
pub struct KeyframeAnimationPlayer<T> {
    paused: bool,
    repeat: bool,
    speed: f32,
    elapsed: f32,
    animation_clip: KeyframeAnimationClip<T>,
}

impl<T> KeyframeAnimationPlayer<T> {
    pub fn new(animation_clip: KeyframeAnimationClip<T>) -> Self {
        Self {
            paused: false,
            repeat: false,
            speed: 1.0,
            elapsed: 0.0,
            animation_clip,
        }
    }
}

impl<T> KeyframeAnimationPlayer<T> {
    /// Start playing an animation, resetting state of the player
    pub fn play(&mut self, handle: KeyframeAnimationClip<T>) -> &mut Self {
        *self = Self {
            animation_clip: handle,
            paused: false,
            repeat: false,
            speed: 1.0,
            elapsed: 0.0,
        };
        self
    }

    /// Set the animation to repeat
    pub fn repeat(&mut self) -> &mut Self {
        self.repeat = true;
        self
    }

    /// Stop the animation from repeating
    pub fn stop_repeating(&mut self) -> &mut Self {
        self.repeat = false;
        self
    }

    /// Pause the animation
    pub fn pause(&mut self) {
        self.paused = true;
    }

    /// Unpause the animation
    pub fn resume(&mut self) {
        self.paused = false;
    }

    /// Is the animation paused
    pub fn is_paused(&self) -> bool {
        self.paused
    }

    /// Speed of the animation playback
    pub fn speed(&self) -> f32 {
        self.speed
    }

    /// Set the speed of the animation playback
    pub fn set_speed(&mut self, speed: f32) -> &mut Self {
        self.speed = speed;
        self
    }

    /// Time elapsed playing the animation
    pub fn elapsed(&self) -> f32 {
        self.elapsed
    }

    /// Seek to a specific time in the animation
    pub fn set_elapsed(&mut self, elapsed: f32) -> &mut Self {
        self.elapsed = elapsed;
        self
    }
}

/// System that will play all animations, using any entity with a
/// [`AnimationPlayer`] and a [`AnimationClip`] as an animation root
pub fn keyframe_animation_player<T: Component>(
    time: Res<Time>,
    mut query: Query<(Entity, &mut T)>,
    mut animation_players: Query<&mut KeyframeAnimationPlayer<T>>,
    names: Query<&Name>,
    children: Query<&Children>,
) where
    Keyframe<T>: Lerp<T>,
    T: Default,
{
    for (entity, mut object) in query.iter_mut() {
        if let Ok(mut player) = animation_players.get_mut(entity) {
            // Continue if paused unless the `AnimationPlayer` was changed
            // This allow the animation to still be updated if the player.elapsed field was
            // manually updated in pause
            if player.paused && !player.is_changed() {
                continue;
            }
            if !player.paused {
                player.elapsed += time.delta_seconds() * player.speed;
            }
            let mut elapsed = player.elapsed;
            if player.repeat {
                elapsed %= player.animation_clip.duration;
            }
            if elapsed < 0.0 {
                elapsed += player.animation_clip.duration;
            }
            'entity: for (path, curves) in &player.animation_clip.curves {
                // PERF: finding the target entity can be optimised
                let mut current_entity = entity;
                // Ignore the first name, it is the root node which we already have
                for part in path.parts.iter().skip(1) {
                    let mut found = false;
                    if let Ok(children) = children.get(current_entity) {
                        for child in children.deref() {
                            if let Ok(name) = names.get(*child) {
                                if name == part {
                                    // Found a children with the right name, continue to the
                                    // next part
                                    current_entity = *child;
                                    found = true;
                                    break;
                                }
                            }
                        }
                    }
                    if !found {
                        warn!("Entity not found for path {:?} on part {:?}", path, part);
                        continue 'entity;
                    }
                }
                for curve in curves {
                    // Some curves have only one keyframe used to set a keyframe
                    if curve.keyframe_timestamps.len() == 1 {
                        *object = Keyframe(T::default())
                            .lerp(&curve.keyframes[0], 0., &*object, &curve.options)
                            .0;
                        continue;
                    }

                    // Find the current keyframe
                    // PERF: finding the current keyframe can be optimised
                    let step_start = match curve
                        .keyframe_timestamps
                        .binary_search_by(|probe| probe.partial_cmp(&elapsed).unwrap())
                    {
                        Ok(i) => i,
                        Err(0) => continue, // this curve isn't started yet
                        Err(n) if n > curve.keyframe_timestamps.len() - 1 => continue, /* this curve is finished */
                        Err(i) => i - 1,
                    };
                    let ts_start = curve.keyframe_timestamps[step_start];
                    let ts_end = curve.keyframe_timestamps[step_start + 1];
                    let lerp = (elapsed - ts_start) / (ts_end - ts_start);

                    // Apply the keyframe
                    *object = curve.keyframes[step_start]
                        .lerp(
                            &curve.keyframes[step_start + 1],
                            lerp,
                            &*object,
                            &curve.options,
                        )
                        .0;
                }
            }
        }
    }
}

/// Adds animation support to an app
#[derive(Default)]
pub struct KeyframeAnimationPlugin;

impl Plugin for KeyframeAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(
            CoreStage::PostUpdate,
            keyframe_animation_player::<Transform>
                .before(TransformSystem::TransformPropagate)
                .after(HierarchySystem::ParentUpdate),
        )
        .add_system(keyframe_animation_player::<Sprite>)
        .add_system(keyframe_animation_player::<Handle<Image>>)
        .add_system(keyframe_animation_player::<TextureAtlasSprite>);
    }
}
