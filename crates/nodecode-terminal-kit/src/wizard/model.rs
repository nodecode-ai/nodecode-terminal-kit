//! Generic wizard model and messages

use super::framework::{WizardFlow, WizardItem};
use crate::components::list::ListState;
use crate::components::text_input::TextInput;
use ratatui::crossterm::event::KeyEvent;

pub struct GenericWizardModel<T: WizardItem> {
    pub items: Vec<T>,
    pub selected_idx: usize,
    pub list_state: ListState,
    pub filter: TextInput,
    pub wizard: Option<WizardFlow<T>>,
    pub view_mode: ViewMode,
    pub pending_delete: Option<T::Id>,
    pub is_open: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViewMode {
    List,
    Wizard,
    Confirmation,
}

impl<T: WizardItem> Default for GenericWizardModel<T> {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            selected_idx: 0,
            list_state: ListState::new(),
            filter: TextInput::new(),
            wizard: None,
            view_mode: ViewMode::List,
            pending_delete: None,
            is_open: false,
        }
    }
}

impl<T: WizardItem> GenericWizardModel<T> {
    pub fn open(&mut self, items: Vec<T>) {
        self.items = items;
        self.is_open = true;
        self.view_mode = ViewMode::List;
        self.selected_idx = 0;
        self.list_state.selected = 0;
        self.list_state.viewport_offset = 0;
    }

    pub fn close(&mut self) {
        self.is_open = false;
        self.wizard = None;
        self.view_mode = ViewMode::List;
        self.pending_delete = None;
        self.list_state.selected = 0;
        self.list_state.viewport_offset = 0;
    }

    pub fn selected_item(&self) -> Option<&T> {
        self.items.get(self.selected_idx)
    }

    pub fn selected_item_mut(&mut self) -> Option<&mut T> {
        self.items.get_mut(self.selected_idx)
    }
}

#[derive(Debug, Clone)]
pub enum GenericWizardMsg<T: WizardItem> {
    Open(Vec<T>),
    Close,
    SelectItem(usize),
    ToggleItem(usize),
    DeleteItem(T::Id),
    ConfirmDelete(T::Id),
    CancelDelete,
    FilterItems(String),
    StartCreate,
    StartEdit(T::Id),
    WizardNext,
    WizardPrevious,
    WizardCancel,
    WizardSave,
    WizardKeyInput(KeyEvent),
    ItemsSaved(Result<Vec<T>, String>),
}
