# SX9 DOMAIN INVENTORY

## Owned Domains

### Hostinger
| Domain | Purpose | DNS | Status |
|--------|---------|-----|--------|
| `sx9.io` | Primary brand | Hostinger | ✓ Active |
| `synaptix9.com` | Enterprise/Partner | Hostinger | ✓ Active |
| `synaptix9.io` | Alternate | Hostinger | Verify |

### Other Registrars
| Domain | Registrar | Purpose | Status |
|--------|-----------|---------|--------|
| `devstackone.dev` | ? | Dev/staging | ✓ Active |
| `dityauto.???` | GoDaddy | Obfuscated ops | Verify TLD |
| `autorepair-tech.net` | GoDaddy | Obfuscated ops | ✓ Active |
| `autorepair-tech.info` | GoDaddy | Obfuscated ops | Verify |

## Cloudflare Account
- [ ] Verify account access
- [ ] List existing zones
- [ ] R2 bucket availability

## Recommended Subdomain Allocation

### sx9.io (Primary)
```
sx9.io                    → Main portal / landing
cdn.sx9.io                → Cloudflare R2 (public assets)
api.sx9.io                → API gateway
docs.sx9.io               → Documentation
status.sx9.io             → Status page
```

### synaptix9.com (Partner/Enterprise)
```
synaptix9.com             → Partner portal
crates.synaptix9.com      → Foundation crate registry (GCP CDN)
partner.synaptix9.com     → Partner CTAS Main access
support.synaptix9.com     → Support portal
```

### devstackone.dev (Development)
```
devstackone.dev           → Dev portal
staging.devstackone.dev   → Staging environment
preview.devstackone.dev   → PR previews
```

### Obfuscated (Ops)
```
dityauto.???              → Kali Plasma update repo
autorepair-tech.net       → Tunnel endpoint
autorepair-tech.info      → Backup tunnel
```

## Action Items
- [ ] Verify all domain ownership
- [ ] Consolidate DNS to Cloudflare where practical
- [ ] Set up R2 buckets
- [ ] Configure CDN origins
- [ ] SSL certificates (Cloudflare auto)




