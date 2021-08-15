#[macro_use] extern crate rusty_interaction;

use rusty_interaction::handler::InteractionHandler;
use rusty_interaction::types::interaction::*;


// This key is needed for verifying incoming Interactions. This verification is mandatory. 
// You can find this key in the Discord Developer Portal. 
const PUB_KEY: &str = "YOUR_APP'S_PUBLIC_KEY"; 
// Fill with your application ID
const APP_ID: u64 = 0; 


// This macro will transform the function to something the handler can use
#[slash_command]
// Function handlers should take an `Interaction` object and should return an `InteractionResponse`
async fn test(ctx: Context) -> Result<InteractionResponse, ()>{
    println!("I HAVE BEEN SUMMONED!!!");
        
    // Return a response by using the `Context.respond` function.
    // `Context.respond` returns an `InteractionResponseBuilder`.
    // You can now build a `InteractionResponse` by using it's functions.
    ctx.respond()
            .message("I was summoned?")
            .finish()
}

// The lib uses actix-web
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Enable the logger
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // Initalize our InteractionHandler
    // This will handle incoming interactions and route them to your own handlers
    let mut handle = InteractionHandler::new(APP_ID, PUB_KEY, None);
    
    // This will tell the handler to route the `/summon` command to the test function. So if someone uses `/summon`, test() will be called.
    // Please note that you'll need to register your commands to Discord if you haven't yet. This library only handles incoming Interactions (as of now),
    // not command management.
    handle.add_global_command("summon", test);

    // Run the API server! NOTE: the server runs at port 10080 (Socket binds to 0.0.0.0:10080)
    // This server starts a HTTP server. You MUST switch to HTTPS if you want to move to production. See example 2 for that
    handle.run(10080).await
    
}