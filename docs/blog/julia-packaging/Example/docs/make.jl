using Documenter
using Example

makedocs(
    sitename = "Example",
    format = Documenter.HTML(),
    modules = [Example]
)

# Documenter can also automatically deploy documentation to gh-pages.
# See "Hosting Documentation" and deploydocs() in the Documenter manual
# for more information.
#=deploydocs(
    repo = "<repository url>"
)=#
