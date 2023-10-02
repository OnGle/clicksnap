use crate::Runner;
use crate::{Action, State};
use async_trait::async_trait;
use std::env;


pub struct T();

#[async_trait]
impl Runner for T {
    async fn exec(&self, st: &State) -> color_eyre::Result<()> {
        match &st.act {
            Action::Test => {
                st.wd.goto(st.url.as_str()).await?;
                st.wd
                    .screenshot(&st.ssp.join("screenshot-tklwebcp.png"))
                    .await?;
                if let Ok(uu) = env::var("TKL_OPENVPN_PROFILE_URL") {
                    // get from envvar
                    st.wd.goto(uu.as_str()).await?;
                } else {
                    // ask interactively
                    print!("URL of created OpenVPN client profile page? ");
                    let line = std::io::stdin().lines().next().unwrap()?;
                    st.wd.goto(line.as_str()).await?;
                }
                st.wd
                    .screenshot(&st.ssp.join("screenshot-openvpn-profile.png"))
                    .await?;
                Ok(())
            }
            Action::Install => {
                // there is nothing to install for openvpn
                Ok(())
            }
        }
    }
}
