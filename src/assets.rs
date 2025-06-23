use std::collections::VecDeque;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<ResourceHandles>();
    app.add_systems(PreUpdate, load_resource_assets);
}

pub trait LoadResource {
    fn load_resource<T: Resource + Asset + Clone + FromWorld>(&mut self) -> &mut Self;
}

impl LoadResource for App {
    fn load_resource<T: Resource + Asset + Clone + FromWorld>(&mut self) -> &mut Self {
        self.init_asset::<T>();

        let world = self.world_mut();
        let value = T::from_world(world);
        let assets = world.resource::<AssetServer>();
        let handle = assets.add(value);
        let mut handles = world.resource_mut::<ResourceHandles>();
        handles
            .pending
            .push_back((handle.untyped(), |world, handle| {
                let assets = world.resource::<Assets<T>>();
                if let Some(value) = assets.get(handle.id().typed::<T>()) {
                    world.insert_resource(value.clone());
                }
            }));

        self
    }
}

type InsertLoadedResource = fn(&mut World, &UntypedHandle);

#[derive(Resource, Default)]
pub struct ResourceHandles {
    pending: VecDeque<(UntypedHandle, InsertLoadedResource)>,
    finished: Vec<UntypedHandle>,
}

impl ResourceHandles {
    pub fn is_all_loaded(&self) -> bool {
        self.pending.is_empty()
    }
}

fn load_resource_assets(world: &mut World) {
    world.resource_scope(|world, mut resource_handles: Mut<ResourceHandles>| {
        world.resource_scope(|world, assets: Mut<AssetServer>| {
            for _ in 0..resource_handles.pending.len() {
                let (handle, insert_fn) = resource_handles.pending.pop_front().unwrap();
                if assets.is_loaded_with_dependencies(&handle) {
                    insert_fn(world, &handle);
                    resource_handles.finished.push(handle);
                } else {
                    resource_handles.pending.push_back((handle, insert_fn));
                }
            }
        });
    });
}

pub mod common_conditions {
    use super::ResourceHandles;
    use bevy::prelude::*;

    pub fn all_assets_loaded(handles: Res<ResourceHandles>) -> bool {
        handles.is_all_loaded()
    }
}
