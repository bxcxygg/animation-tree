use bevy::prelude::*;

use crate::{lerp::Lerp, Keyframe};

impl Lerp<Transform> for Keyframe<Transform> {
    type Scalar = f32;

    fn lerp(
        &self,
        other: &Self,
        scalar: &Self::Scalar,
        target: &Transform,
        options: &Option<Vec<String>>,
    ) -> Self {
        match options {
            Some(ops) => {
                let mut transform = Transform { ..target.clone() };
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

impl Lerp<Sprite> for Keyframe<Sprite> {
    type Scalar = f32;

    fn lerp(
        &self,
        other: &Self,
        scalar: &Self::Scalar,
        _: &Sprite,
        _: &Option<Vec<String>>,
    ) -> Self {
        Keyframe(Sprite {
            custom_size: match (self.0.custom_size, other.0.custom_size) {
                (None, None) => None,
                (None, Some(b)) => Some(b),
                (Some(a), None) => Some(a),
                (Some(a), Some(b)) => Some(a.lerp(b, *scalar)),
            },
            #[cfg(feature = "render")]
            color: Keyframe(self.0.color)
                .lerp(&Keyframe(other.0.color), scalar, &self.0.color, &None)
                .0,
            ..other.0.clone()
        })
    }
}

impl Lerp<Color> for Keyframe<Color> {
    type Scalar = f32;

    fn lerp(
        &self,
        other: &Self,
        scalar: &Self::Scalar,
        _: &Color,
        _: &Option<Vec<String>>,
    ) -> Self {
        let color = match (self.0, other.0) {
            (
                Color::Rgba {
                    red,
                    green,
                    blue,
                    alpha,
                },
                Color::Rgba {
                    red: redo,
                    green: greeno,
                    blue: blueo,
                    alpha: alphao,
                },
            ) => Color::Rgba {
                red: red + (redo + (red * -1.0)) * *scalar,
                green: green + (greeno + (green * -1.0)) * *scalar,
                blue: blue + (blueo + (blue * -1.0)) * *scalar,
                alpha: alpha + (alphao + (alpha * -1.0)) * *scalar,
            },
            (
                Color::RgbaLinear {
                    red,
                    green,
                    blue,
                    alpha,
                },
                Color::RgbaLinear {
                    red: redo,
                    green: greeno,
                    blue: blueo,
                    alpha: alphao,
                },
            ) => Color::RgbaLinear {
                red: red + (redo + (red * -1.0)) * *scalar,
                green: green + (greeno + (green * -1.0)) * *scalar,
                blue: blue + (blueo + (blue * -1.0)) * *scalar,
                alpha: alpha + (alphao + (alpha * -1.0)) * *scalar,
            },
            (
                Color::Hsla {
                    hue,
                    saturation,
                    lightness,
                    alpha,
                },
                Color::Hsla {
                    hue: hueo,
                    saturation: saturationo,
                    lightness: lightnesso,
                    alpha: alphao,
                },
            ) => Color::Hsla {
                hue: hue + (hueo + (hue * -1.0)) * *scalar,
                saturation: saturation + (saturationo + (saturation * -1.0)) * *scalar,
                lightness: lightness + (lightnesso + (lightness * -1.0)) * *scalar,
                alpha: alpha + (alphao + (alpha * -1.0)) * *scalar,
            },
            _ => self.0 + (other.0 + (self.0 * -1.)) * *scalar,
        };
        Keyframe(color)
    }
}

impl Lerp<TextureAtlasSprite> for Keyframe<TextureAtlasSprite> {
    type Scalar = f32;

    fn lerp(
        &self,
        other: &Self,
        scalar: &Self::Scalar,
        target: &TextureAtlasSprite,
        options: &Option<Vec<String>>,
    ) -> Self {
        match options {
            Some(ops) => {
                let mut sprite = TextureAtlasSprite { ..target.clone() };
                for o in ops {
                    match o.as_str() {
                        "index" => sprite.index = other.0.index,
                        "custom_size" => sprite.custom_size = other.0.custom_size,
                        "flip_x" => sprite.flip_x = other.0.flip_x,
                        "flip_y" => sprite.flip_y = other.0.flip_y,
                        "anchor" => sprite.anchor = other.0.anchor.clone(),
                        "color" => sprite.color = other.0.color,
                        _ => {}
                    }
                }
                Keyframe(sprite)
            }
            None => Keyframe(TextureAtlasSprite {
                custom_size: match (self.0.custom_size, other.0.custom_size) {
                    (None, None) => None,
                    (None, Some(b)) => Some(b),
                    (Some(a), None) => Some(a),
                    (Some(a), Some(b)) => Some(a.lerp(b, *scalar)),
                },
                #[cfg(feature = "render")]
                color: Keyframe(self.0.color)
                    .lerp(&Keyframe(other.0.color), scalar, &self.0.color, &None)
                    .0,
                ..other.0.clone()
            }),
        }
    }
}

impl Keyframe<TextureAtlasSprite> {
    pub fn index(values: Vec<usize>) -> Vec<Keyframe<TextureAtlasSprite>> {
        values
            .iter()
            .map(|v| {
                Keyframe(TextureAtlasSprite {
                    index: *v,
                    ..Default::default()
                })
            })
            .collect()
    }
}
