extern crate extract_validity_period;


fn main() {
    let result = extract_validity_period::get_period_from_txt_files();
    println!("{:?}", result);
}
