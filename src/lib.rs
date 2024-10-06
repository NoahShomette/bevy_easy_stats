//! A simple system to track stats in one place using a single system.

use std::fmt::Debug;

use bevy::{prelude::SystemSet, utils::hashbrown::HashMap};
use downcast_rs::{impl_downcast, Downcast};
use dyn_clone::{clone_trait_object, DynClone};

#[cfg(feature = "serde")]
use serde::Deserialize;

pub use commands::{ModifyStatEntityCommands, StatCommandsExt, StatEntityCommandsExt};
pub use events::{ModifyStat, StatAppExt};

mod commands;
mod events;
mod implementations;
pub mod stat_modification;

#[derive(SystemSet, Hash, Debug, Eq, PartialEq, Clone)]
pub enum StatSystemSets {
    ApplyModifications,
}

/// An object containing mappings from a [`StatIdentifier`] to a [`StatData`]
#[derive(Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, Deserialize))]
pub struct Stats {
    pub stats: HashMap<String, Box<dyn StatData>>,
}

impl Stats {
    /// Creates a new stats object
    pub fn new() -> Stats {
        Stats::default()
    }

    /// Adds the given [`StatData`] to the given str id.
    ///
    /// Creates the entry if it doesnt exist
    pub fn add_to_stat_manual(&mut self, stat_id: &str, stat_data: Box<dyn StatData>) {
        let stat = self
            .stats
            .entry(stat_id.to_string())
            .or_insert(stat_data.default());
        stat.add(stat_data);
    }

    /// Sets the given [`StatData`] under the given str id.
    ///
    /// Creates the entry if it doesnt exist
    pub fn set_stat_manual(&mut self, stat_id: &str, stat_data: Box<dyn StatData>) {
        self.stats.insert(stat_id.to_string(), stat_data);
    }

    /// Removes the given stat and its corrosponding [`StatData`]
    pub fn remove_stat_manual(&mut self, stat_id: &str) {
        self.stats.remove(stat_id);
    }

    /// Sets the given stat to default if it exists. Otherwise does nothing
    pub fn reset_stat_manual(&mut self, stat_id: &str) {
        let Some(stat) = self.stats.get_mut(stat_id) else {
            return;
        };

        *stat = stat.default();
    }

    /// Subs the given [`StatData`] from the given str id.
    ///
    /// Creates the entry if it doesnt exist
    pub fn sub_from_stat_manual(&mut self, stat_id: &str, stat_data: Box<dyn StatData>) {
        let stat = self
            .stats
            .entry(stat_id.to_string())
            .or_insert(stat_data.default());
        stat.sub(stat_data);
    }

    /// Gets the [`StatData`] for the requested [`StatIdentifier`].
    #[allow(clippy::borrowed_box)]
    pub fn get_stat_manual(&self, stat_id: &str) -> Option<&Box<dyn StatData>> {
        self.stats.get(stat_id)
    }

    /// Adds the given [`StatData`] to the requested [`StatIdentifier`].
    ///
    /// Creates the entry if it doesnt exist
    pub fn add_to_stat(&mut self, stat_id: &dyn StatIdentifier, stat_data: Box<dyn StatData>) {
        self.add_to_stat_manual(stat_id.identifier(), stat_data)
    }

    /// Sets the given [`StatData`] to the requested [`StatIdentifier`].
    ///
    /// Creates the entry if it doesnt exist
    pub fn set_stat(&mut self, stat_id: &impl StatIdentifier, stat_data: Box<dyn StatData>) {
        self.set_stat_manual(stat_id.identifier(), stat_data)
    }

    /// Removes the given stat and its corrosponding [`StatData`]
    pub fn remove_stat(&mut self, stat_id: &impl StatIdentifier) {
        self.remove_stat_manual(stat_id.identifier())
    }

    /// Sets the given stat to default if it exists. Otherwise does nothing
    pub fn reset_stat(&mut self, stat_id: &impl StatIdentifier) {
        self.reset_stat_manual(stat_id.identifier())
    }

    /// Subs the given [`StatData`] from the requested [`StatIdentifier`].
    ///
    /// Creates the entry if it doesnt exist
    pub fn sub_from_stat(&mut self, stat_id: &impl StatIdentifier, stat_data: Box<dyn StatData>) {
        self.sub_from_stat_manual(stat_id.identifier(), stat_data)
    }

