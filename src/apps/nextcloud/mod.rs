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
                st.sleep(1000).await;

                let form = st.wd.form(By::ClassName("login-form")).await?;

                form.set_by_name("user", "admin").await?;
                form.set_by_name("password", &st.pse.app_pass).await?;

                st.wd
                    .screenshot(&st.ssp.join("screenshot-login.png"))
                    .await?;

                form.submit_direct().await?;
                st.sleep(1000).await; // waiting for redirect

                // popup occurs on first login, we try and get rid of it if it's there
                if let Ok(elem) = st.wd.find(By::Css(".modal-container__close")).await {
                    elem.click().await?;
                    st.sleep(200).await; // waiting for redirect
                }

                st.wd
                    .screenshot(&st.ssp.join("screenshot-files.png"))
                    .await?;
                st.wd
                    .goto(
                        st.url
                            .join("/index.php/settings/admin?sectionid=general")?
                            .as_str(),
                    )
                    .await?;
                st.wd
                    .screenshot(&st.ssp.join("screenshot-settings.png"))
                    .await?;
                st.wd
                    .goto(st.url.join("/index.php/settings/apps")?.as_str())
                    .await?;

                (st.wd
                    .query(By::Css(
                        "div.section:nth-child(1) > div:nth-child(5) > button:nth-child(1)",
                    ))
                    .first()
                    .await?)
                    .wait_until()
                    .displayed()
                    .await?;
                st.wd
                    .screenshot(&st.ssp.join("screenshot-market.png"))
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
