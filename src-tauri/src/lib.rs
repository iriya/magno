use rdev::{listen, Event, EventType, Key, Button};
use std::sync::atomic::{AtomicBool, AtomicI64, Ordering};
use tauri::{Manager, Emitter, Position, PhysicalPosition};
use enigo::{Enigo, Key as EnigoKey, KeyboardControllable, MouseControllable};

static CTRL_PRESSED: AtomicBool = AtomicBool::new(false);
static IS_DRAGGING: AtomicBool = AtomicBool::new(false);
static MOUSE_X: AtomicI64 = AtomicI64::new(0);
static MOUSE_Y: AtomicI64 = AtomicI64::new(0);

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .setup(|app| {
            let handle = app.handle().clone();

            std::thread::spawn(move || {
                let mut enigo = Enigo::new();

                let callback = move |event: Event| {
                    match event.event_type {
                        // 1. 实时记录鼠标位置
                        EventType::MouseMove { x, y } => {
                            MOUSE_X.store(x as i64, Ordering::SeqCst);
                            MOUSE_Y.store(y as i64, Ordering::SeqCst);
                        }
                        EventType::KeyPress(Key::ControlLeft) | EventType::KeyPress(Key::ControlRight) => {
                            CTRL_PRESSED.store(true, Ordering::SeqCst);
                        }
                        EventType::KeyRelease(Key::ControlLeft) | EventType::KeyRelease(Key::ControlRight) => {
                            CTRL_PRESSED.store(false, Ordering::SeqCst);
                            IS_DRAGGING.store(false, Ordering::SeqCst);
                        }
                        EventType::ButtonPress(Button::Left) => {
                            if CTRL_PRESSED.load(Ordering::SeqCst) {
                                IS_DRAGGING.store(true, Ordering::SeqCst);
                            }
                        }
                        EventType::ButtonRelease(Button::Left) => {
                            if CTRL_PRESSED.load(Ordering::SeqCst) && IS_DRAGGING.load(Ordering::SeqCst) {
                                IS_DRAGGING.store(false, Ordering::SeqCst);

                                // 获取当前鼠标释放时的坐标
                                let current_x = MOUSE_X.load(Ordering::SeqCst) as i32;
                                let current_y = MOUSE_Y.load(Ordering::SeqCst) as i32;

                                // 2. 模拟复制 Ctrl + C
                                std::thread::sleep(std::time::Duration::from_millis(50));
                                enigo.key_down(EnigoKey::Control);
                                enigo.key_click(EnigoKey::Layout('c'));
                                enigo.key_up(EnigoKey::Control);

                                // 3. 读取剪贴板内容并定位显示浮窗
                                let handle_clone = handle.clone();
                                std::thread::spawn(move || {
                                    std::thread::sleep(std::time::Duration::from_millis(50));
                                    use tauri_plugin_clipboard_manager::ClipboardExt;
                                    if let Ok(text) = handle_clone.clipboard().read_text() {
                                        let trimmed = text.trim();
                                        if !trimmed.is_empty() {
                                            if let Some(window) = handle_clone.get_webview_window("translator") {
                                                // 设置浮窗位置到鼠标右下方（偏移 15px 避免挡住光标）
                                                let _ = window.set_position(Position::Physical(PhysicalPosition {
                                                    x: current_x + 15,
                                                    y: current_y + 15,
                                                }));

                                                let _ = window.show();
                                                let _ = window.set_focus();
                                                let _ = window.emit("selection-captured", trimmed);
                                            }
                                        }
                                    }
                                });
                            }
                        }
                        _ => {}
                    }
                };

                let _ = listen(callback);
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running magno application");
}