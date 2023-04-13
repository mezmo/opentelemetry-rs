use quick_protobuf::MessageRead;

mod opentelemetry_types;
mod validation;
mod opentelemetry_internal {
    pub use crate::opentelemetry_types::opentelemetry::proto::metrics::v1::*;
    pub use crate::validation::common::CommonValidate;
    pub use crate::validation::metrics::MetricValidate;
}

pub mod opentelemetry {
    pub mod common {
        pub use crate::opentelemetry_internal::CommonValidate as Validate;
        pub use crate::opentelemetry_internal::{
            mod_AnyValue::OneOfvalue as AnyValueOneOfvalue, AnyValue, ArrayValue,
            InstrumentationScope, KeyValue, Resource,
        };
    }

    pub mod metrics {
        pub use crate::opentelemetry_internal::MetricValidate as Validate;
        pub use crate::opentelemetry_internal::{
            mod_AnyValue::OneOfvalue as AnyValueOneOfvalue,
            mod_Exemplar::OneOfvalue as ExemplarOneOfvalue,
            mod_ExponentialHistogramDataPoint::Buckets as ExponentialHistogramDataPointBuckets,
            mod_Metric::OneOfdata as MetricOneOfdata,
            mod_NumberDataPoint::OneOfvalue as NumberDataPointOneOfvalue,
            mod_SummaryDataPoint::ValueAtQuantile as SummaryDataPointValueAtQuantile,
            AggregationTemporality, AnyValue, ArrayValue, Exemplar, ExponentialHistogram,
            ExponentialHistogramDataPoint, ExportMetricsServiceRequest, Gauge, Histogram,
            HistogramDataPoint, InstrumentationScope, KeyValue, Metric, NumberDataPoint, Resource,
            ResourceMetrics, ScopeMetrics, Sum, Summary, SummaryDataPoint,
        };
    }

    pub mod logs {}
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

#[cfg(test)]
mod tests {
    use crate::validation::common::CommonValidate;
    use crate::validation::metrics::MetricValidate;
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
}
