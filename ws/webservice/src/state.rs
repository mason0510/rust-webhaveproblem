use super::models::Course;
use std::sync::Mutex;

/// 应用程序当前的状态
pub struct AppState {
    pub health_check_response: String,
    pub visit_count: Mutex<u32>,
    pub courses: Mutex<Vec<Course>>,
}
