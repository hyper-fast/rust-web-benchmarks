use std::str::FromStr;
use regex::Regex;

#[derive(PartialEq, Debug)]
pub enum MetricsError {
    ParseError
}

pub struct Report {
    framework_name: String,
    max_memory: String,
    metrics: Metrics,
}

const REPORT_HEADER: &str = "| Framework Name | Latency.Avg | Latency.Stdev | Latency.Max | Request.Total | Request.Req/Sec | Transfer.Total | Transfer.Rate | Max. Memory Usage |";
const TABLE_SEPARATOR: &str = "\n|---|---|---|---|---|---|---|---|---|---|\n";

impl Report {
    pub fn new(framework_name: &str,
               max_memory: f64,
               metrics: Metrics) -> Self {
        Self {
            framework_name: framework_name.to_string(),
            metrics,
            max_memory: format!("{:.1}MB", max_memory),
        }
    }

    pub fn generate_from(reports: &Vec<Report>) -> String {
        let mut res = String::new();

        res.push_str(REPORT_HEADER);
        res.push_str(TABLE_SEPARATOR);

        for r in reports {
            let row = format!("|{}|{}|{}|{}|{}|{}|{}|{}|{}|",
                              r.framework_name,
                              r.metrics.latency.avg,
                              r.metrics.latency.std_env,
                              // r.metrics.latency.min,
                              r.metrics.latency.max,
                              r.metrics.request.total,
                              r.metrics.request.req_per_sec,
                              r.metrics.transfer.total,
                              r.metrics.transfer.rate,
                              r.max_memory);
            res.push_str(&row);
            res.push('\n');
        }

        res.pop(); // drop last '\n'

        res
    }
}

#[derive(PartialEq, Debug)]
pub struct Metrics {
    latency: Latency,
    request: Request,
    transfer: Transfer,
}

// parse std output from rewrk result
// something like this:
//      Beginning round 1...
//      Benchmarking 500 connections @ http://127.0.0.1:3000 for 30 second(s)
//        Latencies:
//          Avg      Stdev    Min      Max
//          0.50ms   1.22ms   0.02ms   41.93ms
//        Requests:
//          Total: 30178057 Req/Sec: 1006342.33
//        Transfer:
//          Total: 3.65 GB Transfer Rate: 124.76 MB/Sec
impl FromStr for Metrics {
    type Err = MetricsError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let latency_regex = Regex::new(r"Latency\s+(\d+\.\d+us)\s+(\d+\.\d+us)\s+(\d+\.\d+ms)").unwrap();
        let total_requests_regex = Regex::new(r"(\d+) requests in").unwrap();
        let total_data_read_regex = Regex::new(r", (\d+\.\d+GB) read").unwrap();
        let req_per_sec_regex = Regex::new(r"Requests/sec: (\d+\.\d+)").unwrap();
        let transfer_per_sec_regex = Regex::new(r"Transfer/sec:\s+(\d+\.\d+MB)").unwrap();

        let avg_latency = latency_regex.captures(input).and_then(|cap| cap.get(1)).map(|m| m.as_str().to_string());
        let stddev_latency = latency_regex.captures(input).and_then(|cap| cap.get(2)).map(|m| m.as_str().to_string());
        let max_latency = latency_regex.captures(input).and_then(|cap| cap.get(3)).map(|m| m.as_str().to_string());

        let total_requests = total_requests_regex.captures(input).and_then(|cap| cap.get(1)).map(|m| m.as_str().to_string());
        let req_per_sec = req_per_sec_regex.captures(input).and_then(|cap| cap.get(1)).map(|m| m.as_str().to_string());
        let total_data_read = total_data_read_regex.captures(input).and_then(|cap| cap.get(1)).map(|m| m.as_str().to_string());
        let transfer_per_sec = transfer_per_sec_regex.captures(input).and_then(|cap| cap.get(1)).map(|m| m.as_str().to_string());

        // Constructing structs from the local variables
        let latency = Latency {
            avg: avg_latency.unwrap_or_default(),
            std_env: stddev_latency.unwrap_or_default(),
            max: max_latency.unwrap_or_default(),
        };

