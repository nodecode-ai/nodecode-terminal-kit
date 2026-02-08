//! Generic wizard framework for multi-step configuration

pub mod framework;
pub mod layout;
pub mod model;
pub mod steps;
pub mod text_step;
pub mod view;

pub use framework::{ItemListView, StepAction, WizardFlow, WizardItem, WizardMode, WizardStep};
pub use layout::{input_step_layout, padded_list_layout};
pub use model::{GenericWizardModel, GenericWizardMsg, ViewMode};
pub use steps::SummaryStep;
pub use text_step::SimpleTextStep;
pub use view::generic_wizard_view;
