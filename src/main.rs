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

fn main() {
    let path = "calendar.txt";
    let mut reader =  csv::Reader::from_file(path).unwrap().has_headers(true);

    let mut start_dates: Vec<u32> = vec![402112300];// to prevent the use of possibly uninitialized `start_dates`
    let mut end_dates: Vec<u32> = vec![0];

    for row in reader.decode() {
        let a_calendar: Calendar = row.unwrap();
        start_dates.push(a_calendar.start_date);
        end_dates.push(a_calendar.end_date);
    }

    let path = "calendar_dates.txt";
    let mut reader =  csv::Reader::from_file(path).unwrap().has_headers(true);

    for row in reader.decode() {
        let a_calendar_date: CalendarDate = row.unwrap();

        //just like Google FeedValidator, we enlarge the validity period with exception dates added
        // but we do not shrink it with exception dates removed.
        // https://github.com/google/transitfeed/blob/master/transitfeed/serviceperiod.py#L80
        if a_calendar_date.exception_type == 1 {
            end_dates.push(a_calendar_date.date);
        }

    }
    //TODO : calendar.txt is required, but can be empty : this use case is not handled for now

    let calendar_min_start_date: &u32 = start_dates.iter().min().unwrap();
    let calendar_max_end_date: &u32 = end_dates.iter().max().unwrap();

    println!("start : {}", calendar_min_start_date );
    println!("end : {}", calendar_max_end_date );
}
