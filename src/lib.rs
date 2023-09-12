use quick_protobuf::MessageRead;

mod opentelemetry_types;
mod validation;

pub mod opentelemetry {
    pub mod common {
        pub use crate::opentelemetry_types::opentelemetry::proto::common::v1::{
            mod_AnyValue::OneOfvalue as AnyValueOneOfvalue, AnyValue, ArrayValue,
            InstrumentationScope, KeyValue, Resource,
        };
        pub use crate::validation::common::CommonValidate as Validate;
    }

    pub mod metrics {
        pub use crate::opentelemetry::common::{
            AnyValue, AnyValueOneOfvalue, ArrayValue, InstrumentationScope, KeyValue, Resource,
        };
        pub use crate::opentelemetry_types::opentelemetry::proto::metrics::v1::{
            mod_Exemplar::OneOfvalue as ExemplarOneOfvalue,
            mod_ExponentialHistogramDataPoint::Buckets as ExponentialHistogramDataPointBuckets,
            mod_Metric::OneOfdata as MetricOneOfdata,
            mod_NumberDataPoint::OneOfvalue as NumberDataPointOneOfvalue,
            mod_SummaryDataPoint::ValueAtQuantile as SummaryDataPointValueAtQuantile,
            AggregationTemporality, Exemplar, ExponentialHistogram, ExponentialHistogramDataPoint,
            ExportMetricsServiceRequest, Gauge, Histogram, HistogramDataPoint, Metric,
            NumberDataPoint, ResourceMetrics, ScopeMetrics, Sum, Summary, SummaryDataPoint,
        };
        pub use crate::validation::metrics::MetricValidate as Validate;
    }

    pub mod logs {
        pub use crate::opentelemetry::common::{
            AnyValue, AnyValueOneOfvalue, ArrayValue, InstrumentationScope, KeyValue, Resource,
        };
        pub use crate::opentelemetry_types::opentelemetry::proto::logs::v1::{
            ExportLogsServiceRequest, LogRecord, ResourceLogs, ScopeLogs, SeverityNumber,
        };
        pub use crate::validation::logs::LogValidate as Validate;
    }

    pub mod trace {
        pub use crate::opentelemetry::common::{
            AnyValue, AnyValueOneOfvalue, ArrayValue, InstrumentationScope, KeyValue, Resource,
        };
        pub use crate::opentelemetry_types::opentelemetry::proto::trace::v1::{
            mod_Span::Event as SpanEvent, mod_Span::Link as SpanLink, mod_Span::SpanKind,
            mod_Status::StatusCode, ResourceSpans, ScopeSpans, Span, Status, ExportTraceServiceRequest,
        };
        pub use crate::validation::trace::TraceValidate as Validate;
    }
}

#[derive(thiserror::Error, std::fmt::Debug)]
pub enum Error {
    #[error(transparent)]
    InvalidProtobuf(#[from] quick_protobuf::Error),
    #[error("{0}")]
    Other(String),
    #[error(transparent)]
    Parse(#[from] combine::error::UnexpectedParse),
    #[error(transparent)]
    UrlParse(#[from] url::ParseError),
    #[error("empty label value")]
    EmptyLabelValue,
}

use std::convert::TryFrom;

impl<'a> TryFrom<&'a [u8]> for opentelemetry::metrics::ExportMetricsServiceRequest<'a> {
    type Error = crate::Error;

    fn try_from(bytes: &'a [u8]) -> Result<Self, Self::Error> {
        let mut reader = quick_protobuf::BytesReader::from_bytes(bytes);
        Ok(opentelemetry::metrics::ExportMetricsServiceRequest::from_reader(&mut reader, bytes)?)
    }
}

impl<'a> TryFrom<&'a [u8]> for opentelemetry::logs::ExportLogsServiceRequest<'a> {
    type Error = crate::Error;

    fn try_from(bytes: &'a [u8]) -> Result<Self, Self::Error> {
        let mut reader = quick_protobuf::BytesReader::from_bytes(bytes);
        Ok(opentelemetry::logs::ExportLogsServiceRequest::from_reader(
            &mut reader,
            bytes,
        )?)
    }
}

impl<'a> TryFrom<&'a [u8]> for opentelemetry::trace::ExportTraceServiceRequest<'a> {
    type Error = crate::Error;

    fn try_from(bytes: &'a [u8]) -> Result<Self, Self::Error> {
        let mut reader = quick_protobuf::BytesReader::from_bytes(bytes);
        Ok(opentelemetry::trace::ExportTraceServiceRequest::from_reader(
            &mut reader,
            bytes,
        )?)
    }
}

#[cfg(test)]
mod tests {
    use crate::validation::common::CommonValidate;
    use crate::validation::logs::LogValidate;
    use crate::validation::metrics::MetricValidate;
    use crate::validation::trace::TraceValidate;
    use quick_protobuf::{BytesReader, Writer};
    use std::borrow::Cow;

