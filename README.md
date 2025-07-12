# Cloudflare Workers Rust - Where Am I API

🦀 A high-performance Rust-based Cloudflare Worker that provides geolocation information using Cloudflare's edge headers.

[![Deploy to Cloudflare](https://deploy.workers.cloudflare.com/button)](https://deploy.workers.cloudflare.com/?url=https://github.com/fernandodilland/cloudflare-workers-where-i-am-api)

## Features

- **⚡ High Performance**: Built with Rust for maximum speed
- **🪶 Lightweight**: Compiled to WebAssembly for minimal size
- **🌐 CORS Ready**: Configurable cross-origin request support
- **📍 Geolocation**: Extracts location data from Cloudflare headers
- **🔒 Secure**: Production-ready with configurable CORS origins
- **🚀 Easy Deploy**: Multiple deployment options available

## Quick Start

### API Endpoint

**`GET /`** - Returns geolocation information based on your IP address

```bash
curl https://your-worker-name.your-subdomain.workers.dev/
```

**Response:**
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

### Testing with Postman

1. Create a **GET** request to your Worker URL
2. **Optional Headers** for CORS testing:
   - `Origin`: `https://yourdomain.com`
   - `Content-Type`: `application/json`
3. **Expected Response**: `200 OK` with geolocation JSON data

## Deployment

### 🚀 Quick Deploy (Recommended)

**Option 1: Automated Scripts**
```bash
# Windows
./deploy.bat

# Linux/Mac
./deploy.sh
```

**Option 2: Manual Commands**
```bash
cargo install worker-build
worker-build --release
wrangler deploy
```

### 🤖 GitHub Actions (Production)

Automated deployment on every push to `main`:

1. **Repository Settings** → **Secrets and Variables** → **Actions**
2. **Add Secret**: `CLOUDFLARE_API_TOKEN` ([Get token here](https://dash.cloudflare.com/profile/api-tokens))
3. **Push to main** → Auto-deploy triggered

### � Troubleshooting

| Error | Solution |
|-------|----------|
| `cargo: not found` | Use deployment scripts or GitHub Actions |
| `worker-build: not found` | Run `cargo install worker-build` |
| `wrangler: not found` | Run `npm install -g @cloudflare/wrangler` |

## Configuration

### Environment Variables

Configure CORS origins for enhanced security:

```toml
# wrangler.toml
[vars]
CORS_ALLOWED_ORIGINS = "https://myapp.com,https://api.myapp.com"
```

**Security Best Practices:**
- ❌ Never use `*` in production
- ✅ Use HTTPS-only origins
- ✅ Specify exact domains
- ✅ Regular security audits

### Custom Domain Setup

For full geolocation header support, use a custom domain:

1. **DNS Configuration**:
   ```
   Type: A
   Name: api
   Content: 192.0.2.1
   Proxy: ✅ Enabled
   ```

2. **Cloudflare Dashboard**:
   - **Workers** → **Your Worker** → **Settings** → **Domains & Routes**
   - **Add Route**: `api.yourdomain.com/*`

3. **Enable Geolocation Headers**:
   - **Dashboard** → **Your Domain** → **Rules** → **Settings**
   - **Enable**: "Add visitor location headers"

## Development

### Prerequisites

- [Rust](https://rustup.rs/) (latest stable)
- [Node.js](https://nodejs.org/) and npm
- [Wrangler CLI](https://developers.cloudflare.com/workers/wrangler/)

### Local Development

```bash
# Install dependencies
npm install -g @cloudflare/wrangler
cargo install worker-build

# Run locally
wrangler dev
```

## Available Headers

The API extracts these Cloudflare geolocation headers:

| Header | Description |
|--------|-------------|
| `CF-IPCountry` | Country code (US, CA, etc.) |
| `CF-IPCity` | City name |
| `CF-IPContinent` | Continent code (NA, EU, etc.) |
| `CF-IPLongitude` | Longitude coordinate |
| `CF-IPLatitude` | Latitude coordinate |
| `CF-Region` | Region/state name |
| `CF-Region-Code` | Region/state code |
| `CF-Metro-Code` | Metro area code |
| `CF-Postal-Code` | Postal/ZIP code |
| `CF-Timezone` | Timezone identifier |

**Note**: Full header availability requires a custom domain with visitor location headers enabled.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
