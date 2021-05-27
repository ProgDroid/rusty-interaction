#[macro_use] extern crate rusty_interaction;

use rusty_interaction::handler::InteractionHandler;
use rusty_interaction::types::interaction::*;

// Used for getting TLS to work
use rustls::internal::pemfile::{certs, pkcs8_private_keys};
use rustls::{NoClientAuth, ServerConfig};
use std::fs::File;
use std::io::BufReader;

use std::time::Duration;
use async_std::task;

use rusty_interaction::actix::Arbiter;

const PUB_KEY: &str = "YOUR_PUBLIC_KEY"; 


#[slash_command]
async fn test(ctx: Context) -> InteractionResponse{


    let m = ctx.clone();
    // Spawn a new thread before sending a response. 
    Arbiter::spawn(async move {

        // Wait three seconds and delete
        task::sleep(Duration::from_secs(3)).await;

        m.delete_original().await;
    });
    

    return ctx.respond()
        .message("I was summoned?")
        .finish();
}

// The lib uses actix-web
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    
    let mut handle = InteractionHandler::new(PUB_KEY);
    
    
    handle.add_global_command("summon", test);
 
    let mut config = ServerConfig::new(NoClientAuth::new());
    let cert_file = &mut BufReader::new(File::open("cert.pem").unwrap());
    let key_file = &mut BufReader::new(File::open("key.pem").unwrap());
    let cert_chain = certs(cert_file).unwrap();
    let mut keys = pkcs8_private_keys(key_file).unwrap();
    config.set_single_cert(cert_chain, keys.remove(0)).unwrap();

    return handle.run_ssl(config).await;
    
}
