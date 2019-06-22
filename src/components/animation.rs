use amethyst::{
    animation::AnimationSetPrefab,
    assets::{PrefabData, ProgressCounter},
    derive::PrefabData,
    ecs::Entity,
    ecs::{Component, DenseVecStorage},
    error::Error,
    renderer::sprite::{prefab::SpriteScenePrefab, SpriteRender},
};

use serde::{Deserialize, Serialize};

/// `AnimationId` is the ID used in an `AnimationSet`, used to identify which
/// animation to play.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, PartialOrd, Serialize)]
pub enum AnimationId {
    Die,
    Explode,
    Jump,
    Move,
    Idle,
    Shoot,
}

/// `AnimationPrefabData` type used for loading of `SpriteScene`s and their
/// `AnimationSet`s.
#[derive(Clone, Debug, Deserialize, PrefabData)]
pub struct AnimationPrefabData {
    /// Information for rendering a scene with sprites.
    sprite_scene: SpriteScenePrefab,
    /// Аll animations that can be run on the `Entity`.
    animation_set: AnimationSetPrefab<AnimationId, SpriteRender>,
}

#[derive(Component, Debug)]
#[storage(DenseVecStorage)]
pub struct Animation {
    pub current: AnimationId,
    pub types: Vec<AnimationId>,
}