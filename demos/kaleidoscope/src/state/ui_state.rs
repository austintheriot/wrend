use std::{cell::RefCell, rc::Rc};

use yew::UseStateHandle;

use crate::graphics::{FilterType, GenerationType};

#[derive(Clone, Debug, PartialEq)]
pub struct UiState {
    filter_type_ref: Rc<RefCell<FilterType>>,
    generation_type_ref: Rc<RefCell<GenerationType>>,
    filter_type: UseStateHandle<FilterType>,
    generation_type: UseStateHandle<GenerationType>,
    applied_filters: UseStateHandle<Vec<FilterType>>,
}

impl UiState {
    pub fn new(
        // hack: for some reason, Yew when de-referencing values from `UseStateHandle`, stale
        // values are returned, so this is a hack around that (using refs to get state and UseStateHandle
        // to set state)
        //
        // getters:
        filter_type_ref: Rc<RefCell<FilterType>>,
        generation_type_ref: Rc<RefCell<GenerationType>>,
        // setters:
        filter_type: UseStateHandle<FilterType>,
        generation_type: UseStateHandle<GenerationType>,
        applied_filters: UseStateHandle<Vec<FilterType>>,
    ) -> Self {
        Self {
            filter_type_ref,
            generation_type_ref,
            filter_type,
            generation_type,
            applied_filters,
        }
    }

    pub fn filter_type(&self) -> &UseStateHandle<FilterType> {
        &self.filter_type
    }

    pub fn generation_type(&self) -> &UseStateHandle<GenerationType> {
        &self.generation_type
    }

    pub fn filter_type_ref(&self) -> Rc<RefCell<FilterType>> {
        Rc::clone(&self.filter_type_ref)
    }

    pub fn generation_type_ref(&self) -> Rc<RefCell<GenerationType>> {
        Rc::clone(&self.generation_type_ref)
    }

    pub fn applied_filters(&self) -> &UseStateHandle<Vec<FilterType>> {
        &self.applied_filters
    }
}
