use crate::key::{VirtualKey, PRESSED_KEYS_STATE};
use std::time::Instant;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct GuiApp {
    label: String,
    #[serde(skip)]
    value: f32,
    #[serde(skip)]
    fade_start_time: Option<Instant>,
    #[serde(skip)]
    last_keys: Vec<VirtualKey>,
    #[serde(skip)]
    is_fading: bool,
}

impl Default for GuiApp {
    fn default() -> Self {
        Self {
            label: "Hello World!".to_owned(),
            value: 2.7,
            fade_start_time: None,
            last_keys: Vec::new(),
            is_fading: false,
        }
    }
}

impl eframe::App for GuiApp {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Rgba::TRANSPARENT.to_array()
    }
    
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // 持续刷新UI以实现动画效果
        ctx.request_repaint();

        let pressed_state = PRESSED_KEYS_STATE.lock().unwrap();
        let current_keys: Vec<VirtualKey> = pressed_state.keys.iter().cloned().collect();
        let should_show = pressed_state.should_show_ui;

        // 检测按键释放，开始淡出动画
        if !current_keys.is_empty() && !self.last_keys.is_empty() && current_keys != self.last_keys
        {
            // 按键组合发生变化，更新显示
            self.last_keys = current_keys.clone();
            self.is_fading = false;
            self.fade_start_time = None;
        } else if current_keys.is_empty() && !self.last_keys.is_empty() {
            // 所有按键释放，开始淡出
            if self.fade_start_time.is_none() {
                self.fade_start_time = Some(Instant::now());
                self.is_fading = true;
            }
        } else if !current_keys.is_empty() {
            // 有按键按下，更新显示
            self.last_keys = current_keys.clone();
            self.is_fading = false;
            self.fade_start_time = None;
        }

        drop(pressed_state); // 释放锁

        // 计算透明度
        let alpha = if self.is_fading {
            if let Some(fade_start) = self.fade_start_time {
                let elapsed = fade_start.elapsed().as_millis() as f32;
                let fade_duration = 1000.0; // 1秒淡出
                (1.0 - (elapsed / fade_duration)).max(0.0)
            } else {
                1.0
            }
        } else if !self.last_keys.is_empty() {
            1.0
        } else {
            0.0
        };

        // 如果完全透明且在淡出状态，清除按键记录
        if alpha <= 0.0 && self.is_fading {
            self.last_keys.clear();
            self.is_fading = false;
            self.fade_start_time = None;
        }

        // 显示按键弹窗
        if alpha > 0.0 && !self.last_keys.is_empty() {
            self.show_key_popup(ctx, alpha);
        }
    }
}

