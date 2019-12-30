# Language model

## Introduction

## Prerequisites

-   Download the latest complete `cirrussearch` dump from https://dumps.wikimedia.org/other/cirrussearch/current/ for your language of interest.
    -   At the time of writing I used
        -   `enwiki-20191202-cirrussearch-content.json.gz`
        -   `frwiki-20191202-cirrussearch-content.json.gz`
        -   `dewiki-20191202-cirrussearch-content.json.gz`
    -   Historical dumps are available at https://dumps.wikimedia.org/other/cirrussearch/

## References

-   Hand-crafted word lists (recommended)
    -   http://wordlist.aspell.net/12dicts-readme/
    -   `3of6game` is used, and we strip suffixes as we don't care about British vs. American spellings. We trust that the top 100k words in Wikipedia land on some canonical spelling.
-   English word list
    -   https://packages.debian.org/sid/text/wamerican-insane
-   Lots of word lists
    -   https://packages.debian.org/sid/wordlist

### How to extract the DEB wordlist file on a Mac

-   Download the DEB file from a reference link above
-   Put it into its own dir

```
ar -x wamerican-insane_2018.04.16-1_all.deb
tar xvf data.tar.xz
```