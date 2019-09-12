use failure::Error;
use jenkins_api::{Jenkins, JenkinsBuilder};

mod config;
mod gerrit;

pub use config::Config;
pub use gerrit::check_gerrit_review;

pub(crate) fn jenkins(config: &Config) -> Result<Jenkins, Error> {
    let mut jenkins = JenkinsBuilder::new(&config.jenkins_url);
    match &config.user {
        Some(user) => {
            jenkins = jenkins.with_user(&user, config.password.as_ref().map(|a| a.as_str()))
        }
        None => (),
    }

    jenkins.build()
}
