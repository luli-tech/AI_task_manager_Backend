# Docker Deployment Guide for Render

This project includes an optimized Docker configuration for **60-80% faster builds** on Render through intelligent caching.

## Why Docker?

### Build Time Comparison

**Without Docker (Native Rust):**
```
First build: ~15-20 minutes
Subsequent builds: ~15-20 minutes (rebuilds everything)
```

**With Docker (Multi-stage with caching):**
```
First build: ~15-20 minutes (creates cache layers)
Subsequent builds: ~3-5 minutes (only rebuilds changed code) ⚡
```

### How It Works

The `Dockerfile` uses a **3-stage build process**:

1. **Stage 1 - Dependencies**: Compiles all Cargo dependencies (cached)
2. **Stage 2 - Application**: Builds your source code (fast, uses cached deps)
3. **Stage 3 - Runtime**: Creates minimal runtime image (~100MB)

**Key Optimization:**
- Dependencies are built in a separate layer
- When you change source code, only Stage 2 rebuilds
- Dependencies (Stage 1) are reused from cache
- **Result**: 60-80% faster builds!

## Setup on Render

### 1. Using render.yaml (Recommended)

The project includes a `render.yaml` file for Infrastructure as Code:

1. Go to https://dashboard.render.com
2. Click **"New +"** → **"Blueprint"**
3. Connect your GitHub repository
4. Render will automatically detect `render.yaml`
5. Click **"Apply"**
6. Set required secrets in the dashboard:
   - `GOOGLE_CLIENT_ID`
   - `GOOGLE_CLIENT_SECRET`
   - `GOOGLE_REDIRECT_URI`

**That's it!** Render will:
- Create the web service with Docker runtime
- Create the PostgreSQL database
- Link them together
- Enable automatic deployments

### 2. Manual Setup

If you prefer manual configuration:

1. **Create PostgreSQL Database:**
   - Dashboard → New + → PostgreSQL
   - Name: `task-manager-db`
   - Plan: Free (or paid for production)
   - Copy the Internal Database URL

2. **Create Web Service:**
   - Dashboard → New + → Web Service
   - Connect your repository
   - Configure:
     ```
     Name: task-manager-api
     Runtime: Docker
     Branch: main
     Dockerfile Path: ./Dockerfile
     Docker Context: .
     ```

3. **Set Environment Variables:**
   ```
   DATABASE_URL=<from-postgres-service>
   JWT_SECRET=<generate-strong-secret>
   JWT_EXPIRATION_HOURS=24
   GOOGLE_CLIENT_ID=<your-client-id>
   GOOGLE_CLIENT_SECRET=<your-client-secret>
   GOOGLE_REDIRECT_URI=https://your-app.onrender.com/api/auth/google/callback
   HOST=0.0.0.0
   PORT=10000
   RUST_LOG=info,task_manager=info
   ```

## Understanding the Dockerfile

### Stage 1: Dependencies (Cached Layer)
```dockerfile
FROM rust:1.75-slim as dependencies
# Copies only Cargo.toml and Cargo.lock
# Builds a dummy project to compile dependencies
# This layer is cached until dependencies change
```

### Stage 2: Application Build
```dockerfile
FROM rust:1.75-slim as builder
# Copies cached dependencies from Stage 1
# Copies actual source code
# Builds only the application code (fast!)
```

### Stage 3: Runtime Image
```dockerfile
FROM debian:bookworm-slim
# Copies only the compiled binary
# Minimal image size (~100MB)
# No build tools, just runtime dependencies
```

## Build Caching Behavior

### When Dependencies Change (Cargo.toml/Cargo.lock):
```
✅ Stage 1: REBUILD (compile new dependencies)
✅ Stage 2: REBUILD (use new dependencies)
✅ Stage 3: REBUILD (new binary)
Time: ~15-20 minutes
```

### When Only Source Code Changes:
```
✅ Stage 1: CACHED (reuse compiled dependencies)
✅ Stage 2: REBUILD (compile changed code only)
✅ Stage 3: REBUILD (new binary)
Time: ~3-5 minutes ⚡
```

