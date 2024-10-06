use std::time::Duration;

use crate::StatData;

#[cfg_attr(feature = "serde", typetag::serde)]
impl StatData for Duration {
    fn add(&mut self, other: Box<dyn StatData>) {
        if let Some(other) = other.downcast_ref::<Duration>() {
            *self += *other;
        }
    }

    fn default(&self) -> Box<dyn StatData> {
        Box::new(Duration::ZERO)
    }

    fn sub(&mut self, other: Box<dyn StatData>) {
        if let Some(other) = other.downcast_ref::<Duration>() {
            *self -= *other;
        }
    }
}

// U ints ---------------------------------------------------

#[cfg_attr(feature = "serde", typetag::serde)]
impl StatData for u128 {
    fn add(&mut self, other: Box<dyn StatData>) {
        if let Some(other) = other.downcast_ref::<u128>() {
            *self = self.saturating_add(*other);
        }
    }

    fn default(&self) -> Box<dyn StatData> {
        Box::new(0u128)
    }

    fn sub(&mut self, other: Box<dyn StatData>) {
        if let Some(other) = other.downcast_ref::<u128>() {
            *self = self.saturating_sub(*other);
        }
    }
}

#[cfg_attr(feature = "serde", typetag::serde)]
impl StatData for u64 {
    fn add(&mut self, other: Box<dyn StatData>) {
        if let Some(other) = other.downcast_ref::<u64>() {
            *self = self.saturating_add(*other);
        }
    }

    fn default(&self) -> Box<dyn StatData> {
        Box::new(0u64)
    }

    fn sub(&mut self, other: Box<dyn StatData>) {
        if let Some(other) = other.downcast_ref::<u64>() {
            *self = self.saturating_sub(*other);
        }
    }
}

#[cfg_attr(feature = "serde", typetag::serde)]
impl StatData for u32 {
    fn add(&mut self, other: Box<dyn StatData>) {
        if let Some(other) = other.downcast_ref::<u32>() {
            *self = self.saturating_add(*other);
        }
    }

    fn default(&self) -> Box<dyn StatData> {
        Box::new(0u32)
    }

    fn sub(&mut self, other: Box<dyn StatData>) {
        if let Some(other) = other.downcast_ref::<u32>() {
            *self = self.saturating_sub(*other);
        }
    }
}

#[cfg_attr(feature = "serde", typetag::serde)]
impl StatData for u16 {
    fn add(&mut self, other: Box<dyn StatData>) {
        if let Some(other) = other.downcast_ref::<u16>() {
            *self = self.saturating_add(*other);
        }
    }

    fn default(&self) -> Box<dyn StatData> {
        Box::new(0u16)
    }

    fn sub(&mut self, other: Box<dyn StatData>) {
        if let Some(other) = other.downcast_ref::<u16>() {
            *self = self.saturating_sub(*other);
        }
    }
}

#[cfg_attr(feature = "serde", typetag::serde)]
impl StatData for u8 {
    fn add(&mut self, other: Box<dyn StatData>) {
        if let Some(other) = other.downcast_ref::<u8>() {
            *self = self.saturating_add(*other);
        }
    }

    fn default(&self) -> Box<dyn StatData> {
        Box::new(0u8)
    }

    fn sub(&mut self, other: Box<dyn StatData>) {
        if let Some(other) = other.downcast_ref::<u8>() {
            *self = self.saturating_sub(*other);
        }
    }
}

// FLOATS ---------------------------------------------------

#[cfg_attr(feature = "serde", typetag::serde)]
impl StatData for f64 {
    fn add(&mut self, other: Box<dyn StatData>) {
        if let Some(other) = other.downcast_ref::<f64>() {
            *self += other;
        }
    }

    fn default(&self) -> Box<dyn StatData> {
        Box::new(0f64)
    }

    fn sub(&mut self, other: Box<dyn StatData>) {
        if let Some(other) = other.downcast_ref::<f64>() {
            *self -= other;
        }
    }
}

#[cfg_attr(feature = "serde", typetag::serde)]
impl StatData for f32 {
    fn add(&mut self, other: Box<dyn StatData>) {
        if let Some(other) = other.downcast_ref::<f32>() {
            *self += other;
        }
    }

    fn default(&self) -> Box<dyn StatData> {
        Box::new(0f32)
    }

    fn sub(&mut self, other: Box<dyn StatData>) {
        if let Some(other) = other.downcast_ref::<f32>() {
            *self -= other;
        }
    }
}

// Signed Ints ---------------------------------------------------

#[cfg_attr(feature = "serde", typetag::serde)]
impl StatData for i128 {
    fn add(&mut self, other: Box<dyn StatData>) {
        if let Some(other) = other.downcast_ref::<i128>() {
            *self += other;
        }
    }

    fn default(&self) -> Box<dyn StatData> {
        Box::new(0i128)
    }

    fn sub(&mut self, other: Box<dyn StatData>) {
        if let Some(other) = other.downcast_ref::<i128>() {
            *self -= other;
        }
    }
}

#[cfg_attr(feature = "serde", typetag::serde)]
impl StatData for i64 {
    fn add(&mut self, other: Box<dyn StatData>) {
        if let Some(other) = other.downcast_ref::<i64>() {
            *self += other;
        }
    }

    fn default(&self) -> Box<dyn StatData> {
        Box::new(0i64)
    }

    fn sub(&mut self, other: Box<dyn StatData>) {
        if let Some(other) = other.downcast_ref::<i64>() {
            *self -= other;
        }
    }
}

#[cfg_attr(feature = "serde", typetag::serde)]
impl StatData for i32 {
    fn add(&mut self, other: Box<dyn StatData>) {
        if let Some(other) = other.downcast_ref::<i32>() {
            *self += other;
        }
    }

    fn default(&self) -> Box<dyn StatData> {
        Box::new(0i32)
    }

    fn sub(&mut self, other: Box<dyn StatData>) {
        if let Some(other) = other.downcast_ref::<i32>() {
            *self -= other;
        }
    }
}

#[cfg_attr(feature = "serde", typetag::serde)]
impl StatData for i16 {
    fn add(&mut self, other: Box<dyn StatData>) {
        if let Some(other) = other.downcast_ref::<i16>() {
            *self += other;
        }
    }

    fn default(&self) -> Box<dyn StatData> {
        Box::new(0i16)
    }

    fn sub(&mut self, other: Box<dyn StatData>) {
        if let Some(other) = other.downcast_ref::<i16>() {
            *self -= other;
        }
    }
}

#[cfg_attr(feature = "serde", typetag::serde)]
impl StatData for i8 {
    fn add(&mut self, other: Box<dyn StatData>) {
        if let Some(other) = other.downcast_ref::<i8>() {
            *self += other;
        }
    }

    fn default(&self) -> Box<dyn StatData> {
        Box::new(0i8)
    }

    fn sub(&mut self, other: Box<dyn StatData>) {
        if let Some(other) = other.downcast_ref::<i8>() {
            *self -= other;
        }
    }
}
