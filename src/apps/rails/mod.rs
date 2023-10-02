use crate::Runner;
use crate::{Action, State};
use async_trait::async_trait;


pub struct T();

#[async_trait]
impl Runner for T {
    async fn exec(&self, st: &State) -> color_eyre::Result<()> {
        match &st.act {
            Action::Test => {
                // if there is a tkl-webcp page we're good
                st.wd.goto(st.url.as_str()).await?;
                st.wd
                    .screenshot(&st.ssp.join("screenshot-tklwebcp.png"))
                    .await?;
                Ok(())
            }
            Action::Install => {
                // there is nothing to install for rails
                Ok(())
            }
        }
    }
}