### When Nothing Changes:
```
✅ Stage 1: CACHED
✅ Stage 2: CACHED
✅ Stage 3: CACHED
Time: ~1-2 minutes (just deployment)
```

## Monitoring Build Performance

In Render logs, you'll see:

**Cache Hit:**
```
---> Using cache
---> 8f3a2b1c4d5e
```

**Cache Miss:**
```
---> Running in 8f3a2b1c4d5e
```

## Files Included

### `Dockerfile`
Multi-stage build configuration with dependency caching

### `.dockerignore`
Excludes unnecessary files from Docker build context:
- `.git/` - Git history
- `target/` - Local build artifacts
- `.env` - Environment files
- Documentation files

Smaller build context = faster uploads to Render

### `render.yaml`
Infrastructure as Code configuration:
- Web service with Docker runtime
- PostgreSQL database
- Environment variables
- Build filters for smart rebuilds

## Optimization Tips

### 1. Build Filters
The `render.yaml` includes build filters:
```yaml
buildFilter:
  paths:
    - src/**
    - Cargo.toml
    - Cargo.lock
    - migrations/**
```

**Benefit**: Render only rebuilds when these files change, not on README updates, etc.

### 2. Smaller Base Images
Using `slim` variants reduces image size:
- `rust:1.75-slim` instead of `rust:1.75`
- `debian:bookworm-slim` for runtime

### 3. Layer Ordering
Dockerfile layers are ordered by change frequency:
1. System dependencies (rarely change)
2. Cargo dependencies (change occasionally)
3. Source code (change frequently)

## Troubleshooting

### Build Takes Long Time
- **First build**: Normal, creating cache layers
- **Subsequent builds still slow**: Check if `Cargo.lock` is changing
- **Solution**: Commit `Cargo.lock` to git

### Out of Memory During Build
- **Cause**: Compiling Rust can be memory-intensive
- **Solution**: Upgrade to Starter plan ($7/month) for more RAM

### Cache Not Working
- **Check**: Is `Cargo.lock` committed to git?
- **Check**: Did you change `Cargo.toml`?
- **Solution**: Render caches Docker layers automatically

## Deployment Flow

```
1. Push code to GitHub
   ↓
2. GitHub Actions runs tests (with caching)
   ↓
3. Tests pass ✅
   ↓
4. Render detects new commit
   ↓
5. Render pulls code
   ↓
6. Docker build starts:
   - Stage 1: Use cached dependencies ⚡
   - Stage 2: Build changed code
   - Stage 3: Create runtime image
   ↓
7. Deploy new version (zero downtime)
   ↓
8. Health check passes ✅
   ↓
9. Old version replaced
```

## Cost Optimization

### Free Tier
- Web Service: 750 hours/month
- PostgreSQL: 90 days free trial
- **Limitation**: Sleeps after 15min inactivity
- **Build time**: ~3-5 minutes with caching

### Starter Plan ($7/month)
- No sleep
- Faster builds (more CPU/RAM)
- **Build time**: ~2-3 minutes with caching

## Comparing Deployment Methods

| Feature | Docker | Native Rust |
|---------|--------|-------------|
| First Build | ~15-20 min | ~15-20 min |
| Rebuild (code change) | ~3-5 min ⚡ | ~15-20 min |
| Rebuild (deps change) | ~15-20 min | ~15-20 min |
| Image Size | ~100 MB | ~2 GB |
| Cache Efficiency | Excellent | Good |
| Setup Complexity | Easy | Easy |

**Recommendation**: Use Docker for significantly faster deployments!

## Next Steps

1. ✅ Files are already created (`Dockerfile`, `.dockerignore`, `render.yaml`)
2. Push to GitHub
3. Follow setup instructions above
4. Enjoy faster builds! 🚀

## Support

- **Render Docs**: https://render.com/docs/docker
- **Docker Best Practices**: https://docs.docker.com/develop/dev-best-practices/
