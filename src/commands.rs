use std::marker::PhantomData;

use bevy::{
    ecs::system::{EntityCommand, EntityCommands},
    prelude::{Commands, Component, Entity, World},
};

use crate::{stat_modification::ModificationType, StatData, StatIdentifier, Stats};

/// Make changes to an entities stats in a deferred patter using commands.
pub struct ModifyStatEntityCommands<
    'a,
    StatCollection: AsMut<Stats> + Send + Sync + 'static + Component,
> {
    target_entity: Entity,
    target_component: PhantomData<StatCollection>,
    commands: Commands<'a, 'a>,
}

impl<'a, StatCollection: AsMut<Stats> + Send + Sync + 'static + Component>
    ModifyStatEntityCommands<'a, StatCollection>
{
    /// Return the entity id that is targeted for stat changes
    pub fn id(&self) -> Entity {
        self.target_entity
    }

    /// Change the target entity
    pub fn entity(&mut self, entity: Entity) {
        self.target_entity = entity
    }

    /// Return a reference to the commands object contained
    pub fn commands(&mut self) -> &mut Commands<'a, 'a> {
        &mut self.commands
    }
}

impl<StatCollection: AsMut<Stats> + Send + Sync + 'static + Component>
    ModifyStatEntityCommands<'_, StatCollection>
{
    /// Get entity commands for the targeted entity
    pub fn entity_commands(&mut self) -> EntityCommands<'_> {
        let id = self.id();
        self.commands().entity(id)
    }

    /// Queue a command to perform an add with the given [`StatData`] to the targeted [`StatIdentifier`]
    pub fn add(
        &mut self,
        stat_id: impl StatIdentifier + 'static + Send + Sync,
        stat_data: impl StatData,
    ) -> &mut Self {
        self.entity_commands()
            .queue(modify_entity_stat::<StatCollection>(
                stat_id,
                ModificationType::add(stat_data),
            ));
        self
    }

    /// Queue a command to perform a sub with the given [`StatData`] to the targeted [`StatIdentifier`]
    pub fn sub(
        &mut self,
        stat_id: impl StatIdentifier + 'static + Send + Sync,
        stat_data: impl StatData,
    ) -> &mut Self {
        self.entity_commands()
            .queue(modify_entity_stat::<StatCollection>(
                stat_id,
                ModificationType::sub(stat_data),
            ));
        self
    }

    /// Queue a command to perform a set with the given [`StatData`] to the targeted [`StatIdentifier`]
    pub fn set(
        &mut self,
        stat_id: impl StatIdentifier + 'static + Send + Sync,
        stat_data: impl StatData,
    ) -> &mut Self {
        self.entity_commands()
            .queue(modify_entity_stat::<StatCollection>(
                stat_id,
                ModificationType::set(stat_data),
            ));
        self
    }

    /// Queue a command to perform a remove to the targeted [`StatIdentifier`]
    pub fn remove(&mut self, stat_id: impl StatIdentifier + 'static + Send + Sync) -> &mut Self {
        self.entity_commands()
            .queue(modify_entity_stat::<StatCollection>(
                stat_id,
                ModificationType::remove(),
            ));
        self
    }

    /// Queue a command to perform a reset to the targeted [`StatIdentifier`]
    pub fn reset(&mut self, stat_id: impl StatIdentifier + 'static + Send + Sync) -> &mut Self {
        self.entity_commands()
            .queue(modify_entity_stat::<StatCollection>(
                stat_id,
                ModificationType::reset(),
            ));
        self
    }
}

pub trait StatCommandsExt {
    /// Returns a [`ModifyStatEntityCommands`] object for the given entity
    fn entity_stats<StatCollection: AsMut<Stats> + Send + Sync + 'static + Component>(
        &mut self,
        entity: Entity,
    ) -> ModifyStatEntityCommands<'_, StatCollection>;

    /// Modify a single stat on an entity
    fn modify_stat<StatCollection: AsMut<Stats> + Send + Sync + 'static + Component>(
        &mut self,
        entity: Entity,
        stat_id: impl StatIdentifier + 'static + Send + Sync,
        modification_type: ModificationType,
    );
}

impl<'a> StatCommandsExt for Commands<'a, 'a> {
    /// Get access to an object to modify entity stats
    fn entity_stats<StatCollection: AsMut<Stats> + Send + Sync + 'static + Component>(
        &mut self,
        entity: Entity,
    ) -> ModifyStatEntityCommands<'_, StatCollection> {
        ModifyStatEntityCommands {
            target_entity: entity,
            target_component: PhantomData,
            commands: self.reborrow(),
        }
    }

    /// Modify a single stat on an entity
    fn modify_stat<StatCollection: AsMut<Stats> + Send + Sync + 'static + Component>(
        &mut self,
        entity: Entity,
        stat_id: impl StatIdentifier + 'static + Send + Sync,
        modification_type: ModificationType,
    ) {
        self.entity(entity)
            .modify_stat::<StatCollection>(stat_id, modification_type);
    }
}

