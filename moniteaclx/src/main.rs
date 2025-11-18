#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use slint::quit_event_loop;
use slint::ModelRc;
use slint::VecModel;
use std::error::Error;
use std::rc::Rc;

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    let mw = MainWindow::new()?;

    let membs = Rc::new(VecModel::from(vec![
        TMember {
            name: "One".into(),
            description: "the one desc".into(),
            gain: 10,
        },
        TMember {
            name: "Two".into(),
            description: "the two desc".into(),
            gain: 20,
        },
    ]));

    mw.set_members(ModelRc::from(membs.clone()));
    mw.on_update_ctrl(move |s, val| {
        println!("update_ctrl called: {} {}", s, val);
    });
    mw.on_add_member(move || {
        membs.push(TMember {
            name: "New".into(),
            description: "new desc".into(),
            gain: 50,
        });
    });
    mw.on_close({
        move || {
            quit_event_loop().unwrap();
        }
    });

    mw.run()?;

    Ok(())
}
