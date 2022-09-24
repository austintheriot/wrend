use std::{cell::RefCell, rc::Rc};

use yew::UseStateHandle;

use crate::graphics::{FilterType, GenerationType};

#[derive(Clone, Debug, PartialEq)]
pub struct UiState {
    generation_type_ref: Rc<RefCell<GenerationType>>,
    applied_filters_ref: Rc<RefCell<Vec<FilterType>>>,

    generation_type: UseStateHandle<GenerationType>,
    applied_filters: UseStateHandle<Vec<FilterType>>,
}

impl UiState {
    pub fn new(
        // hack: for some reason, Yew when de-referencing values from `UseStateHandle`, stale
        // values are returned, so this is a hack around that (using refs to get state and UseStateHandle
        // to set state)

        // getters:
        generation_type_ref: Rc<RefCell<GenerationType>>,
        applied_filters_ref: Rc<RefCell<Vec<FilterType>>>,

        // setters:
        generation_type: UseStateHandle<GenerationType>,
        applied_filters: UseStateHandle<Vec<FilterType>>,
    ) -> Self {
        Self {
            generation_type_ref,
            applied_filters_ref,

            generation_type,
            applied_filters,
        }
    }

    pub fn generation_type(&self) -> &UseStateHandle<GenerationType> {
        &self.generation_type
    }

    pub fn applied_filters(&self) -> &UseStateHandle<Vec<FilterType>> {
        &self.applied_filters
    }

    pub fn generation_type_ref(&self) -> Rc<RefCell<GenerationType>> {
        Rc::clone(&self.generation_type_ref)
    }

    pub fn applied_filters_ref(&self) -> Rc<RefCell<Vec<FilterType>>> {
        Rc::clone(&self.applied_filters_ref)
    }
}
