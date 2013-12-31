Rust tutorials_NG
=================

A community initiative to write a brand-new, feature-complete and easy to understand tutorial for the [Rust programming language](http://www.rust-lang.org/).

[Check it out!](http://adridu59.github.io/rust-tuts/)

Diving in..
-----------

The documentation system is powered by [Pandoc](http://johnmacfarlane.net/pandoc/) in order to build HTML5 and LaTeX output.

Want to contribute? Fork it and start editing the Markdown (.md) files.

Note: you can create files under the `drafts/` folder and build them with `make drafts`, the source files will be ignored by git.

Want to build it? Dependencies:
- WEB: `pandoc`
- TEX: `lualatex`, [Noticia Text](http://www.fontsquirrel.com/fonts/noticia-text) (ttf) + [TeX Gyre Heros](http://www.gust.org.pl/projects/e-foundry/tex-gyre/heros) (otf) + [DejaVu Sans Mono](http://dejavu-fonts.org/wiki/Download) (ttf) extracted under the `font/` folder (can be spoofed with minor changes)

Want to look at the unit tests? They are located under the code/ folder.

Note: the supplied `run-tests.sh` will check if all test files pass with your local Rust compiler.