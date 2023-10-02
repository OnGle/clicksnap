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
                let mut u = st.url.clone();
                u.set_scheme("http").map_err(|_| ParseError::InvalidPort)?; // FIXME?
                st.wd.goto(u.as_str()).await?;
                st.wd
                    .screenshot(&st.ssp.join("screenshot-tklwebcp.png"))
                    .await?;
                let mut u = st.url.clone();
                u.set_scheme("https").map_err(|_| ParseError::InvalidPort)?; // FIXME?
                u.set_username("root")
                    .map_err(|_| ParseError::InvalidPort)?; // FIXME?
                u.set_password(Some(&st.pse.root_pass))
                    .map_err(|_| ParseError::InvalidPort)?; // FIXME?
                st.wd.goto(u.as_str()).await?;
                (st.wd.query(By::Id("headerName")).first().await?)
                    .wait_until()
                    .displayed()
                    .await?;
                st.wd
                    .screenshot(&st.ssp.join("screenshot-filemanager.png"))
                    .await?;
                // the following elements are difficult to find other than by XPath
                // "Disk Usage" menu button
                let du = st
                    .wd
                    .query(By::XPath("/html/body/div[2]/div[3]/ul[1]/li[3]/div"))
                    .first()
                    .await?;
                du.wait_until().displayed().await?;
                du.click().await?;
                // "Treemap" button
                let treemap = st
                    .wd
                    .query(By::XPath("/html/body/div[14]/div[2]/div[4]/h3/span"))
                    .first()
                    .await?;
                treemap.wait_until().displayed().await?;
                treemap.click().await?;
                // actual treemap element
                (st.wd.query(By::ClassName("treemappanel")).first().await?)
                    .wait_until()
                    .displayed()
                    .await?;
                st.wd
                    .screenshot(&st.ssp.join("screenshot-diskusage.png"))
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
