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
                st.wd
                    .screenshot(&st.ssp.join("screenshot-landing.png"))
                    .await?;
                st.wait(By::XPath("//a[contains(text(), 'Go to the dashboard')]"))
                    .await?
                    .click()
                    .await?;

                // login screen
                st.wait(By::Id("x")).await?.send_keys("admin").await?;
                st.wait(By::Id("q"))
                    .await?
                    .send_keys(&st.pse.app_pass)
                    .await?;
                st.wd
                    .screenshot(&st.ssp.join("screenshot-login.png"))
                    .await?;
                st.wait(By::Name("login_action[login]"))
                    .await?
                    .click()
                    .await?;

                // dashboard
                st.wait(By::Id("create_sample_contents"))
                    .await?
                    .click()
                    .await?;
                st.wait(By::Id("create_demo_users")).await?.click().await?;
                st.wait(By::Id("create_demo_email_lists"))
                    .await?
                    .click()
                    .await?;
                st.wait(By::Id("cancel_button")).await?.click().await?;

                // demo website
                st.wait(By::XPath("//a[contains(text(), 'View website now')]"))
                    .await?
                    .click()
                    .await?;
                st.wd
                    .screenshot(&st.ssp.join("screenshot-mainpage.png"))
                    .await?;
                st.wait(By::XPath(
                    "//a[contains(@title, 'Go to the site dashboard')]",
                ))
                .await?
                .click()
                .await?;
                st.wd
                    .screenshot(&st.ssp.join("screenshot-dashboard.png"))
                    .await?;

                // demo collections
                st.wait(By::XPath("//a[contains(text(), 'Collections')]"))
                    .await?
                    .click()
                    .await?;
                st.wait(By::XPath("//a[contains(text(), 'Home')]"))
                    .await?
                    .click()
                    .await?;
                st.wd
                    .screenshot(&st.ssp.join("screenshot-collections.png"))
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
