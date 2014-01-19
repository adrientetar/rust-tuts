# Flags declarations

BASE_DOC_OPTS := --from=markdown --smart
HTML_OPTS = $(BASE_DOC_OPTS) --to=html5 --css=../css/rust.css --section-divs --template=template/template.html
TEX_OPTS = $(BASE_DOC_OPTS) --to=latex --standalone --number-sections --latex-engine=lualatex --template=template/template.tex
EPUB_OPTS = $(BASE_DOC_OPTS) --to=epub
WEB :=
TEX :=
DRAFTS :=

# http://lincolnmullen.com/blog/make-and-pandoc/
# http://stackoverflow.com/a/9934724

WEB += $(patsubst source/%.md,tutorial/%.html,$(wildcard source/*.md))
tutorial/%.html: source/%.md
	pandoc -o $@ $(HTML_OPTS) $<

WEB += tutorial/tutorial.html
tutorial/tutorial.html: template/title.md $(wildcard source/ch-*.md)
	pandoc -o $@ $(HTML_OPTS) $^

TEX += tutorial/tutorial.tex
tutorial/tutorial.tex: template/title-TeX.md $(wildcard source/ch-*.md)
	pandoc -o $@ $(TEX_OPTS) $^

TEX += tutorial/tutorial.pdf
tutorial/tutorial.pdf: tutorial/tutorial.tex
	lualatex -output-directory=tutorial $<

DRAFTS += $(patsubst drafts/%.md,tutorial/%.html,$(wildcard drafts/*.md))
tutorial/%.html: drafts/%.md
	pandoc -o $@ $(HTML_OPTS) $<

# Rules

.PHONY: clean
clean:
	rm -f tutorial/*
tclean:
	rm -f tutorial/*.aux tutorial/*.log tutorial/*.out
dclean:
	rm -f $(DRAFTS)

docs: $(WEB)
tex: $(TEX) tclean
all: docs tex
drafts: $(DRAFTS)