    #[test]
    fn roundtrip_metrics_gauge_data() {
        use crate::opentelemetry::metrics::{
            AnyValue, AnyValueOneOfvalue, Exemplar, ExemplarOneOfvalue,
            ExportMetricsServiceRequest, Gauge, InstrumentationScope, KeyValue, Metric,
            MetricOneOfdata, NumberDataPoint, NumberDataPointOneOfvalue, Resource, ResourceMetrics,
            ScopeMetrics,
        };

        let mut out = vec![];

        let key_value = KeyValue {
            key: Cow::from("test"),
            value: Some(AnyValue {
                value: AnyValueOneOfvalue::string_value(Cow::from("test")),
            }),
        };

        let metrics_data = ExportMetricsServiceRequest {
            resource_metrics: vec![ResourceMetrics {
                resource: Some(Resource {
                    attributes: vec![key_value.clone()],
                    dropped_attributes_count: 10,
                }),
                scope_metrics: vec![ScopeMetrics {
                    scope: Some(InstrumentationScope {
                        name: Cow::from("test_name"),
                        version: Cow::from("1.2.3"),
                        attributes: vec![key_value.clone()],
                        dropped_attributes_count: 10,
                    }),
                    metrics: vec![Metric {
                        name: Cow::from("test_name"),
                        description: Cow::from("test_description"),
                        unit: Cow::from("123.[psi]"),
                        data: MetricOneOfdata::gauge(Gauge {
                            data_points: vec![NumberDataPoint {
                                attributes: vec![key_value.clone()],
                                start_time_unix_nano: 1681339577345243523,
                                time_unix_nano: 1681339577345243523,
                                value: NumberDataPointOneOfvalue::as_int(10),
                                exemplars: vec![Exemplar {
                                    filtered_attributes: vec![key_value.clone()],
                                    time_unix_nano: 1681339577345243523,
                                    value: ExemplarOneOfvalue::as_int(10),
                                    span_id: Cow::from("test".as_bytes()),
                                    trace_id: Cow::from("test".as_bytes()),
                                }],
                                flags: 1,
                            }],
                        }),
                    }],
                    schema_url: Cow::from("https://some_url.com"),
                }],
                schema_url: Cow::from("https://some_url.com"),
            }],
        };
        {
            let mut writer = Writer::new(&mut out);
            writer
                .write_message(&metrics_data)
                .expect("failed to write");
        }

        let expected: [u8; 250] = [
            248, 1, 10, 245, 1, 10, 18, 10, 14, 10, 4, 116, 101, 115, 116, 18, 6, 10, 4, 116, 101,
            115, 116, 16, 10, 18, 200, 1, 10, 36, 10, 9, 116, 101, 115, 116, 95, 110, 97, 109, 101,
            18, 5, 49, 46, 50, 46, 51, 26, 14, 10, 4, 116, 101, 115, 116, 18, 6, 10, 4, 116, 101,
            115, 116, 32, 10, 18, 137, 1, 10, 9, 116, 101, 115, 116, 95, 110, 97, 109, 101, 18, 16,
            116, 101, 115, 116, 95, 100, 101, 115, 99, 114, 105, 112, 116, 105, 111, 110, 26, 9,
            49, 50, 51, 46, 91, 112, 115, 105, 93, 42, 95, 10, 93, 58, 14, 10, 4, 116, 101, 115,
            116, 18, 6, 10, 4, 116, 101, 115, 116, 17, 131, 249, 119, 254, 111, 81, 85, 23, 25,
            131, 249, 119, 254, 111, 81, 85, 23, 42, 46, 58, 14, 10, 4, 116, 101, 115, 116, 18, 6,
            10, 4, 116, 101, 115, 116, 17, 131, 249, 119, 254, 111, 81, 85, 23, 34, 4, 116, 101,
            115, 116, 42, 4, 116, 101, 115, 116, 49, 10, 0, 0, 0, 0, 0, 0, 0, 64, 1, 49, 10, 0, 0,
            0, 0, 0, 0, 0, 26, 20, 104, 116, 116, 112, 115, 58, 47, 47, 115, 111, 109, 101, 95,
            117, 114, 108, 46, 99, 111, 109, 26, 20, 104, 116, 116, 112, 115, 58, 47, 47, 115, 111,
            109, 101, 95, 117, 114, 108, 46, 99, 111, 109,
        ];

        assert_eq!(&out, &expected[..]);

        let read_message = {
            let mut reader = BytesReader::from_bytes(&out);
            reader
                .read_message::<ExportMetricsServiceRequest>(&out)
                .expect("Cannot read message")
        };
        assert_eq!(metrics_data, read_message);
        metrics_data.validate().expect("validation failed");
    }

    #[test]
    fn roundtrip_metrics_sum_data() {
        use crate::opentelemetry::metrics::{
            AggregationTemporality, AnyValue, AnyValueOneOfvalue, Exemplar, ExemplarOneOfvalue,
            ExportMetricsServiceRequest, InstrumentationScope, KeyValue, Metric, MetricOneOfdata,
            NumberDataPoint, NumberDataPointOneOfvalue, Resource, ResourceMetrics, ScopeMetrics,
            Sum,
        };

        let mut out = vec![];

        let key_value = KeyValue {
            key: Cow::from("test"),
            value: Some(AnyValue {
                value: AnyValueOneOfvalue::string_value(Cow::from("test")),
            }),
        };

        let metrics_data = ExportMetricsServiceRequest {
            resource_metrics: vec![ResourceMetrics {
                resource: Some(Resource {
                    attributes: vec![key_value.clone()],
                    dropped_attributes_count: 10,
                }),
                scope_metrics: vec![ScopeMetrics {
                    scope: Some(InstrumentationScope {
                        name: Cow::from("test_name"),
                        version: Cow::from("1.2.3"),
                        attributes: vec![key_value.clone()],
                        dropped_attributes_count: 10,
                    }),
                    metrics: vec![Metric {
                        name: Cow::from("test_name"),
                        description: Cow::from("test_description"),
                        unit: Cow::from("123.[psi]"),
                        data: MetricOneOfdata::sum(Sum {
                            data_points: vec![NumberDataPoint {
                                attributes: vec![key_value.clone()],
                                start_time_unix_nano: 1681339577345243523,
                                time_unix_nano: 1681339577345243523,
                                value: NumberDataPointOneOfvalue::as_int(10),
                                exemplars: vec![Exemplar {
                                    filtered_attributes: vec![key_value.clone()],
                                    time_unix_nano: 1681339577345243523,
                                    value: ExemplarOneOfvalue::as_int(10),
                                    span_id: Cow::from("test".as_bytes()),
                                    trace_id: Cow::from("test".as_bytes()),
                                }],
                                flags: 1,
                            }],
                            aggregation_temporality:
                                AggregationTemporality::AGGREGATION_TEMPORALITY_UNSPECIFIED,
                            is_monotonic: true,
                        }),
                    }],
                    schema_url: Cow::from("https://some_url.com"),
                }],
                schema_url: Cow::from("https://some_url.com"),
            }],
        };
        {
            let mut writer = Writer::new(&mut out);
            writer
                .write_message(&metrics_data)
                .expect("failed to write");
        }

        let expected: [u8; 252] = [
            250, 1, 10, 247, 1, 10, 18, 10, 14, 10, 4, 116, 101, 115, 116, 18, 6, 10, 4, 116, 101,
            115, 116, 16, 10, 18, 202, 1, 10, 36, 10, 9, 116, 101, 115, 116, 95, 110, 97, 109, 101,
            18, 5, 49, 46, 50, 46, 51, 26, 14, 10, 4, 116, 101, 115, 116, 18, 6, 10, 4, 116, 101,
            115, 116, 32, 10, 18, 139, 1, 10, 9, 116, 101, 115, 116, 95, 110, 97, 109, 101, 18, 16,
            116, 101, 115, 116, 95, 100, 101, 115, 99, 114, 105, 112, 116, 105, 111, 110, 26, 9,
            49, 50, 51, 46, 91, 112, 115, 105, 93, 58, 97, 10, 93, 58, 14, 10, 4, 116, 101, 115,
            116, 18, 6, 10, 4, 116, 101, 115, 116, 17, 131, 249, 119, 254, 111, 81, 85, 23, 25,
            131, 249, 119, 254, 111, 81, 85, 23, 42, 46, 58, 14, 10, 4, 116, 101, 115, 116, 18, 6,
            10, 4, 116, 101, 115, 116, 17, 131, 249, 119, 254, 111, 81, 85, 23, 34, 4, 116, 101,
            115, 116, 42, 4, 116, 101, 115, 116, 49, 10, 0, 0, 0, 0, 0, 0, 0, 64, 1, 49, 10, 0, 0,
            0, 0, 0, 0, 0, 24, 1, 26, 20, 104, 116, 116, 112, 115, 58, 47, 47, 115, 111, 109, 101,
            95, 117, 114, 108, 46, 99, 111, 109, 26, 20, 104, 116, 116, 112, 115, 58, 47, 47, 115,
            111, 109, 101, 95, 117, 114, 108, 46, 99, 111, 109,
        ];

        assert_eq!(&out, &expected[..]);

        let read_message = {
            let mut reader = BytesReader::from_bytes(&out);
            reader
                .read_message::<ExportMetricsServiceRequest>(&out)
                .expect("Cannot read message")
        };
        assert_eq!(metrics_data, read_message);
        metrics_data.validate().expect("validation failed");
    }

