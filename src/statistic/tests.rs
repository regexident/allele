use galvanic_assert::matchers::*;

mod timed_fn {

    use super::*;
    use crate::statistic::timed;
    use chrono;
    use std::{thread, time::Duration};

    #[test]
    fn timed_function_calls_return_a_time_greater_0() {
        let result = timed(|| {
            thread::sleep(Duration::from_millis(141));
        })
        .run();

        expect_that!(
            &result.time.duration(),
            greater_than_or_equal(chrono::Duration::milliseconds(141))
        );
    }

    #[test]
    fn timed_function_calls_measure_time_in_nanoseconds() {
        let result = timed(|| {
            thread::sleep(Duration::from_nanos(141));
        })
        .run();

        expect_that!(
            &result.time.duration(),
            greater_than_or_equal(chrono::Duration::nanoseconds(141))
        );
    }
}

mod processing_time_display {

    use crate::statistic::ProcessingTime;
    use chrono;

    #[test]
    fn zero_is_formatted_as_0s() {
        assert_eq!(ProcessingTime::zero().to_string(), "0s");
    }

    #[test]
    fn sub_second_durations_are_formatted_human_readably() {
        let time = ProcessingTime::from(chrono::Duration::milliseconds(1_001));
        assert_eq!(time.to_string(), "1s 1ms");
    }

    #[test]
    fn minute_and_hour_durations_are_formatted_human_readably() {
        assert_eq!(
            ProcessingTime::from(chrono::Duration::seconds(61)).to_string(),
            "1m 1s"
        );
        assert_eq!(
            ProcessingTime::from(chrono::Duration::seconds(3_601)).to_string(),
            "1h 1s"
        );
    }

    #[test]
    fn multi_day_durations_are_formatted_human_readably() {
        assert_eq!(
            ProcessingTime::from(chrono::Duration::days(7)).to_string(),
            "7days"
        );
    }
}