pub trait StatEntityCommandsExt {
    /// Get a special command pattern object to affect changes to an entity
    fn entity_stats<StatCollection: AsMut<Stats> + Send + Sync + 'static + Component>(
        &mut self,
    ) -> ModifyStatEntityCommands<'_, StatCollection>;

    /// Modify a single stat on an entity
    fn modify_stat<StatCollection: AsMut<Stats> + Send + Sync + 'static + Component>(
        &mut self,
        stat_id: impl StatIdentifier + 'static + Send + Sync,
        modification_type: ModificationType,
    );
}

impl<'a> StatEntityCommandsExt for EntityCommands<'a> {
    fn modify_stat<StatCollection: AsMut<Stats> + Send + Sync + 'static + Component>(
        &mut self,
        stat_id: impl StatIdentifier + 'static + Send + Sync,
        modification_type: ModificationType,
    ) {
        self.queue(modify_entity_stat::<StatCollection>(
            stat_id,
            modification_type,
        ));
    }

    fn entity_stats<StatCollection: AsMut<Stats> + Send + Sync + 'static + Component>(
        &mut self,
    ) -> ModifyStatEntityCommands<'_, StatCollection> {
        ModifyStatEntityCommands {
            target_entity: self.id(),
            target_component: PhantomData,
            commands: self.commands(),
        }
    }
}

fn modify_entity_stat<StatCollection: AsMut<Stats> + Send + Sync + 'static + Component>(
    stat_id: impl StatIdentifier + 'static + Send + Sync,
    modification_type: ModificationType,
) -> impl EntityCommand {
    move |entity: Entity, world: &mut World| {
        if let Ok(mut entity_mut) = world.get_entity_mut(entity) {
            if let Some(mut stat_collection) = entity_mut.get_mut::<StatCollection>() {
                let stats = stat_collection.as_mut().as_mut();

                match modification_type {
                    ModificationType::Add(data) => {
                        stats.add_to_stat_manual(stat_id.identifier(), data)
                    }
                    ModificationType::Sub(data) => {
                        stats.sub_from_stat_manual(stat_id.identifier(), data)
                    }
                    ModificationType::Remove => stats.remove_stat_manual(stat_id.identifier()),
                    ModificationType::Set(data) => {
                        stats.set_stat_manual(stat_id.identifier(), data)
                    }
                    ModificationType::Reset => stats.reset_stat_manual(stat_id.identifier()),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Hash)]
    pub struct EnemiesKilled;

    impl StatIdentifier for EnemiesKilled {
        fn identifier(&self) -> &'static str {
            "Enemies Killed"
        }
    }

    #[derive(Component)]
    pub struct EntityStats {
        stats: Stats,
    }

    impl AsMut<Stats> for EntityStats {
        fn as_mut(&mut self) -> &mut Stats {
            &mut self.stats
        }
    }

    #[test]
    fn entity_commands() {
        let mut world = World::new();
        let entity = world
            .spawn(EntityStats {
                stats: Stats::new(),
            })
            .id();

        let mut commands = world.commands();

        let mut stats = commands.entity_stats::<EntityStats>(entity);
        stats.add(EnemiesKilled, 5u64);
        stats.add(EnemiesKilled, 10u64);
        world.flush();

        assert_eq!(
            *world
                .entity(entity)
                .get::<EntityStats>()
                .unwrap()
                .stats
                .get_stat_downcast::<u64>(&EnemiesKilled)
                .unwrap(),
            15u64
        );

        let mut commands = world.commands();
        let mut stats = commands.entity_stats::<EntityStats>(entity);
        stats.reset(EnemiesKilled);
        stats.add(EnemiesKilled, 15u64);
        stats.sub(EnemiesKilled, 5u64);
        stats.sub(EnemiesKilled, 7u64);
        world.flush();

        assert_eq!(
            *world
                .entity(entity)
                .get::<EntityStats>()
                .unwrap()
                .stats
                .get_stat_downcast::<u64>(&EnemiesKilled)
                .unwrap(),
            3u64
        );

        let mut commands = world.commands();
        let mut stats = commands.entity_stats::<EntityStats>(entity);
        stats.set(EnemiesKilled, 7u64);
        world.flush();

        assert_eq!(
            *world
                .entity(entity)
                .get::<EntityStats>()
                .unwrap()
                .stats
                .get_stat_downcast::<u64>(&EnemiesKilled)
                .unwrap(),
            7u64
        );

        let mut commands = world.commands();
        let mut stats = commands.entity_stats::<EntityStats>(entity);
        stats.remove(EnemiesKilled);
        world.flush();

        assert_eq!(
            world
                .entity(entity)
                .get::<EntityStats>()
                .unwrap()
                .stats
                .get_stat_downcast::<u64>(&EnemiesKilled),
            None
        );
    }
}
