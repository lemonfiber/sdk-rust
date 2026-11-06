use crate::contract::{SetupAnswerBody, SetupRecoverBody, WizardReport};
use crate::line::Asked;
use crate::transport::Method;
use crate::{Client, Envelope, Error};

const SETUP: &str = "/api/setup";

const WIZARD: &str = "wizard";

impl Client {
    /// Where the first-run setup stands and what it is still asking for. Asking changes
    /// nothing.
    ///
    /// # Errors
    ///
    /// Every [`Error`] a request can meet.
    pub async fn setup(&self) -> Result<Envelope<WizardReport>, Error> {
        self.expect(Asked::read(SETUP.to_owned()), WIZARD).await
    }

    /// Answers the question setup is on. Sent once.
    ///
    /// # Errors
    ///
    /// Every [`Error`] a request can meet.
    pub async fn setup_answer(
        &self,
        answer: &SetupAnswerBody,
    ) -> Result<Envelope<WizardReport>, Error> {
        let asked = Asked::sending(Method::Post, format!("{SETUP}/answer"), answer)?;
        self.expect(asked, WIZARD).await
    }

    /// On past a step that only informs. Sent once.
    ///
    /// # Errors
    ///
    /// Every [`Error`] a request can meet.
    pub async fn setup_next(&self) -> Result<Envelope<WizardReport>, Error> {
        self.expect(Asked::once(Method::Post, format!("{SETUP}/next")), WIZARD)
            .await
    }

    /// Back to the question before. Sent once.
    ///
    /// # Errors
    ///
    /// Every [`Error`] a request can meet.
    pub async fn setup_back(&self) -> Result<Envelope<WizardReport>, Error> {
        self.expect(Asked::once(Method::Post, format!("{SETUP}/back")), WIZARD)
            .await
    }

    /// Writes the reviewed answers, answered once the writing is done. Sent once.
    ///
    /// # Errors
    ///
    /// Every [`Error`] a request can meet.
    pub async fn setup_apply(&self) -> Result<Envelope<WizardReport>, Error> {
        self.expect(Asked::once(Method::Post, format!("{SETUP}/apply")), WIZARD)
            .await
    }

    /// Takes one way out of an apply that stopped part-way. Sent once.
    ///
    /// # Errors
    ///
    /// Every [`Error`] a request can meet.
    pub async fn setup_recover(
        &self,
        choice: &SetupRecoverBody,
    ) -> Result<Envelope<WizardReport>, Error> {
        let asked = Asked::sending(Method::Post, format!("{SETUP}/recover"), choice)?;
        self.expect(asked, WIZARD).await
    }
}
