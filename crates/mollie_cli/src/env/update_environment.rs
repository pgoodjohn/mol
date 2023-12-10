use super::config;
use super::Environments;
use log::debug;

pub fn set_environment(new_env: &Environments) {
    let old_config = config::from_file().unwrap();

    let mut new_config = old_config.clone();
    match new_env {
        Environments::Prod => {
            new_config.api_url = String::from("https://api.mollie.com");
        }
        Environments::Dev => {
            new_config.api_url = String::from("https://api.mollie.dev");
        }
    }

    debug!("Old config: {:?}", old_config);
    debug!("New config: {:?}", new_config);

    config::save_to_file(new_config).unwrap();
}