        let request = Request {
            total: total_requests.unwrap_or_default(),
            req_per_sec: req_per_sec.unwrap_or_default(),
        };

        let metrics = Metrics {
            latency,
            request,
            transfer: Transfer {
                total: total_data_read.unwrap_or_default(),
                rate: transfer_per_sec.unwrap_or_default(),
            },
        };

        Ok(metrics)
    }
}

#[derive(PartialEq, Debug)]
struct Latency {
    avg: String,
    std_env: String,
    // min: String,
    max: String,
}

// parse for
// 0.50ms   1.22ms   0.02ms   41.93ms
// impl FromStr for Latency {
//     type Err = MetricsError;
//
//     fn from_str(value: &str) -> Result<Self, Self::Err> {
//         let parts = value.trim().split_whitespace()
//             .collect::<Vec<&str>>();
//
//         if let [avg, std_env, min, max, ..] = parts[..] {
//             Ok(Self {
//                 avg: avg.to_string(),
//                 std_env: std_env.to_string(),
//                 min: min.to_string(),
//                 max: max.to_string(),
//             })
//         } else {
//             Err(MetricsError::ParseError)
//         }
//     }
// }

#[derive(PartialEq, Debug)]
struct Request {
    total: String,
    req_per_sec: String,
}

// parse for
// Total: 30178057 Req/Sec: 1006342.33
// impl FromStr for Request {
//     type Err = MetricsError;
//
//     fn from_str(value: &str) -> Result<Self, Self::Err> {
//         let parts = value.trim().split_whitespace()
//             .collect::<Vec<&str>>();
//
//         // ["Total:", total, "Req/Sec:", rps]
//         if let [_, total, _, req_per_sec, ..] = parts[..] {
//             Ok(Self {
//                 total: total.to_string(),
//                 req_per_sec: req_per_sec.to_string(),
//             })
//         } else {
//             Err(MetricsError::ParseError)
//         }
//     }
// }

#[derive(PartialEq, Debug)]
struct Transfer {
    total: String,
    rate: String,
}

