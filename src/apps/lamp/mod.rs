use crate::Runner;
use crate::{Action, State};
use async_trait::async_trait;
use thirtyfour::prelude::*;
use url::ParseError;

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
                let mut u = st.url.clone();
                u.set_port(Some(12322))
                    .map_err(|_| ParseError::InvalidPort)?; // FIXME?
                st.wd.goto(u.as_str()).await?;
                (st.wd.find(By::Name("auth[username]")).await?)
                    .send_keys("adminer")
                    .await?;
                (st.wd.find(By::Name("auth[password]")).await?)
                    .send_keys(&st.pse.root_pass)
                    .await?;
                (st.wd.find(By::Css("input[type='submit']")).await?)
                    .click()
                    .await?;
                st.wd
                    .screenshot(&st.ssp.join("screenshot-adminer-frontpage.png"))
                    .await?;
                (st.wd.find(By::Id("Db-mysql")).await?).click().await?;
                st.wd
                    .screenshot(&st.ssp.join("screenshot-adminer-database.png"))
                    .await?;
                Ok(())
            }
            Action::Install => {
                // there is nothing to install for lamp
                Ok(())
            }
        }
    }
}