    /// Gets the [`StatData`] for the requested [`StatIdentifier`].
    #[allow(clippy::borrowed_box)]
    pub fn get_stat(&self, stat_id: &impl StatIdentifier) -> Option<&Box<dyn StatData>> {
        self.stats.get(stat_id.identifier())
    }

    /// Gets the [`StatData`] for the requested [`StatIdentifier`] and attempts to downcast it into the given type
    pub fn get_stat_downcast<'a, Stat: StatData + 'static>(
        &'a self,
        stat_id: &impl StatIdentifier,
    ) -> Option<&'a Stat> {
        let stat = self.stats.get(stat_id.identifier())?;

        stat.downcast_ref::<Stat>()
    }
}

/// Represents a unique stat
pub trait StatIdentifier {
    /// A unique identifier str for this specific stat identifier
    fn identifier(&self) -> &'static str;
}

impl StatIdentifier for Box<dyn StatIdentifier> {
    fn identifier(&self) -> &'static str {
        self.as_ref().identifier()
    }
}

/// A type that can be used as a stat
///
/// Must include `#[typetag::serde` on any implementations
#[cfg_attr(feature = "serde", typetag::serde(tag = "type"))]
pub trait StatData: Downcast + DynClone + Debug + Send + Sync {
    /// Constructs a new boxed [`StatData`]
    fn new(stat: Self) -> Box<Self>
    where
        Self: Sized,
    {
        Box::new(stat)
    }
    /// Creates a new instance of the same kind of stat data
    fn default(&self) -> Box<dyn StatData>;
    /// Adds the given other to this stat data
    fn add(&mut self, other: Box<dyn StatData>);
    /// Subtracts the given other from this stat data
    fn sub(&mut self, other: Box<dyn StatData>);
}
clone_trait_object!(StatData);
impl_downcast!(StatData);

#[cfg_attr(feature = "serde", typetag::serde)]
impl StatData for Box<dyn StatData> {
    fn default(&self) -> Box<dyn StatData> {
        self.as_ref().default()
    }

    fn add(&mut self, other: Box<dyn StatData>) {
        self.as_mut().add(other)
    }

    fn sub(&mut self, other: Box<dyn StatData>) {
        self.as_mut().sub(other)
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use bevy::prelude::{Component, World};
    use commands::StatEntityCommandsExt;
    #[cfg(feature = "serde")]
    use serde::Serialize;
    use stat_modification::ModificationType;

    use super::*;

    pub struct EnemiesKilled;

    impl StatIdentifier for EnemiesKilled {
        fn identifier(&self) -> &'static str {
            "Enemies Killed"
        }
    }

    #[test]
    fn core_features() {
        let mut stats = Stats::new();
        let id = EnemiesKilled;
        let stat_data = StatData::new(0u64);

        stats.add_to_stat(&id, stat_data);
        assert_eq!(*stats.get_stat_downcast::<u64>(&id).unwrap(), 0);
        stats.add_to_stat(&id, StatData::new(5u64));
        assert_eq!(*stats.get_stat_downcast::<u64>(&id).unwrap(), 5);

        stats.sub_from_stat(&id, StatData::new(3u64));
        assert_eq!(*stats.get_stat_downcast::<u64>(&id).unwrap(), 2);

        stats.reset_stat(&id);
        assert_eq!(*stats.get_stat_downcast::<u64>(&id).unwrap(), 0);

        stats.set_stat(&id, StatData::new(3u64));
        assert_eq!(*stats.get_stat_downcast::<u64>(&id).unwrap(), 3);

        // If the stat has already been added then it ignores invalid stat types
        stats.add_to_stat(&id, StatData::new(5.3f32));
        assert_eq!(*stats.get_stat_downcast::<u64>(&id).unwrap(), 3);

        stats.remove_stat(&id);
        assert_eq!(stats.get_stat_downcast::<u64>(&id), None);

        // If the stat has been cleared then you can add new stats of different types
        stats.add_to_stat(&id, StatData::new(5.3f32));
        assert_eq!(*stats.get_stat_downcast::<f32>(&id).unwrap(), 5.3);
    }