impl GuiApp {
    fn show_key_popup(&self, ctx: &egui::Context, alpha: f32) {
        let screen_rect = ctx.screen_rect();
        // println!("screen_rect: {:?}", screen_rect);
        // 计算按键显示的总宽度
        let key_width = 50.0;
        let spacing = 10.0;
        let total_width = (self.last_keys.len() as f32) * key_width
            + ((self.last_keys.len() - 1) as f32) * spacing;
        // this popup should be in parent viewport center

        // 位置在屏幕居中偏下
        let popup_pos = egui::pos2(
            screen_rect.center().x - total_width / 2.0,
            screen_rect.max.y - 150.0,
        );

        // 使用Area代替Window，实现无边框悬浮
        egui::Area::new(egui::Id::new("key_display"))
            .order(egui::Order::Foreground)
            .fixed_pos(popup_pos)
            .show(ctx, |ui| {
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                    for (i, key) in self.last_keys.iter().enumerate() {
                        if i > 0 {
                            // 添加连接符
                            ui.add_space(5.0);
                            ui.label(
                                egui::RichText::new("+")
                                    .color(egui::Color32::from_rgba_unmultiplied(
                                        255,
                                        255,
                                        255,
                                        (255.0 * alpha) as u8,
                                    ))
                                    .size(16.0),
                            );
                            ui.add_space(5.0);
                        }

                        // 绘制按键
                        self.draw_key_button(ui, key, alpha);
                    }
                });
            });
    }

    fn draw_key_button(&self, ui: &mut egui::Ui, key: &VirtualKey, alpha: f32) {
        let key_icon = self.get_key_icon(key);
        let key_size = egui::vec2(50.0, 50.0);

        // 创建蓝色按键背景（类似图片风格）
        let button_color =
            egui::Color32::from_rgba_unmultiplied(70, 130, 255, (220.0 * alpha) as u8); // 蓝色背景
        let border_color = egui::Color32::from_rgba_unmultiplied(0, 0, 0, (255.0 * alpha) as u8); // 深蓝色边框

        let (rect, _) = ui.allocate_exact_size(key_size, egui::Sense::hover());

        // 绘制按键背景和边框（更大的圆角）
        ui.painter().rect_stroke(
            rect,
            egui::CornerRadius::same(5),
            egui::Stroke::new(10.0, border_color),
            egui::StrokeKind::Outside,
        );
        ui.painter()
            .rect_filled(rect, egui::CornerRadius::ZERO, button_color); // 增大圆角半径

        // 绘制按键图标/文字（白色文字）
        let text_color =
            egui::Color32::from_rgba_unmultiplied(255, 255, 255, (255.0 * alpha) as u8);

        ui.painter().text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            key_icon,
            egui::FontId::proportional(20.0), // 稍微增大字体
            text_color,
        );
    }

    fn get_key_icon(&self, key: &VirtualKey) -> String {
        match key {
            VirtualKey::LeftControl | VirtualKey::RightControl => "Ctrl".to_string(),
            VirtualKey::LeftShift | VirtualKey::RightShift => "Shift".to_string(),
            VirtualKey::Key0 => "0".to_string(),
            VirtualKey::Key1 => "1".to_string(),
            VirtualKey::Key2 => "2".to_string(),
            VirtualKey::Key3 => "3".to_string(),
            VirtualKey::Key4 => "4".to_string(),
            VirtualKey::Key5 => "5".to_string(),
            VirtualKey::Key6 => "6".to_string(),
            VirtualKey::Key7 => "7".to_string(),
            VirtualKey::Key8 => "8".to_string(),
            VirtualKey::Key9 => "9".to_string(),
            VirtualKey::KeyA => "A".to_string(),
            VirtualKey::KeyB => "B".to_string(),
            VirtualKey::KeyC => "C".to_string(),
            VirtualKey::KeyD => "D".to_string(),
            VirtualKey::KeyE => "E".to_string(),
            VirtualKey::KeyF => "F".to_string(),
            VirtualKey::KeyG => "G".to_string(),
            VirtualKey::KeyH => "H".to_string(),
            VirtualKey::KeyI => "I".to_string(),
            VirtualKey::KeyJ => "J".to_string(),
            VirtualKey::KeyK => "K".to_string(),
            VirtualKey::KeyL => "L".to_string(),
            VirtualKey::KeyM => "M".to_string(),
            VirtualKey::KeyN => "N".to_string(),
            VirtualKey::KeyO => "O".to_string(),
            VirtualKey::KeyP => "P".to_string(),
            VirtualKey::KeyQ => "Q".to_string(),
            VirtualKey::KeyR => "R".to_string(),
            VirtualKey::KeyS => "S".to_string(),
            VirtualKey::KeyT => "T".to_string(),
            VirtualKey::KeyU => "U".to_string(),
            VirtualKey::KeyV => "V".to_string(),
            VirtualKey::KeyW => "W".to_string(),
            VirtualKey::KeyX => "X".to_string(),
            VirtualKey::KeyY => "Y".to_string(),
            VirtualKey::KeyZ => "Z".to_string(),
            VirtualKey::Backspace => "Del".to_string(),
            VirtualKey::Alt => "Alt".to_string(),
            VirtualKey::Space => "Space".to_string(),
            VirtualKey::UpArrow => "Up".to_string(),
            VirtualKey::DownArrow => "Down".to_string(),
            VirtualKey::LeftArrow => "Left".to_string(),
            VirtualKey::RightArrow => "Right".to_string(),
            // VirtualKey::KeyF1 => "F1".to_string(),
            // VirtualKey::KeyF2 => "F2".to_string(),
            // VirtualKey::KeyF3 => "F3".to_string(),
            // VirtualKey::KeyF4 => "F4".to_string(),
            // VirtualKey::KeyF5 => "F5".to_string(),
            // VirtualKey::KeyF6 => "F6".to_string(),
            // VirtualKey::KeyF7 => "F7".to_string(),
            // VirtualKey::KeyF8 => "F8".to_string(),
            // VirtualKey::KeyF9 => "F9".to_string(),
            // VirtualKey::KeyF10 => "F10".to_string(),
            _ => format!("{:?}", key),
        }
    }
}
