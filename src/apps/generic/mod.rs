use super::{BoxRunner, Runner, State};
use async_trait::async_trait;

pub mod landing;
pub mod webmin;

pub struct GenericRunner(pub Box<[BoxRunner]>);

impl Default for GenericRunner {
    fn default() -> Self {
        Self(Box::new([
            Box::new(webmin::T()) as BoxRunner,
            Box::new(landing::T()) as BoxRunner,
        ]))
    }
}

#[async_trait]
impl Runner for GenericRunner {
    // dummy unused impl
    async fn exec(&self, _: &State) -> color_eyre::Result<()> {
        unreachable!()
    }

    async fn exec_full(&self, st: &State) -> color_eyre::Result<()> {
        for r in self.0.iter() {
            r.exec(st).await?
        }
        Ok(())
    }
}
