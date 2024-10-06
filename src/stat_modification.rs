use crate::StatData;

/// A modification to apply to a stat
pub enum ModificationType {
    /// Adds the data contained to the stat
    Add(Box<dyn StatData>),
    /// Subtracts the data contained from the stat
    Sub(Box<dyn StatData>),
    /// Removes the stat entirely
    Remove,
    /// Resets the stat to default *IF* it exists
    Reset,
    /// Sets the stat to the data contained
    Set(Box<dyn StatData>),
}

impl ModificationType {
    /// Create a new [`ModificationType::Add`]
    pub fn add(stat_data: impl StatData) -> Self {
        Self::Add(Box::new(stat_data))
    }
    /// Create a new [`ModificationType::Sub`]
    pub fn sub(stat_data: impl StatData) -> Self {
        Self::Sub(Box::new(stat_data))
    }
    /// Create a new [`ModificationType::Set`]
    pub fn set(stat_data: impl StatData) -> Self {
        Self::Set(Box::new(stat_data))
    }
    /// Create a new [`ModificationType::Remove`]
    pub fn remove() -> Self {
        Self::Remove
    }
    /// Create a new [`ModificationType::Reset`]
    pub fn reset() -> Self {
        Self::Reset
    }
}
