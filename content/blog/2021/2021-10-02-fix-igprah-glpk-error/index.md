---
title: 解决igraph使用optimap_函数报错：GLPK is not available, Unimplemented function call
author: R package build
date: '2021-10-02'
slug: fix-igprah-glpk-error
categories:
  - Blog
tags:
  - R
  - igraph
description: 有错就要解决
---

在使用igraph的测试用例时，发生GLPK相关的报错：

```R
> g <- make_graph("Zachary")
> oc <- cluster_optimal(g)
Error in cluster_optimal(g) : 
  At optimal_modularity.c:85 : GLPK is not available, Unimplemented function call
```

GitHub的帖子[#273](https://github.com/igraph/rigraph/issues/273)对该问题进行了一些
积极的讨论，不过主要集中在MacOS系统上。而我要解决的是CentOS上的问题。

不过原理相通，加上`cluster_optimal`函数文档的描述，大体知道了CRAN不允许igraph团队
内置该库，所以从1.2.1版本后就移除了，因此需要安装包之前在相关系统上安装好该库，
这样该包安装的时候就能够编译相应的函数。否则，相应的函数使用就会报错。

一种解决的思路就是安装之前的版本，我尝试了下，发现一些编译报错。可能是旧代码存在一些
bug吧，所以只能用最新版本。

这样需要先用root权限安装库：

```sh
yum install glpk glpk-devel
```

然后再安装：

```R
install.packages("igraph")
```

安装时间会比较长。

如果仔细观察的话，会发现g++的命令中会指定加入`-lglpk`选项用于加入相关的库进行编译。

```R
g++ -m64 -std=gnu++11 -shared -L/usr/lib64/R/lib -Wl,-z,relro -o igraph.so AMD/Source/amd.o AMD/Source/amd_1.o AMD/Source/amd_2.o AMD/Source/amd_aat.o AMD/Source/amd_control.o AMD/Source/amd_defaults.o AMD/Source/amd_dump.o AMD/Source/amd_global.o AMD/Source/amd_info.o AMD/Source/amd_order.o AMD/Source/amd_post_tree.o AMD/Source/amd_postorder.o AMD/Source/amd_preprocess.o AMD/Source/amd_valid.o AMD/Source/amdbar.o CHOLMOD/Check/cholmod_check.o CHOLMOD/Check/cholmod_read.o CHOLMOD/Check/cholmod_write.o CHOLMOD/Cholesky/cholmod_amd.o CHOLMOD/Cholesky/cholmod_analyze.o CHOLMOD/Cholesky/cholmod_colamd.o CHOLMOD/Cholesky/cholmod_etree.o CHOLMOD/Cholesky/cholmod_factorize.o CHOLMOD/Cholesky/cholmod_postorder.o CHOLMOD/Cholesky/cholmod_rcond.o CHOLMOD/Cholesky/cholmod_resymbol.o CHOLMOD/Cholesky/cholmod_rowcolcounts.o CHOLMOD/Cholesky/cholmod_rowfac.o CHOLMOD/Cholesky/cholmod_solve.o CHOLMOD/Cholesky/cholmod_spsolve.o CHOLMOD/Core/cholmod_aat.o CHOLMOD/Core/cholmod_add.o CHOLMOD/Core/cholmod_band.o CHOLMOD/Core/cholmod_change_factor.o CHOLMOD/Core/cholmod_common.o CHOLMOD/Core/cholmod_complex.o CHOLMOD/Core/cholmod_copy.o CHOLMOD/Core/cholmod_dense.o CHOLMOD/Core/cholmod_error.o CHOLMOD/Core/cholmod_factor.o CHOLMOD/Core/cholmod_memory.o CHOLMOD/Core/cholmod_sparse.o CHOLMOD/Core/cholmod_transpose.o CHOLMOD/Core/cholmod_triplet.o CHOLMOD/Core/cholmod_version.o CHOLMOD/MatrixOps/cholmod_drop.o CHOLMOD/MatrixOps/cholmod_horzcat.o CHOLMOD/MatrixOps/cholmod_norm.o CHOLMOD/MatrixOps/cholmod_scale.o CHOLMOD/MatrixOps/cholmod_sdmult.o CHOLMOD/MatrixOps/cholmod_ssmult.o CHOLMOD/MatrixOps/cholmod_submatrix.o CHOLMOD/MatrixOps/cholmod_symmetry.o CHOLMOD/MatrixOps/cholmod_vertcat.o CHOLMOD/Modify/cholmod_rowadd.o CHOLMOD/Modify/cholmod_rowdel.o CHOLMOD/Modify/cholmod_updown.o CHOLMOD/Partition/cholmod_camd.o CHOLMOD/Partition/cholmod_ccolamd.o CHOLMOD/Partition/cholmod_csymamd.o CHOLMOD/Partition/cholmod_metis.o CHOLMOD/Partition/cholmod_nesdis.o CHOLMOD/Supernodal/cholmod_super_numeric.o CHOLMOD/Supernodal/cholmod_super_solve.o CHOLMOD/Supernodal/cholmod_super_symbolic.o COLAMD/Source/colamd.o COLAMD/Source/colamd_global.o DensityGrid.o DensityGrid_3d.o NetDataTypes.o NetRoutines.o SuiteSparse_config/SuiteSparse_config.o adjlist.o arpack.o array.o atlas.o attributes.o basic_query.o bfgs.o bigint.o bignum.o bipartite.o blas.o bliss.o bliss/bliss_heap.o bliss/defs.o bliss/graph.o bliss/orbit.o bliss/partition.o bliss/uintseqhash.o bliss/utils.o cattributes.o centrality.o cliquer/cliquer.o cliquer/cliquer_graph.o cliquer/reorder.o cliques.o clustertool.o cocitation.o cohesive_blocks.o coloring.o community.o complex.o components.o conversion.o cores.o cs/cs_add.o cs/cs_amd.o cs/cs_chol.o cs/cs_cholsol.o cs/cs_compress.o cs/cs_counts.o cs/cs_cumsum.o cs/cs_dfs.o cs/cs_dmperm.o cs/cs_droptol.o cs/cs_dropzeros.o cs/cs_dupl.o cs/cs_entry.o cs/cs_ereach.o cs/cs_etree.o cs/cs_fkeep.o cs/cs_gaxpy.o cs/cs_happly.o cs/cs_house.o cs/cs_ipvec.o cs/cs_leaf.o cs/cs_load.o cs/cs_lsolve.o cs/cs_ltsolve.o cs/cs_lu.o cs/cs_lusol.o cs/cs_malloc.o cs/cs_maxtrans.o cs/cs_multiply.o cs/cs_norm.o cs/cs_permute.o cs/cs_pinv.o cs/cs_post.o cs/cs_print.o cs/cs_pvec.o cs/cs_qr.o cs/cs_qrsol.o cs/cs_randperm.o cs/cs_reach.o cs/cs_scatter.o cs/cs_scc.o cs/cs_schol.o cs/cs_spsolve.o cs/cs_sqr.o cs/cs_symperm.o cs/cs_tdfs.o cs/cs_transpose.o cs/cs_updown.o cs/cs_usolve.o cs/cs_util.o cs/cs_utsolve.o decomposition.o distances.o dotproduct.o dqueue.o drl_graph.o drl_graph_3d.o drl_layout.o drl_layout_3d.o drl_parse.o eigen.o embedding.o fast_community.o feedback_arc_set.o flow.o foreign-dl-lexer.o foreign-dl-parser.o foreign-gml-lexer.o foreign-gml-parser.o foreign-graphml.o foreign-lgl-lexer.o foreign-lgl-parser.o foreign-ncol-lexer.o foreign-ncol-parser.o foreign-pajek-lexer.o foreign-pajek-parser.o foreign.o forestfire.o fortran_intrinsics.o games.o gengraph_box_list.o gengraph_degree_sequence.o gengraph_graph_molloy_hash.o gengraph_graph_molloy_optimized.o gengraph_mr-connected.o gengraph_powerlaw.o gengraph_random.o glet.o glpk_support.o gml_tree.o hacks.o heap.o igraph_buckets.o igraph_cliquer.o igraph_error.o igraph_estack.o igraph_fixed_vectorlist.o igraph_grid.o igraph_hashtable.o igraph_heap.o igraph_hrg.o igraph_hrg_types.o igraph_marked_queue.o igraph_psumtree.o igraph_set.o igraph_stack.o igraph_strvector.o igraph_trie.o infomap.o infomap_FlowGraph.o infomap_Greedy.o infomap_Node.o interrupt.o iterators.o lad.o lapack.o layout.o layout_dh.o layout_fr.o layout_gem.o layout_kk.o lsap.o matching.o math.o matrix.o maximal_cliques.o memory.o microscopic_update.o mixing.o motifs.o operators.o optimal_modularity.o other.o paths.o plfit/error.o plfit/gss.o plfit/kolmogorov.o plfit/lbfgs.o plfit/options.o plfit/plfit.o plfit/zeta.o pottsmodel_2.o progress.o prpack.o prpack/prpack_base_graph.o prpack/prpack_igraph_graph.o prpack/prpack_preprocessed_ge_graph.o prpack/prpack_preprocessed_gs_graph.o prpack/prpack_preprocessed_scc_graph.o prpack/prpack_preprocessed_schur_graph.o prpack/prpack_result.o prpack/prpack_solver.o prpack/prpack_utils.o qsort.o qsort_r.o random.o random_walk.o sbm.o scan.o scg.o scg_approximate_methods.o scg_exact_scg.o scg_kmeans.o scg_optimal_method.o scg_utils.o separators.o sir.o spanning_trees.o sparsemat.o spectral_properties.o spmatrix.o st-cuts.o statusbar.o structural_properties.o structure_generators.o sugiyama.o topology.o triangles.o type_indexededgelist.o types.o vector.o vector_ptr.o version.o visitors.o walktrap.o walktrap_communities.o walktrap_graph.o walktrap_heap.o zeroin.o dgetv0.o dlaqrb.o dmout.o dnaitr.o dnapps.o dnaup2.o dnaupd.o dnconv.o dneigh.o dneupd.o dngets.o dsaitr.o dsapps.o dsaup2.o dsaupd.o dsconv.o dseigt.o dsesrt.o dseupd.o dsgets.o dsortc.o dsortr.o dstatn.o dstats.o dstqrb.o dvout.o ivout.o second.o wrap.o simpleraytracer/Color.o simpleraytracer/Light.o simpleraytracer/Point.o simpleraytracer/RIgraphRay.o simpleraytracer/Ray.o simpleraytracer/RayTracer.o simpleraytracer/RayVector.o simpleraytracer/Shape.o simpleraytracer/Sphere.o simpleraytracer/Triangle.o simpleraytracer/unit_limiter.o uuid/R.o uuid/clear.o uuid/compare.o uuid/copy.o uuid/gen_uuid.o uuid/isnull.o uuid/pack.o uuid/parse.o uuid/unpack.o uuid/unparse.o rinterface.o rinterface_extra.o lazyeval.o -lxml2 -lz -lm -ldl -lgmp -lglpk -L/usr/lib64/R/lib -lRlapack -L/usr/lib64/R/lib -lRblas -lgfortran -lm -lquadmath -lgfortran -lm -lquadmath -L/usr/lib64/R/lib -lR
```

如果还是报错的话，可能就是这里的g++可能编译时没有用到相应的库，应该可以手动指定。
这个可以参考[CentOS/Redhat R包使用最新的gcc编译](https://shixiangwang.github.io/blog/use-new-gcc-on-centos-for-r/)这篇文章的环境变量设置部分或者文章[*How to specify (non-R) library path for dynamic library loading in R?*](https://newbedev.com/how-to-specify-non-r-library-path-for-dynamic-library-loading-in-r)。

成功测试的结果如下：

```R
> library(igraph)
> 
> g <- make_graph("Zachary")
> 
> ## We put everything into a big 'try' block, in case 
> ## igraph was compiled without GLPK support
> 
> ## The calculation only takes a couple of seconds
> oc <- cluster_optimal(g)
> 
> 
> 
> 
> 
> oc
IGRAPH clustering optimal, groups: 4, mod: 0.42
+ groups:
  $`1`
   [1]  1  2  3  4  8 12 13 14 18 20 22
  
  $`2`
  [1]  5  6  7 11 17
  
  $`3`
   [1]  9 10 15 16 19 21 23 27 30 31 33 34
  
  $`4`
  + ... omitted several groups/vertices
```

**更新**

需要注意，如果你使用的是conda环境的R，那么conda在调用自带的编译器编译时会找不到系统
yum安装的库，这个问题可以通过conda安装[`glpk`库](https://anaconda.org/conda-forge/glpk)来解决：

```bash
conda install -c conda-forge glpk
```




