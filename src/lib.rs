extern crate rustc_serialize;
extern crate csv;

#[derive(RustcDecodable, Debug)]
struct Calendar {
    service_id: String,
    monday: u32,
    tuesday: u32,
    wednesday: u32,
    thursday: u32,
    friday: u32,
    saturday: u32,
    sunday: u32,
    start_date: u32,
    end_date: u32,
}

#[derive(RustcDecodable, Debug)]
struct CalendarDate {
    service_id: String,
    date: u32,
    exception_type: u32,
}

pub fn get_period_from_txt_files() -> (u32, u32) {
    let mut validity_start_date: u32 = u32::max_value();
    let mut validity_end_date: u32 = u32::min_value();

    let path = "calendar.txt";
    let cal = csv::Reader::from_file(path);
    if let Ok(cal) = cal {
        let mut reader = cal.has_headers(true);
        for row in reader.decode() {
            let a_calendar: Calendar = row.unwrap();
            validity_start_date = std::cmp::min(validity_start_date, a_calendar.start_date);
            validity_end_date = std::cmp::max(validity_end_date, a_calendar.end_date);
        }
    }

    let path = "calendar_dates.txt";
    let cal = csv::Reader::from_file(path);
    if let Ok(cal) = cal {
        let mut reader = cal.has_headers(true);
        for row in reader.decode() {
            let a_calendar_date: CalendarDate = row.unwrap();

            //like Google FeedValidator, we enlarge the validity period with exception dates added
            // but we do not shrink it with exception dates removed.
            // https://github.com/google/transitfeed/blob/master/transitfeed/serviceperiod.py#L80
            if a_calendar_date.exception_type == 1 {
                validity_start_date = std::cmp::min(validity_start_date, a_calendar_date.date);
                validity_end_date = std::cmp::max(validity_end_date, a_calendar_date.date);
            }
        }
    }

    (validity_start_date, validity_end_date)
}
