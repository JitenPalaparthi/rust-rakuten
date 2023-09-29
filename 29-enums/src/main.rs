fn main() { 
    let today:Day = Day::Friday;
    today_is(today);
    today_is(Day::Saturday);

    let s1:Schedule = Schedule { day: Day::Thursday, time: 12.45 };
    println!("{:?}",s1)
}


fn today_is(day:Day){
    match day{
      Day::Sunday=>println!("Today is Sunday.. Happy sunday"),
      Day::Monday=>println!("Today is Monday.. Busy Monday"),
      Day::Tuesday=>println!("Today is Tuesday.. Relaxed Tuesday"),
      Day::Wednesday=>println!("Today is Wednesday.. Midweek Wednesday"),
      Day::Thursday=>println!("Today is Thursday.. Thirsty Thursday"),
      Day::Friday=>println!("Today is Friday.. Funnay Friday"),
      Day::Saturday=>println!("Today is Saturday.. Sleepy Saturday"),
      _ =>(), // default:
    }
}

#[derive(Debug)]
enum Day{
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}


#[derive(Debug)]
struct Schedule{
    day:Day, // enum
    time:f32, // normal primitive field
}

