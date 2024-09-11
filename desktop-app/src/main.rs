// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;

slint::include_modules!();

const HOUSE: f64 = 0.35;
const ME: f64 = 0.35;
const HEALTH: f64 = 0.05;
const COMPANY: f64 = 0.05;
const REST: f64 = 0.20;

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;

    ui.on_divide_income({
        let ui_handle = ui.as_weak();
        move |string| {
            let ui = ui_handle.unwrap();
            let num : f64 = string.trim().parse().unwrap();
            let house: f64 = num * HOUSE;
            let me: f64 = num * ME;
            let health: f64 = num * HEALTH;
            let company: f64 = num * COMPANY;
            let rest: f64 = num * REST;
            let result = format!("House: {:.2}\nHealth: {:.2}\nCompany: {:.2}\nMe: {:.2}\nRest: {:.2}", house, health, company, me, rest);
            ui.set_results(result.into());
        }
    });

    ui.run()?;

    Ok(())
}
