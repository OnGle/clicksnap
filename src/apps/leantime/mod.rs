use super::{App, State, Step};
use futures::FutureExt;
use thirtyfour::prelude::*;

pub const APP: App = App {
    test: &[
        Step {
            name: "dashboard",
            f: |st: &State| {
                {
                    async {
                        // sometimes login redirects wrong? We just retry and hopefully it works
                        let form = st.wd.form(By::Id("login")).await?;
                        form.set_by_name("username", &st.pse.app_email).await?;
                        form.set_by_name("password", &st.pse.app_pass).await?;
                        form.submit_direct().await?;

                        //st.wait(By::Id("firstTask")).await?.send_keys("Setup Leantime").await?;
                        if let Ok(form) = st.wd.form(By::Css("form#firstTaskOnboarding")).await {
                            form.set_by_name("headline", "Setup Leantime").await?;
                            form.submit().await?;
                        }

                        st.sleep(10_000).await;

                        st.wait(By::XPath("//a[contains(text(), 'explore on my own')]")).await?.click().await?;

                        st.goto("dashboard/home").await?;
                        st.wait(By::Css("div.tw-h-full.minCalendar")).await?;
                        Ok(())
                    }
                }
                .boxed()
            },
            ..Step::default()
        },
        Step {
            name: "project",
            f: |st: &State| {
                {
                    async {
                        st.goto("dashboard/show").await?;

                        for _ in 0..10 {
                            if st.wait(By::Css("div.nyroModalCont")).await.is_ok() {
                                break;
                            }
                        }
                        if st.wait(By::Css("div.nyroModalCont")).await.is_ok() {
                            st.wait(By::Css("div.nyroModalCont a.btn-primary"))
                                .await?
                                .click()
                                .await?;
                        }
                        st.sleep(1_000).await;

                        Ok(())
                    }
                }
                .boxed()
            },
            ..Step::default()
        },
        Step {
            name: "plugins",
            f: |st: &State| {
                {
                    async {
                        st.goto("plugins/marketplace").await?;

                        st.wait(By::LinkText("Learn More"))
                            .await?
                            .wait_until()
                            .clickable()
                            .await?;

                        st.sleep(10_000).await;

                        Ok(())
                    }
                }
                .boxed()
            },
            ..Step::default()
        },
    ],
    ..App::default()
};
