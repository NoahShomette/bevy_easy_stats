use std::marker::PhantomData;

use bevy::{
    app::{App, PostUpdate},
    prelude::{on_event, Event, EventReader, IntoSystemConfigs, ResMut, Resource},
};

use crate::{stat_modification::ModificationType, StatData, StatIdentifier, StatSystemSets, Stats};

pub trait StatAppExt {
    /// Register a new stat resource, adds the [`ModifyStats`] event, and adds a system to automatically handle those events and update the stats on event.
    fn register_stat_resource<
        StatCollection: AsMut<Stats> + Send + Sync + 'static + Resource + Default,
    >(
        &mut self,
    );
}

impl StatAppExt for App {
    fn register_stat_resource<
        StatCollection: AsMut<Stats> + Send + Sync + 'static + Resource + Default,
    >(
        &mut self,
    ) {
        self.add_event::<ModifyStat<StatCollection>>();
        self.init_resource::<StatCollection>();
        self.add_systems(
            PostUpdate,
            handle_stat_modifications::<StatCollection>
                .run_if(on_event::<ModifyStat<StatCollection>>)
                .in_set(StatSystemSets::ApplyModifications),
        );
    }
}

/// An event that modifies a stat in a resource
#[derive(Event)]
pub struct ModifyStat<StatCollection: AsMut<Stats>> {
    stat_id: Box<dyn StatIdentifier + 'static + Send + Sync>,
    modification_type: ModificationType,
    pd: PhantomData<StatCollection>,
}

impl<StatCollection: AsMut<Stats>> ModifyStat<StatCollection> {
    /// Create a new event
    pub fn new(
        stat_id: impl StatIdentifier + 'static + Send + Sync,
        modification_type: ModificationType,
    ) -> Self {
        Self {
            stat_id: Box::new(stat_id),
            modification_type,
            pd: PhantomData,
        }
    }

    /// Create a new add event
    pub fn add(
        stat_id: impl StatIdentifier + 'static + Send + Sync,
        stat_data: impl StatData,
    ) -> Self {
        Self {
            stat_id: Box::new(stat_id),
            modification_type: ModificationType::add(stat_data),
            pd: PhantomData,
        }
    }

    /// Create a new sub event
    pub fn sub(
        stat_id: impl StatIdentifier + 'static + Send + Sync,
        stat_data: impl StatData,
    ) -> Self {
        Self {
            stat_id: Box::new(stat_id),
            modification_type: ModificationType::sub(stat_data),
            pd: PhantomData,
        }
    }

    /// Create a new set event
    pub fn set(
        stat_id: impl StatIdentifier + 'static + Send + Sync,
        stat_data: impl StatData,
    ) -> Self {
        Self {
            stat_id: Box::new(stat_id),
            modification_type: ModificationType::set(stat_data),
            pd: PhantomData,
        }
    }

    /// Create a new remove event
    pub fn remove(stat_id: impl StatIdentifier + 'static + Send + Sync) -> Self {
        Self {
            stat_id: Box::new(stat_id),
            modification_type: ModificationType::remove(),
            pd: PhantomData,
        }
    }

    /// Create a new reset event
    pub fn reset(stat_id: impl StatIdentifier + 'static + Send + Sync) -> Self {
        Self {
            stat_id: Box::new(stat_id),
            modification_type: ModificationType::reset(),
            pd: PhantomData,
        }
    }
}

fn handle_stat_modifications<StatCollection: AsMut<Stats> + Send + Sync + 'static + Resource>(
    mut resource: ResMut<StatCollection>,
    mut event_reader: EventReader<ModifyStat<StatCollection>>,
) {
    let stats = resource.as_mut().as_mut();
    for event in event_reader.read() {
        match &event.modification_type {
            ModificationType::Add(data) => {
                stats.add_to_stat_manual(event.stat_id.identifier(), data.clone())
            }
            ModificationType::Sub(data) => {
                stats.sub_from_stat_manual(event.stat_id.identifier(), data.clone())
            }
            ModificationType::Remove => stats.remove_stat_manual(event.stat_id.identifier()),
            ModificationType::Set(data) => {
                stats.set_stat_manual(event.stat_id.identifier(), data.clone())
            }
            ModificationType::Reset => stats.reset_stat_manual(event.stat_id.identifier()),
        }
    }
}

#[cfg(test)]
mod tests {
    use bevy::{
        app::{App, PostUpdate, PreUpdate},
        prelude::{EventWriter, IntoSystemConfigs, Res, Resource},
    };

    use crate::{
        events::{ModifyStat, StatAppExt},
        StatIdentifier, StatSystemSets, Stats,
    };

    #[derive(Hash)]
    pub struct EnemiesKilled;

    impl StatIdentifier for EnemiesKilled {
        fn identifier(&self) -> &'static str {
            "Enemies Killed"
        }
    }

    #[derive(Resource, Default)]
    pub struct ResourceStats {
        stats: Stats,
    }

    impl AsMut<Stats> for ResourceStats {
        fn as_mut(&mut self) -> &mut Stats {
            &mut self.stats
        }
    }

    #[test]
    fn resource_stats() {
        let mut app = App::new();
        app.insert_resource(ResourceStats {
            stats: Stats::default(),
        });

        app.register_stat_resource::<ResourceStats>();
        app.add_systems(
            PreUpdate,
            |mut event_writer: EventWriter<ModifyStat<ResourceStats>>| {
                event_writer.send(ModifyStat::add(EnemiesKilled, 2u64));
            },
        );
        app.add_systems(
            PostUpdate,
            (|res: Res<ResourceStats>| {
                assert_eq!(
                    *res.as_ref()
                        .stats
                        .get_stat_downcast::<u64>(&EnemiesKilled)
                        .unwrap(),
                    2u64
                );
            })
            .after(StatSystemSets::ApplyModifications),
        );
        app.run();
    }
}
