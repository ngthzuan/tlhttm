use std::net::ToSocketAddrs;
use std::time::Duration;
use tauri::Manager;
use tauri_plugin_notification::NotificationExt;
use tauri_plugin_opener::OpenerExt;

/// Thông tin về ứng dụng PC và hệ điều hành trả về cho Web Laravel
#[derive(serde::Serialize)]
pub struct AppInfo {
    pub app_name: String,
    pub app_version: String,
    pub tauri_version: String,
    pub os_name: String,
    pub os_family: String,
    pub os_arch: String,
    pub is_desktop_app: bool,
    pub school: String,
    pub platform_title: String,
}

/// Kết quả kích hoạt/tắt chế độ khảo thí an toàn
#[derive(serde::Serialize)]
pub struct ExamModeResult {
    pub success: bool,
    pub exam_mode_active: bool,
    pub message: String,
}

/// Thông tin trạng thái trình biên dịch g++ (GCC/MinGW) trên máy học sinh
#[derive(serde::Serialize)]
pub struct CompilerInfo {
    pub installed: bool,
    pub compiler_name: String,
    pub version: Option<String>,
    pub binary_path: Option<String>,
    pub message: String,
    pub raw_output: Option<String>,
}

/// 1. Command: Lấy thông tin ứng dụng PC và OS
#[tauri::command]
fn get_app_info(app: tauri::AppHandle) -> AppInfo {
    AppInfo {
        app_name: app.package_info().name.clone(),
        app_version: app.package_info().version.to_string(),
        tauri_version: tauri::VERSION.to_string(),
        os_name: std::env::consts::OS.to_string(),
        os_family: std::env::consts::FAMILY.to_string(),
        os_arch: std::env::consts::ARCH.to_string(),
        is_desktop_app: true,
        school: "Trường THCS Long Phú".to_string(),
        platform_title: "TRỢ LÝ HỌC TẬP THÔNG MINH — GDPT 2018".to_string(),
    }
}

/// 2. Command: Bật/Tắt chế độ khảo thí an toàn (Fullscreen + Always on top)
#[tauri::command]
fn set_exam_mode(window: tauri::Window, enable: bool) -> Result<ExamModeResult, String> {
    #[cfg(desktop)]
    {
        window
            .set_fullscreen(enable)
            .map_err(|e| format!("Không thể chuyển đổi toàn màn hình: {}", e))?;

        window
            .set_always_on_top(enable)
            .map_err(|e| format!("Không thể đổi thuộc tính Always On Top: {}", e))?;

        if enable {
            let _ = window.set_focus();
        }
    }

    #[cfg(not(desktop))]
    {
        let _ = window;
    }

    Ok(ExamModeResult {
        success: true,
        exam_mode_active: enable,
        message: if enable {
            "Đã kích hoạt Chế độ Khảo thí An toàn (Toàn màn hình & Khóa ứng dụng trên cùng chống gian lận).".to_string()
        } else {
            "Đã tắt Chế độ Khảo thí. Cửa sổ trở lại bình thường.".to_string()
        },
    })
}

/// 3. Command: Kiểm tra sự hiện diện của trình biên dịch g++ (MinGW/GCC)
#[tauri::command]
fn check_local_compiler() -> CompilerInfo {
    #[cfg(target_os = "windows")]
    use std::os::windows::process::CommandExt;

    let mut cmd = std::process::Command::new("g++");
    cmd.arg("--version");
    #[cfg(target_os = "windows")]
    cmd.creation_flags(0x08000000); // CREATE_NO_WINDOW để không nháy cửa sổ console cmd

    match cmd.output() {
        Ok(output) if output.status.success() => {
            let full_out = String::from_utf8_lossy(&output.stdout).trim().to_string();
            let first_line = full_out.lines().next().unwrap_or("").to_string();

            // Tìm đường dẫn thực thi của g++ qua where.exe
            let mut path_cmd = std::process::Command::new("where.exe");
            path_cmd.arg("g++");
            #[cfg(target_os = "windows")]
            path_cmd.creation_flags(0x08000000);

            let bin_path = path_cmd.output().ok().and_then(|po| {
                if po.status.success() {
                    Some(
                        String::from_utf8_lossy(&po.stdout)
                            .lines()
                            .next()
                            .unwrap_or("")
                            .trim()
                            .to_string(),
                    )
                } else {
                    None
                }
            });

            CompilerInfo {
                installed: true,
                compiler_name: "g++ (GCC / MinGW)".to_string(),
                version: Some(first_line),
                binary_path: bin_path,
                message: "Đã phát hiện trình biên dịch g++. Sẵn sàng phục vụ module Lập Trình C++ Online Judge siêu tốc trên PC!".to_string(),
                raw_output: Some(full_out),
            }
        }
        _ => CompilerInfo {
            installed: false,
            compiler_name: "g++".to_string(),
            version: None,
            binary_path: None,
            message: "Chưa cài đặt trình biên dịch g++ trên máy tính. Để chấm code C++ cục bộ, vui lòng cài đặt MinGW-w64/GCC.".to_string(),
            raw_output: None,
        },
    }
}

