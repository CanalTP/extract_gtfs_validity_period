extern crate extract_validity_period;

fn test_wrapper(path_to_txt:&str, expected_result:(u32,u32)) {
    let init_path = std::env::current_dir().unwrap();
    let test_path = std::path::Path::new(path_to_txt);
    assert!(std::env::set_current_dir(&test_path).is_ok());

    let result = extract_validity_period::get_period_from_txt_files();
    assert_eq!(result, expected_result);

    assert!(std::env::set_current_dir(&init_path).is_ok());
}

#[test]
fn test_full_gtfs_case() {
    test_wrapper("tests/fixtures/full_gtfs", (20140801, 20150801))
}

#[test]
fn test_nominal_case() {
    test_wrapper("tests/fixtures/nominal", (20161125, 20171124))
}

#[test]
fn test_empty_calendar() {
    test_wrapper("tests/fixtures/empty_calendar", (20160504, 20160930))
}

#[test]
fn test_empty_calendar_dates() {
    test_wrapper("tests/fixtures/empty_calendar_dates", (20161107, 20170205))
}

#[test]
#[ignore]
fn test_just_calendar_dates() {
    test_wrapper("tests/fixtures/no_calendar", (20161125, 20171124))
}

#[test]
#[ignore]
fn test_just_calendar() {
    test_wrapper("tests/fixtures/no_calendar_dates", (20161125, 20171124))
}
