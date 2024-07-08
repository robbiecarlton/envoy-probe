use std::net::ToSocketAddrs;

use holochain_client::{AdminWebsocket, AppAuthenticationTokenIssued, AppWebsocket, IssueAppAuthenticationTokenPayload};
use anyhow::Result;

// the installed_app_id of the single hosted happ in holo dev server
pub const HDS_HOSTED_HAPP_ID: &str = "uhCkkCQHxC8aG3v3qwD_5Velo1IHE1RdxEr9-tuNSK15u63m1LPOo";
// these are the default app ports for HDS. Should only update them if you've changed the ports when running HDS
pub const ADMIN_PORT: u16 = 4444;
pub const APP_PORT: u16 = 4565;


#[tokio::main]
async fn main() {
    let admin_ws = 
        AdminWebsocket::connect((std::net::Ipv4Addr::LOCALHOST, ADMIN_PORT)).await
        .unwrap_or_else(|e| panic!("Failed to connect admin_ws {:?}", e));
    
    let app_interfaces = admin_ws.list_app_interfaces().await.expect("failed on list_app_interfaces");

    dbg!(app_interfaces);
        
    let apps = admin_ws.list_apps(None).await.expect("failed on list_apps");

    dbg!(apps);
}


// used if you need to make an app ws call.
async fn get_authenticated_app_ws(
    admin_ws: &AdminWebsocket,
    app_id: String,
    app_ws_addr: impl ToSocketAddrs
) -> Result<AppWebsocket> {
    let token = issue_app_authentication_token(admin_ws, app_id).await;

    // We don't use the signer, as we make our own signatures
    let signer = holochain_client::ClientAgentSigner::new();

    let app_ws = AppWebsocket::connect(
        app_ws_addr,
        token,
        signer.into(),
    )
    .await
    .unwrap_or_else(|e| panic!("Failed to connect app_ws {:?}", e));

    Ok(app_ws)
}

async fn issue_app_authentication_token(
    admin_ws: &AdminWebsocket,
    app_id: String,
) -> Vec<u8> {
    let AppAuthenticationTokenIssued {
        token ,
        expires_at: _,
    } = admin_ws.issue_app_auth_token(IssueAppAuthenticationTokenPayload {
        installed_app_id: app_id,
        expiry_seconds: 0,
        single_use: false,
    }).await.expect("failed to issue my boy");

    token
}