use crate::Runner;
use crate::{Action, State};
use async_trait::async_trait;
use thirtyfour::prelude::*;

pub struct T();

#[async_trait]
impl Runner for T {
    async fn exec(&self, st: &State) -> color_eyre::Result<()> {
        match &st.act {
            Action::Test => {
                st.wd.goto(st.url.as_str()).await?;
                (st.wd.find(By::Id("user")).await?)
                    .send_keys("admin")
                    .await?;
                (st.wd.find(By::Id("password")).await?)
                    .send_keys(&st.pse.app_pass)
                    .await?;
                st.wd
                    .screenshot(&st.ssp.join("screenshot-login.png"))
                    .await?;
                (st.wd.find(By::Id("submit")).await?).click().await?;
                st.wd
                    .screenshot(&st.ssp.join("screenshot-files.png"))
                    .await?;
                let u = st.url.join("/index.php/settings/admin?sectionid=general")?;
                st.wd.goto(u.as_str()).await?;
                st.wd
                    .screenshot(&st.ssp.join("screenshot-settings.png"))
                    .await?;
                let u = st.url.join("/index.php/apps/market/#/")?;
                st.wd.goto(u.as_str()).await?;
                (st.wd.query(By::ClassName("app-preview")).first().await?)
                    .wait_until()
                    .displayed()
                    .await?;
                st.wd
                    .screenshot(&st.ssp.join("screenshot-market.png"))
                    .await?;
                Ok(())
            }
            Action::Install => {
                // there is nothing to install
                Ok(())
            }
        }
    }
}
