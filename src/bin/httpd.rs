use clippy::data::AppDatabase;
use clippy::web::renderer::Renderer;
use dotenv::dotenv;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "httpd")]
struct Opt {
    #[structopt(default_value = "sqlite:data.db")]
    connection_string: String,
    #[structopt(short, long, parse(from_os_str), default_value = "templates/")]
    template_dir: PathBuf,
}

fn main() {
    dotenv().ok();
    let opt = Opt::from_args();

    let rt = tokio::runtime::Runtime::new().expect("failed to spawn tokio runtime");

    let handle = rt.handle().clone();

    rt.block_on(async move {
        let renderer = Renderer::new(opt.template_dir);
        let database = AppDatabase::new(&opt.connection_string).await;

        let config = clippy::RocketConfig { renderer, database };

        clippy::rocket(config)
            .launch()
            .await
            .expect("failed to launch rocket server")
    });
}
