// 日期时间处理（使用 chrono 库）

use chrono::{DateTime, Utc, Local, NaiveDate, NaiveTime, NaiveDateTime, Duration, Datelike, Timelike};

/// # 获取当前时间
pub fn current_time_demo() {
    println!("\n=== 获取当前时间 ===");
    
    // UTC 时间
    let utc_now: DateTime<Utc> = Utc::now();
    println!("UTC 时间: {}", utc_now);
    println!("格式化: {}", utc_now.format("%Y-%m-%d %H:%M:%S"));
    
    // 本地时间
    let local_now: DateTime<Local> = Local::now();
    println!("\n本地时间: {}", local_now);
    println!("格式化: {}", local_now.format("%Y-%m-%d %H:%M:%S"));
    
    // 时间戳
    let timestamp = utc_now.timestamp();
    let timestamp_millis = utc_now.timestamp_millis();
    println!("\n时间戳（秒）: {}", timestamp);
    println!("时间戳（毫秒）: {}", timestamp_millis);
}

/// # 创建日期时间
pub fn create_datetime_demo() {
    println!("\n=== 创建日期时间 ===");
    
    // 从年月日创建日期
    let date = NaiveDate::from_ymd_opt(2024, 12, 25).unwrap();
    println!("日期: {}", date);
    
    // 从时分秒创建时间
    let time = NaiveTime::from_hms_opt(14, 30, 0).unwrap();
    println!("时间: {}", time);
    
    // 组合日期和时间
    let datetime = NaiveDateTime::new(date, time);
    println!("日期时间: {}", datetime);
    
    // 从字符串解析
    let parsed = NaiveDate::parse_from_str("2024-12-25", "%Y-%m-%d");
    match parsed {
        Ok(date) => println!("\n解析日期: {}", date),
        Err(e) => println!("解析错误: {}", e),
    }
    
    let parsed = NaiveDateTime::parse_from_str(
        "2024-12-25 14:30:00",
        "%Y-%m-%d %H:%M:%S"
    );
    match parsed {
        Ok(dt) => println!("解析日期时间: {}", dt),
        Err(e) => println!("解析错误: {}", e),
    }
    
    // 从时间戳创建
    let timestamp = 1703505600; // 2024-12-25 12:00:00 UTC
    if let Some(dt) = DateTime::from_timestamp(timestamp, 0) {
        println!("\n从时间戳创建: {}", dt);
    }
}

/// # 日期时间格式化
pub fn formatting_demo() {
    println!("\n=== 日期时间格式化 ===");
    
    let now = Local::now();
    
    println!("默认: {}", now);
    println!("RFC 2822: {}", now.to_rfc2822());
    println!("RFC 3339: {}", now.to_rfc3339());
    
    println!("\n自定义格式:");
    println!("YYYY-MM-DD: {}", now.format("%Y-%m-%d"));
    println!("MM/DD/YYYY: {}", now.format("%m/%d/%Y"));
    println!("DD.MM.YYYY: {}", now.format("%d.%m.%Y"));
    
    println!("\n时间格式:");
    println!("HH:MM:SS: {}", now.format("%H:%M:%S"));
    println!("HH:MM: {}", now.format("%H:%M"));
    println!("12小时制: {}", now.format("%I:%M:%S %p"));
    
    println!("\n完整格式:");
    println!("完整: {}", now.format("%Y-%m-%d %H:%M:%S %Z"));
    println!("带星期: {}", now.format("%A, %B %d, %Y"));
    println!("中文: {}", now.format("%Y年%m月%d日 %H时%M分%S秒"));
}

/// # 日期时间运算
pub fn arithmetic_demo() {
    println!("\n=== 日期时间运算 ===");
    
    let now = Utc::now();
    println!("当前时间: {}", now.format("%Y-%m-%d %H:%M:%S"));
    
    // 加减时间
    let one_hour_later = now + Duration::hours(1);
    let one_day_ago = now - Duration::days(1);
    let one_week_later = now + Duration::weeks(1);
    
    println!("\n1小时后: {}", one_hour_later.format("%Y-%m-%d %H:%M:%S"));
    println!("1天前: {}", one_day_ago.format("%Y-%m-%d %H:%M:%S"));
    println!("1周后: {}", one_week_later.format("%Y-%m-%d %H:%M:%S"));
    
    // Duration 创建
    let duration = Duration::seconds(90);
    println!("\n90秒 = {} 分钟", duration.num_minutes());
    
    let duration = Duration::days(7);
    println!("7天 = {} 小时", duration.num_hours());
    println!("7天 = {} 周", duration.num_weeks());
    
    // 时间差
    let date1 = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
    let date2 = NaiveDate::from_ymd_opt(2024, 12, 31).unwrap();
    let diff = date2.signed_duration_since(date1);
    
    println!("\n2024-01-01 到 2024-12-31:");
    println!("相差 {} 天", diff.num_days());
}

/// # 日期时间比较
pub fn comparison_demo() {
    println!("\n=== 日期时间比较 ===");
    
    let date1 = NaiveDate::from_ymd_opt(2024, 12, 25).unwrap();
    let date2 = NaiveDate::from_ymd_opt(2024, 12, 31).unwrap();
    
    println!("date1: {}", date1);
    println!("date2: {}", date2);
    
    println!("\ndate1 == date2: {}", date1 == date2);
    println!("date1 < date2: {}", date1 < date2);
    println!("date1 > date2: {}", date1 > date2);
    
    // 判断是否在范围内
    let now = NaiveDate::from_ymd_opt(2024, 12, 28).unwrap();
    let in_range = now >= date1 && now <= date2;
    println!("\n{} 在 {} 到 {} 之间: {}", now, date1, date2, in_range);
}

