// #[derive(Clone, Copy)]

pub enum Item {
    First,
    Second,
    Third,
}

impl Clone for Item {
    fn clone(&self) -> Self {
        *self
    }
}

impl Copy for Item {}

impl Item {
    pub fn index(&self) -> usize {
        match self {
            Item::First => 0,
            Item::Second => 1,
            Item::Third => 2,
        }
    }
}

pub trait ItemCollection {
    fn get_item(&self, item: Item) -> f64;
    fn set_item(&mut self, item: Item, value: f64);

    fn sum(&self) -> f64 {
        let mut sum = 0.0;
        for i in [Item::First, Item::Second, Item::Third].iter() {
            sum += self.get_item(*i);
        }
        sum
    }

    fn is_default(&self) -> bool {
        [Item::First, Item::Second, Item::Third]
            .iter()
            .all(|&i| self.get_item(i) == 0.0)
    }
}

pub struct Tuple(u32, f32, f64);

impl Default for Tuple {
    fn default() -> Self {
        Self(0, 0.0, 0.0)
    }
}

impl ItemCollection for Tuple {
    fn get_item(&self, item: Item) -> f64 {
        match item {
            Item::First => self.0 as f64,
            Item::Second => self.1 as f64,
            Item::Third => self.2,
        }
    }

    fn set_item(&mut self, item: Item, value: f64) {
        match item {
            Item::First => self.0 = value as u32,
            Item::Second => self.1 = value as f32,
            Item::Third => self.2 = value,
        }
    }
}

pub struct Array([f64; 3]);

impl Default for Array {
    fn default() -> Self {
        Self([0.0; 3])
    }
}

impl ItemCollection for Array {
    fn get_item(&self, item: Item) -> f64 {
        self.0[item.index()]
    }

    fn set_item(&mut self, item: Item, value: f64) {
        self.0[item.index()] = value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_sum_and_default<T: ItemCollection + Default>() {
        let mut collection = T::default();
        assert!(collection.is_default());
        assert_eq!(collection.sum(), 0.0);

        collection.set_item(Item::First, 10.0);
        collection.set_item(Item::Second, 20.0);
        collection.set_item(Item::Third, 30.0);

        assert!(!collection.is_default());
        assert_eq!(collection.sum(), 60.0);
    }

    #[test]
    fn test_tuple() {
        test_sum_and_default::<Tuple>();
    }

    #[test]
    fn test_array() {
        test_sum_and_default::<Array>();
    }
}