    #[test]
    fn roundtrip_metrics_histogram_data() {
        use crate::opentelemetry::metrics::{
            AggregationTemporality, AnyValue, AnyValueOneOfvalue, Exemplar, ExemplarOneOfvalue,
            ExportMetricsServiceRequest, Histogram, HistogramDataPoint, InstrumentationScope,
            KeyValue, Metric, MetricOneOfdata, Resource, ResourceMetrics, ScopeMetrics,
        };

        let mut out = vec![];

        let key_value = KeyValue {
            key: Cow::from("test"),
            value: Some(AnyValue {
                value: AnyValueOneOfvalue::string_value(Cow::from("test")),
            }),
        };

        let metrics_data = ExportMetricsServiceRequest {
            resource_metrics: vec![ResourceMetrics {
                resource: Some(Resource {
                    attributes: vec![key_value.clone()],
                    dropped_attributes_count: 10,
                }),
                scope_metrics: vec![ScopeMetrics {
                    scope: Some(InstrumentationScope {
                        name: Cow::from("test_name"),
                        version: Cow::from("1.2.3"),
                        attributes: vec![key_value.clone()],
                        dropped_attributes_count: 10,
                    }),
                    metrics: vec![Metric {
                        name: Cow::from("test_name"),
                        description: Cow::from("test_description"),
                        unit: Cow::from("123.[psi]"),
                        data: MetricOneOfdata::histogram(Histogram {
                            data_points: vec![HistogramDataPoint {
                                attributes: vec![key_value.clone()],
                                start_time_unix_nano: 1681339577345243523,
                                time_unix_nano: 1681339577345243523,
                                count: 10,
                                sum: 3.7_f64,
                                bucket_counts: Cow::from(vec![1, 2, 3]),
                                explicit_bounds: Cow::from(vec![1.3_f64, 5.9_f64]),
                                exemplars: vec![Exemplar {
                                    filtered_attributes: vec![key_value.clone()],
                                    time_unix_nano: 1681339577345243523,
                                    value: ExemplarOneOfvalue::as_int(10),
                                    span_id: Cow::from("test".as_bytes()),
                                    trace_id: Cow::from("test".as_bytes()),
                                }],
                                flags: 1,
                                min: 0.1_f64,
                                max: 9.9_f64,
                            }],
                            aggregation_temporality:
                                AggregationTemporality::AGGREGATION_TEMPORALITY_CUMULATIVE,
                        }),
                    }],
                    schema_url: Cow::from("https://some_url.com"),
                }],
                schema_url: Cow::from("https://some_url.com"),
            }],
        };
        {
            let mut writer = Writer::new(&mut out);
            writer
                .write_message(&metrics_data)
                .expect("failed to write");
        }

        let expected: [u8; 325] = [
            195, 2, 10, 192, 2, 10, 18, 10, 14, 10, 4, 116, 101, 115, 116, 18, 6, 10, 4, 116, 101,
            115, 116, 16, 10, 18, 147, 2, 10, 36, 10, 9, 116, 101, 115, 116, 95, 110, 97, 109, 101,
            18, 5, 49, 46, 50, 46, 51, 26, 14, 10, 4, 116, 101, 115, 116, 18, 6, 10, 4, 116, 101,
            115, 116, 32, 10, 18, 212, 1, 10, 9, 116, 101, 115, 116, 95, 110, 97, 109, 101, 18, 16,
            116, 101, 115, 116, 95, 100, 101, 115, 99, 114, 105, 112, 116, 105, 111, 110, 26, 9,
            49, 50, 51, 46, 91, 112, 115, 105, 93, 74, 169, 1, 10, 164, 1, 74, 14, 10, 4, 116, 101,
            115, 116, 18, 6, 10, 4, 116, 101, 115, 116, 17, 131, 249, 119, 254, 111, 81, 85, 23,
            25, 131, 249, 119, 254, 111, 81, 85, 23, 33, 10, 0, 0, 0, 0, 0, 0, 0, 41, 154, 153,
            153, 153, 153, 153, 13, 64, 50, 24, 1, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 3,
            0, 0, 0, 0, 0, 0, 0, 58, 16, 205, 204, 204, 204, 204, 204, 244, 63, 154, 153, 153, 153,
            153, 153, 23, 64, 66, 46, 58, 14, 10, 4, 116, 101, 115, 116, 18, 6, 10, 4, 116, 101,
            115, 116, 17, 131, 249, 119, 254, 111, 81, 85, 23, 34, 4, 116, 101, 115, 116, 42, 4,
            116, 101, 115, 116, 49, 10, 0, 0, 0, 0, 0, 0, 0, 80, 1, 89, 154, 153, 153, 153, 153,
            153, 185, 63, 97, 205, 204, 204, 204, 204, 204, 35, 64, 16, 2, 26, 20, 104, 116, 116,
            112, 115, 58, 47, 47, 115, 111, 109, 101, 95, 117, 114, 108, 46, 99, 111, 109, 26, 20,
            104, 116, 116, 112, 115, 58, 47, 47, 115, 111, 109, 101, 95, 117, 114, 108, 46, 99,
            111, 109,
        ];

        assert_eq!(&out, &expected[..]);

        let read_message = {
            let mut reader = BytesReader::from_bytes(&out);
            reader
                .read_message::<ExportMetricsServiceRequest>(&out)
                .expect("Cannot read message")
        };
        assert_eq!(metrics_data, read_message);
        metrics_data.validate().expect("validation failed");
    }

