use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
use yew::html::{Component, ComponentLink};

pub struct WeakComponentLink<T: Component>(Rc<RefCell<Option<ComponentLink<T>>>>);

impl<T: Component> Clone for WeakComponentLink<T> {
    fn clone(&self) -> Self {
        WeakComponentLink(self.0.clone())
    }
}

impl<T: Component> Default for WeakComponentLink<T> {
    fn default() -> Self {
        WeakComponentLink(Rc::new(RefCell::new(None)))
    }
}

impl<T: Component> Deref for WeakComponentLink<T> {
    type Target = Rc<RefCell<Option<ComponentLink<T>>>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Component> PartialEq for WeakComponentLink<T> {
    fn eq(&self, other: &WeakComponentLink<T>) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

// OLD
// pub struct WeakComponentLink<T: Component>(Rc<RefCell<Option<ComponentLink<T>>>>);

// impl<T: Component> Clone for WeakComponentLink<T> {
//     fn clone(&self) -> Self {
//         Self(Rc::clone(&self.0))
//     }
// }

// impl<T: Component> Default for WeakComponentLink<T> {
//     fn default() -> Self {
//         Self(Rc::default())
//     }
// }

// impl<T: Component> Deref for WeakComponentLink<T> {
//     type Target = Rc<RefCell<Option<ComponentLink<T>>>>;

//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }

// impl<T: Component> PartialEq for WeakComponentLink<T> {
//     fn eq(&self, other: &Self) -> bool {
//         Rc::ptr_eq(&self.0, &other.0)
//     }
// }