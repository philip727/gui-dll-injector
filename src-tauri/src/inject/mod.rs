pub mod commands;
pub mod errors;

pub trait Injectable {
    unsafe fn inject(&self, dll: &str) -> anyhow::Result<()>;
}