    #[test]
    fn roundtrip_metrics_exponential_histogram_data() {
        use crate::opentelemetry::metrics::{
            AggregationTemporality, AnyValue, AnyValueOneOfvalue, Exemplar, ExemplarOneOfvalue,
            ExponentialHistogram, ExponentialHistogramDataPoint,
            ExponentialHistogramDataPointBuckets, ExportMetricsServiceRequest,
            InstrumentationScope, KeyValue, Metric, MetricOneOfdata, Resource, ResourceMetrics,
            ScopeMetrics,
        };

        let mut out = vec![];

        let key_value = KeyValue {
            key: Cow::from("test"),
            value: Some(AnyValue {
                value: AnyValueOneOfvalue::string_value(Cow::from("test")),
            }),
        };

        let metrics_data = ExportMetricsServiceRequest {
            resource_metrics: vec![ResourceMetrics {
                resource: Some(Resource {
                    attributes: vec![key_value.clone()],
                    dropped_attributes_count: 10,
                }),
                scope_metrics: vec![ScopeMetrics {
                    scope: Some(InstrumentationScope {
                        name: Cow::from("test_name"),
                        version: Cow::from("1.2.3"),
                        attributes: vec![key_value.clone()],
                        dropped_attributes_count: 10,
                    }),
                    metrics: vec![Metric {
                        name: Cow::from("test_name"),
                        description: Cow::from("test_description"),
                        unit: Cow::from("123.[psi]"),
                        data: MetricOneOfdata::exponential_histogram(ExponentialHistogram {
                            data_points: vec![ExponentialHistogramDataPoint {
                                attributes: vec![key_value.clone()],
                                start_time_unix_nano: 1681339577345243523,
                                time_unix_nano: 1681339577345243523,
                                count: 10,
                                sum: 3.7_f64,
                                scale: 10,
                                zero_count: 12,
                                positive: Some(ExponentialHistogramDataPointBuckets {
                                    offset: 1,
                                    bucket_counts: vec![18446744073709551615],
                                }),
                                negative: Some(ExponentialHistogramDataPointBuckets {
                                    offset: 1,
                                    bucket_counts: vec![0, 18446744073709551615],
                                }),
                                flags: 1,
                                exemplars: vec![Exemplar {
                                    filtered_attributes: vec![key_value.clone()],
                                    time_unix_nano: 1681339577345243523,
                                    value: ExemplarOneOfvalue::as_int(10),
                                    span_id: Cow::from("test".as_bytes()),
                                    trace_id: Cow::from("test".as_bytes()),
                                }],
                                min: 0.1_f64,
                                max: 9.9_f64,
                                zero_threshold: 3.3_f64,
                            }],
                            aggregation_temporality:
                                AggregationTemporality::AGGREGATION_TEMPORALITY_CUMULATIVE,
                        }),
                    }],
                    schema_url: Cow::from("https://some_url.com"),
                }],
                schema_url: Cow::from("https://some_url.com"),
            }],
        };
        {
            let mut writer = Writer::new(&mut out);
            writer
                .write_message(&metrics_data)
                .expect("failed to write");
        }

        let expected: [u8; 333] = [
            203, 2, 10, 200, 2, 10, 18, 10, 14, 10, 4, 116, 101, 115, 116, 18, 6, 10, 4, 116, 101,
            115, 116, 16, 10, 18, 155, 2, 10, 36, 10, 9, 116, 101, 115, 116, 95, 110, 97, 109, 101,
            18, 5, 49, 46, 50, 46, 51, 26, 14, 10, 4, 116, 101, 115, 116, 18, 6, 10, 4, 116, 101,
            115, 116, 32, 10, 18, 220, 1, 10, 9, 116, 101, 115, 116, 95, 110, 97, 109, 101, 18, 16,
            116, 101, 115, 116, 95, 100, 101, 115, 99, 114, 105, 112, 116, 105, 111, 110, 26, 9,
            49, 50, 51, 46, 91, 112, 115, 105, 93, 82, 177, 1, 10, 172, 1, 10, 14, 10, 4, 116, 101,
            115, 116, 18, 6, 10, 4, 116, 101, 115, 116, 17, 131, 249, 119, 254, 111, 81, 85, 23,
            25, 131, 249, 119, 254, 111, 81, 85, 23, 33, 10, 0, 0, 0, 0, 0, 0, 0, 41, 154, 153,
            153, 153, 153, 153, 13, 64, 48, 20, 57, 12, 0, 0, 0, 0, 0, 0, 0, 66, 13, 8, 2, 16, 255,
            255, 255, 255, 255, 255, 255, 255, 255, 1, 74, 15, 8, 2, 16, 0, 16, 255, 255, 255, 255,
            255, 255, 255, 255, 255, 1, 80, 1, 90, 46, 58, 14, 10, 4, 116, 101, 115, 116, 18, 6,
            10, 4, 116, 101, 115, 116, 17, 131, 249, 119, 254, 111, 81, 85, 23, 34, 4, 116, 101,
            115, 116, 42, 4, 116, 101, 115, 116, 49, 10, 0, 0, 0, 0, 0, 0, 0, 97, 154, 153, 153,
            153, 153, 153, 185, 63, 105, 205, 204, 204, 204, 204, 204, 35, 64, 113, 102, 102, 102,
            102, 102, 102, 10, 64, 16, 2, 26, 20, 104, 116, 116, 112, 115, 58, 47, 47, 115, 111,
            109, 101, 95, 117, 114, 108, 46, 99, 111, 109, 26, 20, 104, 116, 116, 112, 115, 58, 47,
            47, 115, 111, 109, 101, 95, 117, 114, 108, 46, 99, 111, 109,
        ];

        assert_eq!(&out, &expected[..]);

        let read_message = {
            let mut reader = BytesReader::from_bytes(&out);
            reader
                .read_message::<ExportMetricsServiceRequest>(&out)
                .expect("Cannot read message")
        };
        assert_eq!(metrics_data, read_message);
        metrics_data.validate().expect("validation failed");
    }

    #[test]
    fn roundtrip_metrics_summary_data() {
        use crate::opentelemetry::metrics::{
            AnyValue, AnyValueOneOfvalue, ExportMetricsServiceRequest, InstrumentationScope,
            KeyValue, Metric, MetricOneOfdata, Resource, ResourceMetrics, ScopeMetrics, Summary,
            SummaryDataPoint, SummaryDataPointValueAtQuantile,
        };

        let mut out = vec![];

        let key_value = KeyValue {
            key: Cow::from("test"),
            value: Some(AnyValue {
                value: AnyValueOneOfvalue::string_value(Cow::from("test")),
            }),
        };

        let metrics_data = ExportMetricsServiceRequest {
            resource_metrics: vec![ResourceMetrics {
                resource: Some(Resource {
                    attributes: vec![key_value.clone()],
                    dropped_attributes_count: 10,
                }),
                scope_metrics: vec![ScopeMetrics {
                    scope: Some(InstrumentationScope {
                        name: Cow::from("test_name"),
                        version: Cow::from("1.2.3"),
                        attributes: vec![key_value.clone()],
                        dropped_attributes_count: 10,
                    }),
                    metrics: vec![Metric {
                        name: Cow::from("test_name"),
                        description: Cow::from("test_description"),
                        unit: Cow::from("123.[psi]"),
                        data: MetricOneOfdata::summary(Summary {
                            data_points: vec![SummaryDataPoint {
                                attributes: vec![key_value.clone()],
                                start_time_unix_nano: 1681339577345243523,
                                time_unix_nano: 1681339577345243523,
                                count: 10,
                                sum: 3.7_f64,
                                quantile_values: vec![SummaryDataPointValueAtQuantile {
                                    quantile: 1.0_f64,
                                    value: 2.0_f64,
                                }],
                                flags: 1,
                            }],
                        }),
                    }],
                    schema_url: Cow::from("https://some_url.com"),
                }],
                schema_url: Cow::from("https://some_url.com"),
            }],
        };
        {
            let mut writer = Writer::new(&mut out);
            writer
                .write_message(&metrics_data)
                .expect("failed to write");
        }

        let expected: [u8; 230] = [
            228, 1, 10, 225, 1, 10, 18, 10, 14, 10, 4, 116, 101, 115, 116, 18, 6, 10, 4, 116, 101,
            115, 116, 16, 10, 18, 180, 1, 10, 36, 10, 9, 116, 101, 115, 116, 95, 110, 97, 109, 101,
            18, 5, 49, 46, 50, 46, 51, 26, 14, 10, 4, 116, 101, 115, 116, 18, 6, 10, 4, 116, 101,
            115, 116, 32, 10, 18, 118, 10, 9, 116, 101, 115, 116, 95, 110, 97, 109, 101, 18, 16,
            116, 101, 115, 116, 95, 100, 101, 115, 99, 114, 105, 112, 116, 105, 111, 110, 26, 9,
            49, 50, 51, 46, 91, 112, 115, 105, 93, 90, 76, 10, 74, 58, 14, 10, 4, 116, 101, 115,
            116, 18, 6, 10, 4, 116, 101, 115, 116, 17, 131, 249, 119, 254, 111, 81, 85, 23, 25,
            131, 249, 119, 254, 111, 81, 85, 23, 33, 10, 0, 0, 0, 0, 0, 0, 0, 41, 154, 153, 153,
            153, 153, 153, 13, 64, 50, 18, 9, 0, 0, 0, 0, 0, 0, 240, 63, 17, 0, 0, 0, 0, 0, 0, 0,
            64, 64, 1, 26, 20, 104, 116, 116, 112, 115, 58, 47, 47, 115, 111, 109, 101, 95, 117,
            114, 108, 46, 99, 111, 109, 26, 20, 104, 116, 116, 112, 115, 58, 47, 47, 115, 111, 109,
            101, 95, 117, 114, 108, 46, 99, 111, 109,
        ];

        assert_eq!(&out, &expected[..]);

        let read_message = {
            let mut reader = BytesReader::from_bytes(&out);
            reader
                .read_message::<ExportMetricsServiceRequest>(&out)
                .expect("Cannot read message")
        };
        assert_eq!(metrics_data, read_message);
        metrics_data.validate().expect("validation failed");
    }

