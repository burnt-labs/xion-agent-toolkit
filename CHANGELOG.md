# Changelog

All notable changes to the Xion Agent Toolkit will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.9.0](https://github.com/burnt-labs/xion-agent-toolkit/compare/0.8.2...0.9.0) (2026-03-18)


### Features

* **account:** add account info command ([53776c8](https://github.com/burnt-labs/xion-agent-toolkit/commit/53776c8527a822ccc48e34e485f0ebad9b47f824))
* **account:** add MetaAccount info command ([157b354](https://github.com/burnt-labs/xion-agent-toolkit/commit/157b3542d5665518b73cce1894324cc17d52e7c5))
* **account:** rewrite account info to use OAuth2 API ([d0f1d26](https://github.com/burnt-labs/xion-agent-toolkit/commit/d0f1d262a09d7056d31acb5126219b8a6c5a33ae))
* Asset Builder, Batch Operations, Extended Grants, and MetaAccount Info ([057edc0](https://github.com/burnt-labs/xion-agent-toolkit/commit/057edc05615dec7a6b0b5d8d54e41406c28f7caf))
* **asset:** add Asset Builder module for CW721 NFT deployment and minting ([4fc0919](https://github.com/burnt-labs/xion-agent-toolkit/commit/4fc0919ed52338a2324b16898ff37d3e21d0ea7d))
* **asset:** add CW721 variant support for Phase 2 ([f2874ce](https://github.com/burnt-labs/xion-agent-toolkit/commit/f2874ce8b10a5713a69182338bd19ffe2b45f987))
* **asset:** add Phase 3 features - address prediction and batch minting ([4c405cb](https://github.com/burnt-labs/xion-agent-toolkit/commit/4c405cbfe54c8640d36f371421265365fa327882))
* **batch:** add batch operations for multi-message transactions ([7e2e2de](https://github.com/burnt-labs/xion-agent-toolkit/commit/7e2e2de79643e39a36312786b04839de08c3e8e2))
* **ci:** add release-please for automated release management ([8f534ae](https://github.com/burnt-labs/xion-agent-toolkit/commit/8f534aef197159c5adaffb31f215637c09f8d145))
* **cli:** add --install flag for shell completion auto-installation ([c569813](https://github.com/burnt-labs/xion-agent-toolkit/commit/c56981339754871f3e30b7e76abb89a6d369469a))
* **cli:** add CI/CD integration output formats ([559c48f](https://github.com/burnt-labs/xion-agent-toolkit/commit/559c48f7bd7d3f2e36ff84cd78d86faad40c56bd))
* **cli:** add contract execute command ([017d44f](https://github.com/burnt-labs/xion-agent-toolkit/commit/017d44f0b2ae5dee83121db02270718aa1ffc159))
* **cli:** add contract query and treasury export/import commands ([35ee428](https://github.com/burnt-labs/xion-agent-toolkit/commit/35ee4289b41c2ab1f4b9deb31181d13bdc6be6a5))
* **cli:** add contract query and treasury export/import commands ([ddfde4e](https://github.com/burnt-labs/xion-agent-toolkit/commit/ddfde4e1e8eee046d669492f338a8f0811431670))
* **cli:** add generic contract instantiation commands ([ae430bc](https://github.com/burnt-labs/xion-agent-toolkit/commit/ae430bc456be5fb4df6a9aaa2bf53f52b823391a))
* **cli:** add shell completion support ([98cef04](https://github.com/burnt-labs/xion-agent-toolkit/commit/98cef046bd820c8814cd46a3ac1841fb5c62e726))
* **cli:** add shell completion support with --install flag ([9edbb94](https://github.com/burnt-labs/xion-agent-toolkit/commit/9edbb948ab9cbdab9d44cb4e7513184b6c21b83a))
* **cli:** Complete Phase 2.4 - CLI integration ([b1a53d9](https://github.com/burnt-labs/xion-agent-toolkit/commit/b1a53d935a0aa0d5d2db4fd7abea7803b3131452))
* **cli:** Initialize Xion Agent Toolkit with basic CLI framework ([0eaff64](https://github.com/burnt-labs/xion-agent-toolkit/commit/0eaff6441cc2e3a5fa7163038db514ef07d1f507))
* **config:** Enhance configuration management and OAuth2 endpoint discovery ([cbb9a50](https://github.com/burnt-labs/xion-agent-toolkit/commit/cbb9a5019ed5bc4f75614db5f3444bf72fcec71d))
* **config:** replace OS keyring with AES-256-GCM encrypted files ([8f6c64b](https://github.com/burnt-labs/xion-agent-toolkit/commit/8f6c64bbcf261426c04a6551c26c789f1fbb2173))
* contract execute command and wiremock integration tests ([e01a2b6](https://github.com/burnt-labs/xion-agent-toolkit/commit/e01a2b6f5a9a6574d6d7949c153810c672c418e2))
* **error:** complete error recovery enhancement ([3d3b739](https://github.com/burnt-labs/xion-agent-toolkit/commit/3d3b739a4ac8d3bd737da249b2ad22685c83ea1d))
* implement grant-config and fee-config CLI commands ([85fc4a0](https://github.com/burnt-labs/xion-agent-toolkit/commit/85fc4a0d8e54fe203724106d50fae06f89cbc77a))
* **oauth:** add refresh token expiration tracking ([26684fd](https://github.com/burnt-labs/xion-agent-toolkit/commit/26684fdf6308786d89f67b43a7f523c812091fbd))
* **oauth:** Complete Phase 2 - OAuth2 client orchestration ([a9970a1](https://github.com/burnt-labs/xion-agent-toolkit/commit/a9970a1d57138372b87b337c61992d2d7c210a1b))
* **oauth:** Complete Phase 2.1 - OAuth2 infrastructure ([ecdcc15](https://github.com/burnt-labs/xion-agent-toolkit/commit/ecdcc156660d4ce6d1dba2014253510951b61662))
* **oauth:** Complete Phase 2.2 - Callback Server & Token Manager ([c75072d](https://github.com/burnt-labs/xion-agent-toolkit/commit/c75072d7c10a3cc2a667ba5803556600a0901b58))
* Phase 2 & Phase 3 - CI/CD Output, Predicted Address, Batch Treasury Ops ([bcc2fc9](https://github.com/burnt-labs/xion-agent-toolkit/commit/bcc2fc921ea23c6e4038031c4a8cd3736d4b8f74))
* **phase2:** implement error recovery and transaction monitoring ([12c0173](https://github.com/burnt-labs/xion-agent-toolkit/commit/12c0173228dc5c0f69ed9b71e34f9bcd5c944716))
* **phase3:** add Predicted Address Computation and Batch Treasury Operations ([101cafa](https://github.com/burnt-labs/xion-agent-toolkit/commit/101cafaa3589cc865a6c7321ca01bae74a54cbb8))
* **plans:** Add status.json for project planning and update treasury-automation.md with OAuth2 improvements ([f74384b](https://github.com/burnt-labs/xion-agent-toolkit/commit/f74384b313be86b19bf38ddc806c78c61588970b))
* Skills documentation improvements and CLI enhancements ([c5d9557](https://github.com/burnt-labs/xion-agent-toolkit/commit/c5d9557290095c8ec2cb2399e523a276ac3222fd))
* **skills:** add parameter validation framework for AI agents ([d2b08c6](https://github.com/burnt-labs/xion-agent-toolkit/commit/d2b08c6798e5ef937636afaacae0e3d173afb8ab))
* **skills:** add parameter validation framework for AI agents ([5a796d9](https://github.com/burnt-labs/xion-agent-toolkit/commit/5a796d9ed36d55f419b5d1ef7bccf8fba4cfe252))
* **skills:** add xion-asset skill for CW721 NFT operations ([b078ce0](https://github.com/burnt-labs/xion-agent-toolkit/commit/b078ce0487653f0f0f28e8b17d0d8d3d7443603b))
* **skills:** Complete Phase 4 - Agent Skills implementation ([a70f253](https://github.com/burnt-labs/xion-agent-toolkit/commit/a70f253120801942328d097001f170977d723373))
* **skills:** implement fund.sh and withdraw.sh scripts ([ff69086](https://github.com/burnt-labs/xion-agent-toolkit/commit/ff6908696e3225d1cd9767f235708842d44f3ae6))
* **skills:** integrate with xion-skills and add xion-dev entry skill ([302bae7](https://github.com/burnt-labs/xion-agent-toolkit/commit/302bae71b27de1eeb5d4adf9f3156e8ce1b64e9c))
* **skills:** integrate with xion-skills and comprehensive documentation improvement ([869e6c4](https://github.com/burnt-labs/xion-agent-toolkit/commit/869e6c468197032a5d32fc56665e5735694c96a9))
* **tests:** Add mock data and skill test scripts for Skills Test Framework ([b10c525](https://github.com/burnt-labs/xion-agent-toolkit/commit/b10c5259d0950d81cd73a88f1f1de0739f7e45df))
* **tests:** add Skills Test Framework ([a289992](https://github.com/burnt-labs/xion-agent-toolkit/commit/a289992e49ba0e2cc6875c6ed2a53150df848d6e))
* **treasury:** add 12 new grant presets for extended type support ([e7acbd5](https://github.com/burnt-labs/xion-agent-toolkit/commit/e7acbd52d635ca9fe7ab12de09d60379bcda4860))
* **treasury:** add admin management, params update, and chain query ([6873ea8](https://github.com/burnt-labs/xion-agent-toolkit/commit/6873ea8e4b3ba3b34cde783bf6c4ef49cdc2de86))
* **treasury:** add create command with encoding support ([7bef99e](https://github.com/burnt-labs/xion-agent-toolkit/commit/7bef99e34ebbe628a89d52fa768091d9962b89f0))
* **treasury:** add is_oauth2_app and name support to params update ([6a2e3e7](https://github.com/burnt-labs/xion-agent-toolkit/commit/6a2e3e7b411459a8b2b72bc351149196b932861e))
* **treasury:** add transaction format fix plan and update status ([2d9baf2](https://github.com/burnt-labs/xion-agent-toolkit/commit/2d9baf2e49e852d4eabfec837f1fdb6475c063a8))
* **treasury:** add wait_for_treasury_creation polling and integration tests ([71360bb](https://github.com/burnt-labs/xion-agent-toolkit/commit/71360bbabdbcb98240fc166192ff111a375ff059))
* **treasury:** enhance grant-config with presets and security rules ([5e120d4](https://github.com/burnt-labs/xion-agent-toolkit/commit/5e120d42bc3575fbaa77c539dcd11e304c2d81cd))
* **treasury:** implement fund and withdraw operations ([d5b8243](https://github.com/burnt-labs/xion-agent-toolkit/commit/d5b824368e0504b153f2def99c7a426d89278486))
* **treasury:** Implement Phase 3 - Treasury management ([ca1f082](https://github.com/burnt-labs/xion-agent-toolkit/commit/ca1f0820968868e159ecb9323df46301fd471193))


### Bug Fixes

* add faucet command and REST API URL fix ([d52ea11](https://github.com/burnt-labs/xion-agent-toolkit/commit/d52ea116956d94d626480645452b4e195f1a250e))
* **api:** handle both base64 and direct JSON responses in query_contract_smart ([34666a0](https://github.com/burnt-labs/xion-agent-toolkit/commit/34666a0f4d56943cb6c72c64a180294a192d9bfe))
* **asset:** add missing re-export of AssetBuilderManager ([81e31aa](https://github.com/burnt-labs/xion-agent-toolkit/commit/81e31aab7c8cb7ebd909a5efde5d171aa5dd019b))
* **ci:** fix release-please workflow errors ([#36](https://github.com/burnt-labs/xion-agent-toolkit/issues/36)) ([9efc27f](https://github.com/burnt-labs/xion-agent-toolkit/commit/9efc27f3247371bc7921523bb99eb41e1697f8de))
* **ci:** quote if expression to fix YAML parsing ([ea09b4d](https://github.com/burnt-labs/xion-agent-toolkit/commit/ea09b4d230bd201fd23d12748c2a611542cc940b))
* **ci:** quote if expression to fix YAML parsing ([5752b40](https://github.com/burnt-labs/xion-agent-toolkit/commit/5752b40ad68d0532970f81eb78da0b81910f62bd))
* **ci:** trigger Release on tag push; use vX.Y.Z tag format ([18e565a](https://github.com/burnt-labs/xion-agent-toolkit/commit/18e565af925bb676b401e8db3d272e9c05e040dd))
* **ci:** use absolute path for xion-toolkit symlink ([d92b79b](https://github.com/burnt-labs/xion-agent-toolkit/commit/d92b79bddb0858fea996fe2d04414845f398a353))
* **cli:** Fix compilation errors and duplicate code ([8c45fa4](https://github.com/burnt-labs/xion-agent-toolkit/commit/8c45fa4fe72e44d6f3717927d39646150ca6c07b))
* **cli:** normalize filter_type from kebab-case to snake_case ([1524355](https://github.com/burnt-labs/xion-agent-toolkit/commit/1524355e2f17a087a529cfb117fa90e2d13e4ef5))
* **deps:** update quinn-proto to version 0.11.14 ([4e5c496](https://github.com/burnt-labs/xion-agent-toolkit/commit/4e5c49621fbc5bd57785e2995161de07b6b37c19))
* **docs:** add network_name field to all NetworkConfig examples ([89d3d38](https://github.com/burnt-labs/xion-agent-toolkit/commit/89d3d38a8ec475818689d025d49d14ccb5b87e22))
* **docs:** Fix doc test placeholders ([597ffd5](https://github.com/burnt-labs/xion-agent-toolkit/commit/597ffd55b108860f373430683737998f42f08211))
* **lint:** resolve shellcheck warnings in run_all.sh ([ca200c5](https://github.com/burnt-labs/xion-agent-toolkit/commit/ca200c52045ff5d83fa5274cc5ec7fb9c3266169))
* **network:** add separate rest_url for chain queries ([7c2f228](https://github.com/burnt-labs/xion-agent-toolkit/commit/7c2f228179d209abdc99aa875af02811098bc346))
* **oauth:** call /api/v1/me to get MetaAccount address ([a69814a](https://github.com/burnt-labs/xion-agent-toolkit/commit/a69814a2d0e857f126adab05c3176f82191c6dda))
* **plans:** update treasury plans progress and notes ([29672ea](https://github.com/burnt-labs/xion-agent-toolkit/commit/29672eaef521eb3d79bbed02a72bf30b108ad0a8))
* **qc:** complete P1 and P2 issue fixes ([d166c61](https://github.com/burnt-labs/xion-agent-toolkit/commit/d166c61021947ea2c417b0e2a7480318e4814728))
* **qc:** resolve all issues from QC cross-review (P0-P3) ([30f06b6](https://github.com/burnt-labs/xion-agent-toolkit/commit/30f06b615587a794878a60064536b76f4b028fd4))
* **qc:** resolve all P3 low-priority issues from cross-review ([0c3fb15](https://github.com/burnt-labs/xion-agent-toolkit/commit/0c3fb15270baef59d31de4906742d4019bf040a4))
* **qc:** resolve critical issues from cross-review ([66d3615](https://github.com/burnt-labs/xion-agent-toolkit/commit/66d36154296ded80661821f85c7abb6f04640d69))
* release workflow and CI improvements ([1e9aad1](https://github.com/burnt-labs/xion-agent-toolkit/commit/1e9aad15d8401229edd9f34872073458c3811315))
* **release:** avoid draft recursion and repo-less dispatch ([0993528](https://github.com/burnt-labs/xion-agent-toolkit/commit/0993528d7542a59fc9123e85c9bfdaf46219a681))
* **release:** avoid recursive release PR creation ([5505d0a](https://github.com/burnt-labs/xion-agent-toolkit/commit/5505d0a56751739dc47f97269f41503e0965ae57))
* **release:** dispatch cargo-dist after release creation ([#7](https://github.com/burnt-labs/xion-agent-toolkit/issues/7)) ([5dfab86](https://github.com/burnt-labs/xion-agent-toolkit/commit/5dfab867227c6d3a0192fc5762884454499fa32c))
* **release:** downgrade version to 0.8.1 in manifest and configuration files ([#38](https://github.com/burnt-labs/xion-agent-toolkit/issues/38)) ([851107b](https://github.com/burnt-labs/xion-agent-toolkit/commit/851107b41dc3526b8aaebcb32d596cb076caad85))
* **release:** manually create git tag when draft:true ([#34](https://github.com/burnt-labs/xion-agent-toolkit/issues/34)) ([5ffc915](https://github.com/burnt-labs/xion-agent-toolkit/commit/5ffc9158a2108bac5adddd3d53b083f1fb5e7194))
* **release:** remove Linux musl target ([bca29c0](https://github.com/burnt-labs/xion-agent-toolkit/commit/bca29c06db4535c5cb6d09bc60189c72d5bff19c))
* **release:** remove Linux musl target from documentation ([25b77fa](https://github.com/burnt-labs/xion-agent-toolkit/commit/25b77fa055720a8eb3b35534984e91ec97f4f055))
* **release:** set draft=true to support immutable releases ([8556889](https://github.com/burnt-labs/xion-agent-toolkit/commit/855688923cd9b38decf6a2efb2b049068d23a357))
* **release:** update release-please configuration ([#5](https://github.com/burnt-labs/xion-agent-toolkit/issues/5)) ([1e7d05a](https://github.com/burnt-labs/xion-agent-toolkit/commit/1e7d05a7f16290617d28e4f437868d52da9e4406))
* resolve CI clippy errors and simplify withdraw command ([0276265](https://github.com/burnt-labs/xion-agent-toolkit/commit/0276265849bd148470aa7bd32f17deedc6177208))
* **skills:** correct treasury create script flags ([1120f9e](https://github.com/burnt-labs/xion-agent-toolkit/commit/1120f9e664e4072c47033a99580fabace96e4921))
* **skills:** resolve non-blocking QC warnings ([f0586c8](https://github.com/burnt-labs/xion-agent-toolkit/commit/f0586c8c66c0cc8a29231414e1024aea061f1a4c))
* **skills:** resolve QC critical issues ([866e8e3](https://github.com/burnt-labs/xion-agent-toolkit/commit/866e8e390dc6fc059ed064de5a75a863c0eda666))
* **skills:** review and optimize all skills ([d8516ba](https://github.com/burnt-labs/xion-agent-toolkit/commit/d8516baf0dd15b2c77f92e5916b1e50206ae52c7))
* **test:** add serial(encryption_key) to tests modifying env var ([368299c](https://github.com/burnt-labs/xion-agent-toolkit/commit/368299c0d126dfb3d4132bcaf9530ba1906007fa))
* **tests:** add trap cleanup and consistent bash options ([a26d6b9](https://github.com/burnt-labs/xion-agent-toolkit/commit/a26d6b9c1fa2a795d37ea827f4779a0df930f0f4))
* **test:** use consistent serial group for encryption key tests ([4ec9e9b](https://github.com/burnt-labs/xion-agent-toolkit/commit/4ec9e9b52b1fbc0d0a3d7c0f37b36940159cb7d9))
* **transaction:** update transaction format analysis and root cause documentation ([eda96e1](https://github.com/burnt-labs/xion-agent-toolkit/commit/eda96e1c474881f505f8ef615a1616ba039b24ee))
* **treasury:** add validation and tests for params update ([f191d5e](https://github.com/burnt-labs/xion-agent-toolkit/commit/f191d5ee1f827ab731ecda33831eead375172d9b))
* **treasury:** correct Coin protobuf field order ([2a39216](https://github.com/burnt-labs/xion-agent-toolkit/commit/2a39216b77b9eb2deee04ad94b9789d2e923f958))
* **treasury:** correct OAuth2 API message format ([4d8b54b](https://github.com/burnt-labs/xion-agent-toolkit/commit/4d8b54bcbbd2f8931f0cb4fb1131049df148dc36))
* **treasury:** handle Option&lt;ProtobufAny&gt; correctly in manager.rs ([f98896b](https://github.com/burnt-labs/xion-agent-toolkit/commit/f98896bab02fa06e8d81ad05b4b233073deb092d))
* **treasury:** pass correct type_url in grant config ([7170e21](https://github.com/burnt-labs/xion-agent-toolkit/commit/7170e21cf406ebe35c658fb421b3b853aadc1176))
* **treasury:** standardize transaction message formats for treasury operations ([a810606](https://github.com/burnt-labs/xion-agent-toolkit/commit/a810606284c6a5998d42260f22194061d6dddda9))
* **treasury:** use Binary type for protobuf Any value field ([f87c0fe](https://github.com/burnt-labs/xion-agent-toolkit/commit/f87c0fe32292f4cadb78cb052c9b478adeeb41ae))
* **treasury:** use camelCase for API field names ([111eddb](https://github.com/burnt-labs/xion-agent-toolkit/commit/111eddb79a47efc32857cf4d0437bc35f2b218b1))
* **treasury:** use number array encoding for msg/salt fields ([08346c1](https://github.com/burnt-labs/xion-agent-toolkit/commit/08346c143b22ad9666dbead22016a5a79a0ebb8d))
* **treasury:** use raw JSON object for MsgExecuteContract.msg field ([b1607ed](https://github.com/burnt-labs/xion-agent-toolkit/commit/b1607ed28fb5ee10c454b22608043da6f90b3274))
* **tx:** address QC review critical issues ([5e9370b](https://github.com/burnt-labs/xion-agent-toolkit/commit/5e9370b1aefb8df1dc19b57134aee843d883bfa7))
* **tx:** Fix `tx wait` ([e512541](https://github.com/burnt-labs/xion-agent-toolkit/commit/e51254176c23e3cc5ff1596337d72203e7c1128a))
* **tx:** use correct Cosmos SDK REST API endpoint for tx wait ([2d81ce2](https://github.com/burnt-labs/xion-agent-toolkit/commit/2d81ce2d8ce901add741a47bac185b375f1c14fe))


### Refactoring

* **ci:** merge workflows into unified ci.yml ([c543e9c](https://github.com/burnt-labs/xion-agent-toolkit/commit/c543e9c672c7d36b50e3d63b5f20d6c3c37232d3))
* **ci:** separate E2E tests, reorganize workflows ([e76a61e](https://github.com/burnt-labs/xion-agent-toolkit/commit/e76a61e965f85157cb412c46d466a0cc00031cce))
* **ci:** test-skills.yml build xion-toolkit locally ([957f810](https://github.com/burnt-labs/xion-agent-toolkit/commit/957f810a513f2f5e60cdac4c362bc83251ff3bbd))
* **ci:** test-skills.yml download binary from CI with fallback ([f995cb7](https://github.com/burnt-labs/xion-agent-toolkit/commit/f995cb7d584ba0831c0bf3a9e285adc681c20e1d))
* **cli:** Move instantiate commands from treasury to contract subcommand ([b571e9d](https://github.com/burnt-labs/xion-agent-toolkit/commit/b571e9de91d2c37dc54abdbcd6867e369187176c))
* **cli:** rename binary from 'xion' to 'xion-toolkit' ([a6b5422](https://github.com/burnt-labs/xion-agent-toolkit/commit/a6b5422e359df3201fe1cf4775e80fdc6d88eeba))
* **config:** remove local network configuration ([ad64175](https://github.com/burnt-labs/xion-agent-toolkit/commit/ad64175eddf2d910131c1fab1ea8667f7e23dfe9))
* **config:** rename XION_TOOLkit_key to XION_ci_encryption_key ([6b76a15](https://github.com/burnt-labs/xion-agent-toolkit/commit/6b76a15c09f3f41173d2f538059ff522ee09af5b))
* **config:** Separate network config from user credentials ([6940718](https://github.com/burnt-labs/xion-agent-toolkit/commit/6940718e9d156337562cf60d20537441a2cc957b))
* Rename project from xion-agent-cli to xion-agent-toolkit ([e12771f](https://github.com/burnt-labs/xion-agent-toolkit/commit/e12771fa59ff4b743dbc51061fdf45e664c557f8))
* **skills:** optimize descriptions and structure for skills.sh ([0f6a71a](https://github.com/burnt-labs/xion-agent-toolkit/commit/0f6a71adcf5e8caea437fc5250ac44b90db1b72b))
* **tests:** reorganize test scripts and add comprehensive E2E tests ([1d7e3ef](https://github.com/burnt-labs/xion-agent-toolkit/commit/1d7e3ef7142b54caf8c1d7d00a4d775ed097c97d))
* **treasury:** add generic contract instantiation methods ([b3bd0cd](https://github.com/burnt-labs/xion-agent-toolkit/commit/b3bd0cd4e41491fbecf6a54011a1380fc2bb5a8a))
* **treasury:** extract common broadcast_execute_contract helper ([3cb6840](https://github.com/burnt-labs/xion-agent-toolkit/commit/3cb6840b82ef2073bcce9ea2219dd9a1d61f250f))
* **treasury:** standardize JSON formatting and encoding for treasury messages ([65a65c4](https://github.com/burnt-labs/xion-agent-toolkit/commit/65a65c4104899d5481ecc1ce1106a1bcb072abbb))
* **treasury:** use DaoDao Indexer for listing treasuries ([1cf34a1](https://github.com/burnt-labs/xion-agent-toolkit/commit/1cf34a1cffecf50b604a1c6e37003b887f10e191))
* **treasury:** use DaoDao Indexer for query_treasury ([12458e5](https://github.com/burnt-labs/xion-agent-toolkit/commit/12458e50475875233672acfd0c214efa2d4e0abf))
* use official types from xion-types and treasury crates ([16e5df1](https://github.com/burnt-labs/xion-agent-toolkit/commit/16e5df1acf9563aa7824f4e03e8992bb58d92598))


### Documentation

* add AI Agent installation guide and skills.sh support ([50f2efa](https://github.com/burnt-labs/xion-agent-toolkit/commit/50f2efa4430cc6a3abe958124ffc545e39194dc4))
* add Asset Builder documentation for Phase 1+2+3 ([446c4b7](https://github.com/burnt-labs/xion-agent-toolkit/commit/446c4b705ede8b61cc1cbfabeb17a8baf7668e84))
* add key reference implementations to AGENTS.md ([a6ef783](https://github.com/burnt-labs/xion-agent-toolkit/commit/a6ef7834f43a55bc213b93b5aa70725bfa8a0caf))
* add pre-commit checklist to AGENTS.md ([ffe89cb](https://github.com/burnt-labs/xion-agent-toolkit/commit/ffe89cbb83da41a75caa6f72ddcb90854063ea81))
* add QUICK-REFERENCE.md links to all documentation files ([94f5acf](https://github.com/burnt-labs/xion-agent-toolkit/commit/94f5acf907bf0bc2c3e611eb36a00576b592e6ff))
* add test serialization rules to AGENTS.md ([efe2a60](https://github.com/burnt-labs/xion-agent-toolkit/commit/efe2a60da31f57ac7d6045cf339736fd86c66b82))
* **agents:** Add language standards for conversation and documentation ([fb9cc64](https://github.com/burnt-labs/xion-agent-toolkit/commit/fb9cc64e6cdf10bcbcccb4df4deca924fb9bca00))
* **AGENTS:** add OAuth2 API service message formats and encoding rules ([d94e654](https://github.com/burnt-labs/xion-agent-toolkit/commit/d94e65454fa360f946ae423bd3bcda0f34063408))
* **cli-reference:** add contract execute command documentation ([5c5b9f5](https://github.com/burnt-labs/xion-agent-toolkit/commit/5c5b9f5cf594a8aa09aceb7d1565130879ea5cbf))
* complete documentation improvement ([9653df6](https://github.com/burnt-labs/xion-agent-toolkit/commit/9653df6f785a97f8eaa16be6f7173b8789a5edd0))
* Comprehensive documentation update ([aea210e](https://github.com/burnt-labs/xion-agent-toolkit/commit/aea210e80f6db2bfde6e4adc98282c3e04cadeab))
* **contributing:** add shell completion setup tip ([e77c5fc](https://github.com/burnt-labs/xion-agent-toolkit/commit/e77c5fcbdb95bb073b87968378cc765bf7dba998))
* **CONTRIBUTING:** enhance contribution guidelines and documentation structure ([4ac23a5](https://github.com/burnt-labs/xion-agent-toolkit/commit/4ac23a5cb28df78433576ef19832518879d9d0cd))
* **e2e:** update test results and investigation status ([e0949a7](https://github.com/burnt-labs/xion-agent-toolkit/commit/e0949a70ce8cd78f539fa8c153d32dd19eaee45f))
* enhance README with AI Agent integration instructions ([212c527](https://github.com/burnt-labs/xion-agent-toolkit/commit/212c52773486d6e21a9a4d89f7ccc11a95c0302d))
* **faucet:** add faucet command documentation ([2670c09](https://github.com/burnt-labs/xion-agent-toolkit/commit/2670c09af29b41e67d13ce58ce449cf4836e9160))
* **network:** document rpc_url as reserved, add rest_url to config show ([ab220ba](https://github.com/burnt-labs/xion-agent-toolkit/commit/ab220ba82adb5d3b7f0899d687d7e542dc91bf1d))
* optimize documentation structure and remove crates.io references ([0a90e22](https://github.com/burnt-labs/xion-agent-toolkit/commit/0a90e227c371c63d6b6335c068958113016c91bb))
* **phase2:** complete Phase 2 P1 documentation ([94ede50](https://github.com/burnt-labs/xion-agent-toolkit/commit/94ede5051c9d061774bd1e203c05dbecb8562ef5))
* **plan:** Clarify instantiate command location in plan background ([0fef8dd](https://github.com/burnt-labs/xion-agent-toolkit/commit/0fef8dd1c0aad9e352650ac1466d8633b125c6cb))
* **plan:** mark release-please automation as complete ([7d4b94e](https://github.com/burnt-labs/xion-agent-toolkit/commit/7d4b94e18e5390f87f7e5471543ef13508b903e0))
* **plans:** add contract-instantiate-refactor plan ([e5f3cb8](https://github.com/burnt-labs/xion-agent-toolkit/commit/e5f3cb8d5f535f2b1cb7769cf19a68ee966d432b))
* **plans:** add feature roadmap and detailed plan documents ([c353caf](https://github.com/burnt-labs/xion-agent-toolkit/commit/c353caf8d7011f7f5f305521fa3ae81f7f755899))
* **plans:** add future enhancement plans ([a26ac3d](https://github.com/burnt-labs/xion-agent-toolkit/commit/a26ac3d87f104ff857573100d3950f1ffa97ab97))
* **plans:** add Phase 2 P1 development plans ([a89ad87](https://github.com/burnt-labs/xion-agent-toolkit/commit/a89ad879e48340ae213066837bb9e95f3542dfaf))
* **plans:** complete E2E testing and update OAuth2 API documentation ([bfddb6b](https://github.com/burnt-labs/xion-agent-toolkit/commit/bfddb6bc79307abd6e9f1a22d67489f7dee482cb))
* **plans:** complete Phase 2 with Skills Test Framework ([06ea47f](https://github.com/burnt-labs/xion-agent-toolkit/commit/06ea47febab677a591502f8669acf8e52fbe2b50))
* **plans:** complete treasury-enhancements with sign-off ([d4f33c3](https://github.com/burnt-labs/xion-agent-toolkit/commit/d4f33c323286afb585f31823cf202132e4122388))
* **plans:** mark Phase 1 features as completed, add E2E tests ([db669e5](https://github.com/burnt-labs/xion-agent-toolkit/commit/db669e5f87f7888d63c10f6f871789418eefedc2))
* **plans:** split documentation-and-e2e into two separate plans ([be342b0](https://github.com/burnt-labs/xion-agent-toolkit/commit/be342b05155283a16b288944b800ae610861c252))
* **plans:** update feature roadmap and status for Phase 1 completion ([5a13a46](https://github.com/burnt-labs/xion-agent-toolkit/commit/5a13a4678adca612ba35600fe940b59b092bd182))
* **plans:** update OAuth2 PKCE implementation details and add new files ([05aad10](https://github.com/burnt-labs/xion-agent-toolkit/commit/05aad109e70ca6e4dd0e21e6d3b20d90bd55c46a))
* **plans:** update Phase 2 status - P1 features complete ([5395eac](https://github.com/burnt-labs/xion-agent-toolkit/commit/5395eac492e84abfa29efcb6f879d2aa3cf74371))
* **plans:** update progress to 98% with verified xion_address fix ([551d31d](https://github.com/burnt-labs/xion-agent-toolkit/commit/551d31dc0ea600c706130be8b522d7b423ae6a39))
* **plans:** update status - tx monitoring in review ([5ace516](https://github.com/burnt-labs/xion-agent-toolkit/commit/5ace5161ec65404913245ebf50886f651878106d))
* **plans:** update treasury-automation progress to 95% ([c8dc4f4](https://github.com/burnt-labs/xion-agent-toolkit/commit/c8dc4f48d49dd5ad5ba066a751e3d128675a859d))
* **plans:** update treasury-enhancements plan with attribute management ([ecb9a7a](https://github.com/burnt-labs/xion-agent-toolkit/commit/ecb9a7a9df983e66edc8fcc2f4294a0d41e83e10))
* **plan:** Update checklist - Phase 1 fully completed ([13292b0](https://github.com/burnt-labs/xion-agent-toolkit/commit/13292b0529a648af573a6ff2cfdeb6f70dca3fde))
* **plan:** Update skill-param-validation checklist - QA SIGN-OFF ([a1d9de1](https://github.com/burnt-labs/xion-agent-toolkit/commit/a1d9de11df39ba3639f36b985caa27c9b015eefb))
* **plan:** Update treasury-automation checklist - Phase 1 completed ([c8a3e2c](https://github.com/burnt-labs/xion-agent-toolkit/commit/c8a3e2ce8543d8d374a45f59458e163e413a169c))
* **plan:** Update treasury-automation.md - Phase 2 & 3 completed ([90546d7](https://github.com/burnt-labs/xion-agent-toolkit/commit/90546d73eb133c9670246b65d535e8fe421b3486))
* **readme:** restructure for human users ([61c1ad7](https://github.com/burnt-labs/xion-agent-toolkit/commit/61c1ad7a941064d13f267021f2006ed91ee097a1))
* **readme:** Update configuration architecture documentation ([c6994b0](https://github.com/burnt-labs/xion-agent-toolkit/commit/c6994b097d6c0a5291b046f84d7be5e5d7074e62))
* remove manual skills installation, keep only npx skills add method ([e790450](https://github.com/burnt-labs/xion-agent-toolkit/commit/e790450cb5a236e995a74cc9a56a4d748d6c73da))
* remove manual skills installation, keep only npx skills add method ([b2e4d0a](https://github.com/burnt-labs/xion-agent-toolkit/commit/b2e4d0a5a3bf6a5ade77b1fedfe432eee0109125))
* reorganize documentation structure ([35dfae9](https://github.com/burnt-labs/xion-agent-toolkit/commit/35dfae9a5179ff6aa2fa866465971f4610917c89))
* simplify shell completion section in README and QUICK-REFERENCE ([eb00706](https://github.com/burnt-labs/xion-agent-toolkit/commit/eb007061a7ae9bc6ae15a84267af6312faa4d15c))
* **skill:** Fix SKILL.md Quick Reference warnings ([0a00526](https://github.com/burnt-labs/xion-agent-toolkit/commit/0a0052607aac3dd8641dcac4b9978056aa862517))
* update .env.example and CONTRIBUTING.md for OAuth2 configuration ([5675164](https://github.com/burnt-labs/xion-agent-toolkit/commit/56751642cc65ad448d1831521e8e5e348c84b01a))
* update CHANGELOG for v0.2.0 release ([969752b](https://github.com/burnt-labs/xion-agent-toolkit/commit/969752bca2688c46d2e7e888c6500e7d19552a04))
* update CHANGELOG with E2E test reorganization ([27f89ca](https://github.com/burnt-labs/xion-agent-toolkit/commit/27f89ca95f71173ae5d9c38976f4de66a4982632))
* update cli-reference with --name and --is-oauth2-app flags ([9ddc6f3](https://github.com/burnt-labs/xion-agent-toolkit/commit/9ddc6f320993b84d66b11fbc0d030e9c089a5a21))
* update documentation for Phase 2 and Phase 3 features ([82f5826](https://github.com/burnt-labs/xion-agent-toolkit/commit/82f5826cb96a6112f3d15fc81d457ea9af2257f5))
* update documentation with new CLI commands ([37d0828](https://github.com/burnt-labs/xion-agent-toolkit/commit/37d0828f4cbed92723f42cafc0fd6734545c1609))
* update E2E testing progress to 70% ([2f37e57](https://github.com/burnt-labs/xion-agent-toolkit/commit/2f37e57240df7d4109ee17cd707d70ba8b4da5d7))
* update lang ([e2ef336](https://github.com/burnt-labs/xion-agent-toolkit/commit/e2ef336667c889bbf223149aa72e6fea1bc0faeb))
* update plan status with QC round 2 results ([097f76d](https://github.com/burnt-labs/xion-agent-toolkit/commit/097f76d1dba31dc3ee2a2edddc67dd62d1894168))
* update plan with skill optimization details ([7418022](https://github.com/burnt-labs/xion-agent-toolkit/commit/74180224a8fd9e28831322e9812ce259b6706a6f))
* update plan with type refactoring status ([c059765](https://github.com/burnt-labs/xion-agent-toolkit/commit/c059765a5a87b7467820778ce3ba38098c386c79))
* update plans and skills for treasury create feature ([fbcd56c](https://github.com/burnt-labs/xion-agent-toolkit/commit/fbcd56cad220acf08199029d5b1916f1e8891612))
* update SKILL.md and plans/status.json for grant/fee config ([200acdf](https://github.com/burnt-labs/xion-agent-toolkit/commit/200acdf063ed1aab01d5ae97a84d4e2695231932))
* update treasury debug plan with OAuth2 API format findings ([1214953](https://github.com/burnt-labs/xion-agent-toolkit/commit/121495330f8315a95d63be40cd7935b199d5df85))


### Tests

* replace mockito with wiremock for async integration tests ([96b517b](https://github.com/burnt-labs/xion-agent-toolkit/commit/96b517b0611918b42dd4627f44e973dab31e07a3))


### Chores

* add gitnexus ([412d0e4](https://github.com/burnt-labs/xion-agent-toolkit/commit/412d0e464c5146a0163491b8a556a9bfbc3bd107))
* archive completed plans and improve docs for AI agents ([b326d75](https://github.com/burnt-labs/xion-agent-toolkit/commit/b326d7571146813c2d8942ef90fcc577362892e8))
* archive plans, improve docs for AI agents, and add params update support ([62ac467](https://github.com/burnt-labs/xion-agent-toolkit/commit/62ac46783436558717676df4d3bb134da9ee400f))
* **ci:** inject OAuth client IDs into release workflow ([2b0c571](https://github.com/burnt-labs/xion-agent-toolkit/commit/2b0c57169b9ebf66841a81de9b62b0008e54b154))
* **ci:** skip CI for release-please automated commits ([a8c5ddf](https://github.com/burnt-labs/xion-agent-toolkit/commit/a8c5ddfd0beac3005724c082aa4eb46c7d6fa110))
* **ci:** use github-build-setup for OAuth client IDs in release workflow ([48f6369](https://github.com/burnt-labs/xion-agent-toolkit/commit/48f6369b38d5c327cca475fd381e437018835f23))
* clean up compiler warnings and dead code ([43bbf2e](https://github.com/burnt-labs/xion-agent-toolkit/commit/43bbf2edb114a40b63d25a98361e390a28373dbd))
* **docs:** enhance AGENTS.md and repository structure for clarity ([68b060f](https://github.com/burnt-labs/xion-agent-toolkit/commit/68b060f1029a7777de8d763cf1cbe7e2fe80c0ab))
* **main:** release 0.4.0 ([#4](https://github.com/burnt-labs/xion-agent-toolkit/issues/4)) ([f338cc2](https://github.com/burnt-labs/xion-agent-toolkit/commit/f338cc20823f25c11c19c72b6c859e3f02eff276))
* **main:** release 0.4.1 ([#6](https://github.com/burnt-labs/xion-agent-toolkit/issues/6)) ([5641256](https://github.com/burnt-labs/xion-agent-toolkit/commit/56412567504a3745d59d7ffd0ad65b8afaf1ec83))
* **main:** release 0.4.2 ([#8](https://github.com/burnt-labs/xion-agent-toolkit/issues/8)) ([c1bf89a](https://github.com/burnt-labs/xion-agent-toolkit/commit/c1bf89a57149ee087de53bcc38c1a7f62d8aa828))
* **main:** release 0.4.3 ([961daa9](https://github.com/burnt-labs/xion-agent-toolkit/commit/961daa9dbe46683587d7c7ddd573425cb3ddec30))
* **main:** release 0.4.3 ([6ab8c1d](https://github.com/burnt-labs/xion-agent-toolkit/commit/6ab8c1d7652e09454ae227de225bb17336314400))
* **main:** release 0.5.0 ([3e22cbe](https://github.com/burnt-labs/xion-agent-toolkit/commit/3e22cbe822ee753d320f60e9e4e7532dd99d4c35))
* **main:** release 0.5.0 ([5354e26](https://github.com/burnt-labs/xion-agent-toolkit/commit/5354e26c8dae7724874bc57ca365661ff985c905))
* **main:** release 0.6.0 ([bc887a9](https://github.com/burnt-labs/xion-agent-toolkit/commit/bc887a9b74a41d8810037b9eaf8302dab065da65))
* **main:** release 0.6.0 ([5a671de](https://github.com/burnt-labs/xion-agent-toolkit/commit/5a671debe3c2c3742230c1f19ded0358d6091181))
* **main:** release 0.7.0 ([e4d4a37](https://github.com/burnt-labs/xion-agent-toolkit/commit/e4d4a373742040e0eee5d74fe363893f961b1ea5))
* **main:** release 0.7.0 ([aa08f83](https://github.com/burnt-labs/xion-agent-toolkit/commit/aa08f83caea56751103066da900928d3ba19bf87))
* **main:** release 0.8.0 ([749bc33](https://github.com/burnt-labs/xion-agent-toolkit/commit/749bc33e4f0eab2c8f97bc3a6013274462ef4764))
* **main:** release 0.8.0 ([b78811d](https://github.com/burnt-labs/xion-agent-toolkit/commit/b78811dd609c61ec17ddbf4fbc7f0cbe3f534c24))
* **main:** release 0.8.1 ([c8517da](https://github.com/burnt-labs/xion-agent-toolkit/commit/c8517da5843cd0000a757ffcea4db1c9f57b4951))
* **main:** release 0.8.1 ([e04d86b](https://github.com/burnt-labs/xion-agent-toolkit/commit/e04d86b9ed6926d969c4d56925f16c1f9640c20a))
* **main:** release 0.8.2 ([#31](https://github.com/burnt-labs/xion-agent-toolkit/issues/31)) ([e7cbf59](https://github.com/burnt-labs/xion-agent-toolkit/commit/e7cbf59a63a45e7d5d0f5d512da81a16bbac6fcd))
* **main:** release 0.8.2 ([#39](https://github.com/burnt-labs/xion-agent-toolkit/issues/39)) ([2f76cb2](https://github.com/burnt-labs/xion-agent-toolkit/commit/2f76cb2cbbfba6fee651e71df6b1a07a418fe863))
* **main:** release xion-agent-toolkit 0.3.0 ([480d69b](https://github.com/burnt-labs/xion-agent-toolkit/commit/480d69b077e134f441d18de25db0735381cd3e43))
* **main:** release xion-agent-toolkit 0.3.0 ([d80db76](https://github.com/burnt-labs/xion-agent-toolkit/commit/d80db766ed46c0d67b53f525977e608258d71dc9))
* **release:** add cargo-dist for automated release process ([b01941e](https://github.com/burnt-labs/xion-agent-toolkit/commit/b01941ebccb6fcf30345726e550c3c55333e5c4d))
* **release:** improve release automation ([8da9ce1](https://github.com/burnt-labs/xion-agent-toolkit/commit/8da9ce12423fe9c75bf2285f633a6f645fb63544))
* **release:** use vX.Y.Z tag format (include-component-in-tag: false) ([d503578](https://github.com/burnt-labs/xion-agent-toolkit/commit/d503578a8cbacb8ba61e41ee101b87138d84e05a))
* remove unnecessary docs and examples ([ec0c449](https://github.com/burnt-labs/xion-agent-toolkit/commit/ec0c44999e4243ce50627615ba1803ce5dc1bbaa))
* **template:** remove cw-template subproject reference ([c92b009](https://github.com/burnt-labs/xion-agent-toolkit/commit/c92b009e7b93c0191c64e570d8f8008ec82de976))
* update gitnexus ([ab51959](https://github.com/burnt-labs/xion-agent-toolkit/commit/ab51959137c9fe5c6dbe6fe8edad891c0695d418))
* Update license information and modify README ([4198c8d](https://github.com/burnt-labs/xion-agent-toolkit/commit/4198c8d971e80ed32aba2639ab48946d681b9bf6))

## [0.8.2](https://github.com/burnt-labs/xion-agent-toolkit/compare/0.8.1...0.8.2) (2026-03-18)


### Bug Fixes

* **ci:** fix release-please workflow errors ([#36](https://github.com/burnt-labs/xion-agent-toolkit/issues/36)) ([9efc27f](https://github.com/burnt-labs/xion-agent-toolkit/commit/9efc27f3247371bc7921523bb99eb41e1697f8de))
* **ci:** quote if expression to fix YAML parsing ([ea09b4d](https://github.com/burnt-labs/xion-agent-toolkit/commit/ea09b4d230bd201fd23d12748c2a611542cc940b))
* **ci:** quote if expression to fix YAML parsing ([5752b40](https://github.com/burnt-labs/xion-agent-toolkit/commit/5752b40ad68d0532970f81eb78da0b81910f62bd))
* release workflow and CI improvements ([1e9aad1](https://github.com/burnt-labs/xion-agent-toolkit/commit/1e9aad15d8401229edd9f34872073458c3811315))
* **release:** downgrade version to 0.8.1 in manifest and configuration files ([#38](https://github.com/burnt-labs/xion-agent-toolkit/issues/38)) ([851107b](https://github.com/burnt-labs/xion-agent-toolkit/commit/851107b41dc3526b8aaebcb32d596cb076caad85))
* **release:** manually create git tag when draft:true ([#34](https://github.com/burnt-labs/xion-agent-toolkit/issues/34)) ([5ffc915](https://github.com/burnt-labs/xion-agent-toolkit/commit/5ffc9158a2108bac5adddd3d53b083f1fb5e7194))
* **release:** set draft=true to support immutable releases ([8556889](https://github.com/burnt-labs/xion-agent-toolkit/commit/855688923cd9b38decf6a2efb2b049068d23a357))


### Chores

* **ci:** skip CI for release-please automated commits ([a8c5ddf](https://github.com/burnt-labs/xion-agent-toolkit/commit/a8c5ddfd0beac3005724c082aa4eb46c7d6fa110))
* **main:** release 0.8.2 ([#31](https://github.com/burnt-labs/xion-agent-toolkit/issues/31)) ([e7cbf59](https://github.com/burnt-labs/xion-agent-toolkit/commit/e7cbf59a63a45e7d5d0f5d512da81a16bbac6fcd))

## [0.8.2](https://github.com/burnt-labs/xion-agent-toolkit/compare/0.8.1...0.8.2) (2026-03-18)


### Bug Fixes

* **ci:** quote if expression to fix YAML parsing ([ea09b4d](https://github.com/burnt-labs/xion-agent-toolkit/commit/ea09b4d230bd201fd23d12748c2a611542cc940b))
* **ci:** quote if expression to fix YAML parsing ([5752b40](https://github.com/burnt-labs/xion-agent-toolkit/commit/5752b40ad68d0532970f81eb78da0b81910f62bd))
* release workflow and CI improvements ([1e9aad1](https://github.com/burnt-labs/xion-agent-toolkit/commit/1e9aad15d8401229edd9f34872073458c3811315))
* **release:** set draft=true to support immutable releases ([8556889](https://github.com/burnt-labs/xion-agent-toolkit/commit/855688923cd9b38decf6a2efb2b049068d23a357))


### Chores

* **ci:** skip CI for release-please automated commits ([a8c5ddf](https://github.com/burnt-labs/xion-agent-toolkit/commit/a8c5ddfd0beac3005724c082aa4eb46c7d6fa110))

## [0.8.1](https://github.com/burnt-labs/xion-agent-toolkit/compare/0.8.0...0.8.1) (2026-03-18)


### Bug Fixes

* add faucet command and REST API URL fix ([d52ea11](https://github.com/burnt-labs/xion-agent-toolkit/commit/d52ea116956d94d626480645452b4e195f1a250e))
* **api:** handle both base64 and direct JSON responses in query_contract_smart ([34666a0](https://github.com/burnt-labs/xion-agent-toolkit/commit/34666a0f4d56943cb6c72c64a180294a192d9bfe))
* **network:** add separate rest_url for chain queries ([7c2f228](https://github.com/burnt-labs/xion-agent-toolkit/commit/7c2f228179d209abdc99aa875af02811098bc346))
* **tx:** Fix `tx wait` ([e512541](https://github.com/burnt-labs/xion-agent-toolkit/commit/e51254176c23e3cc5ff1596337d72203e7c1128a))
* **tx:** use correct Cosmos SDK REST API endpoint for tx wait ([2d81ce2](https://github.com/burnt-labs/xion-agent-toolkit/commit/2d81ce2d8ce901add741a47bac185b375f1c14fe))


### Documentation

* **faucet:** add faucet command documentation ([2670c09](https://github.com/burnt-labs/xion-agent-toolkit/commit/2670c09af29b41e67d13ce58ce449cf4836e9160))
* **network:** document rpc_url as reserved, add rest_url to config show ([ab220ba](https://github.com/burnt-labs/xion-agent-toolkit/commit/ab220ba82adb5d3b7f0899d687d7e542dc91bf1d))


### Chores

* **template:** remove cw-template subproject reference ([c92b009](https://github.com/burnt-labs/xion-agent-toolkit/commit/c92b009e7b93c0191c64e570d8f8008ec82de976))

## [0.8.0](https://github.com/burnt-labs/xion-agent-toolkit/compare/0.7.0...0.8.0) (2026-03-17)


### Features

* **cli:** add --install flag for shell completion auto-installation ([c569813](https://github.com/burnt-labs/xion-agent-toolkit/commit/c56981339754871f3e30b7e76abb89a6d369469a))
* **cli:** add CI/CD integration output formats ([559c48f](https://github.com/burnt-labs/xion-agent-toolkit/commit/559c48f7bd7d3f2e36ff84cd78d86faad40c56bd))
* **cli:** add shell completion support ([98cef04](https://github.com/burnt-labs/xion-agent-toolkit/commit/98cef046bd820c8814cd46a3ac1841fb5c62e726))
* **cli:** add shell completion support with --install flag ([9edbb94](https://github.com/burnt-labs/xion-agent-toolkit/commit/9edbb948ab9cbdab9d44cb4e7513184b6c21b83a))
* **error:** complete error recovery enhancement ([3d3b739](https://github.com/burnt-labs/xion-agent-toolkit/commit/3d3b739a4ac8d3bd737da249b2ad22685c83ea1d))
* Phase 2 & Phase 3 - CI/CD Output, Predicted Address, Batch Treasury Ops ([bcc2fc9](https://github.com/burnt-labs/xion-agent-toolkit/commit/bcc2fc921ea23c6e4038031c4a8cd3736d4b8f74))
* **phase2:** implement error recovery and transaction monitoring ([12c0173](https://github.com/burnt-labs/xion-agent-toolkit/commit/12c0173228dc5c0f69ed9b71e34f9bcd5c944716))
* **phase3:** add Predicted Address Computation and Batch Treasury Operations ([101cafa](https://github.com/burnt-labs/xion-agent-toolkit/commit/101cafaa3589cc865a6c7321ca01bae74a54cbb8))
* **skills:** add parameter validation framework for AI agents ([d2b08c6](https://github.com/burnt-labs/xion-agent-toolkit/commit/d2b08c6798e5ef937636afaacae0e3d173afb8ab))
* **skills:** add parameter validation framework for AI agents ([5a796d9](https://github.com/burnt-labs/xion-agent-toolkit/commit/5a796d9ed36d55f419b5d1ef7bccf8fba4cfe252))
* **tests:** Add mock data and skill test scripts for Skills Test Framework ([b10c525](https://github.com/burnt-labs/xion-agent-toolkit/commit/b10c5259d0950d81cd73a88f1f1de0739f7e45df))
* **tests:** add Skills Test Framework ([a289992](https://github.com/burnt-labs/xion-agent-toolkit/commit/a289992e49ba0e2cc6875c6ed2a53150df848d6e))


### Bug Fixes

* **ci:** use absolute path for xion-toolkit symlink ([d92b79b](https://github.com/burnt-labs/xion-agent-toolkit/commit/d92b79bddb0858fea996fe2d04414845f398a353))
* **lint:** resolve shellcheck warnings in run_all.sh ([ca200c5](https://github.com/burnt-labs/xion-agent-toolkit/commit/ca200c52045ff5d83fa5274cc5ec7fb9c3266169))
* **qc:** complete P1 and P2 issue fixes ([d166c61](https://github.com/burnt-labs/xion-agent-toolkit/commit/d166c61021947ea2c417b0e2a7480318e4814728))
* **qc:** resolve all issues from QC cross-review (P0-P3) ([30f06b6](https://github.com/burnt-labs/xion-agent-toolkit/commit/30f06b615587a794878a60064536b76f4b028fd4))
* **qc:** resolve all P3 low-priority issues from cross-review ([0c3fb15](https://github.com/burnt-labs/xion-agent-toolkit/commit/0c3fb15270baef59d31de4906742d4019bf040a4))
* **qc:** resolve critical issues from cross-review ([66d3615](https://github.com/burnt-labs/xion-agent-toolkit/commit/66d36154296ded80661821f85c7abb6f04640d69))
* **skills:** resolve non-blocking QC warnings ([f0586c8](https://github.com/burnt-labs/xion-agent-toolkit/commit/f0586c8c66c0cc8a29231414e1024aea061f1a4c))
* **skills:** resolve QC critical issues ([866e8e3](https://github.com/burnt-labs/xion-agent-toolkit/commit/866e8e390dc6fc059ed064de5a75a863c0eda666))
* **tx:** address QC review critical issues ([5e9370b](https://github.com/burnt-labs/xion-agent-toolkit/commit/5e9370b1aefb8df1dc19b57134aee843d883bfa7))


### Refactoring

* **ci:** merge workflows into unified ci.yml ([c543e9c](https://github.com/burnt-labs/xion-agent-toolkit/commit/c543e9c672c7d36b50e3d63b5f20d6c3c37232d3))
* **ci:** separate E2E tests, reorganize workflows ([e76a61e](https://github.com/burnt-labs/xion-agent-toolkit/commit/e76a61e965f85157cb412c46d466a0cc00031cce))
* **ci:** test-skills.yml build xion-toolkit locally ([957f810](https://github.com/burnt-labs/xion-agent-toolkit/commit/957f810a513f2f5e60cdac4c362bc83251ff3bbd))
* **ci:** test-skills.yml download binary from CI with fallback ([f995cb7](https://github.com/burnt-labs/xion-agent-toolkit/commit/f995cb7d584ba0831c0bf3a9e285adc681c20e1d))


### Documentation

* **contributing:** add shell completion setup tip ([e77c5fc](https://github.com/burnt-labs/xion-agent-toolkit/commit/e77c5fcbdb95bb073b87968378cc765bf7dba998))
* **CONTRIBUTING:** enhance contribution guidelines and documentation structure ([4ac23a5](https://github.com/burnt-labs/xion-agent-toolkit/commit/4ac23a5cb28df78433576ef19832518879d9d0cd))
* **phase2:** complete Phase 2 P1 documentation ([94ede50](https://github.com/burnt-labs/xion-agent-toolkit/commit/94ede5051c9d061774bd1e203c05dbecb8562ef5))
* **plans:** add Phase 2 P1 development plans ([a89ad87](https://github.com/burnt-labs/xion-agent-toolkit/commit/a89ad879e48340ae213066837bb9e95f3542dfaf))
* **plans:** complete Phase 2 with Skills Test Framework ([06ea47f](https://github.com/burnt-labs/xion-agent-toolkit/commit/06ea47febab677a591502f8669acf8e52fbe2b50))
* **plans:** update Phase 2 status - P1 features complete ([5395eac](https://github.com/burnt-labs/xion-agent-toolkit/commit/5395eac492e84abfa29efcb6f879d2aa3cf74371))
* **plans:** update status - tx monitoring in review ([5ace516](https://github.com/burnt-labs/xion-agent-toolkit/commit/5ace5161ec65404913245ebf50886f651878106d))
* **plan:** Update skill-param-validation checklist - QA SIGN-OFF ([a1d9de1](https://github.com/burnt-labs/xion-agent-toolkit/commit/a1d9de11df39ba3639f36b985caa27c9b015eefb))
* **readme:** restructure for human users ([61c1ad7](https://github.com/burnt-labs/xion-agent-toolkit/commit/61c1ad7a941064d13f267021f2006ed91ee097a1))
* simplify shell completion section in README and QUICK-REFERENCE ([eb00706](https://github.com/burnt-labs/xion-agent-toolkit/commit/eb007061a7ae9bc6ae15a84267af6312faa4d15c))
* **skill:** Fix SKILL.md Quick Reference warnings ([0a00526](https://github.com/burnt-labs/xion-agent-toolkit/commit/0a0052607aac3dd8641dcac4b9978056aa862517))
* update documentation for Phase 2 and Phase 3 features ([82f5826](https://github.com/burnt-labs/xion-agent-toolkit/commit/82f5826cb96a6112f3d15fc81d457ea9af2257f5))
* update plan status with QC round 2 results ([097f76d](https://github.com/burnt-labs/xion-agent-toolkit/commit/097f76d1dba31dc3ee2a2edddc67dd62d1894168))

## [0.7.0](https://github.com/burnt-labs/xion-agent-toolkit/compare/0.6.0...0.7.0) (2026-03-14)


### Features

* **account:** add account info command ([53776c8](https://github.com/burnt-labs/xion-agent-toolkit/commit/53776c8527a822ccc48e34e485f0ebad9b47f824))
* **account:** add MetaAccount info command ([157b354](https://github.com/burnt-labs/xion-agent-toolkit/commit/157b3542d5665518b73cce1894324cc17d52e7c5))
* **account:** rewrite account info to use OAuth2 API ([d0f1d26](https://github.com/burnt-labs/xion-agent-toolkit/commit/d0f1d262a09d7056d31acb5126219b8a6c5a33ae))
* Asset Builder, Batch Operations, Extended Grants, and MetaAccount Info ([057edc0](https://github.com/burnt-labs/xion-agent-toolkit/commit/057edc05615dec7a6b0b5d8d54e41406c28f7caf))
* **asset:** add Asset Builder module for CW721 NFT deployment and minting ([4fc0919](https://github.com/burnt-labs/xion-agent-toolkit/commit/4fc0919ed52338a2324b16898ff37d3e21d0ea7d))
* **asset:** add CW721 variant support for Phase 2 ([f2874ce](https://github.com/burnt-labs/xion-agent-toolkit/commit/f2874ce8b10a5713a69182338bd19ffe2b45f987))
* **asset:** add Phase 3 features - address prediction and batch minting ([4c405cb](https://github.com/burnt-labs/xion-agent-toolkit/commit/4c405cbfe54c8640d36f371421265365fa327882))
* **batch:** add batch operations for multi-message transactions ([7e2e2de](https://github.com/burnt-labs/xion-agent-toolkit/commit/7e2e2de79643e39a36312786b04839de08c3e8e2))
* **skills:** add xion-asset skill for CW721 NFT operations ([b078ce0](https://github.com/burnt-labs/xion-agent-toolkit/commit/b078ce0487653f0f0f28e8b17d0d8d3d7443603b))
* **treasury:** add 12 new grant presets for extended type support ([e7acbd5](https://github.com/burnt-labs/xion-agent-toolkit/commit/e7acbd52d635ca9fe7ab12de09d60379bcda4860))
* **treasury:** add is_oauth2_app and name support to params update ([6a2e3e7](https://github.com/burnt-labs/xion-agent-toolkit/commit/6a2e3e7b411459a8b2b72bc351149196b932861e))


### Bug Fixes

* **asset:** add missing re-export of AssetBuilderManager ([81e31aa](https://github.com/burnt-labs/xion-agent-toolkit/commit/81e31aab7c8cb7ebd909a5efde5d171aa5dd019b))
* **skills:** review and optimize all skills ([d8516ba](https://github.com/burnt-labs/xion-agent-toolkit/commit/d8516baf0dd15b2c77f92e5916b1e50206ae52c7))
* **tests:** add trap cleanup and consistent bash options ([a26d6b9](https://github.com/burnt-labs/xion-agent-toolkit/commit/a26d6b9c1fa2a795d37ea827f4779a0df930f0f4))
* **treasury:** add validation and tests for params update ([f191d5e](https://github.com/burnt-labs/xion-agent-toolkit/commit/f191d5ee1f827ab731ecda33831eead375172d9b))


### Documentation

* add Asset Builder documentation for Phase 1+2+3 ([446c4b7](https://github.com/burnt-labs/xion-agent-toolkit/commit/446c4b705ede8b61cc1cbfabeb17a8baf7668e84))
* add QUICK-REFERENCE.md links to all documentation files ([94f5acf](https://github.com/burnt-labs/xion-agent-toolkit/commit/94f5acf907bf0bc2c3e611eb36a00576b592e6ff))
* optimize documentation structure and remove crates.io references ([0a90e22](https://github.com/burnt-labs/xion-agent-toolkit/commit/0a90e227c371c63d6b6335c068958113016c91bb))
* **plans:** add feature roadmap and detailed plan documents ([c353caf](https://github.com/burnt-labs/xion-agent-toolkit/commit/c353caf8d7011f7f5f305521fa3ae81f7f755899))
* **plans:** mark Phase 1 features as completed, add E2E tests ([db669e5](https://github.com/burnt-labs/xion-agent-toolkit/commit/db669e5f87f7888d63c10f6f871789418eefedc2))
* **plans:** update feature roadmap and status for Phase 1 completion ([5a13a46](https://github.com/burnt-labs/xion-agent-toolkit/commit/5a13a4678adca612ba35600fe940b59b092bd182))
* update cli-reference with --name and --is-oauth2-app flags ([9ddc6f3](https://github.com/burnt-labs/xion-agent-toolkit/commit/9ddc6f320993b84d66b11fbc0d030e9c089a5a21))


### Chores

* archive completed plans and improve docs for AI agents ([b326d75](https://github.com/burnt-labs/xion-agent-toolkit/commit/b326d7571146813c2d8942ef90fcc577362892e8))
* archive plans, improve docs for AI agents, and add params update support ([62ac467](https://github.com/burnt-labs/xion-agent-toolkit/commit/62ac46783436558717676df4d3bb134da9ee400f))

## [0.6.0](https://github.com/burnt-labs/xion-agent-toolkit/compare/0.5.0...0.6.0) (2026-03-11)


### Features

* **cli:** add contract query and treasury export/import commands ([35ee428](https://github.com/burnt-labs/xion-agent-toolkit/commit/35ee4289b41c2ab1f4b9deb31181d13bdc6be6a5))
* **cli:** add contract query and treasury export/import commands ([ddfde4e](https://github.com/burnt-labs/xion-agent-toolkit/commit/ddfde4e1e8eee046d669492f338a8f0811431670))
* Skills documentation improvements and CLI enhancements ([c5d9557](https://github.com/burnt-labs/xion-agent-toolkit/commit/c5d9557290095c8ec2cb2399e523a276ac3222fd))


### Documentation

* remove manual skills installation, keep only npx skills add method ([e790450](https://github.com/burnt-labs/xion-agent-toolkit/commit/e790450cb5a236e995a74cc9a56a4d748d6c73da))
* remove manual skills installation, keep only npx skills add method ([b2e4d0a](https://github.com/burnt-labs/xion-agent-toolkit/commit/b2e4d0a5a3bf6a5ade77b1fedfe432eee0109125))

## [0.5.0](https://github.com/btspoony/xion-agent-toolkit/compare/0.4.3...0.5.0) (2026-03-11)


### Features

* **cli:** add contract execute command ([017d44f](https://github.com/btspoony/xion-agent-toolkit/commit/017d44f0b2ae5dee83121db02270718aa1ffc159))
* contract execute command and wiremock integration tests ([e01a2b6](https://github.com/btspoony/xion-agent-toolkit/commit/e01a2b6f5a9a6574d6d7949c153810c672c418e2))
* **skills:** integrate with xion-skills and add xion-dev entry skill ([302bae7](https://github.com/btspoony/xion-agent-toolkit/commit/302bae71b27de1eeb5d4adf9f3156e8ce1b64e9c))
* **skills:** integrate with xion-skills and comprehensive documentation improvement ([869e6c4](https://github.com/btspoony/xion-agent-toolkit/commit/869e6c468197032a5d32fc56665e5735694c96a9))


### Bug Fixes

* **deps:** update quinn-proto to version 0.11.14 ([4e5c496](https://github.com/btspoony/xion-agent-toolkit/commit/4e5c49621fbc5bd57785e2995161de07b6b37c19))


### Refactoring

* **skills:** optimize descriptions and structure for skills.sh ([0f6a71a](https://github.com/btspoony/xion-agent-toolkit/commit/0f6a71adcf5e8caea437fc5250ac44b90db1b72b))


### Documentation

* add AI Agent installation guide and skills.sh support ([50f2efa](https://github.com/btspoony/xion-agent-toolkit/commit/50f2efa4430cc6a3abe958124ffc545e39194dc4))
* **cli-reference:** add contract execute command documentation ([5c5b9f5](https://github.com/btspoony/xion-agent-toolkit/commit/5c5b9f5cf594a8aa09aceb7d1565130879ea5cbf))
* complete documentation improvement ([9653df6](https://github.com/btspoony/xion-agent-toolkit/commit/9653df6f785a97f8eaa16be6f7173b8789a5edd0))
* enhance README with AI Agent integration instructions ([212c527](https://github.com/btspoony/xion-agent-toolkit/commit/212c52773486d6e21a9a4d89f7ccc11a95c0302d))
* update plan with skill optimization details ([7418022](https://github.com/btspoony/xion-agent-toolkit/commit/74180224a8fd9e28831322e9812ce259b6706a6f))


### Tests

* replace mockito with wiremock for async integration tests ([96b517b](https://github.com/btspoony/xion-agent-toolkit/commit/96b517b0611918b42dd4627f44e973dab31e07a3))

## [0.4.3](https://github.com/btspoony/xion-agent-toolkit/compare/0.4.2...0.4.3) (2026-03-09)


### Bug Fixes

* **release:** avoid draft recursion and repo-less dispatch ([0993528](https://github.com/btspoony/xion-agent-toolkit/commit/0993528d7542a59fc9123e85c9bfdaf46219a681))
* **release:** avoid recursive release PR creation ([5505d0a](https://github.com/btspoony/xion-agent-toolkit/commit/5505d0a56751739dc47f97269f41503e0965ae57))
* **release:** remove Linux musl target ([bca29c0](https://github.com/btspoony/xion-agent-toolkit/commit/bca29c06db4535c5cb6d09bc60189c72d5bff19c))
* **release:** remove Linux musl target from documentation ([25b77fa](https://github.com/btspoony/xion-agent-toolkit/commit/25b77fa055720a8eb3b35534984e91ec97f4f055))

## [0.4.2](https://github.com/btspoony/xion-agent-toolkit/compare/0.4.1...0.4.2) (2026-03-09)


### Bug Fixes

* **release:** dispatch cargo-dist after release creation ([#7](https://github.com/btspoony/xion-agent-toolkit/issues/7)) ([5dfab86](https://github.com/btspoony/xion-agent-toolkit/commit/5dfab867227c6d3a0192fc5762884454499fa32c))

## [0.4.1](https://github.com/btspoony/xion-agent-toolkit/compare/v0.4.0...0.4.1) (2026-03-09)


### Bug Fixes

* **release:** update release-please configuration ([#5](https://github.com/btspoony/xion-agent-toolkit/issues/5)) ([1e7d05a](https://github.com/btspoony/xion-agent-toolkit/commit/1e7d05a7f16290617d28e4f437868d52da9e4406))

## [0.4.0] (2026-03-09)

### Features

* **ci:** add release-please for automated release management ([8f534ae](https://github.com/btspoony/xion-agent-toolkit/commit/8f534aef197159c5adaffb31f215637c09f8d145))
* **cli:** add generic contract instantiation commands ([ae430bc](https://github.com/btspoony/xion-agent-toolkit/commit/ae430bc456be5fb4df6a9aaa2bf53f52b823391a))
* **cli:** Complete Phase 2.4 - CLI integration ([b1a53d9](https://github.com/btspoony/xion-agent-toolkit/commit/b1a53d935a0aa0d5d2db4fd7abea7803b3131452))
* **cli:** Initialize Xion Agent Toolkit with basic CLI framework ([0eaff64](https://github.com/btspoony/xion-agent-toolkit/commit/0eaff6441cc2e3a5fa7163038db514ef07d1f507))
* **config:** Enhance configuration management and OAuth2 endpoint discovery ([cbb9a50](https://github.com/btspoony/xion-agent-toolkit/commit/cbb9a5019ed5bc4f75614db5f3444bf72fcec71d))
* **config:** replace OS keyring with AES-256-GCM encrypted files ([8f6c64b](https://github.com/btspoony/xion-agent-toolkit/commit/8f6c64bbcf261426c04a6551c26c789f1fbb2173))
* implement grant-config and fee-config CLI commands ([85fc4a0](https://github.com/btspoony/xion-agent-toolkit/commit/85fc4a0d8e54fe203724106d50fae06f89cbc77a))
* **oauth:** add refresh token expiration tracking ([26684fd](https://github.com/btspoony/xion-agent-toolkit/commit/26684fdf6308786d89f67b43a7f523c812091fbd))
* **oauth:** Complete Phase 2 - OAuth2 client orchestration ([a9970a1](https://github.com/btspoony/xion-agent-toolkit/commit/a9970a1d57138372b87b337c61992d2d7c210a1b))
* **oauth:** Complete Phase 2.1 - OAuth2 infrastructure ([ecdcc15](https://github.com/btspoony/xion-agent-toolkit/commit/ecdcc156660d4ce6d1dba2014253510951b61662))
* **oauth:** Complete Phase 2.2 - Callback Server & Token Manager ([c75072d](https://github.com/btspoony/xion-agent-toolkit/commit/c75072d7c10a3cc2a667ba5803556600a0901b58))
* **plans:** Add status.json for project planning and update treasury-automation.md with OAuth2 improvements ([f74384b](https://github.com/btspoony/xion-agent-toolkit/commit/f74384b313be86b19bf38ddc806c78c61588970b))
* **skills:** Complete Phase 4 - Agent Skills implementation ([a70f253](https://github.com/btspoony/xion-agent-toolkit/commit/a70f253120801942328d097001f170977d723373))
* **skills:** implement fund.sh and withdraw.sh scripts ([ff69086](https://github.com/btspoony/xion-agent-toolkit/commit/ff6908696e3225d1cd9767f235708842d44f3ae6))
* **treasury:** add admin management, params update, and chain query ([6873ea8](https://github.com/btspoony/xion-agent-toolkit/commit/6873ea8e4b3ba3b34cde783bf6c4ef49cdc2de86))
* **treasury:** add create command with encoding support ([7bef99e](https://github.com/btspoony/xion-agent-toolkit/commit/7bef99e34ebbe628a89d52fa768091d9962b89f0))
* **treasury:** add transaction format fix plan and update status ([2d9baf2](https://github.com/btspoony/xion-agent-toolkit/commit/2d9baf2e49e852d4eabfec837f1fdb6475c063a8))
* **treasury:** add wait_for_treasury_creation polling and integration tests ([71360bb](https://github.com/btspoony/xion-agent-toolkit/commit/71360bbabdbcb98240fc166192ff111a375ff059))
* **treasury:** enhance grant-config with presets and security rules ([5e120d4](https://github.com/btspoony/xion-agent-toolkit/commit/5e120d42bc3575fbaa77c539dcd11e304c2d81cd))
* **treasury:** implement fund and withdraw operations ([d5b8243](https://github.com/btspoony/xion-agent-toolkit/commit/d5b824368e0504b153f2def99c7a426d89278486))
* **treasury:** Implement Phase 3 - Treasury management ([ca1f082](https://github.com/btspoony/xion-agent-toolkit/commit/ca1f0820968868e159ecb9323df46301fd471193))


### Bug Fixes

* **cli:** Fix compilation errors and duplicate code ([8c45fa4](https://github.com/btspoony/xion-agent-toolkit/commit/8c45fa4fe72e44d6f3717927d39646150ca6c07b))
* **cli:** normalize filter_type from kebab-case to snake_case ([1524355](https://github.com/btspoony/xion-agent-toolkit/commit/1524355e2f17a087a529cfb117fa90e2d13e4ef5))
* **docs:** add network_name field to all NetworkConfig examples ([89d3d38](https://github.com/btspoony/xion-agent-toolkit/commit/89d3d38a8ec475818689d025d49d14ccb5b87e22))
* **docs:** Fix doc test placeholders ([597ffd5](https://github.com/btspoony/xion-agent-toolkit/commit/597ffd55b108860f373430683737998f42f08211))
* **oauth:** call /api/v1/me to get MetaAccount address ([a69814a](https://github.com/btspoony/xion-agent-toolkit/commit/a69814a2d0e857f126adab05c3176f82191c6dda))
* **plans:** update treasury plans progress and notes ([29672ea](https://github.com/btspoony/xion-agent-toolkit/commit/29672eaef521eb3d79bbed02a72bf30b108ad0a8))
* resolve CI clippy errors and simplify withdraw command ([0276265](https://github.com/btspoony/xion-agent-toolkit/commit/0276265849bd148470aa7bd32f17deedc6177208))
* **skills:** correct treasury create script flags ([1120f9e](https://github.com/btspoony/xion-agent-toolkit/commit/1120f9e664e4072c47033a99580fabace96e4921))
* **test:** add serial(encryption_key) to tests modifying env var ([368299c](https://github.com/btspoony/xion-agent-toolkit/commit/368299c0d126dfb3d4132bcaf9530ba1906007fa))
* **test:** use consistent serial group for encryption key tests ([4ec9e9b](https://github.com/btspoony/xion-agent-toolkit/commit/4ec9e9b52b1fbc0d0a3d7c0f37b36940159cb7d9))
* **transaction:** update transaction format analysis and root cause documentation ([eda96e1](https://github.com/btspoony/xion-agent-toolkit/commit/eda96e1c474881f505f8ef615a1616ba039b24ee))
* **treasury:** correct Coin protobuf field order ([2a39216](https://github.com/btspoony/xion-agent-toolkit/commit/2a39216b77b9eb2deee04ad94b9789d2e923f958))
* **treasury:** correct OAuth2 API message format ([4d8b54b](https://github.com/btspoony/xion-agent-toolkit/commit/4d8b54bcbbd2f8931f0cb4fb1131049df148dc36))
* **treasury:** handle Option&lt;ProtobufAny&gt; correctly in manager.rs ([f98896b](https://github.com/btspoony/xion-agent-toolkit/commit/f98896bab02fa06e8d81ad05b4b233073deb092d))
* **treasury:** pass correct type_url in grant config ([7170e21](https://github.com/btspoony/xion-agent-toolkit/commit/7170e21cf406ebe35c658fb421b3b853aadc1176))
* **treasury:** standardize transaction message formats for treasury operations ([a810606](https://github.com/btspoony/xion-agent-toolkit/commit/a810606284c6a5998d42260f22194061d6dddda9))
* **treasury:** use Binary type for protobuf Any value field ([f87c0fe](https://github.com/btspoony/xion-agent-toolkit/commit/f87c0fe32292f4cadb78cb052c9b478adeeb41ae))
* **treasury:** use camelCase for API field names ([111eddb](https://github.com/btspoony/xion-agent-toolkit/commit/111eddb79a47efc32857cf4d0437bc35f2b218b1))
* **treasury:** use number array encoding for msg/salt fields ([08346c1](https://github.com/btspoony/xion-agent-toolkit/commit/08346c143b22ad9666dbead22016a5a79a0ebb8d))
* **treasury:** use raw JSON object for MsgExecuteContract.msg field ([b1607ed](https://github.com/btspoony/xion-agent-toolkit/commit/b1607ed28fb5ee10c454b22608043da6f90b3274))


### Refactoring

* **cli:** Move instantiate commands from treasury to contract subcommand ([b571e9d](https://github.com/btspoony/xion-agent-toolkit/commit/b571e9de91d2c37dc54abdbcd6867e369187176c))
* **cli:** rename binary from 'xion' to 'xion-toolkit' ([a6b5422](https://github.com/btspoony/xion-agent-toolkit/commit/a6b5422e359df3201fe1cf4775e80fdc6d88eeba))
* **config:** remove local network configuration ([ad64175](https://github.com/btspoony/xion-agent-toolkit/commit/ad64175eddf2d910131c1fab1ea8667f7e23dfe9))
* **config:** rename XION_TOOLkit_key to XION_ci_encryption_key ([6b76a15](https://github.com/btspoony/xion-agent-toolkit/commit/6b76a15c09f3f41173d2f538059ff522ee09af5b))
* **config:** Separate network config from user credentials ([6940718](https://github.com/btspoony/xion-agent-toolkit/commit/6940718e9d156337562cf60d20537441a2cc957b))
* Rename project from xion-agent-cli to xion-agent-toolkit ([e12771f](https://github.com/btspoony/xion-agent-toolkit/commit/e12771fa59ff4b743dbc51061fdf45e664c557f8))
* **tests:** reorganize test scripts and add comprehensive E2E tests ([1d7e3ef](https://github.com/btspoony/xion-agent-toolkit/commit/1d7e3ef7142b54caf8c1d7d00a4d775ed097c97d))
* **treasury:** add generic contract instantiation methods ([b3bd0cd](https://github.com/btspoony/xion-agent-toolkit/commit/b3bd0cd4e41491fbecf6a54011a1380fc2bb5a8a))
* **treasury:** extract common broadcast_execute_contract helper ([3cb6840](https://github.com/btspoony/xion-agent-toolkit/commit/3cb6840b82ef2073bcce9ea2219dd9a1d61f250f))
* **treasury:** standardize JSON formatting and encoding for treasury messages ([65a65c4](https://github.com/btspoony/xion-agent-toolkit/commit/65a65c4104899d5481ecc1ce1106a1bcb072abbb))
* **treasury:** use DaoDao Indexer for listing treasuries ([1cf34a1](https://github.com/btspoony/xion-agent-toolkit/commit/1cf34a1cffecf50b604a1c6e37003b887f10e191))
* **treasury:** use DaoDao Indexer for query_treasury ([12458e5](https://github.com/btspoony/xion-agent-toolkit/commit/12458e50475875233672acfd0c214efa2d4e0abf))
* use official types from xion-types and treasury crates ([16e5df1](https://github.com/btspoony/xion-agent-toolkit/commit/16e5df1acf9563aa7824f4e03e8992bb58d92598))


### Documentation

* add key reference implementations to AGENTS.md ([a6ef783](https://github.com/btspoony/xion-agent-toolkit/commit/a6ef7834f43a55bc213b93b5aa70725bfa8a0caf))
* add pre-commit checklist to AGENTS.md ([ffe89cb](https://github.com/btspoony/xion-agent-toolkit/commit/ffe89cbb83da41a75caa6f72ddcb90854063ea81))
* add test serialization rules to AGENTS.md ([efe2a60](https://github.com/btspoony/xion-agent-toolkit/commit/efe2a60da31f57ac7d6045cf339736fd86c66b82))
* **agents:** Add language standards for conversation and documentation ([fb9cc64](https://github.com/btspoony/xion-agent-toolkit/commit/fb9cc64e6cdf10bcbcccb4df4deca924fb9bca00))
* **AGENTS:** add OAuth2 API service message formats and encoding rules ([d94e654](https://github.com/btspoony/xion-agent-toolkit/commit/d94e65454fa360f946ae423bd3bcda0f34063408))
* Comprehensive documentation update ([aea210e](https://github.com/btspoony/xion-agent-toolkit/commit/aea210e80f6db2bfde6e4adc98282c3e04cadeab))
* **e2e:** update test results and investigation status ([e0949a7](https://github.com/btspoony/xion-agent-toolkit/commit/e0949a70ce8cd78f539fa8c153d32dd19eaee45f))
* **plan:** Clarify instantiate command location in plan background ([0fef8dd](https://github.com/btspoony/xion-agent-toolkit/commit/0fef8dd1c0aad9e352650ac1466d8633b125c6cb))
* **plan:** mark release-please automation as complete ([7d4b94e](https://github.com/btspoony/xion-agent-toolkit/commit/7d4b94e18e5390f87f7e5471543ef13508b903e0))
* **plans:** add contract-instantiate-refactor plan ([e5f3cb8](https://github.com/btspoony/xion-agent-toolkit/commit/e5f3cb8d5f535f2b1cb7769cf19a68ee966d432b))
* **plans:** add future enhancement plans ([a26ac3d](https://github.com/btspoony/xion-agent-toolkit/commit/a26ac3d87f104ff857573100d3950f1ffa97ab97))
* **plans:** complete E2E testing and update OAuth2 API documentation ([bfddb6b](https://github.com/btspoony/xion-agent-toolkit/commit/bfddb6bc79307abd6e9f1a22d67489f7dee482cb))
* **plans:** complete treasury-enhancements with sign-off ([d4f33c3](https://github.com/btspoony/xion-agent-toolkit/commit/d4f33c323286afb585f31823cf202132e4122388))
* **plans:** split documentation-and-e2e into two separate plans ([be342b0](https://github.com/btspoony/xion-agent-toolkit/commit/be342b05155283a16b288944b800ae610861c252))
* **plans:** update OAuth2 PKCE implementation details and add new files ([05aad10](https://github.com/btspoony/xion-agent-toolkit/commit/05aad109e70ca6e4dd0e21e6d3b20d90bd55c46a))
* **plans:** update progress to 98% with verified xion_address fix ([551d31d](https://github.com/btspoony/xion-agent-toolkit/commit/551d31dc0ea600c706130be8b522d7b423ae6a39))
* **plans:** update treasury-automation progress to 95% ([c8dc4f4](https://github.com/btspoony/xion-agent-toolkit/commit/c8dc4f48d49dd5ad5ba066a751e3d128675a859d))
* **plans:** update treasury-enhancements plan with attribute management ([ecb9a7a](https://github.com/btspoony/xion-agent-toolkit/commit/ecb9a7a9df983e66edc8fcc2f4294a0d41e83e10))
* **plan:** Update checklist - Phase 1 fully completed ([13292b0](https://github.com/btspoony/xion-agent-toolkit/commit/13292b0529a648af573a6ff2cfdeb6f70dca3fde))
* **plan:** Update treasury-automation checklist - Phase 1 completed ([c8a3e2c](https://github.com/btspoony/xion-agent-toolkit/commit/c8a3e2ce8543d8d374a45f59458e163e413a169c))
* **plan:** Update treasury-automation.md - Phase 2 & 3 completed ([90546d7](https://github.com/btspoony/xion-agent-toolkit/commit/90546d73eb133c9670246b65d535e8fe421b3486))
* **readme:** Update configuration architecture documentation ([c6994b0](https://github.com/btspoony/xion-agent-toolkit/commit/c6994b097d6c0a5291b046f84d7be5e5d7074e62))
* reorganize documentation structure ([35dfae9](https://github.com/btspoony/xion-agent-toolkit/commit/35dfae9a5179ff6aa2fa866465971f4610917c89))
* update .env.example and CONTRIBUTING.md for OAuth2 configuration ([5675164](https://github.com/btspoony/xion-agent-toolkit/commit/56751642cc65ad448d1831521e8e5e348c84b01a))
* update CHANGELOG for v0.2.0 release ([969752b](https://github.com/btspoony/xion-agent-toolkit/commit/969752bca2688c46d2e7e888c6500e7d19552a04))
* update CHANGELOG with E2E test reorganization ([27f89ca](https://github.com/btspoony/xion-agent-toolkit/commit/27f89ca95f71173ae5d9c38976f4de66a4982632))
* update documentation with new CLI commands ([37d0828](https://github.com/btspoony/xion-agent-toolkit/commit/37d0828f4cbed92723f42cafc0fd6734545c1609))
* update E2E testing progress to 70% ([2f37e57](https://github.com/btspoony/xion-agent-toolkit/commit/2f37e57240df7d4109ee17cd707d70ba8b4da5d7))
* update lang ([e2ef336](https://github.com/btspoony/xion-agent-toolkit/commit/e2ef336667c889bbf223149aa72e6fea1bc0faeb))
* update plan with type refactoring status ([c059765](https://github.com/btspoony/xion-agent-toolkit/commit/c059765a5a87b7467820778ce3ba38098c386c79))
* update plans and skills for treasury create feature ([fbcd56c](https://github.com/btspoony/xion-agent-toolkit/commit/fbcd56cad220acf08199029d5b1916f1e8891612))
* update SKILL.md and plans/status.json for grant/fee config ([200acdf](https://github.com/btspoony/xion-agent-toolkit/commit/200acdf063ed1aab01d5ae97a84d4e2695231932))
* update treasury debug plan with OAuth2 API format findings ([1214953](https://github.com/btspoony/xion-agent-toolkit/commit/121495330f8315a95d63be40cd7935b199d5df85))


### Chores

* add gitnexus ([412d0e4](https://github.com/btspoony/xion-agent-toolkit/commit/412d0e464c5146a0163491b8a556a9bfbc3bd107))
* **ci:** inject OAuth client IDs into release workflow ([2b0c571](https://github.com/btspoony/xion-agent-toolkit/commit/2b0c57169b9ebf66841a81de9b62b0008e54b154))
* **ci:** use github-build-setup for OAuth client IDs in release workflow ([48f6369](https://github.com/btspoony/xion-agent-toolkit/commit/48f6369b38d5c327cca475fd381e437018835f23))
* clean up compiler warnings and dead code ([43bbf2e](https://github.com/btspoony/xion-agent-toolkit/commit/43bbf2edb114a40b63d25a98361e390a28373dbd))
* **docs:** enhance AGENTS.md and repository structure for clarity ([68b060f](https://github.com/btspoony/xion-agent-toolkit/commit/68b060f1029a7777de8d763cf1cbe7e2fe80c0ab))
* **main:** release xion-agent-toolkit 0.3.0 ([d80db76](https://github.com/btspoony/xion-agent-toolkit/commit/d80db766ed46c0d67b53f525977e608258d71dc9))
* **release:** add cargo-dist for automated release process ([b01941e](https://github.com/btspoony/xion-agent-toolkit/commit/b01941ebccb6fcf30345726e550c3c55333e5c4d))
* **release:** use vX.Y.Z tag format (include-component-in-tag: false) ([d503578](https://github.com/btspoony/xion-agent-toolkit/commit/d503578a8cbacb8ba61e41ee101b87138d84e05a))
* remove unnecessary docs and examples ([ec0c449](https://github.com/btspoony/xion-agent-toolkit/commit/ec0c44999e4243ce50627615ba1803ce5dc1bbaa))
* update gitnexus ([ab51959](https://github.com/btspoony/xion-agent-toolkit/commit/ab51959137c9fe5c6dbe6fe8edad891c0695d418))
* Update license information and modify README ([4198c8d](https://github.com/btspoony/xion-agent-toolkit/commit/4198c8d971e80ed32aba2639ab48946d681b9bf6))
