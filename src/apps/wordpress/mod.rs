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
                    .screenshot(&st.ssp.join("screenshot-main.png"))
                    .await?;
                let mut u = st.url.clone();
                u.set_path("wp-login.php");
                st.wd.goto(u.as_str()).await?;
                (st.wd.find(By::Id("user_login")).await?)
                    .send_keys("admin@example.com")
                    .await?;
                (st.wd.find(By::Id("user_pass")).await?)
                    .send_keys(&st.pse.app_pass)
                    .await?;
                st.wd
                    .screenshot(&st.ssp.join("screenshot-login.png"))
                    .await?;
                (st.wd.find(By::Id("wp-submit")).await?).click().await?;
                st.wd
                    .screenshot(&st.ssp.join("screenshot-dashboard.png"))
                    .await?;
                let mut u = st.url.clone();
                u.set_path("wp-admin/post-new.php");
                st.wd.goto(u.as_str()).await?;
                st.wd
                    .screenshot(&st.ssp.join("screenshot-new-post.png"))
                    .await?;
                Ok(())
            }
            Action::Install => {
                todo!()
            }
        }
    }
}
