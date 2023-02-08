# Azure Functions Custom Handler Examples - Rust 

## Overview

This sample covers how to use [Azure Functions Custom Handlers](https://learn.microsoft.com/en-us/azure/azure-functions/functions-custom-handlers) with Rust with different types of triggers. Custom handlers operate by sending HTTP(s) requests from the functions host to a web server running an application in the background, known as your handler application. The API path that your application needs to listen on should be identical to the folder name that your function definition files reside in (ie your `function.json`).

![](https://learn.microsoft.com/en-us/azure/azure-functions/media/functions-custom-handlers/azure-functions-custom-handlers-overview.png)

## Triggers

The examples that are currently implemented in this repo in the `/src/main.rs` file are:
1. HTTP Trigger
   - Called by: HTTP request sent to http://localhost:7071/api/HttpExample
   - Handler application route: `/HttpExample`
2. Timer Trigger
    - Called By: Function host every 30 seconds (configurable in the `function.json` file)
    - Handler application route: `/TimerTriggerExample`

## Instructions
To run this code: 

1. Install Rust, Azure Functions Core Tools, Visual Studio Code
2. Clone this repository and open in Visual Studio Code
3. (If you are on Windows), open the `host.json` file and change the `defaultExecutablePath` value to `handler.exe`
4. Open a terminal with the root directory as your working directory and run `cargo build --release` followed by `cp target/release/handler .`. This will build the sample rust application and copy the built executable to the root directory. 
5. Finally, to start the functions runtime, run `func start`.

## See Also
- https://learn.microsoft.com/en-us/azure/azure-functions/create-first-function-vs-code-other?tabs=rust
- https://learn.microsoft.com/en-us/azure/azure-functions/functions-bindings-register