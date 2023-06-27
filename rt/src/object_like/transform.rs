use std::ops::Deref;

use either::Either;
use glam::EulerRot;
use serde::{Deserialize, Serialize};

use crate::{Float, Rotor, Transform, Vector};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TransformDeserialize {
    #[serde(with = "either::serde_untagged_optional")]
    params: Option<Either<TransformParams, Vec<TransformParams>>>,
}

impl Deref for TransformDeserialize {
    type Target = Option<Either<TransformParams, Vec<TransformParams>>>;

    fn deref(&self) -> &Self::Target {
        &self.params
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(default)]
#[serde(deny_unknown_fields)]
pub struct TransformParams {
    #[serde(with = "either::serde_untagged")]
    #[serde(alias = "scaling")]
    pub scale: Either<Float, Vector>,
    #[serde(alias = "rotation")]
    pub rotate: Vector,
    #[serde(alias = "translation")]
    pub translate: Vector,
}

impl Default for TransformParams {
    fn default() -> Self {
        Self {
            scale: Either::Right(Vector::ONE),
            rotate: Vector::ZERO,
            translate: Vector::ZERO,
        }
    }
}

pub fn parse_transform(transforms: &TransformDeserialize) -> Transform {
    let default = Either::Left(TransformParams::default());

    transforms.as_ref()
        .unwrap_or(&default)
        .as_ref()
        .map_left(parse_single)
        .left_or_else(|vec_params| {
            vec_params.iter()
                .fold(Transform::IDENTITY, |left_mat, right_params| {
                    let right_mat = parse_single(right_params);
                    left_mat * right_mat
                })
        })
}

fn parse_single(transform: &TransformParams) -> Transform {
    Transform::from_scale_rotation_translation(
        transform.scale.right_or_else(Vector::splat).into(),
        Rotor::from_euler(
            EulerRot::YXZ,
            transform.rotate.y.to_radians(),
            transform.rotate.x.to_radians(),
            transform.rotate.z.to_radians(),
        ),
        transform.translate.into(),
    )
}
