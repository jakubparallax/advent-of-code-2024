use std::fs;

fn is_safe(mut report: Vec<i32>, used: bool) -> bool {
    let is_ascending = report[0] < report[report.len() - 1];
    let mut prev = report[0];

    let report_iter = report.clone().into_iter().skip(1);
    for (index, level) in report_iter.enumerate() {
        let diff = level - prev;

        if is_ascending {
            if diff > 0 && diff < 4 {
                prev = level;
                continue;
            }
        } else {
            if diff < 0 && diff > -4 {
                prev = level;
                continue;
            }
        }
        if used {
            return false;
        }
        
        let mut report_clone = report.clone();
        report_clone.remove(index);
        report.remove(index + 1);
        return is_safe(report_clone, true) || is_safe(report, true);
    }
    true
}

fn count_safe_reports(input: String, with_dampener: bool) -> i32 {
    let lines = input.split('\n');

    let mut safe_report_count = 0;
    for line in lines.into_iter() {
        let levels: Vec<i32> = line.split(" ").map(|level_str| level_str.parse::<i32>().unwrap()).collect();

        if is_safe(levels, with_dampener) {
            safe_report_count += 1;
        }
    }
    safe_report_count
}

pub fn solve() {
    let input: String = fs::read_to_string("inputs/two").unwrap();

    let safe_report_count = count_safe_reports(input.clone(), true);
    println!("Safe reports: {:?}", safe_report_count);

    let safe_report_count = count_safe_reports(input, false);
    println!("Safe reports with dampener: {:?}", safe_report_count);
}
