use bevy::prelude::Component;

#[derive(Debug, Default, Component)]
pub struct GroundDetector {
    pub is_on_ground: WithPrevious<bool>,
    pub hit_speed: f32,
}

#[derive(Debug, Default)]
pub struct WithPrevious<T> {
    pub current: T,
    pub previous: Option<T>
}

impl<T> WithPrevious<T> {
    pub fn new(initial_value: T) -> WithPrevious<T> {
        WithPrevious {
            current: initial_value,
            previous: None,
        }
    }

    pub fn update_value(&mut self, new_value: T)
    where T: Clone, {
        self.previous = Some(self.current.clone());
        self.current = new_value;
    }

    pub fn is_same_as_previous(&self) -> bool
    where T: PartialEq {
        if self.previous.is_none() {
            return false;
        }

        return self.previous.as_ref().unwrap() == &self.current;
    }
}
