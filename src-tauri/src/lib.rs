use rdev::{listen, Event, EventType, Key, Button};
use std::sync::atomic::{AtomicBool, AtomicI64, Ordering};
use tauri::{Manager, Emitter, Position, PhysicalPosition};
use enigo::{Enigo, Key as EnigoKey, KeyboardControllable};

static CTRL_PRESSED: AtomicBool = AtomicBool::new(false);
static DOWN_WITH_CTRL: AtomicBool = AtomicBool::new(false);
static MOUSE_X: AtomicI64 = AtomicI64::new(0);
static MOUSE_Y: AtomicI64 = AtomicI64::new(0);
// 记录鼠标按下时的起始坐标
static START_X: AtomicI64 = AtomicI64::new(0);
static START_Y: AtomicI64 = AtomicI64::new(0);

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
                            DOWN_WITH_CTRL.store(false, Ordering::SeqCst);
                        }
                        EventType::ButtonPress(Button::Left) => {
                            if CTRL_PRESSED.load(Ordering::SeqCst) {
                                DOWN_WITH_CTRL.store(true, Ordering::SeqCst);
                                // 记录按下时的起始坐标
                                START_X.store(MOUSE_X.load(Ordering::SeqCst), Ordering::SeqCst);
                                START_Y.store(MOUSE_Y.load(Ordering::SeqCst), Ordering::SeqCst);
                            }
                        }
                        EventType::ButtonRelease(Button::Left) => {
                            let was_down = DOWN_WITH_CTRL.load(Ordering::SeqCst);
                            DOWN_WITH_CTRL.store(false, Ordering::SeqCst);
                            CTRL_PRESSED.store(false, Ordering::SeqCst);

                            if was_down {
                                // 获取当前鼠标释放时的坐标
                                let current_x = MOUSE_X.load(Ordering::SeqCst);
                                let current_y = MOUSE_Y.load(Ordering::SeqCst);
                                let start_x = START_X.load(Ordering::SeqCst);
                                let start_y = START_Y.load(Ordering::SeqCst);

                                let dx = current_x - start_x;
                                let dy = current_y - start_y;
                                let distance_squared = dx * dx + dy * dy;

                                if distance_squared > 225 {
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
                                                        x: current_x as i32 + 15,
                                                        y: current_y as i32 + 15,
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