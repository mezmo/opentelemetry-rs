use url::Url;

use crate::opentelemetry::trace::{
    ResourceSpans, ScopeSpans, Span, SpanEvent, SpanLink, Status, ExportTraceServiceRequest,
};

use crate::validation::common::*;

pub trait TraceValidate {
    fn validate(&self) -> Result<(), crate::Error>;
}

impl TraceValidate for ExportTraceServiceRequest<'_> {
    fn validate(&self) -> Result<(), crate::Error> {
        for resource_spans in &self.resource_spans {
            resource_spans.validate()?;
        }

        Ok(())
    }
}

impl TraceValidate for ResourceSpans<'_> {
    fn validate(&self) -> Result<(), crate::Error> {
        match &self.resource {
            Some(resource) => resource.validate(),
            None => Ok(()),
        }?;

        for scope_spans in &self.scope_spans {
            scope_spans.validate()?;
        }

        if self.schema_url != "" {
            Url::parse(self.schema_url.as_ref())?;
        }

        Ok(())
    }
}

impl TraceValidate for ScopeSpans<'_> {
    fn validate(&self) -> Result<(), crate::Error> {
        for span in &self.spans {
            span.validate()?;
        }

        if self.schema_url != "" {
            Url::parse(self.schema_url.as_ref())?;
        }

        Ok(())
    }
}

impl TraceValidate for Span<'_> {
    fn validate(&self) -> Result<(), crate::Error> {
        for attribute in &self.attributes {
            attribute.validate()?;
        }

        for event in &self.events {
            event.validate()?;
        }

        for link in &self.links {
            link.validate()?;
        }

        Ok(())
    }
}

impl TraceValidate for SpanEvent<'_> {
    fn validate(&self) -> Result<(), crate::Error> {
        for attribute in &self.attributes {
            attribute.validate()?;
        }

        Ok(())
    }
}

impl TraceValidate for SpanLink<'_> {
    fn validate(&self) -> Result<(), crate::Error> {
        for attribute in &self.attributes {
            attribute.validate()?;
        }

        Ok(())
    }
}

impl TraceValidate for Status<'_> {
    fn validate(&self) -> Result<(), crate::Error> {
        Ok(())
    }
}
