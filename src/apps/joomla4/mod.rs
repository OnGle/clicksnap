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

                st.wd.screenshot(&st.ssp.join("landing.png")).await?;

                let form = st.wd.form(By::Id("login-form-16")).await?;

                form.set_by_name("username", "admin").await?;
                form.set_by_name("password", &st.pse.app_pass).await?;
                form.submit().await?;
                st.sleep(500).await;

                st.wd.screenshot(&st.ssp.join("login.png")).await?;

                let u = st.url.join("administrator")?;
                st.wd.goto(u.as_str()).await?;

                st.wd.screenshot(&st.ssp.join("admin-login.png")).await?;

                (st.wd.find(By::Id("mod-login-username")).await?)
                    .send_keys("admin")
                    .await?;
                (st.wd.find(By::Id("mod-login-password")).await?)
                    .send_keys(&st.pse.root_pass)
                    .await?;
                let submit = st.wd.find(By::Css("button[type='submit']")).await?;
                submit.click().await?;

                st.sleep(1000).await;

                st.wd
                    .screenshot(&st.ssp.join("admin-dashboard.png"))
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
