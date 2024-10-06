# `Bevy Easy Stats`

A simple and easy way to keep track of stats in one place.

## Usage

### 1. Create a new stat identifier

```rust
pub struct EnemiesKilled;
impl StatIdentifier for EnemiesKilled {
    fn identifier(&self) -> &'static str {
        "Enemies Killed"
    }
}
```

### 2. bevy_easy_stats natively supports components and resources as stat collections. These can be automatically updated using built in events and command extensions

```rust
#[derive(Resource, Default)]
pub struct ResourceStats {
    stats: Stats,
}
impl AsMut<Stats> for ResourceStats {
    fn as_mut(&mut self) -> &mut Stats {
        &mut self.stats
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

// If you want to take advantage of built in event based modification functionality make sure to register the stat resource in the app
app.register_stat_resource::<ResourceStats>();
```

### 3. Modify stats using several different ways to match your needs

```rust
fn modify(mut event_writer: EventWriter<ModifyStat<ResourceStats>>, mut commands: Commands) {
  // entity commands ext to modify component based stats
  commands.entity(entity).modify_stat::<EntityStats>(EnemiesKilled, ModificationType::add(5u64));

  /// Gain acces to an object to queue multiple commands across different types for component based stats
  let mut stats = commands.entity_stats::<EntityStats>(entity);
  stats.add(EnemiesKilled, 5u64);
  stats.add(TimePlayed, Duration::new(5, 0));

  // Use events to modify resource based stats
  event_writer.send(ModifyStat::add(EnemiesKilled, 2u64));
  event_writer.send(ModifyStat::add(TimePlayed, Duration::new(5, 0)));
}
```

### 4. Create your own stat types to keep track of whatever you want

```rust
// new stat data type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CropsGrownStat {
    map: HashMap<String, u64>,
}

// Impl the StatData trait
#[typetag::serde]
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

event_writer.send(ModifyStat::add(CropsGrown, CropsGrownStat::new(vec![("Potato".to_string(), 5)])));
event_writer.send(ModifyStat::add(CropsGrown, CropsGrownStat::new(vec![("Dandelion".to_string(), 100)])));
stats.get_stat_downcast::<CropsGrownStat>(&CropsGrown).unwrap() = CropsGrownStat::new(vec![("Dandelion".to_string(), 100), ("Potato".to_string(), 5)])
```

## Future

- Swap from using TypeTag to using SerdeTagged for wasm support
  - [SerdeTagged](https://github.com/qzed/serde_tagged)
  - [TypeTag issue](https://github.com/dtolnay/typetag/issues/54)
