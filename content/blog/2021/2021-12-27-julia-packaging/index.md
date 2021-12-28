---
title: 如何用Julia语言创建软件包
author: 王诗翔
date: '2021-12-27'
slug: julia-packaging
categories:
  - Blog
tags:
  - julia
  - development
description: Julia与软件包开发
---

>源：<https://jaantollander.com/post/how-to-create-software-packages-with-julia-language/>

![](https://jaantollander.com/post/how-to-create-software-packages-with-julia-language/featured.png)

## 介绍

本文将教你如何用Julia编程语言创建一个开源的软件包，并使用基于git的工作流开发软件包。例如，你将了解如何自动化单元测试和文档部署，以及发布包的新版本。此外，我们创建了[Julia播放列表](https://www.youtube.com/watch?v=KdfFN02PuFo&list=PLFi8CfyKbgDFYCba471yBhrGc3n2Pd1fu)的逐步视频教程，以指导你通过这个过程。

## 安装Julia

首先，我们将从[julialang网站](https://julialang.org/)的下载页面安装Julia编程语言。在Linux中，我们可以将存档解压缩到所需的位置。我们将使用`~/software/`目录。

`~/.bashrc`添加配置如下：

```sh
export PATH="$PATH:$HOME/software/julia-1.5.3-linux-x86_64/julia-1.5.3/bin"
```

上面的版本根据你自己的实际情况进行修改。

## Julia REPL

>[How to use Julia REPL for Developing Packages](https://www.youtube.com/watch?v=fu21WXi726A&list=PLFi8CfyKbgDFYCba471yBhrGc3n2Pd1fu)

我们可以通过在命令行输入Julia来打开Julia REPL。Julia REPL有四种不同的模式：

1. Julia模式`julia>`用于测试Julia代码。
2. 包管理模式`pkg>`用于执行包管理命令。可以使用`]`进行激活。
3. 帮助模式`help?>`用于打印帮助和文档。我们可以使用`?`进行激活。
4. Shell模式`shell>`用于执行shell命令。我们可以使用分号`;`进行激活。

我们可以使用回车符从其他模式退回到Julia模式。

## 包结构

我们的包结构将遵循使用Julia语言创建软件包的官方示例。我们可以在[Example.jl](https://github.com/JuliaLang/Example.jl/)中找到示例库。我们可以克隆示例库并对其进行研究。以点开头的目录可能被操作系统隐藏了。我们可以从文件系统设置中显示隐藏的文件。Julia包结构如下所示：

```sh
Example/
├─ .git/
├─ .github/
│  └─ workflows/
│     ├─ TagBot.yml
│     └─ ci.yml
├─ docs/
│  ├─ src/
│  │  └─ index.md
│  ├─ Project.toml
│  └─ make.jl
├─ src/
│  └─ Example.jl
├─ test/
│  └─ runtests.jl
├─ .appveyor.yml
├─ .codecov.yml
├─ .gitignore
├─ LICENSE
├─ Project.toml
└─ README.md
```

## 生成包

>[How to Create Software Packages with Julia Language](https://www.youtube.com/watch?v=KdfFN02PuFo&list=PLFi8CfyKbgDFYCba471yBhrGc3n2Pd1fu)

我们可以使用Julia REPL中的包管理器生成一个新的Julia包。

```julia
pkg> generate Example
```

使用`generate`命令创建`src/Example.jl`和`Project.toml`文件。`src`目录包含软件的源代码。`Project.toml`文件是一个项目详细信息的配置文件，例如名称、UUID、作者、版本、依赖关系和兼容性。附加项和目标为单元测试定义依赖项。

```toml
name = "Example"
uuid = "..."
authors = ["author <email>"]
version = "0.1.0"

[deps]
# <package> = ...

[compat]
julia = "1"

[extras]
Test = "8dfed614-e22c-5e08-85e1-65c5234f0b40"

[targets]
test = ["Test"]
```

我们使用`add`为包添加新的依赖包：

```julia
pkg> activate .
(Example) pkg> add <package>
```

所有的命令文档在[Pkg.jl](https://julialang.github.io/Pkg.jl/)。

## 单元测试

`test/`目录下包含一个Julia包所有用于单元测试的代码。我们可以使用[Test.jl](https://docs.julialang.org/en/v1/stdlib/Test/index.html)模块来创建测试。
下面是存放的测试内容：

```julia
using Test
@test [1, 2] + [2, 1] == [3, 3]
@test_throws BoundsError [1, 2, 3][4]
```

我们可以在包管理模式下使用`test`命令运行单元测试：

```julia
pkg> activate .
(Example) pkg> test
```

稍后，我们设置了GitHub动作来自动执行测试，作为构建过程的一部分。

## 文档

`docs/`目录包含Julia包的文档。[Documenter.jl](https://github.com/JuliaDocs/Documenter.jl)是Julia项目的一个文档包。在生成的`Example`目录下，我们可以使用[DocumenterTools.jl](https://github.com/JuliaDocs/DocumenterTools.jl)初始化文档：

```julia
pkg> add DocumenterTools
julia> using DocumenterTools
julia> DocumenterTools.generate()
```

我们使用Markdown（一种轻量级标记语言）将文档写入`docs/src/`目录。我们还可以使用文档字符串记录代码，如下所示。

```julia
"""A foo function.

# Arguments
- `arg::Int`: Function argument.
"""
function foo(arg::Int)
    # ...
end
```

`Documenter.jl`可以从文档字符串中生成下面的API文档：

~~~
```@docs
foo(::Int)
```
~~~

我们可以使用语法来引用API文档。

~~~
[`foo`](@ref)
~~~

然后通过执行`docs/make.jl`文件来构建文档。

```sh
julia make.jl
```

这将在`docs/build/`目录下生成文档。我们可以通过一个浏览器打开`docs/build/index.html`文件进行预览。稍后，在GitHub部分，我们解释了如何设置GitHub的动作，以将文档作为网页自动发布到GitHub页面。

## 创建Git仓库

在`Example`目录下我们可以使用下面Git命令初始化Git仓库：

```sh
git init
```

该命令创建了`.git/`目录，其中存储了项目的更改历史。我们还应该创建`.gitignore`文件，它告诉Git忽略更改历史中的特定文件，比如二进制文件和构建文件。我们应该从[gitignore](https://github.com/github/gitignore/)模板库中复制相应的模板。对于一个Julia项目，我们复制`Julia.gitignore`文件。

## GitHub仓库

接下来我们导航到GitHub网站并创建一个名为`Example.jl`的仓库。使用`.jl`拓展是命名Julia包的一个传统。

然后，我们添加远程仓库作为我们项目的源头。

```sh
git remote add origin <remote-repository-url>
```

最后，我们应该把本地仓库推送到远程仓库：

```sh
git push origin master
```

## GitHub页面和动作

>[How to Setup GitHub Actions for a Julia Package](https://www.youtube.com/watch?v=Vi4Ntd_Vf4A&list=PLFi8CfyKbgDFYCba471yBhrGc3n2Pd1fu)

我们可以使用GitHub的Actions和Pages特性来运行单元测试，并在web上自动部署文档。我们可以通过将`.github/workflow/`目录添加到我们的存储库中，并在其中创建配置文件来激活GitHub Actions。我们已经将这个实例的`ci.yml`文件分离到`Runtests.yml`和`Documenter.yml`文件来独立运行单元测试和文档。

下面是`Runtests.yml`文件内容：

```yaml
name: Runtests
on: [push, pull_request]
jobs:
  test:
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        julia-version: ['1.4']
        julia-arch: [x64]
        os: [ubuntu-latest] # [ubuntu-latest, windows-latest, macOS-latest]
    steps:
      - uses: actions/checkout@v2
      - uses: julia-actions/setup-julia@latest
        with:
          version: ${{ matrix.julia-version }}
      - uses: julia-actions/julia-buildpkg@latest
      - uses: julia-actions/julia-runtest@latest
```

`Documenter.yml`文件的内容：

```yaml
name: Documenter
on:
  push:
    branches:
      - master
    tags: '*'
  pull_request:
jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - uses: julia-actions/setup-julia@latest
        with:
          version: '1.4'
      - name: Install dependencies
        run: julia --project=docs/ -e 'using Pkg; Pkg.develop(PackageSpec(path=pwd())); Pkg.instantiate()'
      - name: Build and deploy
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }} # For authentication with GitHub Actions token
          DOCUMENTER_KEY: ${{ secrets.DOCUMENTER_KEY }} # For authentication with SSH deploy key
        run: julia --project=docs/ docs/make.jl
```

接下来，我们必须设置GITHUB_TOKEN和DOCUMENTER_KEY的秘值，如[托管文档](https://juliadocs.github.io/Documenter.jl/stable/man/hosting/index.html)部分所解释的那样。首先，在Julia REPL上执行以下命令。

```julia
pkg> activate .
julia> using DocumenterTools
julia> DocumenterTools.genkeys()
```

然后，按照输出的说明进行操作，如下所示:

```
[ Info: add the public key below to https://github.com/$USER/$REPO/settings/keys with read/write access:

[SSH PUBLIC KEY HERE]

[ Info: add a secure environment variable named 'DOCUMENTER_KEY' to https://travis-ci.com/$USER/$REPO/settings (if you deploy using Travis CI) or https://github.com/$USER/$REPO/settings/secrets (if you deploy using GitHub Actions) with value:

[LONG BASE64 ENCODED PRIVATE KEY]
```

Documenter动作将生成`make.jl`的输出到仓库中的gh-pages分支，它形成了GitHub页面的内容。

## 版本控制和发布

### 语义版本控制

软件版本控制对于标识软件的新开发和确保与其他软件的兼容性至关重要。Julia打包遵循语义版本控制的约定，我们以`major.minor.batch`形式更新版本号。我们建议阅读[semver.org](https://semver.org/)网站上关于语义版本的详细描述和说明。

### 包注册登记

在软件工程中，发布指的是发布软件包的新版本。我们可以将Julia包发布到[Julia包注册中心](https://github.com/JuliaRegistries/General)。Julia包注册中心还维护GitHub的注册器应用程序和TagBot GitHub动作，我们可以使用它来自动化发布过程。

我们按下安装应用程序按钮安装[Registrator.jl](https://github.com/JuliaRegistries/Registrator.jl)GitHub应用程序，并选择我们的存储库。

### 标记版本

我们继续将[TagBot](https://github.com/JuliaRegistries/TagBot)GitHub动作添加到我们的库中。TagBot自动标记新版本的发布。我们可以通过添加`TagBot.yml`到`.github/workflows/`来设置它。

```yaml
name: TagBot
on:
  issue_comment:
    types:
      - created
  workflow_dispatch:
jobs:
  TagBot:
    if: github.event_name == 'workflow_dispatch' || github.actor == 'JuliaTagBot'
    runs-on: ubuntu-latest
    steps:
      - uses: JuliaRegistries/TagBot@v1
        with:
          token: ${{ secrets.GITHUB_TOKEN }}
          ssh: ${{ secrets.DOCUMENTER_KEY }}
```

### 发布过程

在设置了Registrator应用和TagBot动作之后，我们可以继续进行发布过程，我们可以使用它来发布我们的包的所有未来版本。

在发布新版本之前，我们应该更新项目`Project.toml`中所有依赖项`[deps]`的兼容性`[compat]`，并检查自动化单元测试是否通过，文档的构建是否没有错误。

要发布一个新版本，我们需要使用语义版本原则更新`Project.toml`中的版本字符串。

```toml
name = "Example"
uuid = "..."
authors = ["author <email>"]
version = "0.1.0"  # update the version string!
...
```

然后，我们可以将更新后的版本字符串添加到Git。

```sh
git add Project.toml
```

最后，我们需要对提交进行如下评论。

```sh
git commit -m "@JuliaRegistrator register" 
```

一旦我们推送了提交，注册器应用程序注册了一个新版本的包，TagBot动作用新版本字符串标记提交。TagBot动作也会触发新包版本的文档构建。

## readme

`README.md`文件的目标是指导人们安装程序，并展示一个简单的使用示例。自述文件应该包含以下部分:

- 简短的描述解释了项目的目的和功能。或者，我们可以包含徽章来指示测试成功、代码覆盖率和构建状态。我们可以使用[Shields IO](https://shields.io/)制作任意徽章。
- 使用实例为开始使用项目提供了一个清晰和直接的例子。
- 安装说明提供了关于如何安装项目的明确说明。
- 开发说明提供了关于如何开始开发项目的说明。它们应该包括如何运行测试和构建文档。

## 许可

软件许可证用来传递用户使用和分发软件包的权限。包在license文本文件中包含他的许可证。我们应该总是在项目中包含许可。我们建议使用[开源许可](https://opensource.org/licenses)，例如MIT许可。如果计划用商业应用程序创建开源软件，那么应该阅读更多关于许可的内容，特别是来自GNU许可的GPL和AGPL等copyleft许可。

## 使用PkgTemplates.jl

一旦您理解了Julia包的基本组件，你就可以通过使用[invenia/PkgTemplates](https://github.com/invenia/PkgTemplates.jl)将打包过程提升到下一个层次。使用PkgTemplates，我们可以通过编写和执行一个Julia脚本来生成包含本文中讨论的所有包组件的Julia包。下面是一个小示例脚本，用于创建MyPackage与最小的Julia版本1.3.0，MIT许可证，SSH的Git库，以及用于单元测试和文档器的GitHub操作：

```
using PkgTemplates
t = Template(
    user="jaantollander",
    dir=".",
    julia=v"1.3.0",
    plugins=[
        License(; name="MIT"),
        Git(; ssh=true),
        GitHubActions(),
        Documenter{GitHubActions}(),
    ]
)
t("MyPackage")
```

该工具是自Julia 1.6以来创建包的默认方式，我们强烈推荐使用它!

## 总结

现在，你可以开始创建和发布自己的Julia软件包，为自己赢得Julia开发人员的头衔。Go and get after it!
