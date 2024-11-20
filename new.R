# Create posts automatically
# Author: Shixiang Wang
# License: MIT
.new_post <- function(post_name, 
                     dir = file.path(getwd(), "posts"),
                     force = FALSE) {
  if (missing(post_name)) {
    stop("A post name must be given!")
  }
  
  post_name2 = paste0(Sys.Date(), "-", tolower(gsub(" ", "-", post_name)))
  path = file.path(dir, post_name2)
  message("Will generate ", path)
  if (dir.exists(path) & !force) {
    message("The path exists while force is FALSE. Exit...")
    return(invisible(NULL))
  }
  
  if (!dir.exists(path)) dir.create(path, recursive = TRUE)
  path_post = file.path(path, "index.qmd")
  
  contents = sprintf(
"---
title: %s
author: Shixiang Wang
date: %s
categories: [journal]
draft: false
description: ''

toc: true

format:
  html:
    code-fold: false
    code-overflow: wrap
    code-tools: false
---
", post_name, Sys.Date())
  
  xfun::write_utf8(contents, path_post)
  if (interactive()) file.edit(path_post)
  message("Start writing now!")
}
