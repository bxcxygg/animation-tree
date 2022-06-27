use bevy::ecs::all_tuples;
use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::utils::HashMap;

pub trait KeyFrameFunction: Send + Sync + 'static {
    fn run(&mut self, commands: &mut Commands, entity: Entity);
}

#[allow(non_snake_case)]
impl<Func: Send + Sync + 'static> KeyFrameFunction for Func
where
    for<'a> &'a mut Func: FnMut(&mut Commands, Entity),
{
    #[inline]
    fn run(&mut self, commands: &mut Commands, entity: Entity) {
        // Yes, this is strange, but `rustc` fails to compile this impl
        // without using this function. It fails to recognise that `func`
        // is a function, potentially because of the multiple impls of `FnMut`
        #[allow(clippy::too_many_arguments)]
        fn call_inner(
            mut f: impl FnMut(&mut Commands, Entity),
            commands: &mut Commands,
            entity: Entity,
        ) {
            f(commands, entity)
        }
        call_inner(self, commands, entity)
    }
}

pub struct Keyframes {
    pub func: Box<dyn KeyFrameFunction>,
}

/// Describes how an attribute of a [`Entity`] should be animated.
///
/// `keyframe_timestamps` and `keyframes` should have the same length.
pub struct VariableCurve {
    /// Timestamp for each of the keyframes.
    pub keyframe_timestamps: Vec<f32>,
    /// List of the keyframes.
    pub keyframes: Vec<Keyframes>,
}

/// Path to an entity, with [`Name`]s. Each entity in a path must have a name.
#[derive(Clone, Debug, Hash, PartialEq, Eq, Default)]
pub struct EntityPath {
    /// Parts of the path
    pub parts: Vec<Name>,
}

#[derive(TypeUuid, Default)]
#[uuid = "d81b7179-0448-4eb0-89fe-c067222725bf"]
pub struct AnimationClip {
    curves: HashMap<EntityPath, Vec<VariableCurve>>,
    duration: f32,
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct AnimationPlayer {
    paused: bool,
    repeat: bool,
    speed: f32,
    elapsed: f32,
    animation_clip: Handle<AnimationClip>,
}

impl Default for AnimationPlayer {
    fn default() -> Self {
        Self {
            paused: false,
            repeat: false,
            speed: 1.0,
            elapsed: 0.0,
            animation_clip: Default::default(),
        }
    }
}

impl AnimationPlayer {
    /// Start playing an animation, resetting state of the player
    pub fn play(&mut self, handle: Handle<AnimationClip>) -> &mut Self {
        *self = Self {
            animation_clip: handle,
            ..Default::default()
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
