use std::path::Path;

use tokio::fs::{self, create_dir};

use crate::util::key::{PK, SK, generate_pk_and_sk};

pub const NODE_KEY_BITS: u32 = 512;

pub const NODE_DIR_PATH: &str = "node";
pub const NODE_KEY_PATH: &str = "node/key";

pub async fn load_key() -> Result<(PK, SK), ()> {
    if !Path::new(NODE_DIR_PATH).exists() {
        info!("Creating the node directory: {}", NODE_DIR_PATH);
        if create_dir(NODE_DIR_PATH).await.is_err() {
            error!("Failed to create the node directory");
            return Err(());
        }
    }
    if Path::new(NODE_KEY_PATH).exists() {
        return read_key().await;
    }
    let Ok((pk, sk)) = generate_key().await else {
        return Err(());
    };
    if !save_key(sk.clone()).await {
        error!("Failed to save the private key: {}", NODE_KEY_PATH)
    }
    Ok((pk, sk))
}
async fn read_key() -> Result<(PK, SK), ()> {
    info!("Loading the private key from a file: {}", NODE_KEY_PATH);
    let Ok(der) = fs::read_to_string(NODE_KEY_PATH).await else {
        error!("Failed to read the private key");
        return Err(());
    };
    let sk = SK { der };
    let Ok(pk) = sk.to_pk() else {
        error!("Failed to convert the private key to a public key");
        return Err(());
    };
    Ok((pk, sk))
}
async fn generate_key() -> Result<(PK, SK), ()> {
    info!("Generating the private key");
    let Ok((_, sk)) = generate_pk_and_sk(NODE_KEY_BITS) else {
        error!("Failed to generate the private key");
        return Err(());
    };
    let Ok(pk) = sk.to_pk() else {
        error!("Failed to convert the private key to a public key");
        return Err(());
    };
    Ok((pk, sk))
}

async fn save_key(sk: SK) -> bool {
    fs::write(NODE_KEY_PATH, sk.der).await.is_ok()
}
