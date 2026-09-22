use rdev::{listen, Event, EventType, Key, Button};
use std::sync::atomic::{AtomicBool, Ordering};
use tauri::{Manager, Emitter};
use enigo::{Enigo, Key as EnigoKey, KeyboardControllable, MouseControllable};

static CTRL_PRESSED: AtomicBool = AtomicBool::new(false);
static IS_DRAGGING: AtomicBool = AtomicBool::new(false);

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .setup(|app| {
            let handle = app.handle().clone();
            
            // 启动后台线程监听全局鼠标与键盘
            std::thread::spawn(move || {
                let mut enigo = Enigo::new();
                
                let callback = move |event: Event| {
                    match event.event_type {
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
                                println!("检测到 Ctrl + 划线释放！");
                                
                                // 1. 模拟复制 Ctrl + C
                                std::thread::sleep(std::time::Duration::from_millis(50));
                                enigo.key_down(EnigoKey::Control);
                                enigo.key_click(EnigoKey::Layout('c'));
                                enigo.key_up(EnigoKey::Control);
                                
                                // 2. 读取剪贴板内容
                                let handle_clone = handle.clone();
                                std::thread::spawn(move || {
                                    std::thread::sleep(std::time::Duration::from_millis(50));
                                    // 这里可以通过 tauri clipboard 插件或 arboard 读取，
                                    // 简化起见，我们通过 Tauri AppHandle 获取剪贴板文本
                                    use tauri_plugin_clipboard_manager::ClipboardExt;
                                    if let Ok(text) = handle_clone.clipboard().read_text() {
                                        let trimmed = text.trim();
                                        if !trimmed.is_empty() {
                                            println!("成功捕获划词: {}", trimmed);
                                            // 3. 显示浮窗并把划词内容发给前端
                                            if let Some(window) = handle_clone.get_webview_window("translator") {
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