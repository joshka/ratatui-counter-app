use app::App;

mod app;
mod tui;

fn main() -> color_eyre::Result<()> {
    tui::install_panic_hook()?;
    let mut terminal = tui::init()?;
    App::default().run(&mut terminal)?;
    tui::restore()?;
    Ok(())
}
