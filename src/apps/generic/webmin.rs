use crate::apps::{State, Step, Steps};
use futures::FutureExt;
use thirtyfour::prelude::*;

pub const STEPS: Steps = &[
    Step {
        name: "webmin-login",
        f: |st: &State| {
            async {
                st.goto_port(12321, "/").await?;
                (st.wd.find(By::Name("user")).await?)
                    .send_keys("root")
                    .await?;
                (st.wd.find(By::Name("pass")).await?)
                    .send_keys(&st.pse.root_pass)
                    .await?;
                Ok(())
            }
            .boxed()
        },
        ..Step::default()
    },
    Step {
        name: "webmin-landing-tklbam",
        f: |st: &State| {
            async {
                let submit = st.wd.find(By::Css("button[type='submit']")).await?;
                submit.click().await?;
                st.wait(By::Id("headln2c")).await?;
                Ok(())
            }
            .boxed()
        },
        ..Step::default()
    },
    Step {
        name: "webmin-dashboard",
        f: |st: &State| {
            async {
                st.wd
                    .find(By::Css("label[for='open_dashboard']"))
                    .await?
                    .click()
                    .await?;
                st.wait(By::Css("span[class='piechart'] canvas")).await?;
                st.sleep(1_000).await;
                Ok(())
            }
            .boxed()
        },
        ..Step::default()
    },
    /*Step {
        name: "webmin-terminal",
        f: |st: &State| {
            async {
                let submit = st
                    .wd
                    .find(By::Css("li[aria-label='Command shell']"))
                    .await?;
                submit.click().await?;
                st.wait(By::Css("div[class='-shell-port- opened']")).await?;
                (st.wd.find(By::Css("input[type='text']")).await?)
                    .send_keys("apt-get update\n")
                    .await?;
                st.sleep(3000).await; // wait for some output
                Ok(())
            }
            .boxed()
        },
        ..Step::default()
    },*/
];
