#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    //env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1980.0, 1100.0)),
        resizable: true,
        ..Default::default()
    };

    let text_entries: Vec<TextEntry> = vec![
        TextEntry {
            uid: "h000006d4gcefbg4092gbb39gfeb27a3bb0a7".to_string(),
            en_text: "Sorry, darling, I haven't got time for underlings.".to_string(),
            sbt_text: "Вибач, серденько, але у мене немає часу на посіпак".to_string(),
            ua_text: "Вибач, серденько, але у мене немає часу на посіпак".to_string(),
        },
        TextEntry {
            uid: "h000011feg0be5g4f09g978eg030b3e2e62c6".to_string(),
            en_text: "Replace Spell".to_string(),
            sbt_text: "Заміна закляття".to_string(),
            ua_text: "Заміна закляття".to_string(),
        },
        TextEntry {
            uid: "h00011ac6ge2f4g4104gbc6fg246521189b73".to_string(),
            en_text: r#"&lt;LSTag Tooltip="MovementSpeed"&gt;Movement Speed&lt;/LSTag&gt;: [1]/[2]"#.to_string(),
            sbt_text: r#"&lt;LSTag Tooltip="MovementSpeed"&gt;Швидкість руху&lt;/LSTag&gt;: [1]/[2]"#.to_string(),
            ua_text: r#"&lt;LSTag Tooltip="MovementSpeed"&gt;Швидкість руху&lt;/LSTag&gt;: [1]/[2]"#.to_string(),
        },
        TextEntry {
            uid: "h00011aceg6d1bg4430g8e5cg8672138c6d82".to_string(),
            en_text: "If that's her 'formula' I can smell, it's even fouler than her blood. Gods below...".to_string(),
            sbt_text: r#"Якщо цей сморід — її "формула", то вона огидніша за її кров. Заради богів..."#.to_string(),
            ua_text: r#"Якщо цей сморід — її "формула", то вона огидніша за її кров. Заради богів..."#.to_string(),
        },
        TextEntry {
            uid: "h0001dc8ag4fd9g4d8ag8775ga6313d6a7ff4".to_string(),
            en_text: "And if you ain't who you say you are, I reckon you're fair game.".to_string(),
            sbt_text: "А як ти не та особа, за котру себе видаєш, то гадаю, що можна й напасти на тебе.".to_string(),
            ua_text: "А як ти не та особа, за котру себе видаєш, то гадаю, що можна й напасти на тебе.".to_string(),
        },
        TextEntry {
            uid: "h000006d4gcefbg4092gbb39gfeb27a3bb0a7".to_string(),
            en_text: "Sorry, darling, I haven't got time for underlings.".to_string(),
            sbt_text: "Вибач, серденько, але у мене немає часу на посіпак".to_string(),
            ua_text: "Вибач, серденько, але у мене немає часу на посіпак".to_string(),
        },
        TextEntry {
            uid: "h000011feg0be5g4f09g978eg030b3e2e62c6".to_string(),
            en_text: "Replace Spell".to_string(),
            sbt_text: "Заміна закляття".to_string(),
            ua_text: "Заміна закляття".to_string(),
        },
        TextEntry {
            uid: "h00011ac6ge2f4g4104gbc6fg246521189b73".to_string(),
            en_text: r#"&lt;LSTag Tooltip="MovementSpeed"&gt;Movement Speed&lt;/LSTag&gt;: [1]/[2]"#.to_string(),
            sbt_text: r#"&lt;LSTag Tooltip="MovementSpeed"&gt;Швидкість руху&lt;/LSTag&gt;: [1]/[2]"#.to_string(),
            ua_text: r#"&lt;LSTag Tooltip="MovementSpeed"&gt;Швидкість руху&lt;/LSTag&gt;: [1]/[2]"#.to_string(),
        },
        TextEntry {
            uid: "h00011aceg6d1bg4430g8e5cg8672138c6d82".to_string(),
            en_text: "If that's her 'formula' I can smell, it's even fouler than her blood. Gods below...".to_string(),
            sbt_text: r#"Якщо цей сморід — її "формула", то вона огидніша за її кров. Заради богів..."#.to_string(),
            ua_text: r#"Якщо цей сморід — її "формула", то вона огидніша за її кров. Заради богів..."#.to_string(),
        },
        TextEntry {
            uid: "h0001dc8ag4fd9g4d8ag8775ga6313d6a7ff4".to_string(),
            en_text: "And if you ain't who you say you are, I reckon you're fair game.".to_string(),
            sbt_text: "А як ти не та особа, за котру себе видаєш, то гадаю, що можна й напасти на тебе.".to_string(),
            ua_text: "А як ти не та особа, за котру себе видаєш, то гадаю, що можна й напасти на тебе.".to_string(),
        },
        TextEntry {
            uid: "h000006d4gcefbg4092gbb39gfeb27a3bb0a7".to_string(),
            en_text: "Sorry, darling, I haven't got time for underlings.".to_string(),
            sbt_text: "Вибач, серденько, але у мене немає часу на посіпак".to_string(),
            ua_text: "Вибач, серденько, але у мене немає часу на посіпак".to_string(),
        },
        TextEntry {
            uid: "h000011feg0be5g4f09g978eg030b3e2e62c6".to_string(),
            en_text: "Replace Spell".to_string(),
            sbt_text: "Заміна закляття".to_string(),
            ua_text: "Заміна закляття".to_string(),
        },
        TextEntry {
            uid: "h00011ac6ge2f4g4104gbc6fg246521189b73".to_string(),
            en_text: r#"&lt;LSTag Tooltip="MovementSpeed"&gt;Movement Speed&lt;/LSTag&gt;: [1]/[2]"#.to_string(),
            sbt_text: r#"&lt;LSTag Tooltip="MovementSpeed"&gt;Швидкість руху&lt;/LSTag&gt;: [1]/[2]"#.to_string(),
            ua_text: r#"&lt;LSTag Tooltip="MovementSpeed"&gt;Швидкість руху&lt;/LSTag&gt;: [1]/[2]"#.to_string(),
        },
        TextEntry {
            uid: "h00011aceg6d1bg4430g8e5cg8672138c6d82".to_string(),
            en_text: "If that's her 'formula' I can smell, it's even fouler than her blood. Gods below...".to_string(),
            sbt_text: r#"Якщо цей сморід — її "формула", то вона огидніша за її кров. Заради богів..."#.to_string(),
            ua_text: r#"Якщо цей сморід — її "формула", то вона огидніша за її кров. Заради богів..."#.to_string(),
        },
        TextEntry {
            uid: "h0001dc8ag4fd9g4d8ag8775ga6313d6a7ff4".to_string(),
            en_text: "And if you ain't who you say you are, I reckon you're fair game.".to_string(),
            sbt_text: "А як ти не та особа, за котру себе видаєш, то гадаю, що можна й напасти на тебе.".to_string(),
            ua_text: "А як ти не та особа, за котру себе видаєш, то гадаю, що можна й напасти на тебе.".to_string(),
        },
    ];

    eframe::run_native(
        "Baldur`s Gate 3 translation",
        options,
        Box::new(|_cc| Box::<MyApp>::new(MyApp::new(text_entries))),
    )
}

struct TextEntry {
    uid: String,
    en_text: String,
    sbt_text: String,
    ua_text: String
}

struct MyApp {
    data: Vec<TextEntry>,
}

impl MyApp {
    fn new(data: Vec<TextEntry>) -> Self {
        Self {
            data
        }
    }

}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            data: vec![],
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("main panel").show(ctx, |ui| {
            egui::ScrollArea::vertical().min_scrolled_height(800.0).show(ui, |ui | {
                egui::Grid::new("main_text_grid").striped(true).min_col_width(400.0).show(ui, |ui| {
                    for row in &mut self.data {
                        ui.label(&row.uid);
                        ui.label(&row.en_text);
                        ui.label(&row.sbt_text);
                        ui.add(egui::TextEdit::multiline(&mut row.ua_text).desired_width(400.0));
                        let _ = ui.button("❗");
                        let _ =ui.button("👍");
                        ui.end_row();
                    };
                });
            });
    });
    }
}