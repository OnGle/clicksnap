use crate::Runner;
use crate::{Action, State};
use async_trait::async_trait;
use thirtyfour::prelude::*;
use url::ParseError;

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
                // hack to scroll to the bottom
                st.wait(By::XPath("//h3[contains(text(), 'Training')]"))
                    .await?
                    .scroll_into_view()
                    .await?;
                st.wd
                    .screenshot(&st.ssp.join("screenshot-docs.png"))
                    .await?;
                // adminer
                let mut u = st.url.clone();
                u.set_scheme("https").map_err(|_| ParseError::InvalidPort)?; // FIXME?
                u.set_port(Some(12322))
                    .map_err(|_| ParseError::InvalidPort)?; // FIXME?
                st.wd.goto(u.as_str()).await?;
                (st.wd.find(By::Name("auth[username]")).await?)
                    .send_keys("adminer")
                    .await?;
                (st.wd.find(By::Name("auth[password]")).await?)
                    .send_keys(&st.pse.root_pass)
                    .await?;
                (st.wd.find(By::Css("input[type='submit']")).await?)
                    .click()
                    .await?;
                st.wd
                    .screenshot(&st.ssp.join("screenshot-adminer-frontpage.png"))
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