    #[test]
    fn roundtrip_key_value() {
        use crate::opentelemetry::metrics::{AnyValue, AnyValueOneOfvalue, KeyValue};

        let mut out = vec![];

        let any_value = AnyValue {
            value: AnyValueOneOfvalue::string_value(Cow::from("test")),
        };

        let message = KeyValue {
            key: Cow::from("test"),
            value: Some(any_value),
        };
        {
            let mut writer = Writer::new(&mut out);
            writer.write_message(&message).expect("failed to write");
        }

        let expected: [u8; 15] = [
            14, 10, 4, 116, 101, 115, 116, 18, 6, 10, 4, 116, 101, 115, 116,
        ];

        assert_eq!(&out, &expected[..]);

        let read_message = {
            let mut reader = BytesReader::from_bytes(&out);
            reader
                .read_message::<KeyValue>(&out)
                .expect("Cannot read message")
        };
        assert_eq!(message, read_message);
        message.validate().expect("validation failed");
    }

    #[test]
    fn roundtrip_metrics_invalid_schema_url() {
        use crate::opentelemetry::metrics::{
            AnyValue, AnyValueOneOfvalue, ExportMetricsServiceRequest, InstrumentationScope,
            KeyValue, Metric, MetricOneOfdata, Resource, ResourceMetrics, ScopeMetrics, Summary,
            SummaryDataPoint, SummaryDataPointValueAtQuantile,
        };

        let mut out = vec![];

        let key_value = KeyValue {
            key: Cow::from("test"),
            value: Some(AnyValue {
                value: AnyValueOneOfvalue::string_value(Cow::from("test")),
            }),
        };

        let metrics_data = ExportMetricsServiceRequest {
            resource_metrics: vec![ResourceMetrics {
                resource: Some(Resource {
                    attributes: vec![key_value.clone()],
                    dropped_attributes_count: 10,
                }),
                scope_metrics: vec![ScopeMetrics {
                    scope: Some(InstrumentationScope {
                        name: Cow::from("test_name"),
                        version: Cow::from("1.2.3"),
                        attributes: vec![key_value.clone()],
                        dropped_attributes_count: 10,
                    }),
                    metrics: vec![Metric {
                        name: Cow::from("test_name"),
                        description: Cow::from("test_description"),
                        unit: Cow::from("123.[psi]"),
                        data: MetricOneOfdata::summary(Summary {
                            data_points: vec![SummaryDataPoint {
                                attributes: vec![key_value.clone()],
                                start_time_unix_nano: 1681339577345243523,
                                time_unix_nano: 1681339577345243523,
                                count: 10,
                                sum: 3.7_f64,
                                quantile_values: vec![SummaryDataPointValueAtQuantile {
                                    quantile: 1.0_f64,
                                    value: 2.0_f64,
                                }],
                                flags: 1,
                            }],
                        }),
                    }],
                    schema_url: Cow::from("https://"),
                }],
                schema_url: Cow::from("https://"),
            }],
        };
        {
            let mut writer = Writer::new(&mut out);
            writer
                .write_message(&metrics_data)
                .expect("failed to write");
        }

        let expected: [u8; 206] = [
            204, 1, 10, 201, 1, 10, 18, 10, 14, 10, 4, 116, 101, 115, 116, 18, 6, 10, 4, 116, 101,
            115, 116, 16, 10, 18, 168, 1, 10, 36, 10, 9, 116, 101, 115, 116, 95, 110, 97, 109, 101,
            18, 5, 49, 46, 50, 46, 51, 26, 14, 10, 4, 116, 101, 115, 116, 18, 6, 10, 4, 116, 101,
            115, 116, 32, 10, 18, 118, 10, 9, 116, 101, 115, 116, 95, 110, 97, 109, 101, 18, 16,
            116, 101, 115, 116, 95, 100, 101, 115, 99, 114, 105, 112, 116, 105, 111, 110, 26, 9,
            49, 50, 51, 46, 91, 112, 115, 105, 93, 90, 76, 10, 74, 58, 14, 10, 4, 116, 101, 115,
            116, 18, 6, 10, 4, 116, 101, 115, 116, 17, 131, 249, 119, 254, 111, 81, 85, 23, 25,
            131, 249, 119, 254, 111, 81, 85, 23, 33, 10, 0, 0, 0, 0, 0, 0, 0, 41, 154, 153, 153,
            153, 153, 153, 13, 64, 50, 18, 9, 0, 0, 0, 0, 0, 0, 240, 63, 17, 0, 0, 0, 0, 0, 0, 0,
            64, 64, 1, 26, 8, 104, 116, 116, 112, 115, 58, 47, 47, 26, 8, 104, 116, 116, 112, 115,
            58, 47, 47,
        ];

        assert_eq!(&out, &expected[..]);

        let read_message = {
            let mut reader = BytesReader::from_bytes(&out);
            reader
                .read_message::<ExportMetricsServiceRequest>(&out)
                .expect("Cannot read message")
        };

        assert_eq!(metrics_data, read_message);

        match metrics_data.validate() {
            Err(e) => assert_eq!(e.to_string(), "empty host"),
            Ok(_) => panic!("Validation should failed"),
        }
    }

