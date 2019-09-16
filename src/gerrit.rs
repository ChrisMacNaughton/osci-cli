use failure::Error;
use jenkins_api::build::BuildStatus;
use log::debug;
use serde::Deserialize;

use nom::{
  IResult,
  bytes::complete::{tag, take_until}
};

use crate::config::Config;
use crate::jenkins;



#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct LastBuild {
    number: u32,
    duration: u32,
    result: String,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct LastBuildOfJob {
    display_name: String,
    last_build: LastBuild,
}

pub fn check_gerrit_review(review_id: usize, config: &Config) -> Result<(), Error> {
    debug!(
        "About to check the status of review {} at {}",
        review_id, config.osci_url
    );
    let jenkins = jenkins(config)?;
    // test_charm_func_smoke
    let a = match jenkins.get_job("test_charm_func_smoke") {
        Ok(r) => r,
        Err(e) => {
            println!("Couldn't get results from OSCI, are you connected to the VPN?");
            debug!("Error: {:?}", e);
            return Ok(())
        }
    };
    // test_charm_func_full
    let b = match jenkins.get_job("test_charm_func_full") {
        Ok(r) => r,
        Err(e) => {
            println!("Couldn't get results from OSCI, are you connected to the VPN?");
            debug!("Error: {:?}", e);
            return Ok(())
        }
    };

    let build = match (filter_builds(&a, review_id), filter_builds(&b, review_id)) {
        (Some(a), Some(b)) =>  {
            if Some(a.timestamp) > Some(b.timestamp) {
                a
            } else {
                b
            }
        },
        (Some(a), None) => a,
        (None, Some(b)) => b,
        (None, None) => {
            println!("No matching build found");
            return Ok(())
        }
    };
    let build = build.get_full_build(&jenkins)?;
    if build.building {
        println!("OSCI is still working on {} ({})", review_id, build.url);
        // println!("Build: {:?}", build);
        return Ok(())
    }
    match build.result {
        Some(result) => {
            let icon = match result {
                BuildStatus::Success => "✓",
                BuildStatus::Failure => "✗",
                _ => "",
            };
            println!("{} OSCI has finished {}: {:?}", icon, review_id, result)
        }
        None => println!("OSCI has no result for {}", review_id),
    }

    // match build.result {
    //     BuildStatus:: =>
    // }
    // let display_name: &String = &a.builds[0].display_name;
    // println!("func_smoke: {:?}: {:?}", display_name,
    //     display_name_to_build_number(display_name));
    // let builds = a.builds.filter(|build| build.other_fields)
    // test_charm_func_full
    Ok(())
}

fn filter_builds(job: &jenkins_api::job::CommonJob, review_id: usize) -> Option<&jenkins_api::build::ShortBuild> {
    match job.builds.iter().filter(|build| {
        match build.display_name {
            Some(ref name) => {
                match display_name_to_build_number(name) {
                    Some(build_number) => build_number == review_id,
                    None => false
                }
            }
            None => false
        }

    }).next() {
        Some(build) => Some(build),
        None => {
            // println!("No matching build found");
            None
        }
    }
}

pub(crate) fn display_name_to_build_number(job_name: &str) -> Option<usize> {
    match jenkins_job(job_name) {
        Ok((_, job)) => Some(job.change_id),
        Err(_) => None
    }
}

#[allow(dead_code)]
struct Job {
    jenkins_number: usize,
    repo: String,
    branch: String,
    change_id: usize,
    revision: usize,
    author: String,
}

fn jenkins_job(input: &str) -> IResult<&str, Job> {
    let (input, _) = tag("#")(input)?;
    let tag_whitespace = tag(" ");
    let take_whitespace = take_until(" ");

    let (input, jenkins_number) = take_whitespace(input)?;
    let (input, _) = tag_whitespace(input)?;
    let (input, repo) = take_whitespace(input)?;
    let (input, _) = tag_whitespace(input)?;
    let (input, branch) = take_whitespace(input)?;
    let (input, _) = tag_whitespace(input)?;
    let (input, change_id) = take_whitespace(input)?;
    let (input, _) = tag_whitespace(input)?;
    let (input, revision) = take_whitespace(input)?;
    let (input, _) = tag_whitespace(input)?;
    let author = input;
    Ok(("", Job {
        jenkins_number: jenkins_number.parse().unwrap(),
        repo: repo.to_string(),
        branch: branch.to_string(),
        change_id: change_id.parse().unwrap(),
        revision: revision.parse().unwrap(),
        author: author.to_string()
    }))
}