/// Lệnh điều hướng về nền tảng web chính
#[tauri::command]
fn reload_platform(window: tauri::WebviewWindow) -> Result<(), String> {
    let url: tauri::Url = "https://tuhoc.thzuan.io.vn"
        .parse()
        .map_err(|e| format!("Lỗi URL: {}", e))?;
    window.navigate(url).map_err(|e| format!("Lỗi điều hướng: {}", e))
}

/// Kiểm tra nhanh kết nối Internet tới máy chủ giáo dục
fn is_online() -> bool {
    if let Ok(mut addrs) = "tuhoc.thzuan.io.vn:443".to_socket_addrs() {
        if let Some(addr) = addrs.next() {
            return std::net::TcpStream::connect_timeout(&addr, Duration::from_millis(1500)).is_ok();
        }
    }
    false
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let app_handle = app.handle().clone();
            let download_dir = app
                .path()
                .download_dir()
                .unwrap_or_else(|_| std::path::PathBuf::from("."));

            // Đọc cấu hình cửa sổ từ tauri.conf.json
            let mut base_config = app.config().app.windows.first().cloned();

            // Xác định URL khởi động dựa vào kết nối Internet
            let target_url = if is_online() {
                tauri::WebviewUrl::External("https://tuhoc.thzuan.io.vn".parse().unwrap())
            } else {
                tauri::WebviewUrl::App("offline.html".into())
            };

            // Nếu cửa sổ chưa được tạo từ trước (do create: false), khởi tạo cửa sổ qua WebviewWindowBuilder
            #[cfg(desktop)]
            if app.get_webview_window("main").is_none() {
                let builder = if let Some(mut config) = base_config.take() {
                    config.url = target_url;
                    tauri::WebviewWindowBuilder::from_config(app, &config)?
                } else {
                    tauri::WebviewWindowBuilder::new(app, "main", target_url)
                        .title("Trợ Lý Học Tập Thông Minh — Trường THCS Long Phú")
                        .inner_size(1280.0, 800.0)
                        .resizable(true)
                        .center()
                };

                let _main_window = builder
                    // Cấu hình chặn điều hướng (Navigation Handler)
                    .on_navigation({
                        let app_handle = app_handle.clone();
                        move |url| {
                            let host = url.host_str().unwrap_or("");
                            let is_internal = host == "tuhoc.thzuan.io.vn"
                                || host == "thzuan.io.vn"
                                || host.ends_with(".thzuan.io.vn")
                                || host == "localhost"
                                || host == "tauri.localhost"
                                || host.is_empty();

                            // Các link Google OAuth hoặc liên kết tài nguyên bên ngoài
                            let is_google = host == "google.com"
                                || host.ends_with(".google.com")
                                || host == "accounts.google.com";

                            if is_google || !is_internal {
                                // Mở bằng trình duyệt mặc định của hệ điều hành
                                let _ = app_handle.opener().open_url(url.as_str(), None::<&str>);
                                false // Chặn mở trong WebView để tránh lỗi OAuth
                            } else {
                                true // Cho phép điều hướng nội bộ bên trong app
                            }
                        }
                    })
                    // Cấu hình tải file (Download Handler) cho Thư viện PDF
                    .on_download({
                        let app_handle = app_handle.clone();
                        move |_webview, event| {
                            match event {
                                tauri::webview::DownloadEvent::Requested { url, destination } => {
                                    let filename = url
                                        .path_segments()
                                        .and_then(|mut s| s.next_back())
                                        .filter(|s| !s.is_empty())
                                        .unwrap_or("tai-lieu-thcs-long-phu.pdf");

                                    let target_path = download_dir.join(filename);
                                    *destination = target_path;
                                    true
                                }
                                tauri::webview::DownloadEvent::Finished { url: _, path, success } => {
                                    if success {
                                        let filename = path
                                            .as_ref()
                                            .and_then(|p| p.file_name())
                                            .map(|n| n.to_string_lossy().to_string())
                                            .unwrap_or_else(|| "Tài liệu PDF".to_string());

                                        let _ = app_handle
                                            .notification()
                                            .builder()
                                            .title("Tải tài liệu hoàn tất")
                                            .body(format!("Đã lưu đề cương/tài liệu \"{}\" về thư mục Downloads của bạn.", filename))
                                            .show();
                                    }
                                    true
                                }
                                _ => true,
                            }
                        }
                    })
                    .build()?;
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_app_info,
            set_exam_mode,
            check_local_compiler,
            reload_platform
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
