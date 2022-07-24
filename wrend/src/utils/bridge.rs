use std::ops::Deref;

/// Enables accepting either a single item or many items when taking function arguments
pub struct Bridge<Item>(Vec<Item>);

impl<Item> Deref for Bridge<Item> {
    type Target = [Item];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<Item> From<Bridge<Item>> for Vec<Item> {
    fn from(bridge: Bridge<Item>) -> Self {
        bridge.0
    }
}

impl<Item: Clone> From<&Bridge<Item>> for Vec<Item> {
    fn from(bridge: &Bridge<Item>) -> Self {
        bridge.0.to_owned()
    }
}

impl<Item> From<(Item, Item)> for Bridge<Item> {
    fn from(items: (Item, Item)) -> Self {
        Bridge(vec![items.0, items.1])
    }
}

impl<Item> From<(Item, Item, Item)> for Bridge<Item> {
    fn from(items: (Item, Item, Item)) -> Self {
        Bridge(vec![items.0, items.1, items.2])
    }
}

impl<Item> From<(Item, Item, Item, Item)> for Bridge<Item> {
    fn from(items: (Item, Item, Item, Item)) -> Self {
        Bridge(vec![items.0, items.1, items.2, items.3])
    }
}

impl<Item> From<(Item, Item, Item, Item, Item)> for Bridge<Item> {
    fn from(items: (Item, Item, Item, Item, Item)) -> Self {
        Bridge(vec![
            items.0,
            items.1,
            items.2,
            items.3,
            items.4,
        ])
    }
}

impl<Item> From<[Item; 1]> for Bridge<Item> {
    fn from(items: [Item; 1]) -> Self {
        Bridge(Vec::from(items))
    }
}

impl<Item> From<[Item; 2]> for Bridge<Item> {
    fn from(items: [Item; 2]) -> Self {
        Bridge(Vec::from(items))
    }
}

impl<Item> From<[Item; 3]> for Bridge<Item> {
    fn from(items: [Item; 3]) -> Self {
        Bridge(Vec::from(items))
    }
}

impl<Item> From<[Item; 4]> for Bridge<Item> {
    fn from(items: [Item; 4]) -> Self {
        Bridge(Vec::from(items))
    }
}

impl<Item> From<[Item; 5]> for Bridge<Item> {
    fn from(items: [Item; 5]) -> Self {
        Bridge(Vec::from(items))
    }
}

impl<Item> From<[Item; 6]> for Bridge<Item> {
    fn from(items: [Item; 6]) -> Self {
        Bridge(Vec::from(items))
    }
}

impl<Item> From<[Item; 7]> for Bridge<Item> {
    fn from(items: [Item; 7]) -> Self {
        Bridge(Vec::from(items))
    }
}


impl<Item> From<[Item; 8]> for Bridge<Item> {
    fn from(items: [Item; 8]) -> Self {
        Bridge(Vec::from(items))
    }
}


impl<Item> From<[Item; 9]> for Bridge<Item> {
    fn from(items: [Item; 9]) -> Self {
        Bridge(Vec::from(items))
    }
}


impl<Item> From<[Item; 10]> for Bridge<Item> {
    fn from(items: [Item; 10]) -> Self {
        Bridge(Vec::from(items))
    }
}


impl<Item> From<[Item; 11]> for Bridge<Item> {
    fn from(items: [Item; 11]) -> Self {
        Bridge(Vec::from(items))
    }
}


impl<Item> From<[Item; 12]> for Bridge<Item> {
    fn from(items: [Item; 12]) -> Self {
        Bridge(Vec::from(items))
    }
}


impl<Item> From<[Item; 13]> for Bridge<Item> {
    fn from(items: [Item; 13]) -> Self {
        Bridge(Vec::from(items))
    }
}


impl<Item> From<[Item; 14]> for Bridge<Item> {
    fn from(items: [Item; 14]) -> Self {
        Bridge(Vec::from(items))
    }
}

impl<Item> From<[Item; 15]> for Bridge<Item> {
    fn from(items: [Item; 15]) -> Self {
        Bridge(Vec::from(items))
    }
}

impl<Item> From<Item> for Bridge<Item> {
    fn from(items: Item) -> Self {
        Bridge(vec![items])
    }
}

impl<Item: Clone> From<&Item> for Bridge<Item> {
    fn from(items: &Item) -> Self {
        Bridge(vec![items.to_owned()])
    }
}

impl<Item> From<Vec<Item>> for Bridge<Item> {
    fn from(items: Vec<Item>) -> Self {
        Bridge(items)
    }
}

impl<Item: Clone> From<&Vec<Item>> for Bridge<Item> {
    fn from(items: &Vec<Item>) -> Self {
        Bridge(items.to_owned())
    }
}

impl<Item: Clone> From<&[Item]> for Bridge<Item> {
    fn from(items: &[Item]) -> Self {
        Bridge(items.to_vec())
    }
}
