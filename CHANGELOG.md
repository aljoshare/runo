# Changelog

## [Unreleased](https://github.com/aljoshare/runo/compare/v0.1.4...HEAD) (2023-08-20)

### Features

* rename regeneration to renewal everywhere
([b6a2436](https://github.com/aljoshare/runo/commit/b6a2436a72c4a63456e94fb3b66c7864815573e1))

### Fixes

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
