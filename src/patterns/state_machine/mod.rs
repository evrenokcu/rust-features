use std::{collections::HashMap, marker::PhantomData};

use uuid::Uuid;

use crate::print_header;

pub(crate) fn execute() {
    print_header("state machine");
    let s = Session::new();
    println!("{:?}", &s);
    if let Ok(mut session) = s.authenticate("evren", "pass") {
        println!("{:?}", &session);
        session.update_property("name", "'evren'");
        println!("{:?}", &session);
        let s = session.logout();
        println!("{:?}", s);
    }
}
trait SessionState {}

#[derive(Debug, Default)]
struct Initial;

#[derive(Debug, Default)]
struct Anonymous;

#[derive(Debug, Default)]
struct Authenticated;

#[derive(Debug, Default)]
struct LoggedOut;

impl SessionState for Initial {}
impl SessionState for Anonymous {}
impl SessionState for Authenticated {}
impl SessionState for LoggedOut {}

#[derive(Debug)]
struct Session<S: SessionState = Initial> {
    phantom: PhantomData<S>,
    sessionId: Uuid,
    props: HashMap<String, String>,
}

impl Session<Initial> {
    fn new() -> Session<Anonymous> {
        Session::<Anonymous> {
            phantom: PhantomData,
            sessionId: Uuid::new_v4(),
            props: HashMap::new(),
        }
    }
    fn resume_from(sessionId: Uuid) -> ResumeResult {
        ResumeResult::Authenticated(Session {
            phantom: PhantomData,
            sessionId,
            props: HashMap::new(),
        })
    }
}

#[derive(Debug)]
enum ResumeResult {
    Invalid,
    Anonymous,
    Authenticated(Session<Authenticated>),
}

impl Session<Anonymous> {
    fn authenticate(
        self,
        username: &str,
        password: &str,
    ) -> Result<Session<Authenticated>, Session<Anonymous>> {
        if username.is_empty() || password.is_empty() {
            return Err(self);
        }
        Ok(Session::<Authenticated> {
            phantom: PhantomData,
            sessionId: self.sessionId,
            props: HashMap::new(),
        })
    }
}

impl Session<Authenticated> {
    fn logout(self) -> Session<LoggedOut> {
        Session::<LoggedOut> {
            phantom: PhantomData,
            sessionId: self.sessionId,
            props: HashMap::new(),
        }
    }
    fn update_property(&mut self, key: &str, val: &str) {
        if let Some(x) = self.props.get_mut(key) {
            *x = val.into();
        } else {
            self.props.insert(key.into(), val.into());
        }
    }
}
