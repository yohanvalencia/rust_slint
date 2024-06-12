slint::include_modules!();

const TAXPER: f64 = 0.30;
const OWNERPER: f64 = 0.55;
const PROFITER: f64 = 0.05;
const OPEXPER: f64 = 0.10;
fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    let ui_handle = ui.as_weak();
    ui.on_divide_income(move |string| {
        let ui = ui_handle.unwrap();
        let num: f64 = string.trim().parse().unwrap();
        let tax = num * TAXPER;
        let owner = num * OWNERPER;
        let profit = num * PROFITER;
        let opex = num * OPEXPER;
        let result = format!("Taxes {:.2}\nOwner: {:.2}\nProfit: {:.2}\nOpEx: {:.2}", {tax}, {owner}, {profit}, {opex});
        ui.set_results(result.into())
    });
    ui.run()
}
