use color_eyre::eyre::WrapErr;
use async_trait::async_trait;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::path::PathBuf;
use thirtyfour::prelude::*;
use url::Url;

use self::generic::GenericRunner;

mod asp_net_core;
mod avideo;
mod b2evolution;
mod bagisto;
mod bugzilla;
mod cakephp;
mod canvas;
mod codeigniter;
mod concrete_cms;
mod core;
mod couchdb;
mod django;
mod dokuwiki;
mod drupal10;
mod drupal7;
mod fileserver;
mod generic;
mod gitea;
mod joomla4;
mod lamp;
mod nextcloud;
mod lapp;
mod nginx_php_fastcgi;
mod nodejs;
mod odoo;
mod openvpn;
mod orangehrm;
mod owncloud;
mod rails;
mod redmine;
mod silverstripe;
mod suitecrm;
mod wordpress;

pub struct Preseeds {
    pub root_pass: String,
    pub db_pass: String,
    pub app_pass: String,
    pub app_email: String,
    pub app_domain: String,
}

pub struct State {
    pub wd: WebDriver,
    pub act: Action,
    pub url: Url,
    pub ssp: PathBuf,
    pub pse: Preseeds,
}

impl State {
    async fn wait(&self, sel: By) -> color_eyre::Result<WebElement> {
        let elt = self.wd.query(sel.clone()).first().await?;
        elt.wait_until().displayed().await.wrap_err(format!("waiting for {:?} to be displayed", sel))?;
        Ok(elt)
    }

    async fn sleep(&self, m: u64) -> () {
        thirtyfour::support::sleep(std::time::Duration::from_millis(m)).await
    }
}

// TODO install is pretty much unused for now
pub enum Action {
    Test,
    Install,
}

// TODO hide some of these methods with type magic
#[async_trait]
pub trait Runner {
    // internal run function that contains the webdriver steps
    // meant to be defined by appliance script
    async fn exec(&self, st: &State) -> color_eyre::Result<()>;

    async fn exec_full(&self, st: &State) -> color_eyre::Result<()> {
        // generic runners should run for most/all appliances
        GenericRunner::default().exec_full(st).await?;
        self.exec(st).await
    }

    // run the scenario and manage the environment boilerplate
    async fn run(&self, st: &State) -> color_eyre::Result<()> {
        let r = self.exec_full(st).await;
        let wd = st.wd.clone();
        let _ = wd.quit().await;
        r
    }
}

type BoxRunner = Box<dyn Runner + Send + Sync>;
type BoxRunnerMap = HashMap<&'static str, BoxRunner>;

pub static RUNNERS: Lazy<BoxRunnerMap> = Lazy::new(|| {
    let mut h: BoxRunnerMap = HashMap::new();
    h.insert("asp-net-core", Box::new(asp_net_core::T()));
    h.insert("avideo", Box::new(avideo::T()));
    h.insert("b2evolution", Box::new(b2evolution::T()));
    h.insert("bagisto", Box::new(bagisto::T()));
    h.insert("bugzilla", Box::new(bugzilla::T()));
    h.insert("cakephp", Box::new(cakephp::T()));
    h.insert("canvas", Box::new(canvas::T()));
    h.insert("codeigniter", Box::new(codeigniter::T()));
    h.insert("concrete-cms", Box::new(concrete_cms::T()));
    h.insert("core", Box::new(core::T()));
    h.insert("couchdb", Box::new(couchdb::T()));
    h.insert("django", Box::new(django::T()));
    h.insert("dokuwiki", Box::new(dokuwiki::T()));
    h.insert("fileserver", Box::new(fileserver::T()));
    h.insert("lamp", Box::new(lamp::T()));
    h.insert("lapp", Box::new(lamp::T()));
    h.insert("mysql", Box::new(lamp::T()));
    h.insert("nginx-php-fastcgi", Box::new(nginx_php_fastcgi::T()));
    h.insert("nodejs", Box::new(nodejs::T()));
    h.insert("odoo", Box::new(odoo::T()));
    h.insert("openvpn", Box::new(openvpn::T()));
    h.insert("owncloud", Box::new(owncloud::T()));
    h.insert("nextcloud", Box::new(nextcloud::T()));
    h.insert("rails", Box::new(rails::T()));
    h.insert("redmine", Box::new(redmine::T()));
    h.insert("wordpress", Box::new(wordpress::T()));
    h.insert("gitea", Box::new(gitea::T()));
    h.insert("drupal7", Box::new(drupal7::T()));
    h.insert("drupal10", Box::new(drupal10::T()));
    h.insert("silverstripe", Box::new(silverstripe::T()));
    h.insert("orangehrm", Box::new(orangehrm::T()));
    h.insert("joomla4", Box::new(joomla4::T()));
    h.insert("suitecrm", Box::new(suitecrm::T()));
    h
});