    pub struct PlayTime;

    impl StatIdentifier for PlayTime {
        fn identifier(&self) -> &'static str {
            "Playtime"
        }
    }

    #[test]
    fn duration() {
        let mut stats = Stats::new();
        let id = PlayTime;
        let stat_data = StatData::new(Duration::new(5, 0));

        stats.add_to_stat(&id, stat_data);

        assert_eq!(
            *stats.get_stat_downcast::<Duration>(&id).unwrap(),
            Duration::new(5, 0)
        );
        stats.add_to_stat(&id, StatData::new(Duration::new(5, 0)));
        assert_eq!(
            *stats.get_stat_downcast::<Duration>(&id).unwrap(),
            Duration::new(10, 0)
        );
        stats.sub_from_stat(&id, StatData::new(Duration::new(3, 0)));
        assert_eq!(
            *stats.get_stat_downcast::<Duration>(&id).unwrap(),
            Duration::new(7, 0)
        );

        // If the stat has already been added then it ignores invalid stat types
        stats.add_to_stat(&id, StatData::new(5.3f32));
        assert_eq!(
            *stats.get_stat_downcast::<Duration>(&id).unwrap(),
            Duration::new(7, 0)
        );
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

    #[derive(Debug, Clone, PartialEq, Eq)]
    #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
    pub struct CropsGrownStat {
        map: HashMap<String, u64>,
    }

    impl CropsGrownStat {
        fn new(contents: Vec<(String, u64)>) -> CropsGrownStat {
            let mut hashmap = HashMap::default();
            for item in contents {
                hashmap.insert(item.0, item.1);
            }
            Self { map: hashmap }
        }
    }

    #[cfg_attr(feature = "serde", typetag::serde)]
    impl StatData for CropsGrownStat {
        #[doc = " Creates a new instance of the same kind of stat data"]
        fn default(&self) -> Box<dyn StatData> {
            Box::new(CropsGrownStat {
                map: HashMap::default(),
            })
        }

        #[doc = " Adds the given other to this stat data"]
        fn add(&mut self, other: Box<dyn StatData>) {
            if let Some(other) = other.downcast_ref::<CropsGrownStat>() {
                for (id, amount) in other.map.iter() {
                    let entry = self.map.entry(id.clone()).or_default();
                    *entry += amount;
                }
            }
        }

        #[doc = " Subtracts the given other from this stat data"]
        fn sub(&mut self, other: Box<dyn StatData>) {
            if let Some(other) = other.downcast_ref::<CropsGrownStat>() {
                for (id, amount) in other.map.iter() {
                    let entry = self.map.entry(id.clone()).or_default();
                    *entry -= amount;
                }
            }
        }
    }

    #[test]
    fn custom_stat_data() {
        let mut world = World::new();
        let entity = world
            .spawn(EntityStats {
                stats: Stats::new(),
            })
            .id();

        let mut commands = world.commands();

        commands.entity(entity).modify_stat::<EntityStats>(
            EnemiesKilled,
            ModificationType::add(CropsGrownStat::new(vec![("Potato".to_string(), 5)])),
        );
        world.flush();

        assert_eq!(
            *world
                .entity(entity)
                .get::<EntityStats>()
                .unwrap()
                .stats
                .get_stat_downcast::<CropsGrownStat>(&EnemiesKilled)
                .unwrap(),
            CropsGrownStat::new(vec![("Potato".to_string(), 5)])
        );
        let mut commands = world.commands();

        commands.entity(entity).modify_stat::<EntityStats>(
            EnemiesKilled,
            ModificationType::add(CropsGrownStat::new(vec![("Potato".to_string(), 5)])),
        );
        commands.entity(entity).modify_stat::<EntityStats>(
            EnemiesKilled,
            ModificationType::add(CropsGrownStat::new(vec![("Dandelion".to_string(), 500)])),
        );
        world.flush();
        assert_eq!(
            *world
                .entity(entity)
                .get::<EntityStats>()
                .unwrap()
                .stats
                .get_stat_downcast::<CropsGrownStat>(&EnemiesKilled)
                .unwrap(),
            CropsGrownStat::new(vec![
                ("Dandelion".to_string(), 500),
                ("Potato".to_string(), 10)
            ])
        );
    }
}
