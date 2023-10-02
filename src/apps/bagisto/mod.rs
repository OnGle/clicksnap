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
                    .screenshot(&st.ssp.join("screenshot-mainpage.png"))
                    .await?;
                // login screen
                st.wd.goto(st.url.join("/admin/login")?.as_str()).await?;
                st.wait(By::Id("email"))
                    .await?
                    .send_keys(&st.pse.app_email)
                    .await?;
                st.wait(By::Id("password"))
                    .await?
                    .send_keys(&st.pse.app_pass)
                    .await?;
                st.wd
                    .screenshot(&st.ssp.join("screenshot-login.png"))
                    .await?;
                st.wait(By::XPath("//button[text() = 'Sign In']"))
                    .await?
                    .click()
                    .await?;
                // dashboard
                st.wait(By::XPath("//h1[text() = 'Dashboard']")).await?;
                st.wd
                    .screenshot(&st.ssp.join("screenshot-dashboard.png"))
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
