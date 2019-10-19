//! Thank you for going to a website you found on a coffee cup, we appreciate it
//! a lot! We hope you are having a great time in Barcelona!
//!
//! # We are The Techno Creatives
//!
//! The Techno Creatives is a consultancy from Gothenburg, Sweden. We work on a
//! lot of different projects focussed on delivering new and exciting digital
//! experiences. Often, that means we are free to pick whichever technology
//! works best for us -- and we've been able to pick Rust a lot of times
//! recently.
//!
//! <!-- TODO: Which projects should be listed here?
//!
//! # What we are doing
//!
//!
//! -->

#![allow(clippy::needless_lifetimes)] // for puns

use std::fmt;

/// Join us!
///
/// At Techno Creatives you will not be joining a boring company where every day
/// is the same. We are a group of people all working with the same goal to help
/// our partners build the best products out there. Our team is always on the
/// lookout for new designers, business developers and business strategist.
/// Don’t be shy and drop a line!
///
/// See our [jobs page](https://technocreatives.com/jobs) (or browse through
/// these docs and their code) to learn more!
pub async fn apply_for_a_job<'a>(you: &'a Person, job: Role) -> Result<Interview, Error> {
    let application_response = fill_out("https://technocreatives.com/jobs").await?;
    let assigment = pick_from(application_response).await?;
    let interview = assigment
        .implement().await?
        .get_review().await?
        .schedule_interview().await?;
    Ok(interview)
}

/// Does your company want to get started with Rust and/or needs some
/// additional people to get going quickly?
pub async fn hire_us<'a, 'cool>(you: &'a Company, project: &'cool Project) -> Result<Success, RetryToken> {
    Ok(Success {})
}

/// Four of us are at RustFest! Come talk to us!
pub(crate) enum UsAtRustFest {
    /// Ross
    ///
    /// ![](https://static1.squarespace.com/static/53b695f2e4b094994074c8ec/5506aefbe4b052040efea485/5506d948e4b072c28ec89fba/1571411032618/ross+220x240.png?format=220w)
    ///
    ///
    /// [Mail](mailto:ross@technocreatives.com)
    Ross,
    /// Eike
    ///
    /// ![](https://static1.squarespace.com/static/53b695f2e4b094994074c8ec/5506aefbe4b052040efea485/588f5403bf629abc01e90f40/1571410648665/eike+220x240.png?format=220w)
    ///
    ///
    /// [Mail](mailto:eike@technocreatives.com)
    Eike,
    /// You may know Pascal from the Rust dev tools team and the CLI WG
    ///
    /// ![](https://static1.squarespace.com/static/53b695f2e4b094994074c8ec/5506aefbe4b052040efea485/5c3c853b0ebbe85c2b9c158a/1571410542318/Pascal+220x240.png?format=220w)
    ///
    /// [Twitter](https://twitter.com/killercup)
    ///
    /// [Mail](mailto:pascal@technocreatives.com)
    Pascal,
    /// Don't ask Brendan about FFI if you are in a hurry.
    ///
    /// ![](https://static1.squarespace.com/static/53b695f2e4b094994074c8ec/5506aefbe4b052040efea485/5a3396b453450a1651970b7f/1571410388041/brendan+220x240.png?format=220w)
    ///
    ///
    /// [Twitter](https://twitter.com/piecritic)
    ///
    /// [Mail](mailto:brendan@technocreatives.com)
    Brendan,
}

#[doc(hidden)]
pub struct Person { }

/// The dev jobs we are currently hiring for.
///
/// See our [jobs](https://technocreatives.com/jobs) page for more!
#[derive(Clone, Copy)]
pub enum Role {
    /// Multi-discipline developers
    ///
    /// Do you like working with multiple platforms or multiple programming
    /// languages in any given week? [Apply here!][here]
    ///
    /// [here]: https://technocreatives.homerun.co/multi-discipline-developer/en
    MultidisciplineDeveloper,
    /// Embedded developers & hardware engineers
    ///
    /// You will be joining our compact but growing team of electronics
    /// engineers and embedded firmware developers in Gothenburg, Sweden.
    /// Together with our colleagues in Shenzhen, China, you will get to work on
    /// projects ranging from automotive to IoT and consumer electronics. By
    /// combining electronics and embedded software with app development, cloud
    /// development, UX/UI design and strategy we are truly full-stack.
    ///
    /// [Apply here!][here]
    ///
    /// [here]: https://technocreatives.homerun.co/embedded-developers-hardware-engineers/en
    EmbeddedDev,
}

/// If you have any questions what the roles entail, don't hesitate to reach
/// out!
impl fmt::Debug for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Awesome job for an awesome person")
    }
}


#[doc(hidden)]
pub struct Interview { }

#[doc(hidden)]
pub struct Error { }

#[doc(hidden)]
pub struct ApplicationResponse { }

async fn fill_out(url: &str) -> Result<ApplicationResponse, Error> {
    Ok(ApplicationResponse {})
}

#[doc(hidden)]
pub struct Assigment { }

async fn pick_from(res: ApplicationResponse) -> Result<Assigment, Error> {
    Ok(Assigment {})
}

impl Assigment {
    async fn implement(&self) -> Result<AssigmentImpl, Error> {
        Ok(AssigmentImpl {})
    }
}

#[doc(hidden)]
pub struct AssigmentImpl { }

impl AssigmentImpl {
    async fn get_review(&self) -> Result<AssigmentReview, Error> {
        Ok(AssigmentReview {})
    }

}
#[doc(hidden)]
pub struct AssigmentReview { }

impl AssigmentReview {
    async fn schedule_interview(&self) -> Result<Interview, Error> {
        Ok(Interview {})
    }
}

#[doc(hidden)]
pub struct Company { }

#[doc(hidden)]
pub struct Project { }

#[doc(hidden)]
pub struct Success { }

#[doc(hidden)]
pub struct RetryToken { }
