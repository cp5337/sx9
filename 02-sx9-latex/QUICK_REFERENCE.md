# RFC LaTeX Pipeline - Quick Reference

## On-Demand RFC Conversion

### Convert Single RFC to LaTeX

```bash
# Example: RFC-9301 (Ring Bus)
python3 sx9-conda/rfc_latex_generator.py --rfc 9301

# Output: 02-sx9-latex/output/rfc-9301.tex
```

### Extract Scholarly References from Zotero

```bash
# Search Zotero for relevant papers
python3 sx9-conda/extract_zotero_refs.py \
    --rfc 9301 \
    --concepts "ring bus" "delta angle" "matroid theory"

# Output: 02-sx9-latex/output/rfc-9301.bib
```

### Push to Overleaf

```bash
# Copy to Overleaf repo
cp 02-sx9-latex/output/rfc-9301.* overleaf-rfc/

# Push
cd overleaf-rfc
git add rfc-9301.*
git commit -m "Add RFC-9301 with references"
git push
```

## Workflow

1. **Identify RFC** needing formal documentation
2. **Generate LaTeX**: `rfc_latex_generator.py --rfc XXXX`
3. **Extract References**: `extract_zotero_refs.py --rfc XXXX --concepts "..."`
4. **Push to Overleaf**: Copy files and git push
5. **Compile PDF**: Overleaf auto-compiles
6. **Review & Iterate**: Edit in Overleaf, pull changes

## Common Use Cases

### For Test Harnesses

```bash
# Generate RFC LaTeX
python3 sx9-conda/rfc_latex_generator.py --rfc 9021

# Extract graph theory papers
python3 sx9-conda/extract_zotero_refs.py \
    --rfc 9021 \
    --concepts "graph convergence" "tutte polynomial" "matroid"

# Use citations in test documentation
```

### For EA Documentation

```bash
# Generate architecture RFCs
python3 sx9-conda/rfc_latex_generator.py --rfc 9116  # ECS

# Extract ECS papers
python3 sx9-conda/extract_zotero_refs.py \
    --rfc 9116 \
    --concepts "entity component system" "game architecture"
```

## Tools Created

1. **`rfc_latex_generator.py`** - RFC markdown → LaTeX
2. **`extract_zotero_refs.py`** - Zotero → BibTeX for citations
3. **`rfc_numbering_audit.py`** - Find duplicate RFCs
4. **`rfc_duplicate_comparison.py`** - Compare RFC versions

## Next Steps

- [ ] Create Overleaf project
- [ ] Add papers to Zotero library
- [ ] Test with RFC-9301 (Ring Bus)
- [ ] Set up automated workflow (GitHub Actions)
