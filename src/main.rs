use hyper::{Request, Response, Uri};
use hyper::body::Incoming;
use hyper::service::service_fn;
use hyper_util::client::legacy::Client;
use hyper_util::rt::TokioExecutor;
use http_body_util::{Full, BodyExt};
use std::convert::Infallible;
use std::net::SocketAddr;
use dotenvy::dotenv;

// Load .env file
dotenv().ok();

// Read values from environment
let api_key = env::var("MAESTRO_API_KEY").expect("MAESTRO_API_KEY not set");
let esplora_url = env::var("ESPLORA_URL").expect("ESPLORA_URL not set"); 

#[tokio::main]
async fn main() {
    let addr: SocketAddr = ([0, 0, 0, 0], 8080).into();
    println!("Proxy listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    
    loop {
        let (stream, _) = listener.accept().await.unwrap();
        let io = hyper_util::rt::TokioIo::new(stream);
        
        tokio::task::spawn(async move {
            if let Err(err) = hyper::server::conn::http1::Builder::new()
                .serve_connection(io, service_fn(proxy_handler))
                .await
            {
                eprintln!("Error serving connection: {:?}", err);
            }
        });
    }
}

async fn proxy_handler(req: Request<Incoming>) -> Result<Response<Full<bytes::Bytes>>, Infallible> {
    // Extract info before consuming request
    let method = req.method().clone();
    let path_and_query = req.uri().path_and_query().map(|x| x.as_str()).unwrap_or("").to_string();
    let timestamp = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC");
    
    println!("[{}] {} {} - Proxying request...", timestamp, method, path_and_query);
    
    // Build new URI with API key as query parameter
    let separator = if path_and_query.contains('?') { "&" } else { "?" };
    
    let target_uri: Uri = format!(
        "{}{}{}api-key={}",
        esplora_url,
        path_and_query,
        separator,
        api_key
    )
    .parse()
    .map_err(|e| {
        eprintln!("Failed to parse URI: {}", e);
        Response::builder()
            .status(400)
            .body(Full::new(bytes::Bytes::from("Invalid URI")))
            .unwrap()
    })?;

    // Clone request body
    let whole_body = req.into_body().collect().await.unwrap().to_bytes();

    // Build new request with API key in query parameter
    let new_req = Request::builder()
        .method(method.clone())
        .uri(target_uri.clone())
        .body(Full::new(whole_body))
        .unwrap();

    // Send request to Esplora
    let https_connector = hyper_tls::HttpsConnector::new();
    let client = Client::builder(TokioExecutor::new()).build(https_connector);
    
    match client.request(new_req).await {
        Ok(resp) => {
            let status = resp.status();
            println!("[{}] {} {} - Response: {}", timestamp, method, path_and_query, status);
            
            let (parts, body) = resp.into_parts();
            let body_bytes = body.collect().await.unwrap().to_bytes();
            
            let response = Response::from_parts(parts, Full::new(body_bytes));
            Ok(response)
        }
        Err(e) => {
            println!("[{}] {} {} - Error: {}", timestamp, method, path_and_query, e);
            
            // Return a 500 error response
            let response = Response::builder()
                .status(500)
                .body(Full::new(bytes::Bytes::from(format!("Proxy error: {}", e))))
                .unwrap();
            Ok(response)
        }
    }
}
