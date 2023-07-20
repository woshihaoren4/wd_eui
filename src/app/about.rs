use eframe::egui;
use eframe::egui::{Align, Context, Layout, RichText, Ui, WidgetText};

#[derive(Debug)]
pub struct About{
    title:&'static str,
    info:&'static str,
    corner:String,
}

impl Default for About {
    fn default() -> Self {
        let title = "这是一个协调测试软件";
        let info = "主要用来测试协调软件的协调工作情况。为方便测试，这里使用了http到grpc的代理。";
        let corner = format!("{}{}",egui::special_emojis::GITHUB," source code");
        Self {title,info,corner}
    }
}

impl super::Tap for About{
    fn name(&self) -> &'static str {
        "ABOUT"
    }

    fn show(&mut self, _ctx: &Context, ui: &mut Ui) {
        ui.vertical_centered(|ui|{
            ui.add_space(50.0);
            ui.label(RichText::from(self.title).size(24.0).strong());
            ui.label(self.info);
        });
        ui.with_layout(Layout::bottom_up(Align::Center),|ui|{
            ui.hyperlink_to( self.corner.as_str(),"https://github.com/woshihaoren4/wd_eui");
            ui.separator();
        });
    }
}