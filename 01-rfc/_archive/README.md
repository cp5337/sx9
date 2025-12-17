# RFC Archive

This directory contains non-canonical RFC variants and supporting documentation that were moved out of the main RFC directory structure to reduce confusion.

## Archive Structure

### `gateway-variants/`

**Moved:** December 14, 2025  
**Reason:** Multiple duplicate versions of RFC-9114 (SX9 Gateway)

**Canonical Version:** `/01-rfc/integration/RFC-9114-SX9-Gateway-Neural-Retrofit.md` (Rev 1.1)

**Archived Variants:**

- `RFC-9114-REV1.1-ANALYSIS.md` - Analysis document
- `RFC-9114-Rev1.1-SX9-Gateway-Neural-Retrofit-COMPLETE.md` - Duplicate
- `RFC-9114-Rev1.1-SX9-Gateway-Neural-Retrofit-ORIGINAL.md` - Original draft
- `RFC-9114-Rev1.1-SX9-Gateway-Neural-Retrofit.md` - Duplicate
- `RFC-9114-SX9-Gateway-Architecture.md` - Earlier version

### `etl-prep-docs/`

**Moved:** December 14, 2025  
**Reason:** Supporting documentation for ETL implementation, not canonical RFCs

**Contents:**

- `cloudflare_deployment/` - Cloudflare R2 implementation guides
  - `R2_CDN_SUBSCRIBER_SERVICE.md`
  - `CHROMADB_GATEWAY_INTEGRATION.md`
  - `Cloud Flare Service Enhancement.md`
- `SYNAPTIX-UNIFIED-ARCHITECTURE-ADDENDUM.md`
- `SYNAPTIX-MASTER-ARCHITECTURE-INDEX.md`
- `SYNAPTIX-UNIFIED-ARCHITECTURE-RFC-COMPLIANT.md`

**Note:** The information from these documents has been incorporated into:

- RFC-9005 v1.2 (Unified Schema with R2 CDN)
- RFC-9105 v1.1 (SPIRES Extraction with R2 uploader)
- Unified Implementation Plan artifact

## Canonical RFC Location

All canonical RFCs are located in:

```
/01-rfc/
├── core/          # RFC-9000 to RFC-9009
├── integration/   # RFC-9100 to RFC-9149
├── cognitive/     # RFC-9300 to RFC-9399
├── application/   # RFC-9400 to RFC-9499
├── platform/      # RFC-9500 to RFC-9599
└── operational/   # RFC-9800 to RFC-9899
```

Refer to `/01-rfc/REGISTRY.md` for the complete RFC index.

---

**Cleanup performed by:** Antigravity AI  
**Date:** 2025-12-14
