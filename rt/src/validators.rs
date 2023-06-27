#![allow(dead_code)]

use serde::{Deserialize, Deserializer};
use serde::de::Unexpected;

use crate::{Float, Vector};

pub fn de_vec3_normalized<'de, D>(deserializer: D) -> Result<Vector, D::Error>
where D: Deserializer<'de> {
    let vec = Vector::deserialize(deserializer)?;

    let norm = vec.try_normalize().ok_or_else(|| {
        serde::de::Error::invalid_value(
            Unexpected::Other(&format!("{vec}")),
            &"non-zero vec3",
        )
    })?;

    Ok(norm)
}

pub fn de_vec3_gte_0<'de, D>(deserializer: D) -> Result<Vector, D::Error>
where D: Deserializer<'de> {
    let vec = Vector::deserialize(deserializer)?;

    if vec.is_negative_bitmask() > 0 {
        return Err(serde::de::Error::invalid_value(
            Unexpected::Other(&format!("{vec}")),
            &"vec3 with all components positive"),
        );
    }

    Ok(vec)
}

pub fn de_vec3_one_clamped<'de, D>(deserializer: D) -> Result<Vector, D::Error>
where D: Deserializer<'de> {
    let vec = Vector::deserialize(deserializer)?;

    let range = 0.0..=1.0;

    if !range.contains(&vec.x) || !range.contains(&vec.y) || !range.contains(&vec.z) {
        return Err(serde::de::Error::invalid_value(
            Unexpected::Other(&format!("{vec}")),
            &"vec3 with all components between 0 and 1"),
        );
    }

    Ok(vec)
}

pub fn de_emission<'de, D>(deserializer: D) -> Result<Option<Vector>, D::Error>
where D: Deserializer<'de> {
    let vec = Option::deserialize(deserializer)?;

    let vec: Vector = match vec {
        None => return Ok(None),
        Some(vec) => vec,
    };

    if vec == Vector::ZERO {
        return Ok(None);
    }

    if vec.is_negative_bitmask() > 0 {
        return Err(serde::de::Error::invalid_value(
            Unexpected::Other(&format!("{vec}")),
            &"emission can't be negative!"),
        );
    }

    Ok(Some(vec))
}

pub fn de_float_fov<'de, D>(deserializer: D) -> Result<Float, D::Error>
where D: Deserializer<'de> {
    let v = Float::deserialize(deserializer)?;

    if !(0.0..=170.0).contains(&v) {
        return Err(serde::de::Error::invalid_value(
            Unexpected::Float(v as f64),
            &"number between 0 and 170"),
        );
    }

    Ok(v)
}

pub fn de_float_gt_0<'de, D>(deserializer: D) -> Result<Float, D::Error>
where D: Deserializer<'de> {
    let v = Float::deserialize(deserializer)?;

    if v <= 0.0 {
        return Err(serde::de::Error::invalid_value(
            Unexpected::Float(v as f64),
            &"number greater than 0"),
        );
    }

    Ok(v)
}

pub fn de_float_gte_0<'de, D>(deserializer: D) -> Result<Float, D::Error>
where D: Deserializer<'de> {
    let v = Float::deserialize(deserializer)?;

    if v < 0.0 {
        return Err(serde::de::Error::invalid_value(
            Unexpected::Float(v as f64),
            &"number greater than or equal to 0"),
        );
    }

    Ok(v)
}

pub fn de_float_one_clamped<'de, D>(deserializer: D) -> Result<Float, D::Error>
where D: Deserializer<'de> {
    let v = Float::deserialize(deserializer)?;

    if !(0.0..=1.0).contains(&v) {
        return Err(serde::de::Error::invalid_value(
            Unexpected::Float(v as f64),
            &"number between 0 and 1"),
        );
    }

    Ok(v)
}

pub fn de_float_gte_1<'de, D>(deserializer: D) -> Result<Float, D::Error>
where D: Deserializer<'de> {
    let v = Float::deserialize(deserializer)?;

    if v < 1.0 {
        return Err(serde::de::Error::invalid_value(
            Unexpected::Float(v as f64),
            &"number greater than or equal to 1"),
        );
    }

    Ok(v)
}

pub fn de_u32_gt_0<'de, D>(deserializer: D) -> Result<u32, D::Error>
where D: Deserializer<'de> {
    let v = u32::deserialize(deserializer)?;

    if v == 0 {
        return Err(serde::de::Error::invalid_value(
            Unexpected::Unsigned(v as u64),
            &"integer greater than 0"),
        );
    }

    Ok(v)
}