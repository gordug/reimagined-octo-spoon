fn main() {
    todo!("Implement the main function");
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
struct RateReport {
    pub cage_id: u64,
    pub start_time: String,
    pub end_time: String,
    pub rate: u64
}

#[derive(Debug, Clone, PartialEq)]
struct RateReportView{
    pub start_time: String,
    pub end_time: String,
    pub rate: u64
}

fn collate_rate_reports(reports: Vec<RateReport>) -> Vec<RateReportView> {
    if reports.len() == 0 {
        return Vec::new();
    }
    let mut output:Vec<RateReportView> = Vec::new();
    let mut reports = reports.clone();
    // sort by starttime then endtime
    reports.sort_unstable_by_key(|k| (k.start_time.clone(), k.end_time.clone()));
    let mut current_report = reports[0].clone();
    reports.remove(0);
    while reports.len() > 0 {
        // If the next report starts at the same time as the current report
        if reports[0].start_time == current_report.start_time {
            // if the next report ends before the current report ends
            if reports[0].end_time < current_report.end_time {
                output.push(RateReportView {
                    start_time: current_report.start_time.clone(),
                    end_time: reports[0].end_time.clone(),
                    rate: current_report.rate + reports[0].rate
                });
                current_report.start_time = reports[0].end_time.clone();
                reports.remove(0);
            }
            // if the next report ends at the same time as the current report ends
            else if reports[0].end_time == current_report.end_time {
                output.push(RateReportView {
                    start_time: current_report.start_time.clone(),
                    end_time: current_report.end_time.clone(),
                    rate: current_report.rate + reports[0].rate
                });
                reports.remove(0);
            }
            // if the next report ends after the current report ends
            else {
                output.push(RateReportView {
                    start_time: current_report.start_time.clone(),
                    end_time: current_report.end_time.clone(),
                    rate: current_report.rate + reports[0].rate
                });
                current_report.start_time = current_report.end_time.clone();
                current_report.end_time = reports[0].end_time.clone();
                current_report.rate = reports[0].rate;
                reports[0].start_time = current_report.end_time.clone();
            }
            if reports.len() == 1 {
                output.push(RateReportView {
                    start_time: current_report.start_time.clone(),
                    end_time: current_report.end_time.clone(),
                    rate: current_report.rate
                });
                reports.remove(0);
            }
            continue;
        }

        // if the next report starts before the current report ends
        if reports[0].start_time < current_report.end_time {
            output.push(RateReportView {
                start_time: current_report.start_time.clone(),
                end_time: reports[0].start_time.clone(),
                rate: current_report.rate
            });
            current_report.start_time = reports[0].start_time.clone();
            continue;
        }

        // if the next report starts at the same time as the current report ends
        if reports[0].start_time == current_report.end_time {
            output.push(RateReportView {
                start_time: current_report.start_time.clone(),
                end_time: current_report.end_time.clone(),
                rate: current_report.rate
            });
            current_report.start_time = current_report.end_time.clone();
            continue;
        }

        // if the next report starts after the current report ends
        if reports[0].start_time > current_report.end_time {
            output.push(RateReportView {
                start_time: current_report.start_time.clone(),
                end_time: current_report.end_time.clone(),
                rate: current_report.rate
            });
            current_report = reports[0].clone();
            reports.remove(0);
            continue;
        }
    }
    output
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn collate_rate_reports_returns_corect_report_1() {
        let mut input:Vec<RateReport> = Vec::new();
        input.push(RateReport {
            cage_id: 1,
            start_time: "2020-01-01T03:00:00Z".to_string(),
            end_time: "2020-01-01T06:00:00Z".to_string(),
            rate: 4
        });
        input.push(RateReport {
            cage_id: 1,
            start_time: "2020-01-01T16:00:00Z".to_string(),
            end_time: "2020-01-01T20:00:00Z".to_string(),
            rate: 5
        });
        input.push(RateReport {
            cage_id: 2,
            start_time: "2020-01-01T04:00:00Z".to_string(),
            end_time: "2020-01-01T05:00:00Z".to_string(),
            rate: 7
        });
        input.push(RateReport {
            cage_id: 2,
            start_time: "2020-01-01T16:00:00Z".to_string(),
            end_time: "2020-01-01T22:00:00Z".to_string(),
            rate: 3
        });
        let mut expected:Vec<RateReportView> = Vec::new();
        expected.push(RateReportView {
            start_time: "2020-01-01T03:00:00Z".to_string(),
            end_time: "2020-01-01T04:00:00Z".to_string(),
            rate: 4
        });
        expected.push(RateReportView {
            start_time: "2020-01-01T04:00:00Z".to_string(),
            end_time: "2020-01-01T05:00:00Z".to_string(),
            rate: 11
        });
        expected.push(RateReportView {
            start_time: "2020-01-01T05:00:00Z".to_string(),
            end_time: "2020-01-01T06:00:00Z".to_string(),
            rate: 4
        });
        expected.push(RateReportView {
            start_time: "2020-01-01T16:00:00Z".to_string(),
            end_time: "2020-01-01T20:00:00Z".to_string(),
            rate: 8
        });
        expected.push(RateReportView {
            start_time: "2020-01-01T20:00:00Z".to_string(),
            end_time: "2020-01-01T22:00:00Z".to_string(),
            rate: 3
        });

        let output = collate_rate_reports(input);
        assert_eq!(output.len(), expected.len());
        assert_eq!(output, expected);
    }
}
