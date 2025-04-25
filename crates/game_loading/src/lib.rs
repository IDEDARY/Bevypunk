use bevy::{prelude::*, platform::collections::HashMap};
use vleue_kinetoscope::*;

/// This struct can be spawned to hold handles you wish not
/// to deallocate when all entities are despawned which use them.
#[derive(Component)]
pub struct AssetLock {
    #[allow(dead_code)]
    pub assets: Vec<UntypedHandle>,
}


/// Priority assets loaded before the game start
#[derive(Resource, Default)]
pub struct PriorityAssets {
    pub video: HashMap<String, Handle<AnimatedImage>>,
}