    #[test]
    fn roundtrip_metrics_real_otlp_request_body() {
        use crate::opentelemetry::metrics::ExportMetricsServiceRequest;

        let out: &[u8] = b"\n\xa7\x02\n\xb8\x01\n)\n\x11service.namespace\x12\x14\n\x12opentelemetry-demo\n!\n\x0cservice.name\x12\x11\n\x0fcurrencyservice\n \n\x15telemetry.sdk.version\x12\x07\n\x051.8.2\n%\n\x12telemetry.sdk.name\x12\x0f\n\ropentelemetry\n\x1f\n\x16telemetry.sdk.language\x12\x05\n\x03cpp\x12j\n\x15\n\x0capp_currency\x12\x051.3.0\x12Q\n\x14app_currency_counter:9\n3\x11\xdc\xf9\0xl\x18W\x17\x19\xb7\xa2\xa1\xb3l\x18W\x171\x02\0\0\0\0\0\0\0:\x16\n\rcurrency_code\x12\x05\n\x03USD\x10\x01\x18\x01";

        ExportMetricsServiceRequest::try_from(out)
            .expect("Parsing of metrics protobuf failed")
            .validate()
            .expect("Validation of metrics protobuf failed");
    }

    #[test]
    fn roundtrip_logs_valid_data() {
        use crate::opentelemetry::logs::{
            AnyValue, AnyValueOneOfvalue, ExportLogsServiceRequest, InstrumentationScope, KeyValue,
            LogRecord, Resource, ResourceLogs, ScopeLogs, SeverityNumber,
        };

        let mut out = vec![];

        let key_value = KeyValue {
            key: Cow::from("test"),
            value: Some(AnyValue {
                value: AnyValueOneOfvalue::string_value(Cow::from("test")),
            }),
        };

        let logs_data = ExportLogsServiceRequest {
            resource_logs: vec![ResourceLogs {
                resource: Some(Resource {
                    attributes: vec![key_value.clone()],
                    dropped_attributes_count: 10,
                }),
                scope_logs: vec![ScopeLogs {
                    scope: Some(InstrumentationScope {
                        name: Cow::from("test_name"),
                        version: Cow::from("1.2.3"),
                        attributes: vec![key_value.clone()],
                        dropped_attributes_count: 10,
                    }),
                    log_records: vec![LogRecord {
                        time_unix_nano: 1681339577345243523,
                        observed_time_unix_nano: 1681339577345243523,
                        severity_number: SeverityNumber::SEVERITY_NUMBER_INFO,
                        severity_text: Cow::from("test_text"),
                        body: Some(AnyValue {
                            value: AnyValueOneOfvalue::string_value(Cow::from("test")),
                        }),
                        attributes: vec![key_value.clone()],
                        dropped_attributes_count: 10,
                        flags: 1,
                        span_id: Cow::from("test".as_bytes()),
                        trace_id: Cow::from("test".as_bytes()),
                    }],
                    schema_url: Cow::from("https://some_url.com"),
                }],
                schema_url: Cow::from("https://some_url.com"),
            }],
        };
        {
            let mut writer = Writer::new(&mut out);
            writer.write_message(&logs_data).expect("failed to write");
        }

        let expected: [u8; 186] = [
            184, 1, 10, 181, 1, 10, 18, 10, 14, 10, 4, 116, 101, 115, 116, 18, 6, 10, 4, 116, 101,
            115, 116, 16, 10, 18, 136, 1, 10, 36, 10, 9, 116, 101, 115, 116, 95, 110, 97, 109, 101,
            18, 5, 49, 46, 50, 46, 51, 26, 14, 10, 4, 116, 101, 115, 116, 18, 6, 10, 4, 116, 101,
            115, 116, 32, 10, 18, 74, 9, 131, 249, 119, 254, 111, 81, 85, 23, 89, 131, 249, 119,
            254, 111, 81, 85, 23, 16, 9, 26, 9, 116, 101, 115, 116, 95, 116, 101, 120, 116, 42, 6,
            10, 4, 116, 101, 115, 116, 50, 14, 10, 4, 116, 101, 115, 116, 18, 6, 10, 4, 116, 101,
            115, 116, 56, 10, 69, 1, 0, 0, 0, 74, 4, 116, 101, 115, 116, 82, 4, 116, 101, 115, 116,
            26, 20, 104, 116, 116, 112, 115, 58, 47, 47, 115, 111, 109, 101, 95, 117, 114, 108, 46,
            99, 111, 109, 26, 20, 104, 116, 116, 112, 115, 58, 47, 47, 115, 111, 109, 101, 95, 117,
            114, 108, 46, 99, 111, 109,
        ];

        assert_eq!(&out, &expected[..]);

        let read_message = {
            let mut reader = BytesReader::from_bytes(&out);
            reader
                .read_message::<ExportLogsServiceRequest>(&out)
                .expect("Cannot read message")
        };
        assert_eq!(logs_data, read_message);
        logs_data.validate().expect("validation failed");
    }

    #[test]
    fn roundtrip_logs_invalid_schema_url() {
        use crate::opentelemetry::logs::{
            AnyValue, AnyValueOneOfvalue, ExportLogsServiceRequest, InstrumentationScope, KeyValue,
            LogRecord, Resource, ResourceLogs, ScopeLogs, SeverityNumber,
        };

        let mut out = vec![];

        let key_value = KeyValue {
            key: Cow::from("test"),
            value: Some(AnyValue {
                value: AnyValueOneOfvalue::string_value(Cow::from("test")),
            }),
        };

        let logs_data = ExportLogsServiceRequest {
            resource_logs: vec![ResourceLogs {
                resource: Some(Resource {
                    attributes: vec![key_value.clone()],
                    dropped_attributes_count: 10,
                }),
                scope_logs: vec![ScopeLogs {
                    scope: Some(InstrumentationScope {
                        name: Cow::from("test_name"),
                        version: Cow::from("1.2.3"),
                        attributes: vec![key_value.clone()],
                        dropped_attributes_count: 10,
                    }),
                    log_records: vec![LogRecord {
                        time_unix_nano: 1681339577345243523,
                        observed_time_unix_nano: 1681339577345243523,
                        severity_number: SeverityNumber::SEVERITY_NUMBER_INFO,
                        severity_text: Cow::from("test_text"),
                        body: Some(AnyValue {
                            value: AnyValueOneOfvalue::string_value(Cow::from("test")),
                        }),
                        attributes: vec![key_value.clone()],
                        dropped_attributes_count: 10,
                        flags: 1,
                        span_id: Cow::from("test".as_bytes()),
                        trace_id: Cow::from("test".as_bytes()),
                    }],
                    schema_url: Cow::from("https://"),
                }],
                schema_url: Cow::from("https://"),
            }],
        };
        {
            let mut writer = Writer::new(&mut out);
            writer.write_message(&logs_data).expect("failed to write");
        }

        let expected: [u8; 161] = [
            159, 1, 10, 156, 1, 10, 18, 10, 14, 10, 4, 116, 101, 115, 116, 18, 6, 10, 4, 116, 101,
            115, 116, 16, 10, 18, 124, 10, 36, 10, 9, 116, 101, 115, 116, 95, 110, 97, 109, 101,
            18, 5, 49, 46, 50, 46, 51, 26, 14, 10, 4, 116, 101, 115, 116, 18, 6, 10, 4, 116, 101,
            115, 116, 32, 10, 18, 74, 9, 131, 249, 119, 254, 111, 81, 85, 23, 89, 131, 249, 119,
            254, 111, 81, 85, 23, 16, 9, 26, 9, 116, 101, 115, 116, 95, 116, 101, 120, 116, 42, 6,
            10, 4, 116, 101, 115, 116, 50, 14, 10, 4, 116, 101, 115, 116, 18, 6, 10, 4, 116, 101,
            115, 116, 56, 10, 69, 1, 0, 0, 0, 74, 4, 116, 101, 115, 116, 82, 4, 116, 101, 115, 116,
            26, 8, 104, 116, 116, 112, 115, 58, 47, 47, 26, 8, 104, 116, 116, 112, 115, 58, 47, 47,
        ];

        assert_eq!(&out, &expected[..]);

        let read_message = {
            let mut reader = BytesReader::from_bytes(&out);
            reader
                .read_message::<ExportLogsServiceRequest>(&out)
                .expect("Cannot read message")
        };

        assert_eq!(logs_data, read_message);

        match logs_data.validate() {
            Err(e) => assert_eq!(e.to_string(), "empty host"),
            Ok(_) => panic!("Validation should failed"),
        }
    }