/// # 获取日期时间组件
pub fn components_demo() {
    println!("\n=== 获取日期时间组件 ===");
    
    let now = Local::now();
    
    println!("年: {}", now.year());
    println!("月: {}", now.month());
    println!("日: {}", now.day());
    println!("星期: {}", now.weekday());
    println!("年内第几天: {}", now.ordinal());
    
    println!("\n时间:");
    println!("时: {}", now.hour());
    println!("分: {}", now.minute());
    println!("秒: {}", now.second());
    println!("纳秒: {}", now.nanosecond());
    
    // 星期判断
    println!("\n是否工作日:");
    use chrono::Weekday;
    match now.weekday() {
        Weekday::Mon | Weekday::Tue | Weekday::Wed | Weekday::Thu | Weekday::Fri => {
            println!("工作日");
        }
        Weekday::Sat | Weekday::Sun => {
            println!("周末");
        }
    }
}

/// # 时区转换
pub fn timezone_demo() {
    println!("\n=== 时区转换 ===");
    
    let utc_time = Utc::now();
    println!("UTC 时间: {}", utc_time.format("%Y-%m-%d %H:%M:%S"));
    
    let local_time = utc_time.with_timezone(&Local);
    println!("本地时间: {}", local_time.format("%Y-%m-%d %H:%M:%S"));
    
    // 时区偏移
    let offset = local_time.offset();
    println!("时区偏移: {}", offset);
}

/// # 实战示例：日期范围
pub fn date_range_demo() {
    println!("\n=== 实战示例：日期范围 ===");
    
    let start = NaiveDate::from_ymd_opt(2024, 12, 1).unwrap();
    let end = NaiveDate::from_ymd_opt(2024, 12, 7).unwrap();
    
    println!("日期范围: {} 到 {}", start, end);
    println!("\n所有日期:");
    
    let mut current = start;
    while current <= end {
        println!("  {} ({})", current, current.format("%A"));
        current = current + Duration::days(1);
    }
}

/// # 实战示例：年龄计算
pub fn age_calculator_demo() {
    println!("\n=== 实战示例：年龄计算 ===");
    
    let birthdate = NaiveDate::from_ymd_opt(1990, 5, 15).unwrap();
    let today = Local::now().date_naive();
    
    let age = today.years_since(birthdate);
    
    println!("出生日期: {}", birthdate);
    println!("今天: {}", today);
    
    if let Some(age_years) = age {
        println!("年龄: {} 岁", age_years);
    }
    
    // 精确计算天数
    let duration = today.signed_duration_since(birthdate);
    println!("出生至今: {} 天", duration.num_days());
}

/// # 实战示例：倒计时
pub fn countdown_demo() {
    println!("\n=== 实战示例：倒计时 ===");
    
    let target = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
    let today = Local::now().date_naive();
    
    let duration = target.signed_duration_since(today);
    let days = duration.num_days();
    
    println!("目标日期: {}", target);
    println!("今天: {}", today);
    
    if days > 0 {
        println!("距离目标还有 {} 天", days);
    } else if days < 0 {
        println!("目标日期已过去 {} 天", -days);
    } else {
        println!("就是今天！");
    }
}

/// # 实战示例：工作日计算
pub fn workday_demo() {
    println!("\n=== 实战示例：工作日计算 ===");
    
    let start = NaiveDate::from_ymd_opt(2024, 12, 1).unwrap();
    let end = NaiveDate::from_ymd_opt(2024, 12, 31).unwrap();
    
    let mut workdays = 0;
    let mut weekends = 0;
    let mut current = start;
    
    while current <= end {
        use chrono::Weekday;
        match current.weekday() {
            Weekday::Sat | Weekday::Sun => weekends += 1,
            _ => workdays += 1,
        }
        current = current + Duration::days(1);
    }
    
    println!("日期范围: {} 到 {}", start, end);
    println!("工作日: {} 天", workdays);
    println!("周末: {} 天", weekends);
    println!("总共: {} 天", workdays + weekends);
}

/// 运行所有日期时间示例
pub fn run_all() {
    println!("\n╔════════════════════════════════════╗");
    println!("║      Rust 日期时间处理详解         ║");
    println!("╚════════════════════════════════════╝");
    
    current_time_demo();
    create_datetime_demo();
    formatting_demo();
    arithmetic_demo();
    comparison_demo();
    components_demo();
    timezone_demo();
    date_range_demo();
    age_calculator_demo();
    countdown_demo();
    workday_demo();
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_date_creation() {
        let date = NaiveDate::from_ymd_opt(2024, 12, 25);
        assert!(date.is_some());
    }
    
    #[test]
    fn test_date_arithmetic() {
        let date = NaiveDate::from_ymd_opt(2024, 12, 25).unwrap();
        let next_day = date + Duration::days(1);
        assert_eq!(next_day.day(), 26);
    }
    
    #[test]
    fn test_date_comparison() {
        let date1 = NaiveDate::from_ymd_opt(2024, 12, 25).unwrap();
        let date2 = NaiveDate::from_ymd_opt(2024, 12, 26).unwrap();
        assert!(date1 < date2);
    }
}
