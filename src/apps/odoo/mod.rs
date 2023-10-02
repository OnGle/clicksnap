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
                let u = st.url.join("/web/login")?;
                st.wd.goto(u.as_str()).await?;
                (st.wd.find(By::Id("login")).await?)
                    .send_keys("admin")
                    .await?;
                (st.wd.find(By::Id("password")).await?)
                    .send_keys(&st.pse.app_pass)
                    .await?;
                (st.wd.find(By::Css("button[type='submit']")).await?)
                    .click()
                    .await?;
                st.wd
                    .screenshot(&st.ssp.join("screenshot-apps.png"))
                    .await?;
                let install = st.wd.query(By::Css("div.oe_module_desc[title=\"eCommerce\"] div.oe_module_action button.btn.btn-primary")).first().await?;
                install.wait_until().displayed().await?;
                install.click().await?;
                (st.wd
                    .query(By::Css("button.oe_kanban_action_button"))
                    .first()
                    .await?)
                    .click()
                    .await?;
                (st.wd.query(By::Css("tbody")).first().await?)
                    .wait_until()
                    .displayed()
                    .await?;
                st.wd
                    .screenshot(&st.ssp.join("screenshot-ecommerce.png"))
                    .await?;
                // messages in general channel
                let u = st
                    .url
                    .join("/web#action=114&active_id=mail.channel_1&cids=1&menu_id=91")?;
                st.wd.goto(u.as_str()).await?;
                st.wd
                    .screenshot(&st.ssp.join("screenshot-messages.png"))
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
