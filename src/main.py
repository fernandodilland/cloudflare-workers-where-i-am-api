# src/main.py

from fastapi import FastAPI, Request
from fastapi.middleware.cors import CORSMiddleware
from fastapi.responses import JSONResponse

# Initialize the FastAPI application
app = FastAPI()

# --- CORS Configuration ---
# Allows any origin (*) to access this endpoint.
# For better security in production, replace "*" with specific domains
# that should have access, for example: ["https://your-site.com"].
app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],  # Allowed origins
    allow_credentials=True,
    allow_methods=["GET"],  # Only allow GET method for this endpoint
    allow_headers=["*"],    # Allow any header in the request
)

# List of Cloudflare geolocation headers we're interested in
CF_GEOLOCATION_HEADERS = [
    "CF-IPCountry",
    "CF-IPCity",
    "CF-IPContinent",
    "CF-IPLongitude",
    "CF-IPLatitude",
    "CF-Region",
    "CF-Region-Code",
    "CF-Metro-Code",
    "CF-Postal-Code",
    "CF-Timezone",
]

@app.get("/where-am-i")
def where_am_i(request: Request):
    """
    Endpoint that extracts geolocation information from Cloudflare headers
    and returns it in JSON format.
    """
    location_data = {}
    
    # Iterate over the list of headers we're looking for
    for header in CF_GEOLOCATION_HEADERS:
        # request.headers.get(header) returns the value if it exists, or None if not.
        value = request.headers.get(header)
        
        # If the header exists in the request, add it to the dictionary
        if value:
            location_data[header] = value
            
    # Return the found data as a JSON response
    return JSONResponse(content=location_data)

# --- Cloudflare to FastAPI Bridge ---
# The `on_fetch` handler is the entry point for the Worker.
# This code passes the request to the FastAPI application.
async def on_fetch(request, env):
    """
    Entry point for the Cloudflare Worker.
    """
    import asgi
    return await asgi.fetch(app, request, env)