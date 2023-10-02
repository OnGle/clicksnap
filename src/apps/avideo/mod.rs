use crate::Runner;
use crate::{Action, State};
use async_trait::async_trait;
use thirtyfour::prelude::*;

// NOTE:
// this appliance is major pain to work with because of DOM/AJAX abuse
// however it is also a good example of how to deal with that

pub struct T();

#[async_trait]
impl Runner for T {
    async fn exec(&self, st: &State) -> color_eyre::Result<()> {
        match &st.act {
            Action::Test => {
                st.wd.goto(st.url.as_str()).await?;
                st.wait(By::Id("rightLoginButton")).await?.click().await?;
                st.wait(By::Id("inputUser"))
                    .await?
                    .send_keys("admin")
                    .await?;
                st.wait(By::Id("inputPassword"))
                    .await?
                    .send_keys(&st.pse.app_pass)
                    .await?;
                // NOTE this is bad but it doesn't work otherwise
                st.sleep(1000).await;
                st.wd
                    .screenshot(&st.ssp.join("screenshot-login.png"))
                    .await?;
                st.wait(By::Id("mainButton")).await?.click().await?;
                // NOTE this is bad but it doesn't work otherwise
                st.sleep(500).await;
                st.wait(By::Css("button[id=buttonMenu]"))
                    .await?
                    .click()
                    .await?;
                // NOTE all of this is pretty bad too...
                let e = st
                    .wait(By::XPath(".//a[contains(@onclick, 'siteConfigurations')]"))
                    .await?;
                e.scroll_into_view().await?;
                e.click().await?;
                st.sleep(2000).await;
                st.wd
                    .screenshot(&st.ssp.join("screenshot-settings.png"))
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
