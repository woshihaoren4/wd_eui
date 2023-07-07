use eframe::{CreationContext, egui, Frame};
use eframe::egui::{Color32, Context, Ui, Widget, WidgetText};

pub trait Tap{
    fn name(&self)-> &'static str;
    fn show(&mut self,ctx: &egui::Context,ui: &mut egui::Ui);
}

pub struct  TapIndex{
    name:&'static str
}

impl Tap for TapIndex {
    fn name(&self) -> &'static str {
        self.name
    }

    fn show(&mut self, ctx: &Context, ui: &mut Ui) {
        ui.label(format!("this page is {}",self.name));
    }
}

#[derive(Default)]
pub struct App{
    tap:Vec<Box<dyn Tap>>,
    tap_index: usize
}

impl App {
    pub fn new(cc:&CreationContext)->Self{
        App::default()
            .add_tag(TapIndex{name:"one"})
            .add_tag(TapIndex{name:"two"})
    }
    fn add_tag<T:Tap + 'static>(mut self,tag:T)->App{
        self.tap.push(Box::new(tag));self
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx,|ui|{
            //显示第一排tap
            ui.horizontal(|ui|{
                for (i,tap) in self.tap.iter().enumerate(){
                    let mut bt = egui::Button::new(tap.name());
                    if i == self.tap_index {
                        bt = bt.fill(Color32::GREEN);
                    }
                    if bt.ui(ui).clicked() {
                        self.tap_index = i;
                    }
                }
            });
            //绘制对应的界面
            ui.separator();
            if self.tap_index < self.tap.len() {
                self.tap[self.tap_index].show(ctx,ui);
            }
        });
    }
}