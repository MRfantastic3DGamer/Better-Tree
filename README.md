## About
This will help you better visualize the folder structure of your app

## Example of folder structure on this repo
<!---BETTER_FILES_TREE-->
```
┏ better-tree━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
┣output.txt                                                     ┃
┣╭ .github────────────────╮                                     ┃
┃├┏ workflows━━━━━━━━━━━┓ │                                     ┃
┃│┣generate-pattern.yml ┃ │                                     ┃
┃│┗━━━━━━━━━━━━━━━━━━━━━┛ │                                     ┃
┃╰────────────────────────╯                                     ┃
┣Cargo.toml                                                     ┃
┣Cargo.lock                                                     ┃
┣┏ target━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓ ┃
┃┣┏ debug━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓ ┃ ┃
┃┃┣better-tree                                              ┃ ┃ ┃
┃┃┣┏ deps━━━━━━━━━━━━━━━━━━━━━━━━━━┓                        ┃ ┃ ┃
┃┃┃┣better_tree-2e6e5ad57528de91.d ┃                        ┃ ┃ ┃
┃┃┃┣better_tree-2e6e5ad57528de91   ┃                        ┃ ┃ ┃
┃┃┃┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛                        ┃ ┃ ┃
┃┃┣better-tree.d                                            ┃ ┃ ┃
┃┃┣┏ build━┓                                                ┃ ┃ ┃
┃┃┃┗━━━━━━━┛                                                ┃ ┃ ┃
┃┃┣.cargo-lock                                              ┃ ┃ ┃
┃┃┣╭ .fingerprint────────────────────╮                      ┃ ┃ ┃
┃┃┃├┏ better-tree-2e6e5ad57528de91━┓ │                      ┃ ┃ ┃
┃┃┃│┣invoked.timestamp             ┃ │                      ┃ ┃ ┃
┃┃┃│┣bin-better-tree.json          ┃ │                      ┃ ┃ ┃
┃┃┃│┣bin-better-tree               ┃ │                      ┃ ┃ ┃
┃┃┃│┣dep-bin-better-tree           ┃ │                      ┃ ┃ ┃
┃┃┃│┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛ │                      ┃ ┃ ┃
┃┃┃╰─────────────────────────────────╯                      ┃ ┃ ┃
┃┃┣┏ incremental━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓ ┃ ┃ ┃
┃┃┃┣┏ better_tree-27t5z6hufnro2━━━━━━━━━━━━━━━━━━━━━━━━━┓ ┃ ┃ ┃ ┃
┃┃┃┃┣┏ s-gzg1nme9s4-0dwssti-7uxkawuusd62n50nznfj0i39o━┓ ┃ ┃ ┃ ┃ ┃
┃┃┃┃┃┣b3p6q9nxwkv66qvn2twf6grh6.o                     ┃ ┃ ┃ ┃ ┃ ┃
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
┃┃┃┃┣s-gzg1nme9s4-0dwssti.lock                          ┃ ┃ ┃ ┃ ┃
┃┃┃┃┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛ ┃ ┃ ┃ ┃
┃┃┃┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛ ┃ ┃ ┃
┃┃┣┏ examples━┓                                             ┃ ┃ ┃
┃┃┃┗━━━━━━━━━━┛                                             ┃ ┃ ┃
┃┃┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛ ┃ ┃
┃┣┏ release━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓                      ┃ ┃
┃┃┣better-tree                         ┃                      ┃ ┃
┃┃┣┏ deps━━━━━━━━━━━━━━━━━━━━━━━━━━┓   ┃                      ┃ ┃
┃┃┃┣better_tree-b2c105fd1fc1bef3.d ┃   ┃                      ┃ ┃
┃┃┃┣better_tree-b2c105fd1fc1bef3   ┃   ┃                      ┃ ┃
┃┃┃┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛   ┃                      ┃ ┃
┃┃┣better-tree.d                       ┃                      ┃ ┃
┃┃┣┏ build━┓                           ┃                      ┃ ┃
┃┃┃┗━━━━━━━┛                           ┃                      ┃ ┃
┃┃┣.cargo-lock                         ┃                      ┃ ┃
┃┃┣╭ .fingerprint────────────────────╮ ┃                      ┃ ┃
┃┃┃├┏ better-tree-b2c105fd1fc1bef3━┓ │ ┃                      ┃ ┃
┃┃┃│┣invoked.timestamp             ┃ │ ┃                      ┃ ┃
┃┃┃│┣bin-better-tree.json          ┃ │ ┃                      ┃ ┃
┃┃┃│┣bin-better-tree               ┃ │ ┃                      ┃ ┃
┃┃┃│┣dep-bin-better-tree           ┃ │ ┃                      ┃ ┃
┃┃┃│┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛ │ ┃                      ┃ ┃
┃┃┃╰─────────────────────────────────╯ ┃                      ┃ ┃
┃┃┣┏ incremental━┓                     ┃                      ┃ ┃
┃┃┃┗━━━━━━━━━━━━━┛                     ┃                      ┃ ┃
┃┃┣┏ examples━┓                        ┃                      ┃ ┃
┃┃┃┗━━━━━━━━━━┛                        ┃                      ┃ ┃
┃┃┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛                      ┃ ┃
┃┣CACHEDIR.TAG                                                ┃ ┃
┃┣.rustc_info.json                                            ┃ ┃
┃┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛ ┃
┣README.md                                                      ┃
┣action.yml                                                     ┃
┣┏ src━━━━━┓                                                    ┃
┃┣main.rs  ┃                                                    ┃
┃┣style.rs ┃                                                    ┃
┃┗━━━━━━━━━┛                                                    ┃
┣╭ .git────────────────────────────────────────╮                ┃
┃├FETCH_HEAD                                   │                ┃
┃├┏ logs━━━━━━━━━━━━┓                          │                ┃
┃│┣┏ refs━━━━━━━━━┓ ┃                          │                ┃
┃│┃┣┏ remotes━━━┓ ┃ ┃                          │                ┃
┃│┃┃┣┏ origin━┓ ┃ ┃ ┃                          │                ┃
┃│┃┃┃┣main    ┃ ┃ ┃ ┃                          │                ┃
┃│┃┃┃┣master  ┃ ┃ ┃ ┃                          │                ┃
┃│┃┃┃┗━━━━━━━━┛ ┃ ┃ ┃                          │                ┃
┃│┃┃┗━━━━━━━━━━━┛ ┃ ┃                          │                ┃
┃│┃┣┏ heads━┓     ┃ ┃                          │                ┃
┃│┃┃┣main   ┃     ┃ ┃                          │                ┃
┃│┃┃┣master ┃     ┃ ┃                          │                ┃
┃│┃┃┗━━━━━━━┛     ┃ ┃                          │                ┃
┃│┃┗━━━━━━━━━━━━━━┛ ┃                          │                ┃
┃│┣HEAD             ┃                          │                ┃
┃│┗━━━━━━━━━━━━━━━━━┛                          │                ┃
┃├ORIG_HEAD                                    │                ┃
┃├┏ info━━━┓                                   │                ┃
┃│┣exclude ┃                                   │                ┃
┃│┗━━━━━━━━┛                                   │                ┃
┃├description                                  │                ┃
┃├COMMIT_EDITMSG                               │                ┃
┃├index                                        │                ┃
┃├┏ hooks━━━━━━━━━━━━━━━━━━━━┓                 │                ┃
┃│┣update.sample             ┃                 │                ┃
┃│┣pre-push.sample           ┃                 │                ┃
┃│┣pre-merge-commit.sample   ┃                 │                ┃
┃│┣pre-rebase.sample         ┃                 │                ┃
┃│┣commit-msg.sample         ┃                 │                ┃
┃│┣post-update.sample        ┃                 │                ┃
┃│┣push-to-checkout.sample   ┃                 │                ┃
┃│┣pre-applypatch.sample     ┃                 │                ┃
┃│┣applypatch-msg.sample     ┃                 │                ┃
┃│┣fsmonitor-watchman.sample ┃                 │                ┃
┃│┣prepare-commit-msg.sample ┃                 │                ┃
┃│┣pre-commit.sample         ┃                 │                ┃
┃│┣pre-receive.sample        ┃                 │                ┃
┃│┗━━━━━━━━━━━━━━━━━━━━━━━━━━┛                 │                ┃
┃├config                                       │                ┃
┃├┏ refs━━━━━━━━━┓                             │                ┃
┃│┣┏ remotes━━━┓ ┃                             │                ┃
┃│┃┣┏ origin━┓ ┃ ┃                             │                ┃
┃│┃┃┣main    ┃ ┃ ┃                             │                ┃
┃│┃┃┣master  ┃ ┃ ┃                             │                ┃
┃│┃┃┗━━━━━━━━┛ ┃ ┃                             │                ┃
┃│┃┗━━━━━━━━━━━┛ ┃                             │                ┃
┃│┣┏ tags━━┓     ┃                             │                ┃
┃│┃┣v1.0.0 ┃     ┃                             │                ┃
┃│┃┣v1.0.2 ┃     ┃                             │                ┃
┃│┃┣v1.0.1 ┃     ┃                             │                ┃
┃│┃┗━━━━━━━┛     ┃                             │                ┃
┃│┣┏ heads━┓     ┃                             │                ┃
┃│┃┣main   ┃     ┃                             │                ┃
┃│┃┣master ┃     ┃                             │                ┃
┃│┃┗━━━━━━━┛     ┃                             │                ┃
┃│┗━━━━━━━━━━━━━━┛                             │                ┃
┃├┏ objects━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓ │                ┃
┃│┣┏ ea━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓ ┃ │                ┃
┃│┃┣8c4bf7f35f6f77f75d92ad8ce8349f6e81ddba ┃ ┃ │                ┃
┃│┃┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛ ┃ │                ┃
┃│┣┏ 01━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓ ┃ │                ┃
┃│┃┣363a1140287c3c2fb16bbd65828e227de02d2f ┃ ┃ │                ┃
┃│┃┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛ ┃ │                ┃
┃│┣┏ e4━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓ ┃ │                ┃
┃│┃┣97cd2533314da95778388264368aec81e2435c ┃ ┃ │                ┃
┃│┃┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛ ┃ │                ┃
┃│┣┏ 00━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓ ┃ │                ┃
┃│┃┣9eed04e0c83855d1b2f461d0df4cfa9de53b50 ┃ ┃ │                ┃
┃│┃┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛ ┃ │                ┃
┃│┣┏ 04━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓ ┃ │                ┃
┃│┃┣e42de685bd17ddb606c5362de6807a4aaaae19 ┃ ┃ │                ┃
┃│┃┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛ ┃ │                ┃
┃│┣┏ 68━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓ ┃ │                ┃
┃│┃┣a6f02ae46dc88c7e05b89a8001e6455c517f0f ┃ ┃ │                ┃
┃│┃┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛ ┃ │                ┃
┃│┣┏ 23━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓ ┃ │                ┃
┃│┃┣521cad08d26aa7d0cf2ee3967ba03879de312f ┃ ┃ │                ┃
┃│┃┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛ ┃ │                ┃
┃│┣┏ 26━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓ ┃ │                ┃
┃│┃┣cd9aeef71219d037cd8d9a2f99e366729397e3 ┃ ┃ │                ┃
┃│┃┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛ ┃ │                ┃
┃│┣┏ a8━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓ ┃ │                ┃
┃│┃┣b6fa0632ce62395a23911834b9192e605f5949 ┃ ┃ │                ┃
┃│┃┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛ ┃ │                ┃
┃│┣┏ af━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓ ┃ │                ┃
┃│┃┣6e72a950a36c4326327d9d729649c4018b5c1f ┃ ┃ │                ┃
┃│┃┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛ ┃ │                ┃
┃│┣┏ 6b━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓ ┃ │                ┃
┃│┃┣2845a91bb69cb7fb77953084cd637c4b4580bd ┃ ┃ │                ┃
┃│┃┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛ ┃ │                ┃
┃│┣┏ 25━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓ ┃ │                ┃
┃│┃┣5d4da38a94d691a062611169580822c896f089 ┃ ┃ │                ┃
┃│┃┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛ ┃ │                ┃
┃│┣┏ pack━┓                                  ┃ │                ┃
┃│┃┗━━━━━━┛                                  ┃ │                ┃
┃│┣┏ 42━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓ ┃ │                ┃
┃│┃┣c4afff9fcc85751a853e73582aba443b2adbb9 ┃ ┃ │                ┃
┃│┃┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛ ┃ │                ┃
┃│┣┏ 0d━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓ ┃ │                ┃
┃│┃┣9f5c32bc973f93a39606721294ee3a921149a7 ┃ ┃ │                ┃
┃│┃┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛ ┃ │                ┃
┃│┣┏ info━┓                                  ┃ │                ┃
┃│┃┗━━━━━━┛                                  ┃ │                ┃
┃│┣┏ fe━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓ ┃ │                ┃
┃│┃┣89f7891731baa2736976c1abe2d8ae1d3f9b91 ┃ ┃ │                ┃
┃│┃┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛ ┃ │                ┃
┃│┣┏ 74━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓ ┃ │                ┃
┃│┃┣5c4c3f945541dcd02a0ca7892b691015cbd593 ┃ ┃ │                ┃
┃│┃┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛ ┃ │                ┃
┃│┣┏ 22━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓ ┃ │                ┃
┃│┃┣3651dfe6e0f68921b01ccb88379f50db33e244 ┃ ┃ │                ┃
┃│┃┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛ ┃ │                ┃
┃│┣┏ 66━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓ ┃ │                ┃
┃│┃┣12012c8cd93019aa0f119209604d34890178c7 ┃ ┃ │                ┃
┃│┃┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛ ┃ │                ┃
┃│┣┏ 84━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓ ┃ │                ┃
┃│┃┣8bb8c7600b47629131aaf936bfb818cddc0253 ┃ ┃ │                ┃
┃│┃┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛ ┃ │                ┃
┃│┣┏ a7━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓ ┃ │                ┃
┃│┃┣f4d07225fb3b28311a49288cb421a4bb24689a ┃ ┃ │                ┃
┃│┃┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛ ┃ │                ┃
┃│┣┏ f9━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓ ┃ │                ┃
┃│┃┣6a690dabed9b437da33e9890a4a485b704b460 ┃ ┃ │                ┃
┃│┃┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛ ┃ │                ┃
┃│┣┏ 32━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓ ┃ │                ┃
┃│┃┣dc25d0c7ac557591c6d2bbb67b83fbc5cb8ad4 ┃ ┃ │                ┃
┃│┃┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛ ┃ │                ┃
┃│┣┏ 90━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓ ┃ │                ┃
┃│┃┣c088592fdf020d9053363d3772eae4b791fe44 ┃ ┃ │                ┃
┃│┃┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛ ┃ │                ┃
┃│┣┏ 28━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓ ┃ │                ┃
┃│┃┣e25ccd8cbbb16e04c219d46a579cfccf1825ab ┃ ┃ │                ┃
┃│┃┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛ ┃ │                ┃
┃│┣┏ 18━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓ ┃ │                ┃
┃│┃┣91327ffc2571305bdbccca2c771e2b9f5984d9 ┃ ┃ │                ┃
┃│┃┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛ ┃ │                ┃
┃│┣┏ da━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓ ┃ │                ┃
┃│┃┣ce918739ba9b187ba5e79273ed15d45c5b950d ┃ ┃ │                ┃
┃│┃┣d190df87a5e8e17c17c34e82b89dd0568ace92 ┃ ┃ │                ┃
┃│┃┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛ ┃ │                ┃
┃│┣┏ 70━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓ ┃ │                ┃
┃│┃┣219d2d9a932e24e75efb3efc399cee5853bac9 ┃ ┃ │                ┃
┃│┃┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛ ┃ │                ┃
┃│┣┏ 78━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓ ┃ │                ┃
┃│┃┣ca5b00ac88620451a9a8ac3d4fbc24566a804e ┃ ┃ │                ┃
┃│┃┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛ ┃ │                ┃
┃│┣┏ 98━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓ ┃ │                ┃
┃│┃┣846d517e42b4585996d37f6df7b11072560843 ┃ ┃ │                ┃
┃│┃┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛ ┃ │                ┃
┃│┣┏ 2c━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓ ┃ │                ┃
┃│┃┣96b96610144e9bb267dc8fc473ca024e5a02d2 ┃ ┃ │                ┃
┃│┃┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛ ┃ │                ┃
┃│┣┏ a6━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓ ┃ │                ┃
┃│┃┣cf8bc1573e3abdc3dd3db1362c72e8e38c8120 ┃ ┃ │                ┃
┃│┃┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛ ┃ │                ┃
┃│┣┏ f1━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓ ┃ │                ┃
┃│┃┣209483fc96ceba1f1242148769cf67ec7bfe5b ┃ ┃ │                ┃
┃│┃┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛ ┃ │                ┃
┃│┣┏ 4d━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓ ┃ │                ┃
┃│┃┣55c83c3e674cc646e1d7cdd5f7ae283f48093b ┃ ┃ │                ┃
┃│┃┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛ ┃ │                ┃
┃│┣┏ ca━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓ ┃ │                ┃
┃│┃┣ad0881e62b9fbb6eca1e76cb9aa14baabcd78e ┃ ┃ │                ┃
┃│┃┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛ ┃ │                ┃
┃│┣┏ f6━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓ ┃ │                ┃
┃│┃┣15d9e8e49992f4f11aa0fecb766348952fb065 ┃ ┃ │                ┃
┃│┃┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛ ┃ │                ┃
┃│┣┏ 71━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓ ┃ │                ┃
┃│┃┣174f5d5deeeeb16361afb989575a5257abf461 ┃ ┃ │                ┃
┃│┃┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛ ┃ │                ┃
┃│┣┏ e1━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓ ┃ │                ┃
┃│┃┣6d564b912c53ff0d6cda5f86de839cad09d5c8 ┃ ┃ │                ┃
┃│┃┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛ ┃ │                ┃
┃│┣┏ 4c━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓ ┃ │                ┃
┃│┃┣74a85c59f8a3c130a91415fb57a90653d22a79 ┃ ┃ │                ┃
┃│┃┣fef3e15931ba39c311c601636dfb34bec7344f ┃ ┃ │                ┃
┃│┃┣af793fd7767748ab9cbcad08a754729ac0a764 ┃ ┃ │                ┃
┃│┃┣4873df9901fd7e3bdb4f8305f0407f3b7153b8 ┃ ┃ │                ┃
┃│┃┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛ ┃ │                ┃
┃│┣┏ 41━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓ ┃ │                ┃
┃│┃┣702fc8fe57b9f4717f30473bdffa15133d28e4 ┃ ┃ │                ┃
┃│┃┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛ ┃ │                ┃
┃│┣┏ 87━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓ ┃ │                ┃
┃│┃┣f541d498c604cf0a9f009cd9165774e7ac1703 ┃ ┃ │                ┃
┃│┃┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛ ┃ │                ┃
┃│┣┏ 1e━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓ ┃ │                ┃
┃│┃┣8a009c7c56078723d592f239c92808069942f3 ┃ ┃ │                ┃
┃│┃┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛ ┃ │                ┃
┃│┣┏ 63━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓ ┃ │                ┃
┃│┃┣5b4f20acfd6ece4e9cf85649c2090175a49158 ┃ ┃ │                ┃
┃│┃┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛ ┃ │                ┃
┃│┣┏ 3d━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓ ┃ │                ┃
┃│┃┣780ff12681a44b987f15ab627dcea28567ac44 ┃ ┃ │                ┃
┃│┃┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛ ┃ │                ┃
┃│┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛ │                ┃
┃├HEAD                                         │                ┃
┃╰─────────────────────────────────────────────╯                ┃
┣.gitignore                                                     ┃
┣Dockerfile                                                     ┃
┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛
```


<!---TEST_CASE-->
```
this should not get updated
```
