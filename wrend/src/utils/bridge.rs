use std::{ops::Deref, rc::Rc};

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
        Bridge(vec![items.0, items.1, items.2, items.3, items.4])
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

impl<Item: Clone> From<Rc<[Item]>> for Bridge<Item> {
    fn from(items: Rc<[Item]>) -> Self {
        Bridge(items.to_vec())
    }
}

impl<Item> From<Box<[Item]>> for Bridge<Item> {
    fn from(items: Box<[Item]>) -> Self {
        Bridge(Vec::from(items))
    }
}

impl<Item> From<Box<Vec<Item>>> for Bridge<Item> {
    fn from(items: Box<Vec<Item>>) -> Self {
        Bridge(*items)
    }
}

impl<Item: Clone> From<Rc<Vec<Item>>> for Bridge<Item> {
    fn from(items: Rc<Vec<Item>>) -> Self {
        Bridge((*items).to_owned())
    }
}

// enables any sized array to be used as a bridge
impl<const U: usize, Item> From<[Item; U]> for Bridge<Item> {
    fn from(items: [Item; U]) -> Self {
        Bridge(Vec::from(items))
    }
}
