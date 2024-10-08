use std::hash::Hash;

use naia_shared::{EntityAuthStatus, ReplicaRefWrapper, ReplicatedComponent, WorldRefType};

use crate::{Client, ReplicationConfig};

// EntityRef
pub struct EntityRef<'s, E: Copy + Eq + Hash + Send + Sync, W: WorldRefType<E>> {
    client: &'s Client<E>,
    world: W,
    entity: E,
}

impl<'s, E: Copy + Eq + Hash + Send + Sync, W: WorldRefType<E>> EntityRef<'s, E, W> {
    pub fn new(client: &'s Client<E>, world: W, entity: &E) -> Self {
        EntityRef {
            client,
            world,
            entity: *entity,
        }
    }

    pub fn id(&self) -> E {
        self.entity
    }

    pub fn has_component<R: ReplicatedComponent>(&self) -> bool {
        self.world.has_component::<R>(&self.entity)
    }

    pub fn component<R: ReplicatedComponent>(&self) -> Option<ReplicaRefWrapper<R>> {
        self.world.component::<R>(&self.entity)
    }

    pub fn replication_config(&self) -> Option<ReplicationConfig> {
        self.client.entity_replication_config(&self.entity)
    }

    pub fn authority(&self) -> Option<EntityAuthStatus> {
        self.client.entity_authority_status(&self.entity)
    }
}
