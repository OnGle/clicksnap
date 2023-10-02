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
                // main page
                st.wd.goto(st.url.as_str()).await?;
                st.wd
                    .screenshot(&st.ssp.join("screenshot-landing.png"))
                    .await?;
                st.wait(By::Id("fauxton")).await?.click().await?;
                // login screen
                st.wait(By::Id("username"))
                    .await?
                    .send_keys("admin")
                    .await?;
                st.wait(By::Id("password"))
                    .await?
                    .send_keys(&st.pse.app_pass)
                    .await?;
                st.wait(By::Id("submit")).await?.click().await?;
                st.sleep(1000).await;
                // dashboard
                st.wd
                    .screenshot(&st.ssp.join("screenshot-fauxton.png"))
                    .await?;
                st.wait(By::XPath("//a[text() = '_users']"))
                    .await?
                    .click()
                    .await?;
                st.wait(By::XPath("//td[@title = '_design/_auth']"))
                    .await?
                    .click()
                    .await?;
                st.sleep(1000).await;
                st.wd
                    .screenshot(&st.ssp.join("screenshot-fauxton-db.png"))
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
