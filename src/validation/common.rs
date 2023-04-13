use crate::opentelemetry::common::{
    AnyValue, AnyValueOneOfvalue, ArrayValue, InstrumentationScope, KeyValue, Resource,
};

pub trait CommonValidate {
    fn validate(&self) -> Result<(), crate::Error>;
}

impl CommonValidate for Resource<'_> {
    fn validate(&self) -> Result<(), crate::Error> {
        for attribute in &self.attributes {
            attribute.validate()?;
        }

        Ok(())
    }
}

impl CommonValidate for InstrumentationScope<'_> {
    fn validate(&self) -> Result<(), crate::Error> {
        for attribute in &self.attributes {
            attribute.validate()?;
        }

        Ok(())
    }
}

impl CommonValidate for KeyValue<'_> {
    fn validate(&self) -> Result<(), crate::Error> {
        match &self.value {
            Some(value) => value.validate(),
            None => Ok(()),
        }?;

        Ok(())
    }
}

impl CommonValidate for ArrayValue<'_> {
    fn validate(&self) -> Result<(), crate::Error> {
        for value in &self.values {
            value.validate()?;
        }
        Ok(())
    }
}

impl CommonValidate for AnyValue<'_> {
    fn validate(&self) -> Result<(), crate::Error> {
        self.value.validate()?;
        Ok(())
    }
}

impl CommonValidate for AnyValueOneOfvalue<'_> {
    fn validate(&self) -> Result<(), crate::Error> {
        Ok(())
    }
}
