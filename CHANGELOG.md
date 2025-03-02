# Changelog

### [v0.2.16](https://github.com/aljoshare/runo/compare/v0.2.15...v0.2.16) (2025-03-02)

#### Features

* **secrets:** support regeneration of secrets after reconfiguration
([2e46778](https://github.com/aljoshare/runo/commit/2e46778c9cee3837444220aa1421b8f9656ae7c3))
* **secrets:** improve error handling
([6cd160b](https://github.com/aljoshare/runo/commit/6cd160b48cde84b4eb557b42894cf50f1df68fc3))
* **secrets:** check if pattern is valid
([44d20c4](https://github.com/aljoshare/runo/commit/44d20c4cd45e8e2360c68b828e804e0cc978ed53))
* **errors:** add error for invalid regex pattern
([79146af](https://github.com/aljoshare/runo/commit/79146af3d5e3d16bfb50315409a0435eb20d39a1))

#### Fixes

* **secrets:** replace deprecated methods
([ba305bb](https://github.com/aljoshare/runo/commit/ba305bb97a96870bcdddb7a49e90e2aba51a749f))
* **deps:** bump rand to 0.9.0 and rand_regex to 0.18.0
([82900f3](https://github.com/aljoshare/runo/commit/82900f3ac46ebc5a839c60f7d802ebc80991b38b))
* **deps:** bump serde from 1.0.217 to 1.0.218
([f55a6bc](https://github.com/aljoshare/runo/commit/f55a6bc54d42a2d63cffa9f0d66b4b2e6abe7ff1))
* **deps:** bump anyhow from 1.0.95 to 1.0.96
([a6bf3d4](https://github.com/aljoshare/runo/commit/a6bf3d400cef6bd0d8b50c22936e89ff67edf0f2))
* **deps:** bump clap from 4.5.28 to 4.5.31
([d42e224](https://github.com/aljoshare/runo/commit/d42e2240f514672a23e8d702b103faf6cc39bd29))
* **deps:** bump clap from 4.5.26 to 4.5.28
([8aa3e82](https://github.com/aljoshare/runo/commit/8aa3e82e35682544b403774f42adf003d947ee79))
* **deps:** bump serde from 1.0.216 to 1.0.217
([41ff05c](https://github.com/aljoshare/runo/commit/41ff05c4480b68fe0d5a66e524b71f1a4927dddd))
* **deps:** bump cron from 0.13.0 to 0.15.0
([82ef096](https://github.com/aljoshare/runo/commit/82ef0965c2620153a869f38b0f0a40d05c2ad3cb))
* **deps:** bump rstest from 0.23.0 to 0.24.0
([e9cb48c](https://github.com/aljoshare/runo/commit/e9cb48c23413a5c3ab568a77214bee2cd796f520))
* **deps:** bump tokio from 1.42.0 to 1.43.0
([2f8bc75](https://github.com/aljoshare/runo/commit/2f8bc755aae135e6cce068a5b400cc419976d4cb))
* **annotations:** improve output during evaluation
([b0cb8eb](https://github.com/aljoshare/runo/commit/b0cb8eb1104fae7b2e6603ccb8d6a7e231880c65))

### [v0.2.15](https://github.com/aljoshare/runo/compare/v0.2.14...v0.2.15) (2025-02-05)

#### Features

* ignore fields already set and support force overwrite
([5e4b403](https://github.com/aljoshare/runo/commit/5e4b40306cdf7511626ca7354396a24cec68f071))

#### Fixes

* **annotations:** filter checksum annotation to prevent infinite loop
([b06847d](https://github.com/aljoshare/runo/commit/b06847d55943134ed8cffb68b6b333e49cc0b98a))
* **annotations:** use values instead of keys for checksum
([793b839](https://github.com/aljoshare/runo/commit/793b839d7170398aa2bc091b56e17b4410e8ebcf))

### [v0.2.14](https://github.com/aljoshare/runo/compare/v0.2.13...v0.2.14) (2025-01-18)

#### Fixes

* **deps:** bump k8s-openapi from 0.23.0 to 0.24.0
([12a714d](https://github.com/aljoshare/runo/commit/12a714d2cea6157eb1a80e1349235f99fb4b6bb5))
* **deps:** bump kube from 0.97.0 to 0.98.0
([c1e950b](https://github.com/aljoshare/runo/commit/c1e950b233b00808da7666890985e14be4a378ba))
* **deps:** bump chrono from 0.4.38 to 0.4.39
([67ac5b5](https://github.com/aljoshare/runo/commit/67ac5b5bad6e7f27537b59816758daac42a4f2ca))
* **deps:** bump anyhow from 1.0.94 to 1.0.95
([bad11d3](https://github.com/aljoshare/runo/commit/bad11d3a59c13ec10950440fe14ceef82d07e84b))
* **deps:** bump clap from 4.5.22 to 4.5.26
([86a1dbe](https://github.com/aljoshare/runo/commit/86a1dbe0b204b0b4a3b617cf1942ed8b2d1d2ddc))
* **deps:** bump thiserror from 2.0.7 to 2.0.11
([b0f3079](https://github.com/aljoshare/runo/commit/b0f307914241eda41c78038982066e23e992f8c7))
* **deps:** bump thiserror from 2.0.4 to 2.0.7
([47cc786](https://github.com/aljoshare/runo/commit/47cc7867176ce87d0bebb4c0649e7e33f62b0626))
* **deps:** bump serde from 1.0.215 to 1.0.216
([75effad](https://github.com/aljoshare/runo/commit/75effad6dfebc20dcbf2c0579318fa818e7e2482))
* **deps:** bump tracing-subscriber from 0.3.18 to 0.3.19
([8642b86](https://github.com/aljoshare/runo/commit/8642b8661740c27149335860475ce7d69c1886d1))
* **deps:** bump hashbrown from 0.15.0 to 0.15.2
([8f9bab0](https://github.com/aljoshare/runo/commit/8f9bab0f36ecd7b5fa6be8abbe5f33714a1ee79a))
* **deps:** bump clap from 4.5.21 to 4.5.22
([a49e953](https://github.com/aljoshare/runo/commit/a49e953b7d985bd491d560e05741ea6d83324052))
* **deps:** bump anyhow from 1.0.93 to 1.0.94
([9466bc3](https://github.com/aljoshare/runo/commit/9466bc3c8a58aefb6551da28d422ccdbcf2fc450))
* **deps:** bump tracing from 0.1.40 to 0.1.41
([6251b98](https://github.com/aljoshare/runo/commit/6251b9813e35cc2d1ff8c45e7ab02656a3a040a0))
* **deps:** bump tokio from 1.41.1 to 1.42.0
([8fbbaba](https://github.com/aljoshare/runo/commit/8fbbabaed2e7a64736fa79e2957a4f7eaaa2abb1))
* **deps:** bump thiserror from 2.0.3 to 2.0.4
([7779429](https://github.com/aljoshare/runo/commit/77794296b570ce2948db9fdf49ea0ebe22494ce6))
* **deps:** bump time from 0.3.36 to 0.3.37
([fa4ba81](https://github.com/aljoshare/runo/commit/fa4ba813d86048cdfa804f07841f19e91cf943f1))
* **deps:** bump kube from 0.96.0 to 0.97.0
([3a82c93](https://github.com/aljoshare/runo/commit/3a82c93825fec3ecd2bab64ae19a2ed40608f863))
* **deps:** bump rustls from 0.23.15 to 0.23.18
([13640f7](https://github.com/aljoshare/runo/commit/13640f7b57ff22f20d825a3d06458ee1673abaab))
* **deps:** bump clap from 4.5.20 to 4.5.21
([4ac18db](https://github.com/aljoshare/runo/commit/4ac18db39d5c3dd25cc3b931bc162434e52492c6))

### [v0.2.13](https://github.com/aljoshare/runo/compare/v0.2.12...v0.2.13) (2024-11-14)

#### Features

* **secrets:** add config checksum per key
([b004051](https://github.com/aljoshare/runo/commit/b004051cd80b74aad92d00595f7359fe79c21ab2))

#### Fixes

* **deps:** bump serde from 1.0.214 to 1.0.215
([7ec0b77](https://github.com/aljoshare/runo/commit/7ec0b77fd93db3e1d13d5b4d7afdafeb2a6d17a2))
* **deps:** bump tokio from 1.41.0 to 1.41.1
([35035dc](https://github.com/aljoshare/runo/commit/35035dc8d81d6f835640f4b35b58f180a263d5a7))
* **deps:** bump cron from 0.12.1 to 0.13.0
([c179d29](https://github.com/aljoshare/runo/commit/c179d293a8de921c741580c4a176f8e17e2f9ef5))
* **deps:** bump thiserror from 1.0.68 to 2.0.3
([3f06a9e](https://github.com/aljoshare/runo/commit/3f06a9e96a4d920dfba98c21d7b80ca5a9e38232))
* **deps:** bump anyhow from 1.0.92 to 1.0.93
([b1765de](https://github.com/aljoshare/runo/commit/b1765de79f68db90556d77291ce3c529261b6069))
* **deps:** bump thiserror from 1.0.66 to 1.0.68
([43e49b8](https://github.com/aljoshare/runo/commit/43e49b805d5daa1b6ba4075869b937cb8c57a78d))
* **deps:** bump anyhow from 1.0.91 to 1.0.92
([4a3e555](https://github.com/aljoshare/runo/commit/4a3e5559da2ece433d9e56b3d57070acfd9287c6))
* **deps:** bump thiserror from 1.0.65 to 1.0.66
([0a3e1fb](https://github.com/aljoshare/runo/commit/0a3e1fbb218878caeb795c89194dfc3187e9505c))
* **deps:** bump serde from 1.0.213 to 1.0.214
([aaaf742](https://github.com/aljoshare/runo/commit/aaaf742623bd77f1b2cbd7dadaa6360a622c7386))

### [v0.2.12](https://github.com/aljoshare/runo/compare/v0.2.11...v0.2.12) (2024-10-25)

#### Fixes

* **deps:** bump kube from 0.93.1 to 0.96.0
([abc04e2](https://github.com/aljoshare/runo/commit/abc04e27bcfae148d2d8877c2254137a251af69a))
* **deps:** bump thiserror from 1.0.64 to 1.0.65
([6ff93c8](https://github.com/aljoshare/runo/commit/6ff93c82a30d433238519ac56ac2d61851332457))
* **deps:** bump regex from 1.11.0 to 1.11.1
([446efe3](https://github.com/aljoshare/runo/commit/446efe30f3cacffe7ca366372f3c52f086954e84))
* **deps:** bump anyhow from 1.0.90 to 1.0.91
([a5c3c11](https://github.com/aljoshare/runo/commit/a5c3c114e9dba7e3597017e807ffaee307c6dbba))
* **deps:** bump serde from 1.0.210 to 1.0.213
([e99393e](https://github.com/aljoshare/runo/commit/e99393ef02d4f99b3bd93e363ec162afc2ee945e))
* **deps:** bump tokio from 1.40.0 to 1.41.0
([ceed656](https://github.com/aljoshare/runo/commit/ceed6567339bc60614d3d5350603a898403e2ebc))
* **deps:** bump anyhow from 1.0.89 to 1.0.90
([514bbbd](https://github.com/aljoshare/runo/commit/514bbbdf09b04ca490623d1c93641e30ca060419))
* **deps:** bump futures from 0.3.30 to 0.3.31
([6f38b13](https://github.com/aljoshare/runo/commit/6f38b1317ec04692d530d9513d74c260c7cca2e6))
* **deps:** bump clap from 4.5.19 to 4.5.20
([e5e1505](https://github.com/aljoshare/runo/commit/e5e150534d3d1ca5c5eae3b9f02159c51a4af260))
* **deps:** bump clap from 4.5.17 to 4.5.19
([a334159](https://github.com/aljoshare/runo/commit/a3341593d6016158b224f710ad461236f00962d2))
* **deps:** bump thiserror from 1.0.63 to 1.0.64
([db1965b](https://github.com/aljoshare/runo/commit/db1965b6a77a937a780690ea6b14cbcadcb6a8ec))
* **deps:** bump regex from 1.10.6 to 1.11.0
([461abd4](https://github.com/aljoshare/runo/commit/461abd4b57218afcbe610eb2402a5c822e289c42))
* **deps:** bump rstest from 0.22.0 to 0.23.0
([58d0d8f](https://github.com/aljoshare/runo/commit/58d0d8f3d97575ad5da8b5caaf19325971462521))
* **deps:** bump anyhow from 1.0.86 to 1.0.89
([4a2a9c5](https://github.com/aljoshare/runo/commit/4a2a9c583fdbf9e655891819100d4ac69ee60135))
* **deps:** bump serde from 1.0.209 to 1.0.210
([54985ed](https://github.com/aljoshare/runo/commit/54985ed520516da300b054ce8bc308f410a1b118))
* **deps:** bump clap from 4.5.16 to 4.5.17
([5e30414](https://github.com/aljoshare/runo/commit/5e30414601baa0784565967f01811560db8ebd66))
* **deps:** bump tokio from 1.39.3 to 1.40.0
([f280df3](https://github.com/aljoshare/runo/commit/f280df3fd296432481d9dbdc8f170695d6255236))
* **deps:** bump regex from 1.10.5 to 1.10.6
([2cf0169](https://github.com/aljoshare/runo/commit/2cf01690d8396c5aa313b59d4b084fe19cfd0fd1))
* **deps:** bump serde from 1.0.204 to 1.0.209
([8c9efec](https://github.com/aljoshare/runo/commit/8c9efecff6e51571fcd8ee71c55229f96bbf70b5))
* **deps:** bump tokio from 1.39.1 to 1.39.3
([6102d6b](https://github.com/aljoshare/runo/commit/6102d6b6af29240a46d0b6a6648f3eac5e3ff6e1))
* **deps:** bump clap from 4.5.10 to 4.5.16
([d8cf2c6](https://github.com/aljoshare/runo/commit/d8cf2c62d896686056d7d2c161695957898a78eb))

### [v0.2.11](https://github.com/aljoshare/runo/compare/v0.2.10...v0.2.11) (2024-08-09)

#### Fixes

* **deps:** set version of time crate explicitly
([c58c8c0](https://github.com/aljoshare/runo/commit/c58c8c05074b925448cb7a76516e8112a7c924de))
* **deps:** bump serde from 1.0.203 to 1.0.204
([dfa134c](https://github.com/aljoshare/runo/commit/dfa134c0f21edfc975da28a64e42654e76e52912))
* **deps:** bump tokio from 1.38.1 to 1.39.1
([4292158](https://github.com/aljoshare/runo/commit/42921588e734f080a2012873fc6e5d26b13a51ba))
* **deps:** bump kube from 0.92.1 to 0.93.1
([d77c2c2](https://github.com/aljoshare/runo/commit/d77c2c26cf1cef372eba161ff69ab2369e6a2be1))
* **deps:** bump clap from 4.5.9 to 4.5.10
([1f33765](https://github.com/aljoshare/runo/commit/1f33765d1df4c7ed6453548b22acc01fb709a343))
* **deps:** bump clap from 4.5.8 to 4.5.9
([7dfc77b](https://github.com/aljoshare/runo/commit/7dfc77b3d3ae656cefc074f549244b3d1e92dcb5))
* **deps:** bump tokio from 1.38.0 to 1.38.1
([9fa4775](https://github.com/aljoshare/runo/commit/9fa4775a37349570aa66725ab8b3a723d29cfc1d))
* **deps:** bump thiserror from 1.0.62 to 1.0.63
([eb2b908](https://github.com/aljoshare/runo/commit/eb2b908743167dafd874d3a7130e27736cf0e00b))
* **deps:** bump thiserror from 1.0.61 to 1.0.62
([8d65675](https://github.com/aljoshare/runo/commit/8d65675939d2f3831ff573e223afa9cd6cdc64fd))
* **deps:** bump clap from 4.5.7 to 4.5.8
([b94378f](https://github.com/aljoshare/runo/commit/b94378fc68ccd552af9ee0e3498587dd9a27940c))

### [v0.2.10](https://github.com/aljoshare/runo/compare/v0.2.9...v0.2.10) (2024-06-22)

#### Fixes

* **deps:** bump kube to 0.92.1 and k8s-openapi to 0.22.0
([35c12ca](https://github.com/aljoshare/runo/commit/35c12ca22ae8c7e62d65889574728dfa7e94e6d4))
* **deps:** bump regex from 1.10.4 to 1.10.5
([6ba8907](https://github.com/aljoshare/runo/commit/6ba89079bf9800dd50999f4ced0f027dcd378271))
* **deps:** bump actix-web from 4.6.0 to 4.8.0
([bf43e1a](https://github.com/aljoshare/runo/commit/bf43e1ae5fa2aa068f48ef58fcbd4584839c330b))
* **deps:** bump clap from 4.5.6 to 4.5.7
([c7ca904](https://github.com/aljoshare/runo/commit/c7ca9049e79d6ad122b5bf58cb2ce4b391832189))
* **deps:** bump anyhow from 1.0.83 to 1.0.86
([6fde64c](https://github.com/aljoshare/runo/commit/6fde64c41a0f6d92f0440ac6d2d59a7b14223a66))
* **deps:** bump clap from 4.5.4 to 4.5.6
([60cf0c5](https://github.com/aljoshare/runo/commit/60cf0c5b023a973075947b19d4f8a8654543c3bc))
* **deps:** bump tokio from 1.37.0 to 1.38.0
([dec41f4](https://github.com/aljoshare/runo/commit/dec41f4c6507c8c5431f7b7f7d7bc3b8b8e13b65))
* **deps:** bump serde from 1.0.201 to 1.0.203
([b55c9ab](https://github.com/aljoshare/runo/commit/b55c9ab4f81be2cad2d33f038721af4b67ca43a6))
* **deps:** bump actix-web from 4.5.1 to 4.6.0
([5b3a0c2](https://github.com/aljoshare/runo/commit/5b3a0c225c99cb0e8a1847c2ee730300718caab6))
* **deps:** bump thiserror from 1.0.60 to 1.0.61
([70c0059](https://github.com/aljoshare/runo/commit/70c0059d6a061ae6781ec9493afcb199c6d62340))
* **deps:** bump serde from 1.0.200 to 1.0.201
([d98880d](https://github.com/aljoshare/runo/commit/d98880de1b38d5b4b57e37ca0557d616edd7f7a2))
* **deps:** bump thiserror from 1.0.59 to 1.0.60
([0b30f90](https://github.com/aljoshare/runo/commit/0b30f9096cf7633793a9639af637a6f70268046e))
* **deps:** bump anyhow from 1.0.82 to 1.0.83
([d3dec48](https://github.com/aljoshare/runo/commit/d3dec48a7293be3206d4de2a616f9b84c5700ebd))

### [v0.2.9](https://github.com/aljoshare/runo/compare/v0.2.8...v0.2.9) (2024-05-05)

#### Fixes

* **deps:** bump serde from 1.0.199 to 1.0.200
([e0325f7](https://github.com/aljoshare/runo/commit/e0325f76b50b4761bb8989a764da7c957c48186f))
* **deps:** bump serde from 1.0.198 to 1.0.199
([5a362d4](https://github.com/aljoshare/runo/commit/5a362d43e40980588143b0a1cba6d98d6d6d387f))
* **deps:** bump thiserror from 1.0.58 to 1.0.59
([54cba6c](https://github.com/aljoshare/runo/commit/54cba6c1bb665adb95420222473ad43e66d35f37))
* **deps:** bump serde from 1.0.197 to 1.0.198
([bba0fb4](https://github.com/aljoshare/runo/commit/bba0fb486a9c259f3dc2418152cad2b18c85fce7))
* **deps:** bump chrono from 0.4.37 to 0.4.38
([85f833d](https://github.com/aljoshare/runo/commit/85f833dad6b1d8c31b877e00bfc78287d5d327de))
* **deps:** bump rustls from 0.23.4 to 0.23.5
([33a7b34](https://github.com/aljoshare/runo/commit/33a7b34e26f83b158564a0d7731a25ece35dd443))
* **deps:** bump anyhow from 1.0.81 to 1.0.82
([2ee38c9](https://github.com/aljoshare/runo/commit/2ee38c9fefe1400332745214a9d72c9769fcfbcf))

### [v0.2.8](https://github.com/aljoshare/runo/compare/v0.2.7...v0.2.8) (2024-04-06)

#### Fixes

* **deps:** bump serde_yaml from 0.9.33 to 0.9.34+deprecated
([1b9c866](https://github.com/aljoshare/runo/commit/1b9c86698f76118ebb2650b0b9be73b83b4de32c))
* **deps:** bump kube from 0.89.0 to 0.90.0
([a959d4b](https://github.com/aljoshare/runo/commit/a959d4bba04476a48e357c2a45c841ea21fd5c04))
* **deps:** bump tokio from 1.36.0 to 1.37.0
([59c8bbb](https://github.com/aljoshare/runo/commit/59c8bbb53bb0e522e911f10b3d6655cd36ddea99))
* **deps:** bump clap from 4.5.3 to 4.5.4
([a0c5dfd](https://github.com/aljoshare/runo/commit/a0c5dfd5a237c15aca9fce4140063433422432cf))
* **deps:** bump h2 from 0.3.24 to 0.3.26
([4558ade](https://github.com/aljoshare/runo/commit/4558ade99c7f9da446d4a1be09ac86da3aa742b0))
* **deps:** bump chrono from 0.4.35 to 0.4.37
([1610da0](https://github.com/aljoshare/runo/commit/1610da0bcaf1625177d9d88e545a11158a3e1e8f))
* **deps:** bump kube from 0.88.1 to 0.89.0
([ae26f24](https://github.com/aljoshare/runo/commit/ae26f248e23ce6b4190868cc419f0a688d8bc343))
* **deps:** bump regex from 1.10.3 to 1.10.4
([6970f19](https://github.com/aljoshare/runo/commit/6970f1988f874b8beb52352e22b75b06bef35218))
* **deps:** bump serde_yaml from 0.9.32 to 0.9.33
([1f50755](https://github.com/aljoshare/runo/commit/1f50755cae5819ac03a23bdfe3a43fbf83e716e1))
* **deps:** bump clap from 4.5.2 to 4.5.3
([cadbe3a](https://github.com/aljoshare/runo/commit/cadbe3a2894394ec3b927950e946cb133c6dc568))
* **deps:** bump anyhow from 1.0.80 to 1.0.81
([9d06259](https://github.com/aljoshare/runo/commit/9d06259db047b8d39b01be31c0711276f9ab1c39))
* **deps:** bump thiserror from 1.0.57 to 1.0.58
([e8fd7b4](https://github.com/aljoshare/runo/commit/e8fd7b422413722ede24ea8ee7061c88fe0cf73c))
* **deps:** bump clap from 4.5.1 to 4.5.2
([40b6162](https://github.com/aljoshare/runo/commit/40b6162584b171cc63dd049c295c6595bf43b958))
* **deps:** bump chrono from 0.4.34 to 0.4.35
([fb8d757](https://github.com/aljoshare/runo/commit/fb8d7578dde0411293acc5e39c6b79fc93a39393))

### [v0.2.7](https://github.com/aljoshare/runo/compare/v0.2.6...v0.2.7) (2024-03-06)

#### Fixes

* **deps:** bump mio from 0.8.9 to 0.8.11
([75afe86](https://github.com/aljoshare/runo/commit/75afe861df769f4d7193d4684b32db4f7b9fd27b))
* **deps:** bump cron from 0.12.0 to 0.12.1
([0ff1413](https://github.com/aljoshare/runo/commit/0ff1413e0255a1e02d060ae4ad4afe688f1eb03d))
* **deps:** bump clap from 4.5.0 to 4.5.1
([ed63b68](https://github.com/aljoshare/runo/commit/ed63b684ca8575651d050b672a0f185a210039c4))
* **deps:** bump anyhow from 1.0.79 to 1.0.80
([0a4aa58](https://github.com/aljoshare/runo/commit/0a4aa588cead2836a0d60cc036d26038537abf23))
* **deps:** bump k8s-openapi from 0.21.0 to 0.21.1
([1efb21f](https://github.com/aljoshare/runo/commit/1efb21f7277f4db3572e230eb21cbccff3ac9384))
* **deps:** bump serde_yaml from 0.9.31 to 0.9.32
([7a8d105](https://github.com/aljoshare/runo/commit/7a8d10516b914c21ee53f0952daf48842a48eb02))
* **deps:** bump serde from 1.0.196 to 1.0.197
([23b5bfa](https://github.com/aljoshare/runo/commit/23b5bfa5bc51d4364db72663034fc8efbd6ff72e))
* **deps:** bump assert_cmd from 2.0.13 to 2.0.14
([9f6e5c2](https://github.com/aljoshare/runo/commit/9f6e5c2ce19992adddd436c712f8fdb85ebd2402))

### [v0.2.6](https://github.com/aljoshare/runo/compare/v0.2.5...v0.2.6) (2024-02-13)

#### Fixes

* **deps:** bump actix-web from 4.4.1 to 4.5.1
([8283deb](https://github.com/aljoshare/runo/commit/8283deb11a665ae6e5c65ede0fb6d8e62001bbb4))
* **deps:** bump thiserror from 1.0.56 to 1.0.57
([1f5ca78](https://github.com/aljoshare/runo/commit/1f5ca78e06b15fb746ec04be0e4f0769d05adf17))
* **deps:** bump chrono from 0.4.33 to 0.4.34
([d2f6d24](https://github.com/aljoshare/runo/commit/d2f6d24d4fde0ae41c4be5cc7a69da02e8bad91e))
* **deps:** bump clap from 4.4.18 to 4.5.0
([81ad514](https://github.com/aljoshare/runo/commit/81ad5146a154972cae76ac851d87c06c055b52eb))

### [v0.2.5](https://github.com/aljoshare/runo/compare/v0.2.4...v0.2.5) (2024-02-12)

#### Fixes

* **deps:** bump k8s-openapi from 0.20.0 to 0.21.0 and kube from 0.87.2 to
0.88.1
([61276c1](https://github.com/aljoshare/runo/commit/61276c16865063be17440422088145e6f96e5858))
* **deps:** bump serde from 1.0.195 to 1.0.196
([7113370](https://github.com/aljoshare/runo/commit/71133707b606a7afcfd13474326eb52758f82c15))
* **deps:** bump serde_yaml from 0.9.30 to 0.9.31
([658dc9a](https://github.com/aljoshare/runo/commit/658dc9ab052b812a1e203f676d5dc01853670eb8))
* **deps:** bump tokio from 1.35.1 to 1.36.0
([04da35f](https://github.com/aljoshare/runo/commit/04da35f52c86e963612d20d961951199b2a7ba73))
* **deps:** bump chrono from 0.4.31 to 0.4.33
([b0f8a23](https://github.com/aljoshare/runo/commit/b0f8a23e00e7f44443ca520ac1d9edd394ec8fed))
* **deps:** bump regex from 1.10.2 to 1.10.3
([6eaa9cf](https://github.com/aljoshare/runo/commit/6eaa9cf9c22332293e64a944ff3cc5ef1123dd36))
* **deps:** bump h2 from 0.3.21 to 0.3.24
([5945f9a](https://github.com/aljoshare/runo/commit/5945f9af042d3c031734ecd972afdacd532ab46f))
* **deps:** bump clap from 4.4.15 to 4.4.18
([c4ca1f6](https://github.com/aljoshare/runo/commit/c4ca1f692d2b89226f75fae98462ffdd873070a1))
* **deps:** bump rand_regex from 0.16.0 to 0.17.0
([5136b13](https://github.com/aljoshare/runo/commit/5136b13cc343ca463e56a445051239df6d6fd9d6))
* **deps:** bump assert_cmd from 2.0.12 to 2.0.13
([d5d7fb7](https://github.com/aljoshare/runo/commit/d5d7fb7bb045b0898dd14066fe2fdb47f1c984c9))
* **deps:** bump clap from 4.4.13 to 4.4.15
([207c621](https://github.com/aljoshare/runo/commit/207c6213617b1e5bf2cfc151ac94a9b555d59b95))
* **deps:** bump serde_yaml from 0.9.29 to 0.9.30
([f6b6f1b](https://github.com/aljoshare/runo/commit/f6b6f1b3d88ba702f6d13eaafdf88b49d3402ba8))
* **deps:** bump serde from 1.0.193 to 1.0.195
([c5a6a73](https://github.com/aljoshare/runo/commit/c5a6a734787d56e5e2c8932951995fbcd36b630f))

### [v0.2.4](https://github.com/aljoshare/runo/compare/v0.2.3...v0.2.4) (2024-01-07)

#### Fixes

* **deps:** bump clap from 4.4.12 to 4.4.13
([a0c21b3](https://github.com/aljoshare/runo/commit/a0c21b39c9c406a1743c21299291f83ea1cd8215))
* **deps:** bump futures from 0.3.29 to 0.3.30
([8461672](https://github.com/aljoshare/runo/commit/846167248d5856ca4c524c6f5d81b59ff63534e6))
* **deps:** bump actix-web from 4.4.0 to 4.4.1
([e4f7459](https://github.com/aljoshare/runo/commit/e4f74599ad2fc1accbce814d83b80b322c5bbd1a))
* **deps:** bump thiserror from 1.0.50 to 1.0.56
([8b61264](https://github.com/aljoshare/runo/commit/8b61264fa31c704d27fe003c65537ea62a6260cc))
* **deps:** bump anyhow from 1.0.75 to 1.0.79
([1882545](https://github.com/aljoshare/runo/commit/188254539446a467c5abdb6bee56dcffa73d79b0))
* **deps:** bump kube from 0.87.1 to 0.87.2
([ef0214e](https://github.com/aljoshare/runo/commit/ef0214e9695cc0f9bf1f38a856a4c077a77b0e17))
* **deps:** bump serde_yaml from 0.9.27 to 0.9.29
([ab43a79](https://github.com/aljoshare/runo/commit/ab43a793434aa984ad9a47e71a12bcdf2703fd7d))
* **deps:** bump tokio from 1.34.0 to 1.35.1
([a85048b](https://github.com/aljoshare/runo/commit/a85048b42a36dbb0ccefe01fbf92397b588602de))
* **deps:** bump unsafe-libyaml from 0.2.9 to 0.2.10
([d951068](https://github.com/aljoshare/runo/commit/d95106824977de3a83e265542b9818391b612d26))
* **deps:** bump clap from 4.4.10 to 4.4.12
([a3f7acf](https://github.com/aljoshare/runo/commit/a3f7acfabcd16aaedbd91a5d02f869e92c2dd761))

### [v0.2.3](https://github.com/aljoshare/runo/compare/v0.2.2...v0.2.3) (2023-11-30)

#### Fixes

* **deps:** Bump serde from 1.0.190 to 1.0.193
([2eaed9e](https://github.com/aljoshare/runo/commit/2eaed9e9e096511c10a4428583c688fbe8ad0c58))
* **deps:** Bump rustix from 0.38.11 to 0.38.25
([4fea569](https://github.com/aljoshare/runo/commit/4fea569850135efc0f11261dc1a3a4e2d378e546))
* **deps:** Bump clap from 4.4.7 to 4.4.10
([bca2925](https://github.com/aljoshare/runo/commit/bca2925579e91e21f8a1a9825fe57a87c3b501c4))
* **deps:** Bump tracing-subscriber from 0.3.17 to 0.3.18
([2e5e956](https://github.com/aljoshare/runo/commit/2e5e95611ece3467199ed95e6270b9778431518c))
* **deps:** Bump tokio from 1.33.0 to 1.34.0
([696d05f](https://github.com/aljoshare/runo/commit/696d05f9acaedd805520373b6b0167efbe9dad73))
* **deps:** Bump kube from 0.86.0 to 0.87.1
([03e82b4](https://github.com/aljoshare/runo/commit/03e82b432d215cbd469be92e3648214fef22f56e))
* **deps:** Bump serde_yaml from 0.9.25 to 0.9.27
([5366a6b](https://github.com/aljoshare/runo/commit/5366a6b08156650245b6a154f82142eb6f5c73b1))
* **deps:** Bump futures from 0.3.28 to 0.3.29
([93478fd](https://github.com/aljoshare/runo/commit/93478fd54ceca0ae5d094097a5964335ac0eafc5))
* **deps:** Bump serde from 1.0.189 to 1.0.190
([ca6fead](https://github.com/aljoshare/runo/commit/ca6feadc183e01c2d8bec9b926730f763416736c))
* **deps:** Bump clap from 4.4.6 to 4.4.7
([0f28975](https://github.com/aljoshare/runo/commit/0f289758465229bfdf13842d41db548c3fd40468))
* **deps:** Bump thiserror from 1.0.49 to 1.0.50
([0499afb](https://github.com/aljoshare/runo/commit/0499afb9f1ab1dfe0203abab4b9841f4f7d76d2b))
* **deps:** Bump tracing from 0.1.39 to 0.1.40
([5237884](https://github.com/aljoshare/runo/commit/52378842fef28884fe4199204cdca25d005fd8b8))
* **deps:** Bump tracing from 0.1.37 to 0.1.39
([c9ece9f](https://github.com/aljoshare/runo/commit/c9ece9f57f84ce609731d84349a9dcc79f83e19e))
* **deps:** Bump regex from 1.10.0 to 1.10.2
([baa0596](https://github.com/aljoshare/runo/commit/baa05967cfd4b5f608d23d2312c88a02ae3716e0))

### [v0.2.2](https://github.com/aljoshare/runo/compare/v0.2.1...v0.2.2) (2023-10-16)

#### Features

* support a more abstract configuration and use it to pass requeue duration as
param
([5c31c89](https://github.com/aljoshare/runo/commit/5c31c89cc70cf15986a22b0a5d3215def150316f))

#### Fixes

* **deps:** Bump serde from 1.0.188 to 1.0.189
([4b8467d](https://github.com/aljoshare/runo/commit/4b8467db3bcf9c6c678e28d3d4931a3d5ac65d16))
* **deps:** Bump regex from 1.9.6 to 1.10.0
([e78e608](https://github.com/aljoshare/runo/commit/e78e608728cd7a043f8a50aaca3a07a0b1c1cb31))
* **deps:** Bump tokio from 1.32.0 to 1.33.0
([91fcf77](https://github.com/aljoshare/runo/commit/91fcf779ddd79e1518f6bef1fd556b72c5d3bc46))
* **deps:** Bump regex from 1.9.5 to 1.9.6
([6244f5a](https://github.com/aljoshare/runo/commit/6244f5a5d77766b959bfe9ff5fe1f575ea49b99c))
* **deps:** Bump clap from 4.4.5 to 4.4.6
([0605a47](https://github.com/aljoshare/runo/commit/0605a470b14dfc9ee5867a512061e9c6696b843c))
* **deps:** Bump thiserror from 1.0.48 to 1.0.49
([1af1734](https://github.com/aljoshare/runo/commit/1af173442dcd926dcf966d7fbbe4e412f68dc737))
* **deps:** Bump clap from 4.4.4 to 4.4.5
([63aab2f](https://github.com/aljoshare/runo/commit/63aab2f9dd399400d2e87bb6f13135d26da730e0))
* **deps:** Bump clap from 4.4.3 to 4.4.4
([465434e](https://github.com/aljoshare/runo/commit/465434eaf327a35fcc4dbc4a20a25060be3bcd20))
* **deps:** Bump chrono from 0.4.30 to 0.4.31
([80e1f51](https://github.com/aljoshare/runo/commit/80e1f519119300e6c99cbe28420c27a203c3b736))
* **deps:** Bump clap from 4.4.2 to 4.4.3
([dc4ee21](https://github.com/aljoshare/runo/commit/dc4ee217cb047a92bfb72f6b2cd262c4da2bcf97))

### [v0.2.1](https://github.com/aljoshare/runo/compare/v0.2.0...v0.2.1) (2023-09-13)

#### Fixes

* **deps:** bump k8s-openapi to 0.20.0
([f2012d8](https://github.com/aljoshare/runo/commit/f2012d88a905896d8ac67558fc42e6c8aa832d0a))
* **deps:** bump kube to 0.86.0
([336c4f9](https://github.com/aljoshare/runo/commit/336c4f95ca60935fa141c9049290ded4c5c55dd6))
* **deps:** bump chrono to 0.4.30
([dbc229b](https://github.com/aljoshare/runo/commit/dbc229ba6388b6451a895de94e2b824f45b6bb6f))

## [v0.2.0](https://github.com/aljoshare/runo/compare/v0.1.5...v0.2.0) (2023-09-04)

### Features

* rename regeneration to renewal everywhere
([486a83b](https://github.com/aljoshare/runo/commit/486a83b8da18ef68b8982d5c10bb6a10290ce3fa))
* **reconciler:** check if the secrets has the managed label
([cdc3ff1](https://github.com/aljoshare/runo/commit/cdc3ff11a5b24ab66a166e2c524ad2f6660da28d))
* add labels.rs
([a60bb4b](https://github.com/aljoshare/runo/commit/a60bb4bb4790a1dff8062719b2ee542358e0cbb1))
* add support for one-shot mode
([ac7660f](https://github.com/aljoshare/runo/commit/ac7660fd41f54f5bd5ada47e5374567036fab60f))

### Fixes

* **deps:** Bump clap from 4.4.1 to 4.4.2
([3470c7b](https://github.com/aljoshare/runo/commit/3470c7b44bb428eea3d6ed09906eef97314ba694))
* **deps:** Bump chrono from 0.4.26 to 0.4.28
([5366301](https://github.com/aljoshare/runo/commit/5366301f70112fdbd12bf096e7638298be995473))
* **deps:** Bump actix-web from 4.3.1 to 4.4.0
([4f13d0c](https://github.com/aljoshare/runo/commit/4f13d0c1081586c4071129318c6e3dbca256df3a))
* **deps:** Bump clap from 4.4.0 to 4.4.1
([4fddfe4](https://github.com/aljoshare/runo/commit/4fddfe4710b797aab06231bcbbefbbffc8bd86f3))
* **deps:** Bump regex from 1.9.3 to 1.9.4
([d3d7d4a](https://github.com/aljoshare/runo/commit/d3d7d4afa4e9dce501833d3659b49b86033ce5a6))
* **annotations:** change default pattern to ascii-only
([8246393](https://github.com/aljoshare/runo/commit/8246393ed359a19e3c4ae99943323c6097589ae4))
* **deps:** Bump clap from 4.3.24 to 4.4.0
([3c468c1](https://github.com/aljoshare/runo/commit/3c468c12a00a27720820147fd5b7dbbf47c2db86))
* **deps:** Bump clap from 4.3.23 to 4.3.24
([0a89f95](https://github.com/aljoshare/runo/commit/0a89f95807b29b9f83c8dae18fa1f0f126499c5a))
* **deps:** Bump clap from 4.3.19 to 4.3.23
([319894e](https://github.com/aljoshare/runo/commit/319894e6aa4c6460725980ffb361a09fd035c93b))
* **deps:** Bump thiserror from 1.0.46 to 1.0.47
([3ff720c](https://github.com/aljoshare/runo/commit/3ff720cc1b8d299b93b4a34594ef130a161c61b2))
* **deps:** Bump anyhow from 1.0.74 to 1.0.75
([e03aedf](https://github.com/aljoshare/runo/commit/e03aedfc38ac9f090e05b2cc3e99903637919a05))
* **deps:** Bump tokio from 1.30.0 to 1.32.0
([6f46f16](https://github.com/aljoshare/runo/commit/6f46f16b5a225247ab1c14fce3d054c5d008772f))
* **deps:** Bump thiserror from 1.0.44 to 1.0.46
([98f8bd6](https://github.com/aljoshare/runo/commit/98f8bd6387971638c9e3cf5a866bd019014cce98))
* **deps:** bump rand_regex from 0.15.1 to 0.16.0
([46f74f4](https://github.com/aljoshare/runo/commit/46f74f418c0de0c7a0d61c9ed4d1f0da673a81ee))
* **deps:** Bump anyhow from 1.0.72 to 1.0.74
([3a8c009](https://github.com/aljoshare/runo/commit/3a8c00958795538d261fd67e1c58dd41e9dd88e4))
* **annotations:** linting error
([d6832cc](https://github.com/aljoshare/runo/commit/d6832cc80e04403b7f2a855d4fba45d8d5a231bf))
* **deps:** bump tokio from 1.29.1 to 1.30.0
([ed3b010](https://github.com/aljoshare/runo/commit/ed3b0108e90b09e34445b2a9f68570e68ca4b674))
* **deps:** bump k8s-openapi from 0.18.0 to 0.19.0
([60ff566](https://github.com/aljoshare/runo/commit/60ff566fec3edddd42ec41380c10f6e0f2e48a86))
* **deps:** bump kube from 0.84.0 to 0.85.0
([fe37566](https://github.com/aljoshare/runo/commit/fe3756600b2bd08a27a0e9da5bfb3ae43c005c6b))
* **deps:** bump regex from 1.9.1 to 1.9.3
([b5bed5e](https://github.com/aljoshare/runo/commit/b5bed5e9ef4235b7980df620051fe61f9f0d8c9a))
* **deps:** bump clap from 4.3.17 to 4.3.19
([f2f1fa2](https://github.com/aljoshare/runo/commit/f2f1fa2459d1435e88252be5a5cd689b32544e25))
* **deps:** bump thiserror from 1.0.43 to 1.0.44
([f5e25ec](https://github.com/aljoshare/runo/commit/f5e25ec3ebe04a4b0eb8fef852e92346a843452e))
* **deps:** bump serde_yaml from 0.9.22 to 0.9.25
([5a9f593](https://github.com/aljoshare/runo/commit/5a9f59307b5bd01ad039afbe4e6f9b11beae4836))

### [v0.1.5](https://github.com/aljoshare/runo/compare/v0.1.4...v0.1.5) (2023-08-26)

#### Features

* rename regeneration to renewal everywhere
([b6a2436](https://github.com/aljoshare/runo/commit/b6a2436a72c4a63456e94fb3b66c7864815573e1))

#### Fixes

* **annotations:** change default pattern to ascii-only
([b99a846](https://github.com/aljoshare/runo/commit/b99a8460dc13d5a4895b6864161af63ed3e28c15))
* **deps:** bump rand_regex from 0.15.1 to 0.16.0
([40774c7](https://github.com/aljoshare/runo/commit/40774c7e0c03806306de866c2d5e2f8fcd157855))
* **annotations:** linting error
([fd19066](https://github.com/aljoshare/runo/commit/fd19066b76c81192f9a4ee28950ea679df20545a))

### [v0.1.4](https://github.com/aljoshare/runo/compare/v0.1.3...v0.1.4) (2023-08-13)

#### Features

* **reconciler:** check if the secrets has the managed label
([268f33d](https://github.com/aljoshare/runo/commit/268f33d0e57e413b1179268c6ac20dc4a9c57bf1))
* add labels.rs
([a3684b2](https://github.com/aljoshare/runo/commit/a3684b25ffb932625701b0bc413832a406f1cb5d))
* add support for one-shot mode
([a7f4315](https://github.com/aljoshare/runo/commit/a7f43156c589ee57e09a01cc683eaa59ae5182fe))

#### Fixes

* **deps:** bump k8s-openapi from 0.18.0 to 0.19.0
([f3ca14c](https://github.com/aljoshare/runo/commit/f3ca14c33ea73b674242b11252dcd6eef16edf66))
* **deps:** bump kube from 0.84.0 to 0.85.0
([cc26b7e](https://github.com/aljoshare/runo/commit/cc26b7e9a375297c08497f6f4499fb1000719695))
* **deps:** bump regex from 1.9.1 to 1.9.3
([573d18f](https://github.com/aljoshare/runo/commit/573d18fe0619efa3b18cb4fdc7ab1c393e609185))
* **deps:** bump thiserror from 1.0.43 to 1.0.44
([c730c4e](https://github.com/aljoshare/runo/commit/c730c4eb3f483b24eb8cf1114d6649c6b50e2606))
* **deps:** bump clap from 4.3.17 to 4.3.19
([ca85973](https://github.com/aljoshare/runo/commit/ca8597342dc9ba2456f052496ffd1f10d78eefa5))
* **deps:** bump anyhow from 1.0.71 to 1.0.72
([748aef0](https://github.com/aljoshare/runo/commit/748aef08b65a19c1822f6492940db1135e1428d3))
* **deps:** bump serde_yaml from 0.9.22 to 0.9.25
([d7eae09](https://github.com/aljoshare/runo/commit/d7eae09568361d4d60f990033ff115436f5b7653))
* **deps:** bump clap from 4.3.11 to 4.3.17
([058e824](https://github.com/aljoshare/runo/commit/058e8244559c85e1d16ca9a50fbb4950aa55f6f2))
* **deps:** bump kube from 0.83.0 to 0.84.0
([fce557a](https://github.com/aljoshare/runo/commit/fce557ae8540568ece99e87962359be9f474726c))

### [v0.1.3](https://github.com/aljoshare/runo/compare/v0.1.2...v0.1.3) (2023-07-11)

#### Features

* support dry-run mode
([d8d1c46](https://github.com/aljoshare/runo/commit/d8d1c46929f943da60deb5f5c08be79011cb3fd9))
* restructure code and support multiple fields
([87fc23c](https://github.com/aljoshare/runo/commit/87fc23cc3c5d98fc632aebe500ea6bee1907c722))
* **annotations:** add method for getting all ids for a secret
([10e44a1](https://github.com/aljoshare/runo/commit/10e44a1d97cbe6f60a3f4702eb27ed101a527ed4))
* add helper methods for kubernetes
([fb87ba7](https://github.com/aljoshare/runo/commit/fb87ba737cb64aecf0089d0e30e33ebee15a67d1))
* add health endpoint
([4c83734](https://github.com/aljoshare/runo/commit/4c83734c472af16a8ed461c92ed6d4f429ce1895))

#### Fixes

* **deps:** bump regex from 1.9.0 to 1.9.1
([ed925b3](https://github.com/aljoshare/runo/commit/ed925b3c502d503de011537124018b42cc578997))
* **deps:** bump serde from 1.0.167 to 1.0.168
([ce9a080](https://github.com/aljoshare/runo/commit/ce9a080c8290ade858bf4052d9987be9425f2e1f))
* **deps:** bump serde from 1.0.166 to 1.0.167
([4fc25a8](https://github.com/aljoshare/runo/commit/4fc25a8421ecdd262c50cde030ac0ccd7d438f3d))
* **deps:** bump thiserror from 1.0.41 to 1.0.43
([5035fe7](https://github.com/aljoshare/runo/commit/5035fe78d079320b92b6a0cec446e36d6e082cd5))
* **deps:** bump regex from 1.8.4 to 1.9.0
([0896124](https://github.com/aljoshare/runo/commit/08961243fb17ff81d83e7331425e50abcbb193b1))
* **deps:** bump thiserror from 1.0.40 to 1.0.41
([ad3965b](https://github.com/aljoshare/runo/commit/ad3965b4e6bf591a0723afc804ac3704339a7dab))

### [v0.1.2](https://github.com/aljoshare/runo/compare/v0.1.1...v0.1.2) (2023-07-05)

#### Fixes

* **deps:** bump serde from 1.0.165 to 1.0.166
([53e123e](https://github.com/aljoshare/runo/commit/53e123efa0ae3e0b636461fab100ed88899c2abd))

### [v0.1.1](https://github.com/aljoshare/runo/compare/v0.1.0...v0.1.1) (2023-07-04)

#### Fixes

* **deps:** bump serde from 1.0.164 to 1.0.165
([069b348](https://github.com/aljoshare/runo/commit/069b3486d1811d157ca73ace947073f1f6944953))

## v0.1.0 (2023-06-30)

### Features

* add initial version of runo
([d5b8120](https://github.com/aljoshare/runo/commit/d5b81200a199f9396cfd081732c95e3221a123d6))
* initial commit
([c8b4578](https://github.com/aljoshare/runo/commit/c8b457893a3474387d2b70ebcfaaecfa9d974056))