    #[test]
    fn roundtrip_logs_real_otlp_request_body() {
        use crate::opentelemetry::logs::ExportLogsServiceRequest;

        let out: &[u8] = b"\n\xb6\x07\n\x8a\x06\nR\n\x0ccontainer.id\x12B\n@9d5056147df1c6b11c6fdad3ddf25fa145aca193691aeaaabda7a1a1bd7f0a55\n\x14\n\thost.arch\x12\x07\n\x05amd64\n\x1b\n\thost.name\x12\x0e\n\x0c9d5056147df1\n*\n\x0eos.description\x12\x18\n\x16Linux 5.15.49-linuxkit\n\x12\n\x07os.type\x12\x07\n\x05linux\nh\n\x14process.command_line\x12P\nN/opt/java/openjdk/bin/java -javaagent:/usr/src/app/opentelemetry-javaagent.jar\n7\n\x17process.executable.path\x12\x1c\n\x1a/opt/java/openjdk/bin/java\n\x11\n\x0bprocess.pid\x12\x02\x18\x01\nT\n\x1bprocess.runtime.description\x125\n3Eclipse Adoptium OpenJDK 64-Bit Server VM 17.0.6+10\n5\n\x14process.runtime.name\x12\x1d\n\x1bOpenJDK Runtime Environment\n&\n\x17process.runtime.version\x12\x0b\n\t17.0.6+10\n\x1b\n\x0cservice.name\x12\x0b\n\tadservice\n)\n\x11service.namespace\x12\x14\n\x12opentelemetry-demo\n\"\n\x16telemetry.auto.version\x12\x08\n\x061.23.0\n \n\x16telemetry.sdk.language\x12\x06\n\x04java\n%\n\x12telemetry.sdk.name\x12\x0f\n\ropentelemetry\n!\n\x15telemetry.sdk.version\x12\x08\n\x061.23.1\x12~\n\x14\n\x12oteldemo.AdService\x12f\tQ\xb3\xed\xd9;-W\x17\x10\t\x1a\x04INFO*2\n0received ad request (context_words=[binoculars])E\x01\0\0\0J\x10\xd5]\xf0\x98\xb0\xdc\xe4\x14p\xdf&*\\Z\xdb8R\x08\xb2\x89\t\x8es\xa7\x1f\x9e\x1a'https://opentelemetry.io/schemas/1.18.0";

        ExportLogsServiceRequest::try_from(out)
            .expect("Parsing of logs protobuf failed")
            .validate()
            .expect("Validation of logs protobuf failed");
    }

    #[test]
    fn roundtrip_trace_valid_data() {
        use crate::opentelemetry::trace::{
            AnyValue, AnyValueOneOfvalue, InstrumentationScope, KeyValue, Resource, ResourceSpans,
            ScopeSpans, Span, SpanEvent, SpanKind, SpanLink, Status, StatusCode, ExportTraceServiceRequest,
        };

        let mut out = vec![];

        let key_value = KeyValue {
            key: Cow::from("test"),
            value: Some(AnyValue {
                value: AnyValueOneOfvalue::string_value(Cow::from("test")),
            }),
        };

        let logs_data = ExportTraceServiceRequest {
            resource_spans: vec![ResourceSpans {
                resource: Some(Resource {
                    attributes: vec![key_value.clone()],
                    dropped_attributes_count: 10,
                }),
                scope_spans: vec![ScopeSpans {
                    scope: Some(InstrumentationScope {
                        name: Cow::from("test_name"),
                        version: Cow::from("1.2.3"),
                        attributes: vec![key_value.clone()],
                        dropped_attributes_count: 10,
                    }),
                    spans: vec![Span {
                        trace_id: Cow::from("trace_id".as_bytes()),
                        span_id: Cow::from("span_id".as_bytes()),
                        parent_span_id: Cow::from("parent_span_id".as_bytes()),
                        trace_state: Cow::from("test_state"),
                        name: Cow::from("test_name"),
                        kind: SpanKind::SPAN_KIND_UNSPECIFIED,
                        start_time_unix_nano: 1681339577345243523,
                        end_time_unix_nano: 1681339577345243523,
                        attributes: vec![key_value.clone()],
                        dropped_attributes_count: 10,
                        events: vec![SpanEvent {
                            time_unix_nano: 1681339577345243523,
                            name: Cow::from("test_name"),
                            attributes: vec![key_value.clone()],
                            dropped_attributes_count: 10,
                        }],
                        dropped_events_count: 10,
                        dropped_links_count: 10,
                        links: vec![SpanLink {
                            trace_id: Cow::from("link_trace_id".as_bytes()),
                            span_id: Cow::from("link_span_id".as_bytes()),
                            trace_state: Cow::from("link_test_state"),
                            attributes: vec![key_value.clone()],
                            dropped_attributes_count: 10,
                        }],
                        status: Some(Status {
                            message: Cow::from("test_message"),
                            code: StatusCode::STATUS_CODE_OK,
                        }),
                    }],
                    schema_url: Cow::from("https://some_url.com"),
                }],
                schema_url: Cow::from("https://some_url.com"),
            }],
        };
        {
            let mut writer = Writer::new(&mut out);
            writer.write_message(&logs_data).expect("failed to write");
        }

        let expected: [u8; 335] = [
            205, 2, 10, 202, 2, 10, 18, 10, 14, 10, 4, 116, 101, 115, 116, 18, 6, 10, 4, 116, 101,
            115, 116, 16, 10, 18, 157, 2, 10, 36, 10, 9, 116, 101, 115, 116, 95, 110, 97, 109, 101,
            18, 5, 49, 46, 50, 46, 51, 26, 14, 10, 4, 116, 101, 115, 116, 18, 6, 10, 4, 116, 101,
            115, 116, 32, 10, 18, 222, 1, 10, 8, 116, 114, 97, 99, 101, 95, 105, 100, 18, 7, 115,
            112, 97, 110, 95, 105, 100, 26, 10, 116, 101, 115, 116, 95, 115, 116, 97, 116, 101, 34,
            14, 112, 97, 114, 101, 110, 116, 95, 115, 112, 97, 110, 95, 105, 100, 42, 9, 116, 101,
            115, 116, 95, 110, 97, 109, 101, 57, 131, 249, 119, 254, 111, 81, 85, 23, 65, 131, 249,
            119, 254, 111, 81, 85, 23, 74, 14, 10, 4, 116, 101, 115, 116, 18, 6, 10, 4, 116, 101,
            115, 116, 80, 10, 90, 38, 9, 131, 249, 119, 254, 111, 81, 85, 23, 18, 9, 116, 101, 115,
            116, 95, 110, 97, 109, 101, 26, 14, 10, 4, 116, 101, 115, 116, 18, 6, 10, 4, 116, 101,
            115, 116, 32, 10, 96, 10, 106, 64, 10, 13, 108, 105, 110, 107, 95, 116, 114, 97, 99,
            101, 95, 105, 100, 18, 12, 108, 105, 110, 107, 95, 115, 112, 97, 110, 95, 105, 100, 26,
            15, 108, 105, 110, 107, 95, 116, 101, 115, 116, 95, 115, 116, 97, 116, 101, 34, 14, 10,
            4, 116, 101, 115, 116, 18, 6, 10, 4, 116, 101, 115, 116, 40, 10, 112, 10, 122, 16, 18,
            12, 116, 101, 115, 116, 95, 109, 101, 115, 115, 97, 103, 101, 24, 1, 26, 20, 104, 116,
            116, 112, 115, 58, 47, 47, 115, 111, 109, 101, 95, 117, 114, 108, 46, 99, 111, 109, 26,
            20, 104, 116, 116, 112, 115, 58, 47, 47, 115, 111, 109, 101, 95, 117, 114, 108, 46, 99,
            111, 109,
        ];

        assert_eq!(&out, &expected[..]);

        let read_message = {
            let mut reader = BytesReader::from_bytes(&out);
            reader
                .read_message::<ExportTraceServiceRequest>(&out)
                .expect("Cannot read message")
        };
        assert_eq!(logs_data, read_message);
        logs_data.validate().expect("validation failed");
    }

