use druid::widget::{Align, Button, Controller, Flex, Label, Stepper, TextBox};
use druid::{
    commands, AppLauncher, Data, Env, Event, EventCtx, Lens, Widget, WidgetExt, WindowDesc,
};
use shared_child::SharedChild;
use std::process::Command;
use std::sync::Arc;

#[derive(Data, Lens)]
struct AppData {
    id: String,
    stage: f64,
    method: String,
    #[data(ignore)]
    proc: Option<Arc<SharedChild>>,
}

impl Clone for AppData {
    fn clone(&self) -> AppData {
        let p = match &self.proc {
            Some(p) => Some(p.clone()),
            None => None,
        };
        AppData {
            id: self.id.clone(),
            stage: self.stage,
            method: self.method.clone(),
            proc: p,
        }
    }
}

pub fn gui() {
    let main_window = WindowDesc::new(ui_builder)
        .with_min_size((350.0, 250.0))
        .window_size((350.0, 250.0))
        //.resizable(false)
        .title("Wheatley".to_string());
    let data = AppData {
        id: "213576498".to_string(),
        stage: 6.0,
        method: "Plain Bob".to_string(),
        proc: None,
    };
    let launcher = AppLauncher::with_window(main_window);
    // let event_sink = launcher.get_external_handle();
    // thread::spawn(move || update_count(rx, event_sink));
    launcher.launch(data).expect("Failed to launch gui");
}

fn ui_builder() -> impl Widget<AppData> {
    let stage_stepper = Flex::row()
        .with_child(Label::dynamic(|data: &AppData, _| {
            format!("Stage: {:.0}", data.stage)
        }))
        .with_spacer(4.0)
        .with_child(
            Stepper::new()
                .with_range(2.0, 24.0)
                .with_step(1.0)
                .lens(AppData::stage),
        )
        .padding(10.0);

    let tower_label = Flex::row()
        .must_fill_main_axis(true)
        .with_child(Label::new("Tower ID:"))
        .with_spacer(4.0)
        .with_child(TextBox::new().lens(AppData::id))
        .padding(10.0);

    let method_label = Flex::row()
        .must_fill_main_axis(true)
        .with_child(Label::new("Method:").controller(CloseController))
        .with_spacer(4.0)
        .with_child(TextBox::new().lens(AppData::method).fix_width(200.0))
        .padding(10.0);

    let start_button = Button::new("Start")
        .on_click(move |_ctx, data: &mut AppData, _env| start(data))
        .fix_width(150.0)
        .padding(10.0);

    Flex::column()
        .with_spacer(10.0)
        .with_child(Align::left(tower_label))
        .with_child(Align::left(stage_stepper))
        .with_child(Align::left(method_label))
        .with_child(Align::left(start_button))
        .expand_width()
}

fn method_full_name(method: &str, stage: &f64) -> String {
    let stage_names = [
        "", "", "", "Singles", "Minimus", "Doubles", "Minor", "Triples", "Major", "Caters",
        "Royal", "Cinques", "Maximus",
    ];
    [method, stage_names[*stage as usize]].join(" ")
}

fn start(data: &mut AppData) {
    match &data.proc {
        Some(p) => p.kill().unwrap(),
        None => {}
    };
    let full_name = method_full_name(&data.method, &data.stage);
    let mut cmd = Command::new("/home/gary/anaconda3/bin/wheatley");
    cmd.arg(&data.id).arg("--method").arg(&full_name);
    //     .spawn()
    // {
    //     Ok(process) => Some(process),
    //     Err(_) => None,
    // };

    let shared_child = SharedChild::spawn(&mut cmd).unwrap();
    data.proc = Some(Arc::new(shared_child));

    // match process {
    //     Some(mut p) => {
    //         p.wait().expect("Wheatley has failed");
    //     }
    //     None => sleep(Duration::from_secs(36000)),
    // }
}

struct CloseController;

impl Controller<AppData, Label<AppData>> for CloseController {
    fn event(
        &mut self,
        child: &mut Label<AppData>,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut AppData,
        env: &Env,
    ) {
        match event {
            Event::Command(cmd) if cmd.is(commands::CLOSE_WINDOW) => {
                match &data.proc {
                    Some(p) => p.kill().unwrap(),
                    None => {}
                };
            }
            _ => child.event(ctx, event, data, env),
        }
    }
}
fn main() {
    gui();
}
