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
                st.wd.goto(st.url.join("index.php/login")?.as_str()).await?;

                let form = st.wd.form(By::ClassName("concrete-login-form")).await?;
                form.set_by_name("uName", "admin").await?;
                form.set_by_name("uPassword", &st.pse.app_pass).await?;
                form.submit_direct().await?;

                if let Ok(el) = st.wait(By::XPath("//button[text() = 'Skip']")).await {
                    el.click().await?;
                    st.sleep(1000).await;
                    st.wait(By::XPath("//button[text() = 'Got It!']"))
                        .await?
                        .click()
                        .await?;
                    st.sleep(1000).await;
                }

                st.wd
                    .screenshot(&st.ssp.join("screenshot-admin.png"))
                    .await?;

                // admin dashboard

                st.wd
                    .goto(st.url.join("index.php/dashboard/system")?.as_str())
                    .await?;
                st.wd
                    .screenshot(&st.ssp.join("screenshot-admin-dashboard.png"))
                    .await?;
                // themes settings page
                // it doesn't seem possible to click it for some reason...
                st.wd
                    .goto(st.url.join("index.php/dashboard/pages")?.as_str())
                    .await?;
                st.wait(By::XPath("//header")).await?;
                st.wd
                    .screenshot(&st.ssp.join("screenshot-themes.png"))
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
