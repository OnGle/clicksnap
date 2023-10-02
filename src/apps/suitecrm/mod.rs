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
                // landing page
                st.wd.goto(st.url.as_str()).await?;
                st.wait(By::Id("login-button")).await?;
                st.wd
                    .screenshot(&st.ssp.join("screenshot-login.png"))
                    .await?;
                st.wait(By::XPath("//input[@name='username']"))
                    .await?
                    .send_keys("admin")
                    .await?;
                st.wait(By::XPath("//input[@name='password']"))
                    .await?
                    .send_keys(&st.pse.app_pass)
                    .await?;
                st.wait(By::Id("login-button")).await?.click().await?;

                st.sleep(1000).await;

                // enter iframe
                st.wait(By::XPath("//iframe")).await?.enter_frame().await?;

                st.wait(By::XPath("//span[text()='My Calls']")).await?;

                // exit iframe
                st.wd.enter_default_frame().await?;

                st.wd
                    .screenshot(&st.ssp.join("screenshot-dashboard.png"))
                    .await?;

                st.wd.goto(st.url.join("#/administration")?).await?;

                st.wait(By::XPath("//scrm-label[@module='administration']"))
                    .await?;

                st.wd
                    .screenshot(&st.ssp.join("screenshot-admin.png"))
                    .await?;

                st.wd
                    .goto(
                        st.url.join(
                            "#/accounts/edit?return_module=Accounts&return_action=DetailView",
                        )?,
                    )
                    .await?;

                st.wait(By::Css("form.create.field-layout")).await?;
                st.wait(By::Css("scrm-varchar-edit > input")).await?;

                st.wd
                    .screenshot(&st.ssp.join("screenshot-accounts.png"))
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
