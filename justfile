all:
    quarto preview
new POST:
    R -e "source('new.R'); .new_post('{{POST}}')"
refresh:
    quarto render index.qmd