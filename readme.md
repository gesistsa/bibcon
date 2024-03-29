# bibcon

The purpose of *bibcon* (BibTeX condenser) is to extract all references cited in Quarto / RMarkdown documents from a main BibTeX library to a "condensed" BibTeX file.

This is a fast (and functioning) reimplementation of [condensebib](https://github.com/andybega/condensebib) by Andreas Beger.

Please pay attention that this software is called *bibcon*, not *Bitcoin*.

# install

Please note that this software is still super experimental

```bash
## install cargo from here: https://doc.rust-lang.org/cargo/getting-started/installation.html
cargo install --git https://github.com/gesistsa/bibcon 
```

# usage

`weat.qmd` contains 5 unique citations, whereas `weat.bib` contains 6 entries. The following generates BibTeX of only 5 entries.

```bash
bibcon tests/weat.qmd -b tests/weat.bib
```

`r1.rmd` and `r2.rmd` contain 4 unique citations, whereas `main.bib` contains 12 entries.

```bash
bibcon -b tests/main.bib tests/r1.rmd tests/r2.rmd
```
