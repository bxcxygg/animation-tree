use bevy::prelude::{Quat, Transform};

use crate::{lerp::Lerp, Keyframe, Vec3};

impl Lerp for Keyframe<Transform> {
    type Scalar = f32;

    type Target = Transform;

    fn lerp(
        &self,
        other: &Self,
        scalar: &Self::Scalar,
        target: &Self::Target,
        options: &Option<Vec<String>>,
    ) -> Self {
        match options {
            Some(ops) => {
                let mut transform = Transform {
                    translation: target.translation,
                    rotation: target.rotation,
                    scale: target.scale,
                };
                for op in ops {
                    match op.as_str() {
                        "translation" => {
                            transform.translation =
                                self.0.translation.lerp(other.0.translation, *scalar);
                        }
                        "scale" => {
                            transform.scale = self.0.scale.lerp(other.0.scale, *scalar);
                        }
                        "rotation" => {
                            transform.rotation =
                                self.0.rotation.normalize().lerp(other.0.rotation, *scalar);
                        }
                        _ => {}
                    }
                }
                Keyframe(transform)
            }
            None => Keyframe(Transform {
                translation: self.0.translation.lerp(other.0.translation, *scalar),
                scale: self.0.scale.lerp(other.0.scale, *scalar),
                rotation: self.0.rotation.normalize().lerp(other.0.rotation, *scalar),
            }),
        }
    }
}

impl Keyframe<Transform> {
    pub fn translation(values: Vec<Vec3>) -> Vec<Keyframe<Transform>> {
        values
            .iter()
            .map(|v| {
                Keyframe(Transform {
                    translation: *v,
                    ..Default::default()
                })
            })
            .collect()
    }

    pub fn scale(values: Vec<Vec3>) -> Vec<Keyframe<Transform>> {
        values
            .iter()
            .map(|v| {
                Keyframe(Transform {
                    scale: *v,
                    ..Default::default()
                })
            })
            .collect()
    }

    pub fn rotation(values: Vec<Quat>) -> Vec<Keyframe<Transform>> {
        values
            .iter()
            .map(|v| {
                Keyframe(Transform {
                    rotation: *v,
                    ..Default::default()
                })
            })
            .collect()
    }
}
