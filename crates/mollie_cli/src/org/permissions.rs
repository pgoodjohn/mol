use crate::config::MollieConfig;

use log::info;
use colored_json::ToColoredJson;
use pad::{Alignment, PadStr};
use mollie_api::{Mollie, models::permission::PermissionsEmbeddedResource};

pub async fn command(config: &MollieConfig, filter_granted: &bool, with_response: bool) -> anyhow::Result<()> {
    let permissions = Mollie::build(&config.bearer_token().unwrap().as_str()).permissions().list().await?;

    if *filter_granted {
        list_granted_permissions(&permissions.embedded)
    } else {
        list_permissions(&permissions.embedded);
    }

    if with_response {
        let pretty_json = jsonxf::pretty_print(&serde_json::to_string(&permissions).unwrap()).unwrap();
        info!("{}", pretty_json.to_colored_json_auto().unwrap());
    }

    Ok(())
}

fn list_permissions(permissions: &PermissionsEmbeddedResource) {
    for permission in permissions.clone().permissions {
        info!(
            "{} | Granted: {} | {}",
            permission
                .id
                .pad_to_width_with_alignment(20, Alignment::Right),
            permission.granted as i32,
            permission.description
        );
    }
}

fn list_granted_permissions(permissions: &PermissionsEmbeddedResource) {
    for permission in permissions.clone().permissions {
        if permission.granted {
            info!(
                "{} | {}",
                permission
                    .id
                    .pad_to_width_with_alignment(20, Alignment::Right),
                permission.description
            );
        }
    }
}
