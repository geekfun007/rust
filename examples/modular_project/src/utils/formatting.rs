// 格式化工具函数

pub fn format_date(year: u32, month: u32, day: u32) -> String {
    format!("{:04}-{:02}-{:02}", year, month, day)
}

pub fn format_time(hour: u32, minute: u32, second: u32) -> String {
    format!("{:02}:{:02}:{:02}", hour, minute, second)
}
