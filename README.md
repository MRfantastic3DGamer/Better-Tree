# About
This will help you better visualize the folder structure of your app

## Instalation

### Prebuilt
just download the executabl file from executables/better-tree


### build from source code

with rust installed


```bash
git clone git@github.com:MRfantastic3DGamer/Better-Tree.git
cd Better-Tree/
cargo build
```

## Use
when in the folder of installed executabl better-tree
```bash
            [req]     [optional]          [optional]      [optional]           [optional]         [optional]
better-tree <folder> <Documentation file> <-n / no-files> <-s / stack-folders> <-H / show-hidden> <-i <relativ path> ignored-locations[0..]>
folder                          : the folder whose structure needs to be built
Documentation file              : the structure will be prented in the codeblock under <!---BETTER_FILES_TREE--> in the specified file
                                to see an example of how you should prepair the documentation file check this README.md
no-files          -n            : if added then no files will be rendered
stack-folders     -s            : if added then folders will be stacked whenever possible
show-hidden       -H            : if added then hidden files and folders will also be rendered
ignored-locations -i <location> : these locations are ignored while rendering

```

## Example of folder structure on this repo
<!---BETTER_FILES_TREE-->
```
┏ better-tree━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
┣output.txt                                                     ┃
┣Cargo.toml                                                     ┃
┣Cargo.lock                                                     ┃
┣┏ target━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓ ┃
┃┣┏ debug━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓ ┃ ┃
┃┃┣better-tree                                              ┃ ┃ ┃
┃┃┣┏ deps━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓        ┃ ┃ ┃
┃┃┃┣libunicode_ident-542f5604bbfe0ac2.rlib         ┃        ┃ ┃ ┃
┃┃┃┣heck-f10b3acfa716255c.d                        ┃        ┃ ┃ ┃
┃┃┃┣libstrsim-5c1fca5cd21bb3db.rmeta               ┃        ┃ ┃ ┃
┃┃┃┣utf8parse-0fd400427d500834.d                   ┃        ┃ ┃ ┃
┃┃┃┣libproc_macro2-4c24e61a51cb79c1.rlib           ┃        ┃ ┃ ┃
┃┃┃┣better_tree-2e6e5ad57528de91.d                 ┃        ┃ ┃ ┃
┃┃┃┣libcolorchoice-a6abfa0520cb84fd.rmeta          ┃        ┃ ┃ ┃
┃┃┃┣anstyle-2ffce3dd7e3a0f17.d                     ┃        ┃ ┃ ┃
┃┃┃┣libclap_builder-c9dfc52345e64d30.rmeta         ┃        ┃ ┃ ┃
┃┃┃┣syn-1ae3e5d2051950d8.d                         ┃        ┃ ┃ ┃
┃┃┃┣clap-3aa19b619b3a008f.d                        ┃        ┃ ┃ ┃
┃┃┃┣libis_terminal_polyfill-254bc9c5bee28d57.rmeta ┃        ┃ ┃ ┃
┃┃┃┣libstrsim-5c1fca5cd21bb3db.rlib                ┃        ┃ ┃ ┃
┃┃┃┣libclap-3aa19b619b3a008f.rmeta                 ┃        ┃ ┃ ┃
┃┃┃┣better_tree-f21e39c8e5541233                   ┃        ┃ ┃ ┃
┃┃┃┣clap_builder-c9dfc52345e64d30.d                ┃        ┃ ┃ ┃
┃┃┃┣libanstyle_query-2670247d99f37bb1.rlib         ┃        ┃ ┃ ┃
┃┃┃┣libclap_builder-c9dfc52345e64d30.rlib          ┃        ┃ ┃ ┃
┃┃┃┣strsim-5c1fca5cd21bb3db.d                      ┃        ┃ ┃ ┃
┃┃┃┣better_tree-f21e39c8e5541233.d                 ┃        ┃ ┃ ┃
┃┃┃┣libproc_macro2-4c24e61a51cb79c1.rmeta          ┃        ┃ ┃ ┃
┃┃┃┣libanstyle-2ffce3dd7e3a0f17.rmeta              ┃        ┃ ┃ ┃
┃┃┃┣libis_terminal_polyfill-254bc9c5bee28d57.rlib  ┃        ┃ ┃ ┃
┃┃┃┣quote-4c88fad813a39737.d                       ┃        ┃ ┃ ┃
┃┃┃┣libclap-3aa19b619b3a008f.rlib                  ┃        ┃ ┃ ┃
┃┃┃┣libanstyle-2ffce3dd7e3a0f17.rlib               ┃        ┃ ┃ ┃
┃┃┃┣libheck-f10b3acfa716255c.rlib                  ┃        ┃ ┃ ┃
┃┃┃┣libutf8parse-0fd400427d500834.rlib             ┃        ┃ ┃ ┃
┃┃┃┣libunicode_ident-542f5604bbfe0ac2.rmeta        ┃        ┃ ┃ ┃
┃┃┃┣libanstream-2add7994000672b1.rlib              ┃        ┃ ┃ ┃
┃┃┃┣colorchoice-a6abfa0520cb84fd.d                 ┃        ┃ ┃ ┃
┃┃┃┣libsyn-1ae3e5d2051950d8.rmeta                  ┃        ┃ ┃ ┃
┃┃┃┣libanstyle_parse-534aae5427a00bf0.rlib         ┃        ┃ ┃ ┃
┃┃┃┣clap_lex-6ab278a595966695.d                    ┃        ┃ ┃ ┃
┃┃┃┣libheck-f10b3acfa716255c.rmeta                 ┃        ┃ ┃ ┃
┃┃┃┣anstyle_parse-534aae5427a00bf0.d               ┃        ┃ ┃ ┃
┃┃┃┣better_tree-2e6e5ad57528de91                   ┃        ┃ ┃ ┃
┃┃┃┣libclap_lex-6ab278a595966695.rmeta             ┃        ┃ ┃ ┃
┃┃┃┣unicode_ident-542f5604bbfe0ac2.d               ┃        ┃ ┃ ┃
┃┃┃┣libclap_lex-6ab278a595966695.rlib              ┃        ┃ ┃ ┃
┃┃┃┣libutf8parse-0fd400427d500834.rmeta            ┃        ┃ ┃ ┃
┃┃┃┣libquote-4c88fad813a39737.rlib                 ┃        ┃ ┃ ┃
┃┃┃┣is_terminal_polyfill-254bc9c5bee28d57.d        ┃        ┃ ┃ ┃
┃┃┃┣clap_derive-79495ce2ef390c57.d                 ┃        ┃ ┃ ┃
┃┃┃┣libanstyle_query-2670247d99f37bb1.rmeta        ┃        ┃ ┃ ┃
┃┃┃┣libanstyle_parse-534aae5427a00bf0.rmeta        ┃        ┃ ┃ ┃
┃┃┃┣anstyle_query-2670247d99f37bb1.d               ┃        ┃ ┃ ┃
┃┃┃┣libquote-4c88fad813a39737.rmeta                ┃        ┃ ┃ ┃
┃┃┃┣libclap_derive-79495ce2ef390c57.so             ┃        ┃ ┃ ┃
┃┃┃┣libanstream-2add7994000672b1.rmeta             ┃        ┃ ┃ ┃
┃┃┃┣libcolorchoice-a6abfa0520cb84fd.rlib           ┃        ┃ ┃ ┃
┃┃┃┣libsyn-1ae3e5d2051950d8.rlib                   ┃        ┃ ┃ ┃
┃┃┃┣proc_macro2-4c24e61a51cb79c1.d                 ┃        ┃ ┃ ┃
┃┃┃┣anstream-2add7994000672b1.d                    ┃        ┃ ┃ ┃
┃┃┃┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛        ┃ ┃ ┃
┃┃┣better-tree.d                                            ┃ ┃ ┃
┃┃┣┏ build━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓              ┃ ┃ ┃
┃┃┃┣┏ proc-macro2-7b1ca16f88880f2c━┓         ┃              ┃ ┃ ┃
┃┃┃┃┣stderr                        ┃         ┃              ┃ ┃ ┃
┃┃┃┃┣output                        ┃         ┃              ┃ ┃ ┃
┃┃┃┃┣invoked.timestamp             ┃         ┃              ┃ ┃ ┃
┃┃┃┃┣root-output                   ┃         ┃              ┃ ┃ ┃
┃┃┃┃┣┏ out━━━━━━━━━━┓              ┃         ┃              ┃ ┃ ┃
┃┃┃┃┃┣proc_macro2.d ┃              ┃         ┃              ┃ ┃ ┃
┃┃┃┃┃┗━━━━━━━━━━━━━━┛              ┃         ┃              ┃ ┃ ┃
┃┃┃┃┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛         ┃              ┃ ┃ ┃
┃┃┃┣┏ proc-macro2-574493ced38b24ec━━━━━━━━━┓ ┃              ┃ ┃ ┃
┃┃┃┃┣build_script_build-574493ced38b24ec   ┃ ┃              ┃ ┃ ┃
┃┃┃┃┣build_script_build-574493ced38b24ec.d ┃ ┃              ┃ ┃ ┃
┃┃┃┃┣build-script-build                    ┃ ┃              ┃ ┃ ┃
┃┃┃┃┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛ ┃              ┃ ┃ ┃
┃┃┃┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛              ┃ ┃ ┃
┃┃┣┏ incremental━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓ ┃ ┃ ┃
┃┃┃┣┏ better_tree-27t5z6hufnro2━━━━━━━━━━━━━━━━━━━━━━━━━┓ ┃ ┃ ┃ ┃
┃┃┃┃┣┏ s-gzgiats5ws-1nw374s-0j219fv47orycblk5dqg4qd49━┓ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣3tmb2m7hu4ya3qdbicub8676e.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣cxresccyi58927mkepdoyufic.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣dtyx7mpwl558mtrj42diw9hel.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣9x4mss8p1et73nbts84f0upo9.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣bkpjz7byzel0anf5ghoqum9in.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣2rffm3cbrw4jmgwowpne48uwc.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣0r3ewcyhmujxwh93yb4zc6gva.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣82hrelzuc7ja9krobxz0n8e0y.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣8kze0kac7xvqivb32ly84d2fu.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣6qk8mnexgvf9q2rkil4qdjdxd.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣6x879mbsa5dx3e03lm2v47k0f.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣3q3i5alveein72lxq58h8zp1u.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣aiwndwvk4wlt2vqxzr4e5wupm.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣8wxvusg9ezcvds0b7pi33e6v6.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣18iklpi7o2szy17jme3voe3wg.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣b8442b6dy35dg8o357jcl71h7.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣cdmomcjuymsbflfwym8gycq49.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣emw0rmzlqyj4oeds1hovow6ot.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣1zggx1bjh6o1qilul25ar5lbt.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣az29ig2ny6htjfuohokoonp88.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣cettakvrn3pdzl7epaewhy1cq.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣9jbmle6ldb9pa75klg7kv52z7.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣cv1mquk68metp4gcoaj1yi3d4.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣71e8hiykmc5pd9inxeqprc74u.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣9evqag1y6lxzbrcehwbq97cio.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣758d9bq001jmxg0dbjydvchyn.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣7huxqu4tu9ojjrce1hxs6nod7.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣a5jqb3sop5ixypcx15nj1xdv9.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣8pertr0lrsgp19dk98ms3cjjg.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣6p82bwp03wpyc9p1la0ixdy1x.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣euwmyxyeh0k349fls54f2ce6j.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣c7fydfqyzmia5ju82zqv0nyw8.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣1300hvvoz9xhgms7nkqlccbnz.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣e8ws8k21sm6d8dfu9e8nlyb10.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣7n99ne1ms3e39nuamq1azjar3.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣bk9eszts5i84l4kdkd35dazwm.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣query-cache.bin                                 ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣4s4o98fkfxmmm1poxxxaeyhd1.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣3crommlv9eys3h3vqu9z9ly29.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣8whmrtpn34i98k3kj08d8qaeh.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣1vuiyxa88abpgy0yty1z1as31.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣0oauomatd9lsejvsg4nickprz.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣910b59hx3p8i0wvpzaazupq21.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣dzn06nvtt0k4uimuzbyijmrc4.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣8pqx4enh0fx8yhtpex28z2kag.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣0yhuh3yc9j8y1tdm7y6oqz97y.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣6kpsayt7jor6bh9kavtgpmzgu.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣dep-graph.bin                                   ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣c7y1r5f8nf775kzu6w3l0eiyo.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣7mpc1wzrvphw7zm7kjtknuygq.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣8f75myqyubpah7ifj6xdl7rpp.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣4oav8xqurc4871rjxhomm1820.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣51qnv29s342sbv9x0wo2pcwa7.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣b1b3ufsi2hctuvqf5fzre7mfv.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣6j1lt6v4kkgtfu2hjff8vs76t.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣1xy8fy96fb89n03nggemmd7fv.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣3iktlrx7z8ckqft5bysz9gaae.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣130e26rwwcxm1m5t50b1ocaur.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣drgl7n3ouuewash9nwlx5lxe6.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣dwksyj2sdxlbhldpelirnsrjq.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣9rhwyt4or4c3yc6odf66k7bg6.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣7u19lhpit3eu6z58hdzqw4rsn.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣d28wxwa2uz3ctwlkbxr4azkll.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣4iygteghr8z8rldsx1u5logae.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣5jzgtn3wm8t1f4js1ah525w9m.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣1p3kqln0ff6ggsk9bnjmeops4.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣bjacshhgwxknpirhyrfpz7ean.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣29xw6soj2g34fbxj4h0wdtkgp.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣b7f4wq9odahvrmsyo9ktl30sh.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣bh98sd6crjszzcno1kcn10a75.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣1pfk08h2ppvfl5p6v8x73qe12.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣9nobahkiphgfejtr4bo2k6nsh.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣boh4aq2unmadv8pw0yjie50fr.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣8vck5lc0gtvn3g324uj11ky0d.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣3uvjyrz1q6fg4r4yf5iukfffd.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣work-products.bin                               ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣5d4gwrsjcvuzo3y3os9wl1rig.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣7f0fceaton7118t3r190vbkre.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣0ca66c5zufua4j7gbaqaojyw0.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣6x61f8gv43vyv9zfprse9v4e5.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣44y1d8omrixsxm70zmdr7qcs8.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣d98kp8e9ksyza3d7vzyhuur4d.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣eo5e8897rn5ljd9djfznstu0o.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣61yzayofx4mrj36uazgam4cyb.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣epq6brr8okk4f9j1v79gar4ba.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣a3tmqdx3es53chf80lfwxdoba.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣4xbqity7uyv4hkwcs3yomtard.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┣s-gzgiats5ws-1nw374s.lock                          ┃ ┃ ┃ ┃ ┃
┃┃┃┃┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛ ┃ ┃ ┃ ┃
┃┃┃┣┏ better_tree-1ls6bodjwuehk━━━━━━━━━━━━━━━━━━━━━━━━━┓ ┃ ┃ ┃ ┃
┃┃┃┃┣┏ s-gzgu5jovux-1mpwlws-a95chze5iokw6ym60w8h8f3zg━┓ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣0g3gye5eac791upyxsi16pjpq.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣2f0ne4ijbgagawtmixrc5m41i.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣0k9l2ho0d1n449llngpnepcd8.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣18priawxoescgctlb32qbgnca.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣by9fxzsd64rgyrw45nddov76m.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣bv3b9z16695cl3glfa44xq2by.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣a4y6rwjmpov7e23ouste4koo3.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣e9nt905dim2fal22ev610y13a.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣d550l3jlwrnyrr5z7472684f1.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣7xxivqvovx4p8lm4kk3nsrqsq.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣3e6re5ime3xuxk3zklfpjeyfg.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣66blq1ecs9hchh8m58h02dwlj.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣7ar0am2bhexv54pgapk2dv6hj.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣0k1i9bconv0fg6n17w8mh6db6.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣2lci8s5zv9otr4cc67ilwn6z5.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣1di46zoxz1y5wla81ug37qp5x.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣6io1f3rr5uiluq07fyomgyngt.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣7v8b6pm02bptsdeprs300a1yz.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣dxw3yhgsm0gv2igw1l2v2g2aw.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣1ebdxoeufeu4kzjz099vxrezk.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣a4kkgdp1euhz3n52j18tgdcrf.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣413ik0usz952v474uewyzsb5n.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣44pdspwvtonm97bcn8gildpu8.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣5oc22nn9b55k6lt64j2pja8tr.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣esxrjevwjcpd66ykvlv1qkc8v.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣03k010cwpgvjm0j573xbd0wa8.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣query-cache.bin                                 ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣8ldt96x8e769fqyu5vlciz600.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣3tdtotn86146vlng15tjwkp81.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣9zqxf7fy6m1rme4zfpgselwzh.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣12nfdotq71z0l3msyvq4hrwtt.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣3ae8dgiu8i4bta8ra2gohigzj.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣4fd6xvmpkcg2q41nnpsmbqml9.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣bjtxiamybidr877qpq25y2zkl.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣7rtm58g9ybmmfspdls28e30cw.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣4p8muiwb63xqtmm89jstmrf80.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣5r4vhx3uu9s9zbw6gpwb6z927.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣dgy2x11523pfjp9nrdyzvbjue.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣a7uuh2rf24us5edfs3422fykr.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣c8zxe6ylods0pcxsjcb6upoms.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣drlfo945mdj1p7ln8ud265c6l.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣dep-graph.bin                                   ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣3e4q3hzd6luno4drehvwwsb55.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣0o2ke724he54mth3z8a94ekew.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣2i7wstahj4yrt1bbra0ol0nxo.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣0z30112qtn4a0613h1usfivr0.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣8uuooy8sxjasmyejopjqe2x81.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣ey1wscquwgulhduudhmdvw5gx.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣db16semv8mqa50pfa5kcpoz9v.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣d122ebh2mvhvdfgzrbxec9fst.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣0g6hxj75auc1kzpll4l1ncqv0.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣1k1v5cyhwsfzkgwsgk0ywmiq7.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣91efm8fmum92ypdemno9fwjbr.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣ejbhffg98ro9guu4wocmnifpc.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣dxgun18vb384924aapis01tzo.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣eji251xhk8jotqymmvwmdu6tl.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣ayujud0zej4irn1o4k3e19tct.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣d93eq1gces9l8irib0ptqfqfw.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣1i6den3b0q0sfb38wqcxu7dp6.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣c5z2a9bwohfxs04g89a3vzrfa.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣e4arsdq9rgwvz46lny4c8se77.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣45sgdatsv9wyvtg5glqsj5prs.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣3ykz3ai977agts7m7g36ikkak.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣work-products.bin                               ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣4hncz4y563sz0b1pspjkzbk41.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣9pu4ql495khgzuuepnuffo4lo.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣7skjftspdzfuq26i63oyvm0dk.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣agrh3683u5bj8dubjpo5ntno2.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣8ol4gkcf4myw3b4snt3t8wdr7.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣ctdw2dkbpk5rfl5s3tfdg8lwa.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣96o3c2rhph5v2wgvwkdstk3iz.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣8sat4w3i3l6b7654iwr18gf6k.o                     ┃ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┣s-gzgu5jovux-1mpwlws.lock                          ┃ ┃ ┃ ┃ ┃
┃┃┃┃┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛ ┃ ┃ ┃ ┃
┃┃┃┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛ ┃ ┃ ┃
┃┃┣┏ examples━┓                                             ┃ ┃ ┃
┃┃┃┗━━━━━━━━━━┛                                             ┃ ┃ ┃
┃┃┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛ ┃ ┃
┃┣┏ release━━━━━━━━━━━━━━━━━━━━━━━━━━┓                        ┃ ┃
┃┃┣better-tree                       ┃                        ┃ ┃
┃┃┣┏ deps━━━━━━━━━━━━━━━━━━━━━━━━━━┓ ┃                        ┃ ┃
┃┃┃┣better_tree-b2c105fd1fc1bef3.d ┃ ┃                        ┃ ┃
┃┃┃┣better_tree-b2c105fd1fc1bef3   ┃ ┃                        ┃ ┃
┃┃┃┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛ ┃                        ┃ ┃
┃┃┣better-tree.d                     ┃                        ┃ ┃
┃┃┣┏ build━┓                         ┃                        ┃ ┃
┃┃┃┗━━━━━━━┛                         ┃                        ┃ ┃
┃┃┣┏ incremental━┓                   ┃                        ┃ ┃
┃┃┃┗━━━━━━━━━━━━━┛                   ┃                        ┃ ┃
┃┃┣┏ examples━┓                      ┃                        ┃ ┃
┃┃┃┗━━━━━━━━━━┛                      ┃                        ┃ ┃
┃┃┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛                        ┃ ┃
┃┣CACHEDIR.TAG                                                ┃ ┃
┃┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛ ┃
┣README.md                                                      ┃
┣┏ src━━━━━┓                                                    ┃
┃┣main.rs  ┃                                                    ┃
┃┣style.rs ┃                                                    ┃
┃┗━━━━━━━━━┛                                                    ┃
┣┏ executables━┓                                                ┃
┃┣better-tree  ┃                                                ┃
┃┗━━━━━━━━━━━━━┛                                                ┃
┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛
```


what are you trying to find scrolling this far down.


while you are here, let me explain why I built this.


This was created as the folder structure created by lenux command tree looks too generic without any options for customizability


# Hope it helps . Bye
