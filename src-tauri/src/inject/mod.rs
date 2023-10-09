pub trait Injectable {
    fn inject(&self, str: &str) -> anyhow::Result<()>;
}
