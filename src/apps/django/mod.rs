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
                // login screen
                st.wait(By::XPath("//a[contains(text(), 'Administration')]"))
                    .await?
                    .click()
                    .await?;
                st.wait(By::Id("id_username"))
                    .await?
                    .send_keys("admin")
                    .await?;
                st.wait(By::Id("id_password"))
                    .await?
                    .send_keys(&*st.pse.app_pass)
                    .await?;
                st.wait(By::XPath("//input[@type = 'submit' and @value = 'Log in']"))
                    .await?
                    .click()
                    .await?;
                // admin dashboard
                st.sleep(1000).await;
                st.wd
                    .screenshot(&st.ssp.join("screenshot-admin.png"))
                    .await?;
                // back to tkl-webcp
                st.wd.goto(st.url.as_str()).await?;
                // click offline docs link
                st.wait(By::XPath("//a[text() = 'offline']"))
                    .await?
                    .click()
                    .await?;
                st.wd
                    .screenshot(&st.ssp.join("screenshot-docs.png"))
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
