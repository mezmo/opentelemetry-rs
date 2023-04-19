use url::Url;

use crate::opentelemetry::logs::{ExportLogsServiceRequest, LogRecord, ResourceLogs, ScopeLogs};

use crate::validation::common::*;

pub trait LogValidate {
    fn validate(&self) -> Result<(), crate::Error>;
}

impl LogValidate for ExportLogsServiceRequest<'_> {
    fn validate(&self) -> Result<(), crate::Error> {
        for resource_log in &self.resource_logs {
            resource_log.validate()?;
        }

        Ok(())
    }
}

impl LogValidate for ResourceLogs<'_> {
    fn validate(&self) -> Result<(), crate::Error> {
        match &self.resource {
            Some(resource) => resource.validate(),
            None => Ok(()),
        }?;

        for scope_log in &self.scope_logs {
            scope_log.validate()?;
        }

        if self.schema_url != "" {
            Url::parse(self.schema_url.as_ref())?;
        }

        Ok(())
    }
}

impl LogValidate for ScopeLogs<'_> {
    fn validate(&self) -> Result<(), crate::Error> {
        match &self.scope {
            Some(scope) => scope.validate(),
            None => Ok(()),
        }?;

        for log_record in &self.log_records {
            log_record.validate()?;
        }

        if self.schema_url != "" {
            Url::parse(self.schema_url.as_ref())?;
        }

        Ok(())
    }
}

impl LogValidate for LogRecord<'_> {
    fn validate(&self) -> Result<(), crate::Error> {
        match &self.body {
            Some(body) => body.validate(),
            None => Ok(()),
        }?;

        for attribute in &self.attributes {
            attribute.validate()?;
        }

        Ok(())
    }
}
