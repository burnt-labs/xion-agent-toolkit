# Changelog

All notable changes to the Xion Agent Toolkit will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
