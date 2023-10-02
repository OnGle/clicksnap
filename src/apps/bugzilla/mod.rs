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
                // login screen (which has really weird behavior)
                // https://www.turnkeylinux.org/forum/support/wed-20180912-1251/bugzilla-needs-legitimate-login-and-password-continue
                st.wait(By::Id("login_link_top")).await?.click().await?;
                st.wait(By::Id("Bugzilla_login_top"))
                    .await?
                    .send_keys(&st.pse.app_email)
                    .await?;
                st.wait(By::Id("Bugzilla_password_top"))
                    .await?
                    .send_keys(&*st.pse.app_pass)
                    .await?;
                st.wait(By::Id("log_in_top")).await?.click().await?;
                st.wait(By::Id("Bugzilla_login"))
                    .await?
                    .send_keys(&st.pse.app_email)
                    .await?;
                st.wait(By::Id("Bugzilla_password"))
                    .await?
                    .send_keys(&*st.pse.app_pass)
                    .await?;
                st.wait(By::Id("log_in")).await?.click().await?;
                // dashboard
                st.wait(By::XPath("//a[text() = 'Administration']"))
                    .await?
                    .click()
                    .await?;
                st.wd
                    .screenshot(&st.ssp.join("screenshot-admin.png"))
                    .await?;
                // new bug form
                st.wait(By::XPath("//a[text() = 'Home']"))
                    .await?
                    .click()
                    .await?;
                st.wait(By::Id("enter_bug")).await?.click().await?;
                st.wd
                    .screenshot(&st.ssp.join("screenshot-new-bug.png"))
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
