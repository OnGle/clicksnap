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
                let u = st.url.clone();
                // webmin login
                st.wd.goto(u.as_str()).await?;
                st.wait(By::Tag("form")).await?;
                let form = st.wd.form(By::Tag("form")).await?;
                form.set_by_name("username", "admin").await?;
                form.set_by_name("password", &st.pse.app_pass).await?;

                st.wd
                    .screenshot(&st.ssp.join("screenshot-login.png"))
                    .await?;

                st.sleep(1000).await;

                form.submit_direct().await?;

                st.sleep(1000).await;
                st.wait(By::ClassName("oxd-pie-chart")).await?;

                // pie chart is animated on load, try to wait until it's done
                st.sleep(1000).await;

                st.wd
                    .screenshot(&st.ssp.join("screenshot-dashboard.png"))
                    .await?;

                st.wd
                    .goto(
                        st.url
                            .join("/web/index.php/admin/viewSystemUsers")?
                            .as_str(),
                    )
                    .await?;

                st.wait(By::ClassName("orangehrm-container")).await?;

                st.wd
                    .screenshot(&st.ssp.join("screenshot-admin-page.png"))
                    .await?;

                Ok(())
            }
            Action::Install => {
                // there is nothing to install for core
                Ok(())
            }
        }
    }
}
