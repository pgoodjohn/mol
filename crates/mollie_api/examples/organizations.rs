use std::env::var;

use mollie_api::Mollie;

#[tokio::main]
async fn main() {
    let auth_token = var("MOLLIE_API_KEY").expect("MOLLIE_API_KEY not set");
    let client = Mollie::build(&auth_token);

    let me = client.organizations().me().await.unwrap();
    println!("Me: {:#?}", me);

    let id = client.organizations().get_by_id(&me.id).await.unwrap();
    println!("Org by Id: {:#?}", id);

    let partner = client.organizations().get_partner().await.unwrap();
    println!("Partner: {:#?}", partner);
}
