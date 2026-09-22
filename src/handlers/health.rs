use crate::config::{Config, Environment};
use crate::lib::response::ApiResponse;
use axum::{Json, extract::State};
use serde::Serialize;
use sysinfo::{ProcessesToUpdate, System, get_current_pid};

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub message: String,
    pub environment: Environment,
}

#[derive(Serialize)]
pub struct DebugHealthResponse {
    pub status: String,
    pub message: String,
    pub environment: Environment,

    pub process: ProcessHealth,
    pub system: SystemHealth,
}

#[derive(Serialize)]
pub struct ProcessHealth {
    pub pid: u32,
    pub cpu_usage_percent: f32,
    pub memory_bytes: u64,
    pub virtual_memory_bytes: u64,
    pub uptime_seconds: u64,
    pub start_time: u64,
    pub status: String,
}

#[derive(Serialize)]
pub struct SystemHealth {
    pub cpu_usage_percent: f32,
    pub cpu_count: usize,

    pub total_memory_bytes: u64,
    pub used_memory_bytes: u64,
    pub available_memory_bytes: u64,

    pub total_swap_bytes: u64,
    pub used_swap_bytes: u64,

    pub load_average_1m: f64,
    pub load_average_5m: f64,
    pub load_average_15m: f64,

    pub uptime_seconds: u64,
}

pub async fn health(State(config): State<Config>) -> Json<ApiResponse<serde_json::Value>> {
    println!("Health Requested");

    if config.environment != Environment::Production {
        let mut system = System::new_all();

        system.refresh_memory();
        system.refresh_cpu_all();

        let pid = get_current_pid().expect("failed to get current PID");

        system.refresh_processes(ProcessesToUpdate::Some(&[pid]), true);

        let process = system.process(pid).expect("current process not found");

        let load = System::load_average();

        let data: serde_json::Value = serde_json::json!(DebugHealthResponse {
            status: "ok".to_string(),
            message: "Server is healthy".to_string(),
            environment: config.environment,

            process: ProcessHealth {
                pid: pid.as_u32(),
                cpu_usage_percent: process.cpu_usage(),
                memory_bytes: process.memory(),
                virtual_memory_bytes: process.virtual_memory(),
                uptime_seconds: process.run_time(),
                start_time: process.start_time(),
                status: process.status().to_string(),
            },

            system: SystemHealth {
                cpu_usage_percent: system.global_cpu_usage(),
                cpu_count: system.cpus().len(),

                total_memory_bytes: system.total_memory(),
                used_memory_bytes: system.used_memory(),
                available_memory_bytes: system.available_memory(),

                total_swap_bytes: system.total_swap(),
                used_swap_bytes: system.used_swap(),

                load_average_1m: load.one,
                load_average_5m: load.five,
                load_average_15m: load.fifteen,

                uptime_seconds: System::uptime(),
            },
        });
        return Json(ApiResponse::new(data, "request_id"));
    }

    let data = serde_json::json!(HealthResponse {
        status: "ok".to_string(),
        message: "Server is healthy".to_string(),
        environment: config.environment,
    });

    return Json(ApiResponse::new(data, "request_id"));
}
