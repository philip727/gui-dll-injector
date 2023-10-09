use serde::Serialize;

use super::TemporaryProcess;

#[derive(Clone, Serialize)]
pub struct ProcessSelectedEvent<'event> {
    pub name: &'event str,
    pub pid: u32,
}
