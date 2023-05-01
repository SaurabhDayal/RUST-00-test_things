use chrono::prelude::*;

#[derive(Debug)]
struct Person{
    name: String,
    date: NaiveDate<>,
}

fn main(){
    
let dt = Utc.with_ymd_and_hms(2014, 7, 8, 9, 10, 11).unwrap(); 
println!();
println!("{dt}");

let p1 = Person {
    name: String::from("ABC"),
    date: NaiveDate::from_ymd_opt(2023, 04, 11).unwrap(),
};
println!();
println!("{:?}",p1);
println!("{:?}",p1.name);
println!("{:?}",p1.date);

}