// parse for
// Total: 3.65 GB Transfer Rate: 124.76 MB/Sec
// impl FromStr for Transfer {
//     type Err = MetricsError;
//
//     fn from_str(value: &str) -> Result<Self, Self::Err> {
//         let parts = value.trim().split_whitespace()
//             .collect::<Vec<&str>>();
//
//         // ["Total:", total, total_unit, "Transfer", "Rate:", rate, rate_unit]
//         if let [_, total, total_unit, _, _, rate, rate_unit, ..] = parts[..] {
//             Ok(Self {
//                 total: format!("{}{}", total, total_unit),
//                 rate: format!("{}{}", rate, rate_unit),
//             })
//         } else {
//             Err(MetricsError::ParseError)
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    mod report {
        use super::*;

        #[test]
        fn generate() {
            let given = vec![
                Report::new("actix-web", 13.7, r#"
                    Running 30s test @ http://127.0.0.1:3000
                      16 threads and 500 connections
                      Thread Stats   Avg      Stdev     Max   +/- Stdev
                        Latency   814.27us  498.47us   8.42ms   69.23%
                        Req/Sec    36.10k     2.64k   74.83k    75.41%
                      17275966 requests in 30.09s, 1.95GB read
                    Requests/sec: 574184.09
                    Transfer/sec:     66.26MB
                "#.parse().expect("parse metric fail")),
                Report::new("axum", 12.4, r#"
                    Running 30s test @ http://127.0.0.1:3000
                      16 threads and 200 connections
                      Thread Stats   Avg      Stdev     Max   +/- Stdev
                        Latency   392.28us  199.70us   4.67ms   70.95%
                        Req/Sec    29.50k     0.98k   33.01k    68.63%
                      14134927 requests in 30.10s, 1.59GB read
                    Requests/sec: 469597.42
                    Transfer/sec:     54.19MB
                "#.parse().expect("parse metric fail")),
            ];

            let actual = Report::generate_from(&given);

            let expect = r#"
| Framework Name | Latency.Avg | Latency.Stdev | Latency.Max | Request.Total | Request.Req/Sec | Transfer.Total | Transfer.Rate | Max. Memory Usage |
|---|---|---|---|---|---|---|---|---|---|
|actix-web|814.27us|498.47us|8.42ms|17275966|574184.09|1.95GB|66.26MB|13.7MB|
|axum|392.28us|199.70us|4.67ms|14134927|469597.42|1.59GB|54.19MB|12.4MB|
"#.trim();

            assert_eq!(actual, expect);
        }
    }

    mod metrics {
        use super::*;

        #[test]
        fn ok() {
            let given = r#"
Running 30s test @ http://127.0.0.1:3000
  16 threads and 500 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency   814.27us  498.47us   8.42ms   69.23%
    Req/Sec    36.10k     2.64k   74.83k    75.41%
  17275966 requests in 30.09s, 1.95GB read
Requests/sec: 574184.09
Transfer/sec:     66.26MB

691 Errors: error shutting down connection: Socket is not connected (os error 57)
            "#;
            let actual = given.parse::<Metrics>();

            let expect = Ok(
                Metrics {
                    latency: Latency {
                        avg: "814.27us".to_string(),
                        std_env: "498.47us".to_string(),
                        // min: "0.02ms".to_string(),
                        max: "8.42ms".to_string(),
                    },
                    request: Request {
                        total: "17275966".to_string(),
                        req_per_sec: "574184.09".to_string(),
                    },
                    transfer: Transfer {
                        total: "1.95GB".to_string(),
                        rate: "66.26MB".to_string(),
                    },
                });

            assert_eq!(actual, expect);
        }
    }

    // mod latency {
    //     use super::*;
    //
    //     #[test]
    //     fn ok() {
    //         let given = "814.27us  498.47us   8.42ms   69.23%";
    //         let actual = given.parse::<Latency>();
    //
    //         let expect = Ok(Latency {
    //             avg: "814.27us".to_string(),
    //             std_env: "498.47us".to_string(),
    //             // min: "0.02ms".to_string(),
    //             max: "8.42ms".to_string(),
    //         });
    //
    //         assert_eq!(actual, expect);
    //     }
    //
    //     #[test]
    //     fn err() {
    //         let given = "0.50ms   1.22ms   0.02ms";
    //         let actual = given.parse::<Latency>();
    //         let expect = Err(MetricsError::ParseError);
    //
    //         assert_eq!(actual, expect);
    //     }
    // }
    //
    // mod request {
    //     use super::*;
    //
    //     #[test]
    //     fn ok() {
    //         let given = "Total: 30178057 Req/Sec: 1006342.33";
    //         let actual = given.parse::<Request>();
    //
    //         let expect = Ok(Request {
    //             total: "30178057".to_string(),
    //             req_per_sec: "1006342.33".to_string(),
    //         });
    //
    //         assert_eq!(actual, expect);
    //     }
    //
    //     #[test]
    //     fn err() {
    //         let given = "Total: 30178057";
    //         let actual = given.parse::<Request>();
    //         let expect = Err(MetricsError::ParseError);
    //
    //         assert_eq!(actual, expect);
    //     }
    // }
    //
    // mod transfer {
    //     use super::*;
    //
    //     #[test]
    //     fn ok() {
    //         let given = "Total: 3.65 GB Transfer Rate: 124.76 MB/Sec";
    //         let actual = given.parse::<Transfer>();
    //
    //         let expect = Ok(Transfer {
    //             total: "3.65GB".to_string(),
    //             rate: "124.76MB/Sec".to_string(),
    //         });
    //
    //         assert_eq!(actual, expect);
    //     }
    //
    //     #[test]
    //     fn err() {
    //         let given = "Total: 3.65 GB";
    //         let actual = given.parse::<Request>();
    //         let expect = Err(MetricsError::ParseError);
    //
    //         assert_eq!(actual, expect);
    //     }
    // }
}
