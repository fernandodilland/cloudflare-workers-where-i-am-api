use worker::*;
use serde::Serialize;

#[derive(Serialize)]
struct LocationData {
    #[serde(rename = "CF-IPCountry")]
    country: Option<String>,
    #[serde(rename = "CF-IPCity")]
    city: Option<String>,
    #[serde(rename = "CF-IPContinent")]
    continent: Option<String>,
    #[serde(rename = "CF-IPLongitude")]
    longitude: Option<String>,
    #[serde(rename = "CF-IPLatitude")]
    latitude: Option<String>,
    #[serde(rename = "CF-Region")]
    region: Option<String>,
    #[serde(rename = "CF-Region-Code")]
    region_code: Option<String>,
    #[serde(rename = "CF-Metro-Code")]
    metro_code: Option<String>,
    #[serde(rename = "CF-Postal-Code")]
    postal_code: Option<String>,
    #[serde(rename = "CF-Timezone")]
    timezone: Option<String>,
}

// Helper function to get allowed origins from environment variable
fn get_allowed_origins(env: &Env) -> Result<Vec<String>> {
    // Try to get CORS_ALLOWED_ORIGINS from environment variables
    match env.var("CORS_ALLOWED_ORIGINS") {
        Ok(origins_str) => {
            // Split by comma and trim whitespace
            let origins: Vec<String> = origins_str
                .to_string()
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
            
            if origins.is_empty() {
                // If empty after filtering, default to "*"
                Ok(vec!["*".to_string()])
            } else {
                Ok(origins)
            }
        }
        Err(_) => {
            // If environment variable is not set, default to "*"
            Ok(vec!["*".to_string()])
        }
    }
}

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    // Configure panic handler for debugging
    console_error_panic_hook::set_once();
    
    // Create the router
    let router = Router::new();
    
    router
        .get_async("/", where_am_i_handler)
        .options_async("/", options_handler)
        .run(req, env)
        .await
}

async fn where_am_i_handler(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    // Get request headers
    let headers = req.headers();
    
    // Extract geolocation information from headers into structured data
    let location_data = LocationData {
        country: headers.get("CF-IPCountry").ok().flatten(),
        city: headers.get("CF-IPCity").ok().flatten(),
        continent: headers.get("CF-IPContinent").ok().flatten(),
        longitude: headers.get("CF-IPLongitude").ok().flatten(),
        latitude: headers.get("CF-IPLatitude").ok().flatten(),
        region: headers.get("CF-Region").ok().flatten(),
        region_code: headers.get("CF-Region-Code").ok().flatten(),
        metro_code: headers.get("CF-Metro-Code").ok().flatten(),
        postal_code: headers.get("CF-Postal-Code").ok().flatten(),
        timezone: headers.get("CF-Timezone").ok().flatten(),
    };
    
    // Create response with CORS enabled
    let mut response = Response::from_json(&location_data)?;
    
    // Get allowed origins from environment variable or default to "*"
    let allowed_origins = get_allowed_origins(&ctx.env)?;
    
    // Determine the allowed origin
    let allowed_origin = if let Ok(Some(origin)) = headers.get("Origin") {
        if allowed_origins.contains(&origin) {
            origin
        } else {
            "null".to_string()
        }
    } else {
        "*".to_string()
    };
    
    // Add CORS headers
    response.headers_mut().set("Access-Control-Allow-Origin", &allowed_origin)?;
    response.headers_mut().set("Access-Control-Allow-Methods", "GET, OPTIONS")?;
    response.headers_mut().set("Access-Control-Allow-Headers", "*")?;
    response.headers_mut().set("Access-Control-Allow-Credentials", "true")?;
    response.headers_mut().set("Content-Type", "application/json")?;
    
    Ok(response)
}

// Handler for OPTIONS requests (CORS preflight)
async fn options_handler(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let mut response = Response::empty()?;
    
    // Get allowed origins from environment variable or default to "*"
    let allowed_origins = get_allowed_origins(&ctx.env)?;
    
    // Get the request origin
    let headers = req.headers();
    let allowed_origin = if let Ok(Some(origin)) = headers.get("Origin") {
        if allowed_origins.contains(&origin) {
            origin
        } else {
            "null".to_string()
        }
    } else {
        "*".to_string()
    };
    
    // Add CORS headers
    response.headers_mut().set("Access-Control-Allow-Origin", &allowed_origin)?;
    response.headers_mut().set("Access-Control-Allow-Methods", "GET, OPTIONS")?;
    response.headers_mut().set("Access-Control-Allow-Headers", "*")?;
    response.headers_mut().set("Access-Control-Allow-Credentials", "true")?;
    response.headers_mut().set("Access-Control-Max-Age", "86400")?;
    
    Ok(response)
}
