use url::Url;

use crate::opentelemetry::metrics::{
    Exemplar, ExemplarOneOfvalue, ExponentialHistogram, ExponentialHistogramDataPoint,
    ExponentialHistogramDataPointBuckets, ExportMetricsServiceRequest, Gauge, Histogram,
    HistogramDataPoint, Metric, MetricOneOfdata, NumberDataPoint, NumberDataPointOneOfvalue,
    ResourceMetrics, ScopeMetrics, Sum, Summary, SummaryDataPoint, SummaryDataPointValueAtQuantile,
};

use combine::{
    error::ParseError,
    parser::{
        byte::{alpha_num, letter},
        choice::choice,
        range::recognize,
    },
    skip_many, skip_many1,
    stream::RangeStream,
    token, Parser,
};

fn parse_name<'a, Input>() -> impl Parser<Input, Output = &'a [u8]> + 'a
where
    Input: RangeStream<Token = u8, Range = &'a [u8]> + 'a,
    // Necessary due to rust-lang/rust#24159
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    recognize((
        skip_many1(letter()),
        skip_many(choice((token(b'_'), alpha_num()))),
    ))
}

use crate::validation::common::*;

pub trait MetricValidate {
    fn validate(&self) -> Result<(), crate::Error>;
}

impl MetricValidate for ExportMetricsServiceRequest<'_> {
    fn validate(&self) -> Result<(), crate::Error> {
        for resource_metric in &self.resource_metrics {
            resource_metric.validate()?;
        }
        Ok(())
    }
}

impl MetricValidate for ResourceMetrics<'_> {
    fn validate(&self) -> Result<(), crate::Error> {
        for scope_metric in &self.scope_metrics {
            scope_metric.validate()?;
        }

        match &self.resource {
            Some(resource) => resource.validate(),
            None => Ok(()),
        }?;

        if self.schema_url != "" {
            Url::parse(self.schema_url.as_ref())?;
        }

        Ok(())
    }
}

impl MetricValidate for ScopeMetrics<'_> {
    fn validate(&self) -> Result<(), crate::Error> {
        match &self.scope {
            Some(scope) => scope.validate(),
            None => Ok(()),
        }?;

        for metric in &self.metrics {
            metric.validate()?;
        }

        if self.schema_url != "" {
            Url::parse(self.schema_url.as_ref())?;
        }

        Ok(())
    }
}

impl MetricValidate for Metric<'_> {
    fn validate(&self) -> Result<(), crate::Error> {
        // MUST adhere to `[a-zA-Z_]([a-zA-Z0-9_])*`
        // Labels values MAY be any sequence of utf-8 (must be utf-8)
        parse_name().parse(self.name.as_bytes())?;

        // self.unit should validated somehow
        // https://unitsofmeasure.org/ucum
        // https://ucum.nlm.nih.gov/ucum-lhc/demo.html

        match &self.data {
            MetricOneOfdata::gauge(val) => val.validate(),
            MetricOneOfdata::sum(val) => val.validate(),
            MetricOneOfdata::histogram(val) => val.validate(),
            MetricOneOfdata::exponential_histogram(val) => val.validate(),
            MetricOneOfdata::summary(val) => val.validate(),
            MetricOneOfdata::None => Ok(()),
        }?;

        Ok(())
    }
}

impl MetricValidate for Gauge<'_> {
    fn validate(&self) -> Result<(), crate::Error> {
        for data_point in &self.data_points {
            data_point.validate()?;
        }
        Ok(())
    }
}

impl MetricValidate for Sum<'_> {
    fn validate(&self) -> Result<(), crate::Error> {
        for data_point in &self.data_points {
            data_point.validate()?;
        }

        Ok(())
    }
}

impl MetricValidate for Histogram<'_> {
    fn validate(&self) -> Result<(), crate::Error> {
        for data_point in &self.data_points {
            data_point.validate()?;
        }

        Ok(())
    }
}

impl MetricValidate for ExponentialHistogram<'_> {
    fn validate(&self) -> Result<(), crate::Error> {
        for data_point in &self.data_points {
            data_point.validate()?;
        }

        Ok(())
    }
}

impl MetricValidate for Summary<'_> {
    fn validate(&self) -> Result<(), crate::Error> {
        for data_point in &self.data_points {
            data_point.validate()?;
        }
        Ok(())
    }
}

impl MetricValidate for NumberDataPoint<'_> {
    fn validate(&self) -> Result<(), crate::Error> {
        for attribute in &self.attributes {
            attribute.validate()?;
        }

        for exemplar in &self.exemplars {
            exemplar.validate()?;
        }

        self.value.validate()?;

        Ok(())
    }
}

impl MetricValidate for HistogramDataPoint<'_> {
    fn validate(&self) -> Result<(), crate::Error> {
        for attribute in &self.attributes {
            attribute.validate()?;
        }

        for exemplar in &self.exemplars {
            exemplar.validate()?;
        }

        Ok(())
    }
}

impl MetricValidate for ExponentialHistogramDataPoint<'_> {
    fn validate(&self) -> Result<(), crate::Error> {
        for attribute in &self.attributes {
            attribute.validate()?;
        }

        for exemplar in &self.exemplars {
            exemplar.validate()?;
        }

        match &self.positive {
            Some(positive) => positive.validate(),
            None => Ok(()),
        }?;

        match &self.negative {
            Some(negative) => negative.validate(),
            None => Ok(()),
        }?;

        Ok(())
    }
}

impl MetricValidate for SummaryDataPoint<'_> {
    fn validate(&self) -> Result<(), crate::Error> {
        for attribute in &self.attributes {
            attribute.validate()?;
        }

        for quantile_value in &self.quantile_values {
            quantile_value.validate()?;
        }

        Ok(())
    }
}

impl MetricValidate for ExponentialHistogramDataPointBuckets {
    fn validate(&self) -> Result<(), crate::Error> {
        Ok(())
    }
}

impl MetricValidate for SummaryDataPointValueAtQuantile {
    fn validate(&self) -> Result<(), crate::Error> {
        Ok(())
    }
}

impl MetricValidate for Exemplar<'_> {
    fn validate(&self) -> Result<(), crate::Error> {
        for filtered_attribute in &self.filtered_attributes {
            filtered_attribute.validate()?;
        }

        self.value.validate()?;

        Ok(())
    }
}

impl MetricValidate for NumberDataPointOneOfvalue {
    fn validate(&self) -> Result<(), crate::Error> {
        Ok(())
    }
}

impl MetricValidate for ExemplarOneOfvalue {
    fn validate(&self) -> Result<(), crate::Error> {
        Ok(())
    }
}
