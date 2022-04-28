use std::sync::Arc;

use crate::error::Error;
use uuid::Uuid as Source;

pub type Uuid = Source;

#[cfg_attr(test, mockall::automock)]
pub trait UuidUtil {
    fn gen(&self) -> Uuid;
    fn parse(&self, input: &str) -> Result<Uuid, Error>;
}

pub type DynUuidUtil = Arc<dyn UuidUtil + Send + Sync>;

pub struct DefaultUuid {}

impl DefaultUuid {
    pub fn new() -> DefaultUuid {
        DefaultUuid {}
    }

    pub fn new_dyn() -> DynUuidUtil {
        Arc::new(DefaultUuid {})
    }
}

impl UuidUtil for DefaultUuid {
    fn gen(&self) -> Uuid {
        Uuid::new_v4()
    }

    fn parse(&self, input: &str) -> Result<Uuid, Error> {
        Ok(Uuid::parse_str(input)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let default_uuid = DefaultUuid::new();
        let id = default_uuid.gen();
        assert!(!id.is_nil());
    }

    #[test]
    fn parse() -> Result<(), Error> {
        let default_uuid = DefaultUuid::new();
        let str = "2238037f-0a10-4632-a716-f22b9cbe993d";
        let id = default_uuid.parse(str)?;
        assert_eq!(id.to_string().as_str(), str);
        Ok(())
    }
}
