# Overleaf Integration for RFC LaTeX Pipeline

## Setup

1. **Create Overleaf Project**: https://www.overleaf.com/project
2. **Get Git URL**: Project → Menu → Git → Copy URL
3. **Clone locally**:
   ```bash
   cd 02-sx9-latex
   git clone https://git.overleaf.com/your-project-id overleaf-rfc
   ```

## Workflow

### Push RFC to Overleaf

```bash
# Generate LaTeX
python3 sx9-conda/rfc_latex_generator.py --rfc 9006

# Copy to Overleaf repo
cp 02-sx9-latex/output/rfc-9006.tex 02-sx9-latex/overleaf-rfc/

# Push to Overleaf
cd 02-sx9-latex/overleaf-rfc
git add rfc-9006.tex
git commit -m "Add RFC-9006"
git push origin master
```

### Batch Export All RFCs

```bash
# Export all RFCs
for rfc in 9006 9007 9008 9009 9020; do
    python3 sx9-conda/rfc_latex_generator.py --rfc $rfc
done

# Push to Overleaf
cp 02-sx9-latex/output/*.tex 02-sx9-latex/overleaf-rfc/
cd 02-sx9-latex/overleaf-rfc
git add *.tex
git commit -m "Batch export RFCs"
git push
```

## Overleaf Features

- **Real-time compilation**: PDFs generated automatically
- **Collaboration**: Share with team
- **Version control**: Git integration
- **Templates**: DoD/IEEE templates available
- **BibTeX**: Zotero integration via BibTeX export

## Next Steps

1. Create Overleaf project
2. Share Git URL
3. Set up automated push workflow