    #[test]
    fn roundtrip_trace_real_otlp_request_body() {
        use crate::opentelemetry::trace::ExportTraceServiceRequest;

        let out: &[u8] = b"\n\xb3\x0b\n\x83\x03\n \n\x15telemetry.sdk.version\x12\x07\n\x051.2.1\n%\n\x12telemetry.sdk.name\x12\x0f\n\ropentelemetry\n\"\n\x16telemetry.sdk.language\x12\x08\n\x06erlang\n$\n\x0cservice.name\x12\x14\n\x12featureflagservice\n8\n\x13service.instance.id\x12!\n\x1ffeatureflagservice@d69d857131ac\n%\n\x17process.runtime.version\x12\n\n\x0811.2.2.8\n\x1e\n\x14process.runtime.name\x12\x06\n\x04BEAM\n<\n\x1bprocess.runtime.description\x12\x1d\n\x1bErlang/OTP 23 erts-11.2.2.8\n/\n\x17process.executable.name\x12\x14\n\x12featureflagservice\x12\x92\x04\n\x1e\n\x15opentelemetry_phoenix\x12\x051.0.0\x12\xef\x03\n\x10\xc4\xce\xa2\"\x12\nVl\xea\xf63E\0\xab\x01(\x12\x08>@\xb3&\xa3)\x08\x97\"\0*\x01/0\x029x\xc4\xb6\xdc[\xc4\x82\x17A9\x90\xcc\xdc[\xc4\x82\x17J=\n\x0cphoenix.plug\x12-\n+Elixir.FeatureflagserviceWeb.PageControllerJ\x19\n\x0ephoenix.action\x12\x07\n\x05indexJ\x19\n\rnet.transport\x12\x08\n\x06IP.TCPJ\x15\n\rnet.peer.port\x12\x04\x18\xb2\x98\x02J\x1a\n\x0bnet.peer.ip\x12\x0b\n\t127.0.0.1J\x14\n\rnet.host.port\x12\x03\x18\x91?J\x1a\n\x0bnet.host.ip\x12\x0b\n\t127.0.0.1J \n\x0fhttp.user_agent\x12\r\n\x0bcurl/7.74.0J\x12\n\x0bhttp.target\x12\x03\n\x01/J\x17\n\x10http.status_code\x12\x03\x18\xc8\x01J\x15\n\x0bhttp.scheme\x12\x06\n\x04httpJ\x11\n\nhttp.route\x12\x03\n\x01/J\x14\n\x0bhttp.method\x12\x05\n\x03GETJ\x18\n\thttp.host\x12\x0b\n\tlocalhostJ\x14\n\x0bhttp.flavor\x12\x05\n\x031.1J\x1d\n\x0ehttp.client_ip\x12\x0b\n\t127.0.0.1z\0\x12\x95\x04\n\x1b\n\x12opentelemetry_ecto\x12\x051.0.0\x12\xf5\x03\n\x10\xc4\xce\xa2\"\x12\nVl\xea\xf63E\0\xab\x01(\x12\x08u\xe5\x7fF\t\xad\xff\x0e\"\x08>@\xb3&\xa3)\x08\x97**featureflagservice.repo.query:featureflags0\x039\xbf$\xbb\xdc[\xc4\x82\x17AN\xef\xc6\xdc[\xc4\x82\x17J\x1e\n\x17total_time_microseconds\x12\x03\x18\xa2\x04J\x18\n\x06source\x12\x0e\n\x0cfeatureflagsJ\x1d\n\x17queue_time_microseconds\x12\x02\x184J\x1e\n\x17query_time_microseconds\x12\x03\x18\xe9\x03J\x1e\n\x16idle_time_microseconds\x12\x04\x18\xf3\xd5#J\x1e\n\x18decode_time_microseconds\x12\x02\x18\x05J\x1f\n\x06db.url\x12\x15\n\x13ecto://ffs_postgresJ\x10\n\x07db.type\x12\x05\n\x03sqlJ\x88\x01\n\x0cdb.statement\x12x\nvSELECT f0.\"id\", f0.\"description\", f0.\"enabled\", f0.\"name\", f0.\"inserted_at\", f0.\"updated_at\" FROM \"featureflags\" AS f0J\x14\n\x0bdb.instance\x12\x05\n\x03ffsz\0";

        ExportTraceServiceRequest::try_from(out)
            .expect("Parsing of trace protobuf failed")
            .validate()
            .expect("Validation of trace protobuf failed");
    }
}
