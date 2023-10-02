use crate::apps::{Runner, State};
use async_trait::async_trait;
use thirtyfour::prelude::*;
use url::ParseError;

pub struct T();

#[async_trait]
impl Runner for T {
    async fn exec(&self, st: &State) -> color_eyre::Result<()> {
        let mut u = st.url.clone();
        // webmin login
        u.set_port(Some(12321))
            .map_err(|_| ParseError::InvalidPort)?; // FIXME?
        st.wd.goto(u.as_str()).await?;
        (st.wd.find(By::Name("user")).await?)
            .send_keys("root")
            .await?;
        (st.wd.find(By::Name("pass")).await?)
            .send_keys(&st.pse.root_pass)
            .await?;
        st.wd
            .screenshot(&st.ssp.join("screenshot-webmin-login.png"))
            .await?;
        // webmin landing page (tklbam)
        let submit = st.wd.find(By::Css("button[type='submit']")).await?;
        submit.click().await?;
        (st.wd.query(By::Id("headln2c")).first().await?)
            .wait_until()
            .displayed()
            .await?;
        st.wd
            .screenshot(&st.ssp.join("screenshot-webmin-landing-tklbam.png"))
            .await?;
        // webmin dashboard
        let submit = st.wd.find(By::Css("label[for='open_dashboard']")).await?;
        submit.click().await?;
        (st.wd.query(By::Css("g[class='ct-labels']")).first().await?)
            .wait_until()
            .displayed()
            .await?;
        st.wd
            .screenshot(&st.ssp.join("screenshot-webmin-dashboard.png"))
            .await?;
        // webmin terminal
        let submit = st
            .wd
            .find(By::Css("li[aria-label='Command shell']"))
            .await?;
        submit.click().await?;
        (st.wd
            .query(By::Css("div[class='-shell-port- opened']"))
            .first()
            .await?)
            .wait_until()
            .displayed()
            .await?;
        (st.wd.find(By::Css("input[type='text']")).await?)
            .send_keys("apt-get update\n")
            .await?;
        st.sleep(3000).await; // wait for some output
        st.wd
            .screenshot(&st.ssp.join("screenshot-webmin-terminal.png"))
            .await?;
        Ok(())
    }
}
