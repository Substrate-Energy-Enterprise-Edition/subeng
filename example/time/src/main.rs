use chrono::prelude::*;
extern crate chrono;


use chrono::{NaiveDate, NaiveDateTime, DateTime, Utc};

fn main() {
    let date_time: NaiveDateTime = NaiveDate::from_ymd(2017, 11, 12).and_hms(17, 33, 44);

    println!(
        "现在时间 {} 是 {} 秒.",
        date_time, date_time.timestamp());

    let date_time_after_a_billion_seconds = NaiveDateTime::from_timestamp(1_000_000_000, 0);
    println!(
        "自从 1970-01-01 00:00:00起，流逝十亿秒后时间是: {}.",
        date_time_after_a_billion_seconds);

        let now: DateTime<Utc> = Utc::now();

        println!("UTC now is: {}", now);
        println!("UTC now in RFC 2822 is: {}", now.to_rfc2822());
        println!("UTC now in RFC 3339 is: {}", now.to_rfc3339());
        println!("UTC now in a custom format is: {}", now.format("%a %b %e %T %Y"));
    

        //  
       
        let mills: i64 = now.timestamp_millis(); // 1609761696945
        let seconds: i64 = now.timestamp();
        let timestr = seconds.to_string();
        println!("当前毫秒时间 {}- 秒时间 {}", mills, timestr);


        //
        let dt: DateTime<Local> = Local.timestamp_millis(mills);
        // date time parsed millis: 2021-01-04 20:01:36.945 +08:00
        println!("从毫秒算出的当前时间 {} -> {}", mills, dt); 


            // Convert the timestamp string into an i64
        let timestamp = "1524820690".parse::<i64>().unwrap();
        
        
        // Create a NaiveDateTime from the timestamp
        let naive = NaiveDateTime::from_timestamp(timestamp, 0);
        let _naive1 = NaiveDateTime::from_timestamp(mills, 0);

        // Create a normal DateTime from the NaiveDateTime
        let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);

        // Format the datetime how you want
        let newdate = datetime.format("%Y-%m-%d %H:%M:%S");

        // Print the newly formatted date and time
        println!("{}", newdate);

        // fn naive_utc(&self) -> NaiveDateTime
        // Returns a view to the naive UTC datetime.

        //fn naive_local(&self) -> NaiveDateTime
        // Returns a view to the naive local datetime.

        // For instance, if you need the timestamp in UTC:
        let _naive_date_time:NaiveDateTime = Utc::now().naive_utc();


}
