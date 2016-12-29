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

    let calendar_min_start_date: &u32 = start_dates.iter().min().unwrap();
    let calendar_max_end_date: &u32 = end_dates.iter().max().unwrap();

    println!("début (calendar): {}", calendar_min_start_date );
    println!("fin (calendar) : {}", calendar_max_end_date );
}
