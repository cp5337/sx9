
use usim_core::usimify_record;

fn main() {
    let record = "John Doe,1234 Main St,01-01-1970";
    let usim = usimify_record(record.to_string());
    println!("{}", serde_json::to_string_pretty(&usim).unwrap());
}
