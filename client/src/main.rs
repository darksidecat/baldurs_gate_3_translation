#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use egui::{Label, RichText, Sense};
use egui_extras::{Column, TableBuilder};
use std::cell::RefCell;
use std::cmp::max;

fn main() -> Result<(), eframe::Error> {
    //env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1980.0, 1100.0)),
        resizable: true,
        ..Default::default()
    };

    let mut text_entries: Vec<TextEntry> = vec![
        TextEntry {
            uid: "h000006d4gcefbg4092gbb39gfeb27a3bb0a7".to_string(),
            en_text: "Sorry, darling, I haven't got time for underlings.".to_string(),
            sbt_text: "Вибач, серденько, але у мене немає часу на посіпак".to_string(),
            ua_text: RefCell::new("Вибач, серденько, але у мене немає часу на посіпак".to_string()),
        },
        TextEntry {
            uid: "h000011feg0be5g4f09g978eg030b3e2e62c6".to_string(),
            en_text: "Replace Spell".to_string(),
            sbt_text: "Заміна закляття".to_string(),
            ua_text: RefCell::new("Заміна закляття".to_string()),
        },
        TextEntry {
            uid: "h00011ac6ge2f4g4104gbc6fg246521189b73".to_string(),
            en_text: r#"&lt;LSTag Tooltip="MovementSpeed"&gt;Movement Speed&lt;/LSTag&gt;: [1]/[2]"#.to_string(),
            sbt_text: r#"&lt;LSTag Tooltip="MovementSpeed"&gt;Швидкість руху&lt;/LSTag&gt;: [1]/[2]"#.to_string(),
            ua_text: RefCell::new(r#"&lt;LSTag Tooltip="MovementSpeed"&gt;Швидкість руху&lt;/LSTag&gt;: [1]/[2]"#.to_string()),
        },
        TextEntry {
            uid: "h00011aceg6d1bg4430g8e5cg8672138c6d82".to_string(),
            en_text: "If that's her 'formula' I can smell, it's even fouler than her blood. Gods below...".to_string(),
            sbt_text: r#"Якщо цей сморід — її "формула", то вона огидніша за її кров. Заради богів..."#.to_string(),
            ua_text: RefCell::new(r#"Якщо цей сморід — її "формула", то вона огидніша за її кров. Заради богів..."#.to_string()),
        },
        TextEntry {
            uid: "h0001dc8ag4fd9g4d8ag8775ga6313d6a7ff4".to_string(),
            en_text: "And if you ain't who you say you are, I reckon you're fair game.".to_string(),
            sbt_text: "А як ти не та особа, за котру себе видаєш, то гадаю, що можна й напасти на тебе.".to_string(),
            ua_text: RefCell::new("А як ти не та особа, за котру себе видаєш, то гадаю, що можна й напасти на тебе.".to_string()),
        },
        TextEntry {
            uid: "h000006d4gcefbg4092gbb39gfeb27a3bb0a7".to_string(),
            en_text: "Sorry, darling, I haven't got time for underlings.".to_string(),
            sbt_text: "Вибач, серденько, але у мене немає часу на посіпак".to_string(),
            ua_text: RefCell::new("Вибач, серденько, але у мене немає часу на посіпак".to_string()),
        },
        TextEntry {
            uid: "h000011feg0be5g4f09g978eg030b3e2e62c6".to_string(),
            en_text: "Replace Spell".to_string(),
            sbt_text: "Заміна закляття".to_string(),
            ua_text: RefCell::new("Заміна закляття".to_string()),
        },
        TextEntry {
            uid: "h00011ac6ge2f4g4104gbc6fg246521189b73".to_string(),
            en_text: r#"&lt;LSTag Tooltip="MovementSpeed"&gt;Movement Speed&lt;/LSTag&gt;: [1]/[2]"#.to_string(),
            sbt_text: r#"&lt;LSTag Tooltip="MovementSpeed"&gt;Швидкість руху&lt;/LSTag&gt;: [1]/[2]"#.to_string(),
            ua_text: RefCell::new(r#"&lt;LSTag Tooltip="MovementSpeed"&gt;Швидкість руху&lt;/LSTag&gt;: [1]/[2]"#.to_string()),
        },
        TextEntry {
            uid: "h00011aceg6d1bg4430g8e5cg8672138c6d82".to_string(),
            en_text: "If that's her 'formula' I can smell, it's even fouler than her blood. Gods below...".to_string(),
            sbt_text: r#"Якщо цей сморід — її "формула", то вона огидніша за її кров. Заради богів..."#.to_string(),
            ua_text: RefCell::new(r#"Якщо цей сморід — її "формула", то вона огидніша за її кров. Заради богів..."#.to_string()),
        },
        TextEntry {
            uid: "h0001dc8ag4fd9g4d8ag8775ga6313d6a7ff4".to_string(),
            en_text: "And if you ain't who you say you are, I reckon you're fair game.".to_string(),
            sbt_text: "А як ти не та особа, за котру себе видаєш, то гадаю, що можна й напасти на тебе.".to_string(),
            ua_text: RefCell::new("А як ти не та особа, за котру себе видаєш, то гадаю, що можна й напасти на тебе.".to_string()),
        },
        TextEntry {
            uid: "h000006d4gcefbg4092gbb39gfeb27a3bb0a7".to_string(),
            en_text: "Sorry, darling, I haven't got time for underlings.".to_string(),
            sbt_text: "Вибач, серденько, але у мене немає часу на посіпак".to_string(),
            ua_text: RefCell::new("Вибач, серденько, але у мене немає часу на посіпак".to_string()),
        },
        TextEntry {
            uid: "h000011feg0be5g4f09g978eg030b3e2e62c6".to_string(),
            en_text: "Replace Spell".to_string(),
            sbt_text: "Заміна закляття".to_string(),
            ua_text: RefCell::new("Заміна закляття".to_string()),
        },
        TextEntry {
            uid: "h00011ac6ge2f4g4104gbc6fg246521189b73".to_string(),
            en_text: r#"&lt;LSTag Tooltip="MovementSpeed"&gt;Movement Speed&lt;/LSTag&gt;: [1]/[2]"#.to_string(),
            sbt_text: r#"&lt;LSTag Tooltip="MovementSpeed"&gt;Швидкість руху&lt;/LSTag&gt;: [1]/[2]"#.to_string(),
            ua_text: RefCell::new(r#"&lt;LSTag Tooltip="MovementSpeed"&gt;Швидкість руху&lt;/LSTag&gt;: [1]/[2]"#.to_string()),
        },
        TextEntry {
            uid: "h00011aceg6d1bg4430g8e5cg8672138c6d82".to_string(),
            en_text: "If that's her 'formula' I can smell, it's even fouler than her blood. Gods below...".to_string(),
            sbt_text: r#"Якщо цей сморід — її "формула", то вона огидніша за її кров. Заради богів..."#.to_string(),
            ua_text: RefCell::new(r#"Якщо цей сморід — її "формула", то вона огидніша за її кров. Заради богів..."#.to_string()),
        },
        TextEntry {
            uid: "h0001dc8ag4fd9g4d8ag8775ga6313d6a7ff4".to_string(),
            en_text: "And if you ain't who you say you are, I reckon you're fair game.".to_string(),
            sbt_text: "А як ти не та особа, за котру себе видаєш, то гадаю, що можна й напасти на тебе.".to_string(),
            ua_text: RefCell::new("А як ти не та особа, за котру себе видаєш, то гадаю, що можна й напасти на тебе.".to_string()),
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
    ua_text: RefCell<String>,
}

struct MyApp {
    data: Vec<TextEntry>,
}

impl MyApp {
    fn new(data: Vec<TextEntry>) -> Self {
        Self { data }
    }
}

impl Default for MyApp {
    fn default() -> Self {
        Self { data: vec![] }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("main panel").show(ctx, |ui| {
            let text_size = egui::TextStyle::Body.resolve(ui.style()).size;
            let h = ui.available_height();

            let mut table = TableBuilder::new(ui)
                .striped(true)
                .resizable(true)
                .vscroll(true)
                .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                .column(Column::auto())
                .column(Column::auto())
                .column(Column::auto())
                .column(Column::auto())
                .column(Column::auto())
                .column(Column::auto())
                .min_scrolled_height(h);

            table
                .header(20.0, |mut header| {
                    header.col(|ui| {
                        ui.strong("contentuuid");
                    });
                    header.col(|ui| {
                        ui.strong("en");
                    });
                    header.col(|ui| {
                        ui.strong("ЩБТ");
                    });
                    header.col(|ui| {
                        ui.strong("uk");
                    });
                    header.col(|ui| {
                        ui.strong("Error");
                    });
                    header.col(|ui| {
                        ui.strong("Submit");
                    });
                })
                .body(|body| {
                    let mut heights = vec![];

                    let l0 = *(body.widths().get(0).unwrap());
                    let l1 = *(body.widths().get(1).unwrap());
                    let l2 = *(body.widths().get(2).unwrap());
                    let l3 = *(body.widths().get(3).unwrap());

                    for i in &self.data {
                        let h1 = ((i.uid.len() as f32 / l0 * 120.0) / 10.0).ceil() * 10.0;
                        let h2 = ((i.en_text.len() as f32 / l1 * 120.0) / 10.0).ceil() * 10.0;
                        let h3 = ((i.sbt_text.len() as f32 / l2 * 120.0) / 10.0).ceil() * 10.0;
                        let h4 = ((i.ua_text.clone().into_inner().len() as f32 / l3 * 120.0)
                            / 10.0)
                            .ceil()
                            * 10.0;

                        heights.push(
                            [h1, h2, h3, h4]
                                .iter()
                                .max_by(|a, b| a.partial_cmp(b).unwrap())
                                .unwrap()
                                .to_owned(),
                        );
                    }
                    body.heterogeneous_rows(heights.into_iter(), |row_index, mut row| {
                        row.col(|ui| {
                            ui.add(
                                Label::new(self.data.get(row_index).unwrap().uid.clone())
                                    .wrap(true),
                            );
                        });
                        row.col(|ui| {
                            ui.add(
                                Label::new(self.data.get(row_index).unwrap().en_text.clone())
                                    .wrap(true),
                            );
                        });
                        row.col(|ui| {
                            ui.add(
                                Label::new(self.data.get(row_index).unwrap().sbt_text.clone())
                                    .wrap(true),
                            );
                        });
                        row.col(|ui| {
                            let r0 = (&self.data).get(row_index).unwrap().uid.len() as f32 / l0;
                            let r1 = (&self.data).get(row_index).unwrap().en_text.len() as f32 / l1;
                            let r2 =
                                (&self.data).get(row_index).unwrap().sbt_text.len() as f32 / l2;
                            let r3 = (&self.data)
                                .get(row_index)
                                .unwrap()
                                .ua_text
                                .clone()
                                .into_inner()
                                .len() as f32
                                / l3;

                            let txt = self.data.get_mut(row_index).unwrap().ua_text.get_mut();
                            let height = [r0, r1, r2, r3]
                                .iter()
                                .max_by(|a, b| a.partial_cmp(b).unwrap())
                                .unwrap()
                                .to_owned()
                                * 7.0;
                            ui.add(
                                egui::TextEdit::multiline(txt)
                                    .desired_rows(height as usize)
                                    .desired_width(ui.available_width()),
                            );
                        });
                        row.col(|ui| {
                            let _ = ui.button("❗");
                        });
                        row.col(|ui| {
                            let _ = ui.button("👍");
                        });
                    })
                })
        });
    }
}
