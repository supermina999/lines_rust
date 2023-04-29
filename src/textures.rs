use bevy::prelude::*;
use crate::constants::*;

#[derive(Resource)]
pub struct Textures {
    pub cell: Handle<Image>,
    pub circles: Vec<Handle<Image>>
}

impl FromWorld for Textures {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        Textures {
            cell: asset_server.load("background.png"),
            circles: {
                let mut circles: Vec<Handle<Image>> = Vec::new();
                for idx in 0..CIRCLE_KINDS {
                    circles.push(asset_server.load(format!("circle{idx}.png")))
                }
                circles
            }
        }
    }
}
