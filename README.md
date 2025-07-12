# Cloudflare Workers Rust - Where Am I API

🦀 A Rust-based Cloudflare Worker that provides geolocation information based on Cloudflare's edge headers.

[![Deploy to Cloudflare](https://deploy.workers.cloudflare.com/button)](https://deploy.workers.cloudflare.com/?url=https://github.com/fernandodilland/cloudflare-workers-where-i-am-api)

## Features

- **Fast**: Built with Rust for maximum performance
- **Lightweight**: Compiled to WebAssembly for minimal size
- **CORS-enabled**: Ready for cross-origin requests with configurable origins
- **Geolocation**: Extracts location data from Cloudflare headers
- **Environment Variables**: Configurable CORS origins for production security
- **One-Click Deploy**: Deploy to Cloudflare with a single button click

## API Endpoints

### `GET /`

Returns geolocation information based on Cloudflare's edge headers.

**Request Example:**
```
GET https://your-worker-name.your-subdomain.workers.dev/
```

**Response Example:**
```json
{
  "CF-IPCountry": "US",
  "CF-IPCity": "San Francisco",
  "CF-IPContinent": "NA",
  "CF-IPLongitude": "-122.39420",
  "CF-IPLatitude": "37.77490",
  "CF-Region": "California",
  "CF-Region-Code": "CA",
  "CF-Metro-Code": "807",
  "CF-Postal-Code": "94107",
  "CF-Timezone": "America/Los_Angeles"
}
```

#### Testing with Postman

1. **Create a new request** in Postman
2. **Set method to GET**
3. **Enter URL**: `https://your-worker-name.your-subdomain.workers.dev/`
4. **Headers** (optional):
   - `Origin`: `https://yourdomain.com` (if testing CORS)
   - `Content-Type`: `application/json`
5. **Send the request**

**Expected Response:**
- **Status Code**: `200 OK`
- **Headers**: 
  - `Content-Type`: `application/json`
  - `Access-Control-Allow-Origin`: `*` (or your specific domain)
- **Body**: JSON with geolocation data (as shown above)

#### Testing CORS with Postman

To test CORS functionality:

1. **Add Origin header**: In Postman, go to Headers tab and add:
   - **Key**: `Origin`
   - **Value**: `https://yourdomain.com`
2. **Send the request**
3. **Check response headers** for:
   - `Access-Control-Allow-Origin`: Should match your domain or be `*`
   - `Access-Control-Allow-Methods`: Should include `GET, OPTIONS`

## Development

### Prerequisites

- Rust (latest stable version)
- Node.js and npm
- Wrangler CLI

### Local Development

1. Install dependencies:
   ```bash
   npm install -g @cloudflare/wrangler
   cargo install worker-build
   ```

2. Run locally:
   ```bash
   wrangler dev
   ```

### Environment Variables

#### CORS Configuration

The API supports configuring CORS (Cross-Origin Resource Sharing) allowed origins through environment variables for enhanced security:

- **`CORS_ALLOWED_ORIGINS`**: Comma-separated list of allowed origins for CORS requests
  - **Default**: `*` (allows all origins)
  - **Production Example**: `https://myapp.com,https://app.myapp.com,https://admin.myapp.com`
  - **Local Development**: `http://localhost:3000,http://localhost:8080`

#### Setting Environment Variables

##### Option 1: Using wrangler.toml (Recommended for Deploy to Cloudflare)

Create or update your `wrangler.toml` file:

```toml
[vars]
CORS_ALLOWED_ORIGINS = "https://myapp.com,https://app.myapp.com"
```

##### Option 2: Using Cloudflare Dashboard

1. Go to your Worker in the Cloudflare Dashboard
2. Navigate to **Settings** → **Environment Variables**
3. Add variable:
   - **Name**: `CORS_ALLOWED_ORIGINS`
   - **Value**: `https://myapp.com,https://app.myapp.com`

##### Option 3: Using Wrangler CLI

```bash
# Set environment variable
wrangler secret put CORS_ALLOWED_ORIGINS
# Enter value when prompted: https://myapp.com,https://app.myapp.com

# Or use vars for non-sensitive data
wrangler vars set CORS_ALLOWED_ORIGINS "https://myapp.com,https://app.myapp.com"
```

#### Security Best Practices

🔒 **Production Security Recommendations**:

1. **Never use `*` in production** - Always specify exact origins
2. **Use HTTPS only** - Only allow `https://` origins in production
3. **Include subdomains carefully** - Only add subdomains you control
4. **Regular audits** - Review allowed origins periodically

**Example Production Configuration**:
```
CORS_ALLOWED_ORIGINS=https://myapp.com,https://www.myapp.com,https://api.myapp.com
```

### Deployment

#### One-Click Deploy to Cloudflare

[![Deploy to Cloudflare](https://deploy.workers.cloudflare.com/button)](https://deploy.workers.cloudflare.com/?url=https://github.com/fernandodilland/cloudflare-workers-where-i-am-api)

When using the Deploy to Cloudflare button:

1. **Automatic Setup**: The button automatically configures the Worker with default settings
2. **Environment Variables**: You can customize `CORS_ALLOWED_ORIGINS` during deployment
3. **Default Security**: By default, it uses `*` for easy setup, but you should change this for production

**Post-Deployment Security Setup**:
1. After deployment, navigate to your Cloudflare Dashboard
2. Go to **Compute (Workers)** > **Select your Worker** > **Settings** > **Variables and Secrets**
3. Find the `CORS_ALLOWED_ORIGINS` variable and modify its value
4. Change it from `*` to your specific domain(s), for example: `https://mydomain.com,https://www.mydomain.com`
5. Click **Save** to apply the changes
6. **No restart required**: Changes to environment variables are applied automatically and take effect immediately
7. Test your application to ensure CORS is working correctly with your specific domains

#### Manual Deployment

```bash
wrangler deploy
```

## Custom Domain Configuration

You can use a custom domain or subdomain instead of the default `*.workers.dev` URL.

### Setting up a Custom Subdomain

1. **DNS Configuration** (in your domain's DNS settings):
   - Go to your domain's DNS management (Cloudflare DNS if using Cloudflare)
   - Add an **A record**:
     - **Name**: your-subdomain (e.g., `api`, `geo`, `location`)
     - **Content**: `192.0.2.1` (or any IP, will be proxied)
     - **Proxy**: ✅ **Enabled** (orange cloud icon)

2. **Worker Route Configuration**:
   - Go to your Worker in Cloudflare Dashboard
   - Navigate to **Settings** > **Domains & Routes**
   - Click **Add** > **Route**
   - Enter route pattern: `your-subdomain.yourdomain.com/*`
   - Select your Worker from the dropdown
   - Click **Save**

### Example Configuration

**DNS Record:**
```
Type: A
Name: api
Content: 192.0.2.1
Proxy: ✅ Enabled
```

**Worker Route:**
```
Route: api.yourdomain.com/*
Worker: your-worker-name
```

**Result:** Your API will be accessible at `https://api.yourdomain.com/`

### Benefits of Custom Domain

- **Professional appearance**: `https://api.yourdomain.com/` vs `https://worker-name.subdomain.workers.dev/`
- **Branding**: Use your own domain
- **SSL/TLS**: Automatic HTTPS with Cloudflare certificates
- **Better SEO**: Custom domains are preferred for production APIs

### Enabling All Geolocation Headers

⚠️ **Important**: To enable all Cloudflare geolocation headers, you must use a custom domain or subdomain (in a Cloudflare zone) and configure the following:

1. **Use a custom domain**: The domain must be managed by Cloudflare (added to your Cloudflare account)
2. **Enable visitor location headers**:
   - Go to your **Cloudflare Dashboard**
   - Select your **domain**
   - Navigate to **Rules** > **Settings**
   - Find the **"Add visitor location headers"** group
   - **Enable** this setting

**Note**: Worker URLs (`*.workers.dev`) may not provide all geolocation headers. For full functionality, use a custom domain with visitor location headers enabled.

**Headers available with custom domain + visitor location headers:**
- `CF-IPCountry`: Country code ✅
- `CF-IPCity`: City name ✅
- `CF-IPContinent`: Continent code ✅
- `CF-IPLongitude`: Longitude coordinate ✅
- `CF-IPLatitude`: Latitude coordinate ✅
- `CF-Region`: Region/state name ✅
- `CF-Region-Code`: Region/state code ✅
- `CF-Metro-Code`: Metro area code ✅
- `CF-Postal-Code`: Postal/ZIP code ✅
- `CF-Timezone`: Timezone ✅

## Cloudflare Headers

The API extracts the following Cloudflare geolocation headers:

- `CF-IPCountry`: Country code
- `CF-IPCity`: City name
- `CF-IPContinent`: Continent code
- `CF-IPLongitude`: Longitude coordinate
- `CF-IPLatitude`: Latitude coordinate
- `CF-Region`: Region/state name
- `CF-Region-Code`: Region/state code
- `CF-Metro-Code`: Metro area code
- `CF-Postal-Code`: Postal/ZIP code
- `CF-Timezone`: Timezone

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
