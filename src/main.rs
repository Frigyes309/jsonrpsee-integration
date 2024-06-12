use jsonrpsee::{RpcModule};
use jsonrpsee::server::ServerBuilder;
use jsonrpsee_core::client::{ClientT};
use jsonrpsee::tokio::main;
use jsonrpsee::types::{ErrorCode, Params};
use jsonrpsee::ws_client::WsClientBuilder;


#[main]
async fn main() {
    let server_mode;
    if std::env::args().len() > 1 {
        server_mode = std::env::args().next_back().map_or(true, |arg| arg == "-s" || arg == "--server")
    } else {
        println!("Mode? [s = server]");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        server_mode = 's' == input.trim().chars().next().unwrap();
    }
    if server_mode {
        println!("Server mode");
        let _ = run_server().await;
    } else {
        println!("Client mode");
        let _ = run_client().await;
    }

}

async fn run_server() -> Result<(), Box<dyn std::error::Error>> {
    // Define the address and port for the server to listen on
    let server_addr:String = String::from("127.0.0.1:3030");

    // Build the server
    let server = ServerBuilder::default()
        .build(server_addr)
        .await?;

    // Create a new RPC module
    let mut module = RpcModule::new(());

    // Add an RPC method to the module
    module.register_method("say_hello", |_, _, _| {
        Ok::<&str, ErrorCode>("Hello, World!".into())
    })?;

    module.register_method("add_i32", |params: Params, _, _| {
        let params = params.parse::<Vec<i32>>().expect("Failed to parse params");
        //println!("Params: {:?}", params);
        //Ok::<i32, ErrorCode>(0)
        Ok::<i32, ErrorCode>(params[0] + params[1])
        //Ok::<i32, ErrorCode>(map.get(&"0".parse().unwrap()).unwrap().as_i64().unwrap() as i32 + map.get(&"1".parse().unwrap()).unwrap().as_i64().unwrap() as i32)
    })?;

    // Start the server and attach the module
    let handle = server.start(module);

    // Keep the server running until interrupted
    handle.stopped().await;

    Ok(())
}

async fn run_client() -> Result<(), Box<dyn std::error::Error>> {
    // Define the address and port where the server is running
    let url:String = String::from("ws://127.0.0.1:3030");

    // Build the client
    let client = WsClientBuilder::new().build(url).await?;
    //let (s, r): (dyn TransportSenderT<Error=()> + Send, dyn TransportReceiverT<Error=()> + Send) =
    //let client = client_builder.build_with_tokio(s, r);

    // Create a new RPC module
    let _module = RpcModule::new(());
    println!("Connection {}",  if client.is_connected() {"was successful"} else { "failed" });
    //let sub: Subscription<i32> = client.subscribe("say_hello", rpc_params![], "unsubscribe_hello").await?;
    let response: serde_json::Value= client.request("say_hello", jsonrpsee::rpc_params![]).await?;
    println!("say_hello response (for no params): {}", response);


    let response: serde_json::Value= client.request("add_i32", jsonrpsee::rpc_params![9, 1]).await?;
    println!("add response (for 9+1): {}", response);

    //println!("Response: {}", response);
    // Add an RPC method to the module
    /*module.register_method("say_hello", |_, _, _| {
        Ok::<&str, ErrorCode>("Hello, World!".into())
    })?;*/

    // Start the server and attach the module
    //let handle = server.start(module);

    // Keep the server running until interrupted
    //handle.stopped().await;

    Ok(())
}