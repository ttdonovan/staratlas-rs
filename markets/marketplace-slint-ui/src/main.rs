slint::include_modules!();

use slint::{ModelRc, StandardListViewItem, VecModel};

use std::rc::Rc;

fn main() -> Result<(), slint::PlatformError> {
    let app_window = AppWindow::new()?;
    let facade = app_window.global::<Facade>();

    let certificate_mints = vec![
        ["Item 1.1", "Item 1.2", "Item 1.3"],
        ["Item 2.1", "Item 2.2", "Item 2.3"],
    ];

    let row_data: Rc<VecModel<ModelRc<StandardListViewItem>>> = Rc::new(VecModel::default());

    for data in certificate_mints {
        let row = Rc::new(VecModel::default());

        row.push(data[0].into());
        row.push(data[1].into());
        row.push(data[2].into());

        row_data.push(row.into());
    }

    facade.set_certificate_mints(row_data.into());

    app_window.run()
}
