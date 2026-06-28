mod app;
use app::App;

fn main() -> Result<(), anyhow::Error> {
    let app = App::init();
    app.execute()
}
