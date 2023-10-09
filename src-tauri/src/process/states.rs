use std::sync::{Mutex, Arc};

use super::TemporaryProcess;

#[derive(Clone)]
pub struct SelectedProcess(pub Arc<Mutex<Option<TemporaryProcess>>>);
