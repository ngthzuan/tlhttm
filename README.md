# Trợ Lý Học Tập Thông Minh — THCS Long Phú (PC Desktop App)

Ứng dụng Desktop PC siêu nhẹ, hiệu năng cao được xây dựng trên nền tảng **Tauri 2.0 (Rust + WebView2)** đóng vai trò là Hybrid WebView Wrapper cho hệ thống:
**"TRỢ LÝ HỌC TẬP THÔNG MINH — Nền Tảng Tự Học & Khảo Thí Trực Tuyến GDPT 2018"**  
🌐 **Địa chỉ web máy chủ**: `https://thzuan.io.vn`  
🏫 **Đơn vị thực hiện**: Trường THCS Long Phú

---

## 🚀 Hướng Dẫn Sử Dụng Nhanh

### 1. Chạy thử nghiệm môi trường phát triển (Dev Mode)
```bash
npm run tauri dev
```

### 2. Đóng gói bộ cài đặt Windows (.exe / .msi)
```bash
npm run tauri build
```
File cài đặt hoàn thiện sẽ được xuất tại thư mục:
`src-tauri/target/release/bundle/nsis/` (hoặc `msi/`).

---

## 🛠️ Tính Năng Kỹ Thuật Đã Triển Khai

1. **Hiệu năng & Kích thước siêu nhẹ**: Sử dụng nhân WebView2 của Windows kết hợp Rust 2.0, dung lượng RAM tiêu thụ chỉ khoảng 30–50MB (so với 300–500MB của Electron).
2. **Cấu hình cửa sổ chuẩn giáo dục**:
   - URL đích: `https://thzuan.io.vn`
   - Kích thước: `1280 x 800`, căn giữa màn hình, tự do co giãn (`resizable: true`).
   - Tiêu đề: `Trợ Lý Học Tập Thông Minh — Trường THCS Long Phú`.
   - Bật `withGlobalTauri: true` cho phép web gọi trực tiếp hàm Rust qua `window.__TAURI__`.
3. **Chặn điều hướng (Navigation Handler)**:
   - Các đường dẫn nội bộ `thzuan.io.vn` hoặc `*.thzuan.io.vn` hoạt động bình thường trong ứng dụng.
   - Các liên kết đăng nhập Google OAuth (`accounts.google.com`, `*.google.com`) hoặc tài nguyên giáo dục bên ngoài tự động mở qua trình duyệt mặc định hệ điều hành bằng plugin `opener` (tránh lỗi 403 Google WebView Block).
4. **Tự động lưu tài liệu PDF (Download Handler)**:
   - Khi học sinh tải đề cương, đề thi, phiếu bài tập từ Thư viện PDF, ứng dụng tự động tải về thư mục `Downloads` của người dùng và bắn thông báo Windows native notification khi hoàn tất.
5. **Trang báo lỗi mất mạng (Offline Fallback)**:
   - Tích hợp trang HTML cục bộ (`offline.html`) với giao diện nhận diện THCS Long Phú hiện đại, thông báo rõ ràng kèm nút **"Thử kết nối lại"** và cơ chế tự động thử kết nối lại theo chu kỳ khi có mạng trở lại.

---

## 📡 Tài Liệu Kết Nối Dành Cho Web Developer (Laravel Integration)

Web Laravel có thể giao tiếp hai chiều với ứng dụng PC thông qua đối tượng toàn cục `window.__TAURI__.core`:

### 1. Nhận diện học sinh đang dùng App PC hay Trình duyệt Web
```javascript
if (window.__TAURI__) {
  const { invoke } = window.__TAURI__.core;
  const appInfo = await invoke('get_app_info');
  console.log('Đang chạy trên PC App:', appInfo.app_version, appInfo.os_name);
} else {
  console.log('Đang chạy trên Trình duyệt thông thường');
}
```

### 2. Bật Chế Độ Khảo Thí An Toàn (Chống gian lận làm bài thi)
```javascript
// Bật chế độ thi: Khóa ứng dụng lên trên cùng và phóng to toàn màn hình
await window.__TAURI__.core.invoke('set_exam_mode', { enable: true });

// Tắt chế độ thi khi học sinh nộp bài:
await window.__TAURI__.core.invoke('set_exam_mode', { enable: false });
```

### 3. Kiểm tra trình biên dịch g++ (Phân hệ Lập Trình C++ Online Judge)
```javascript
const compiler = await window.__TAURI__.core.invoke('check_local_compiler');
if (compiler.installed) {
  console.log('g++ sẵn sàng:', compiler.version);
  console.log('Đường dẫn:', compiler.binary_path);
} else {
  alert(compiler.message);
}
```
