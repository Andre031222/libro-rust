# Compila (xelatex + pdftocairo) cada figuras/src/*.tex (salvo paleta.tex) a PDF y SVG.
FUENTES := $(filter-out src/paleta.tex,$(wildcard src/*.tex))
PDFS := $(patsubst src/%.tex,%.pdf,$(FUENTES))
SVGS := $(PDFS:.pdf=.svg)

all: $(PDFS) $(SVGS)

%.pdf: src/%.tex src/paleta.tex
	cd src && xelatex -interaction=nonstopmode -halt-on-error $*.tex >/dev/null
	mv src/$*.pdf $@
	rm -f src/$*.aux src/$*.log

%.svg: %.pdf
	pdftocairo -svg $< $@

clean:
	rm -f $(PDFS) $(SVGS)

.PHONY: all clean
