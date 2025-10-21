# Changelog

All notable changes to this project will be documented in this file. See [conventional commits](https://www.conventionalcommits.org/) for commit guidelines.

---
## [0.6.4](https://github.com/tyrchen/sqlx-db-tester/compare/v0.6.3..v0.6.4) - 2025-10-21

### Miscellaneous Chores

- fix feature flag bug - ([6dee2bb](https://github.com/tyrchen/sqlx-db-tester/commit/6dee2bbe75c19df4144b219eae395b680f592d27)) - Tyr Chen

### Other

- Update CHANGELOG.md - ([0e0d975](https://github.com/tyrchen/sqlx-db-tester/commit/0e0d975dfd9e1e2196b95888686073527cec3b92)) - Tyr Chen

---
## [0.6.3](https://github.com/tyrchen/sqlx-db-tester/compare/v0.6.2..v0.6.3) - 2025-10-21

### Miscellaneous Chores

- fix sqlx deps - ([94ad064](https://github.com/tyrchen/sqlx-db-tester/commit/94ad064f72c8f00f72a7f774570f365525c7b828)) - Tyr Chen

### Other

- Update CHANGELOG.md - ([b167467](https://github.com/tyrchen/sqlx-db-tester/commit/b167467563dbfff32dc5deaf0475f156de741831)) - Tyr Chen

---
## [0.6.2](https://github.com/tyrchen/sqlx-db-tester/compare/v0.6.0..v0.6.2) - 2025-10-21

### Miscellaneous Chores

- update github action - ([d213020](https://github.com/tyrchen/sqlx-db-tester/commit/d213020b3fc71f49af10553aeb1be4b5fadbec3f)) - Tyr Chen
- add features to exclude mysql - ([0d4ffdc](https://github.com/tyrchen/sqlx-db-tester/commit/0d4ffdcf80c05b6c4b96a6b5d1da4665d2570710)) - Tyr Chen
- ignore failing test in github action due to no direct file access - ([be54eec](https://github.com/tyrchen/sqlx-db-tester/commit/be54eec1282ddb17db47cb7344987abffcaf7cab)) - Tyr Chen

### Other

- Feature/support mysql (#8)

* feat: Add comprehensive MySQL support matching PostgreSQL implementation

- Implement TestMySql struct with automatic database lifecycle management
- Add database creation/deletion with unique UUID-based names
- Support connection pooling via get_pool() method
- Implement CSV loading functionality (load_csv and load_csv_data)
- Add MySQL-specific URL parsing for flexible connection strings
- Include Docker/Podman setup in Makefile with MySQL 8.0 support
- Configure MySQL to run on port 3307 to avoid conflicts
- Use mysql_native_password for compatibility
- Add MySQL service to GitHub Actions workflow for CI/CD
- Create example demonstrating MySQL usage
- Add MySQL-specific migrations for testing
- Update package metadata to reflect MySQL support

The implementation provides feature parity with PostgreSQL, enabling
seamless integration testing with automatic database management.

🤖 Generated with [Claude Code](https://claude.ai/code)

Co-Authored-By: Claude <noreply@anthropic.com>

* chore: update deps

* Update CHANGELOG.md

---------

Co-authored-by: Claude <noreply@anthropic.com> - ([a0a2d64](https://github.com/tyrchen/sqlx-db-tester/commit/a0a2d64ca9bc62025a6c11c3849ffaa9e0fc84e0)) - Tyr Chen

---
## [0.6.0](https://github.com/tyrchen/sqlx-db-tester/compare/v0.5.0..v0.6.0) - 2025-03-30

### Miscellaneous Chores

- upgrade to rust 2024 - ([6ccdb5c](https://github.com/tyrchen/sqlx-db-tester/commit/6ccdb5ce04372a3cf67a72468be8b21ce02d10dc)) - Tyr Chen
- fix gh - ([2cb4f6d](https://github.com/tyrchen/sqlx-db-tester/commit/2cb4f6d3ea4d65808618ce210a1a88800ab412e8)) - Tyr Chen

### Other

- Update CHANGELOG.md - ([ae8deea](https://github.com/tyrchen/sqlx-db-tester/commit/ae8deea4155e8f4943a01f72b724a470f7026e33)) - Tyr Chen

---
## [0.5.0](https://github.com/tyrchen/sqlx-db-tester/compare/v0.4.2..v0.5.0) - 2024-09-25

### Bug Fixes

- fix merge issue and bump version to 0.5 - ([84163ee](https://github.com/tyrchen/sqlx-db-tester/commit/84163eee7ffc6d0f0421e54c0c29070b08a37206)) - Tyr Chen

### Other

- Update CHANGELOG.md - ([1c19b92](https://github.com/tyrchen/sqlx-db-tester/commit/1c19b92f702d3a3cd53c175b85ad4fa0feff75bd)) - Tyr Chen
- keep readme up-to-date (#2) - ([6b75b7b](https://github.com/tyrchen/sqlx-db-tester/commit/6b75b7bd1cdf831ab100901fd4a3f628fb4c3e06)) - hairtail
- Enhance PostgreSQL URL Parsing to Support Optional Database Names (#3)

* feature: support databse_url with database name

* fix: fix connect error

* fix: fix parse postgres url

* fix: fix drop error

* fix: "cannot drop the currently open database"

* update:sqlx version

---------

Co-authored-by: Tyr Chen <tyr.chen@gmail.com> - ([d53708d](https://github.com/tyrchen/sqlx-db-tester/commit/d53708de112255c8bfa6562dae1907e378f99f8d)) - fankaiLiu

### Refactoring

- remove hyphen in test dbname (#4) - ([31e0ef0](https://github.com/tyrchen/sqlx-db-tester/commit/31e0ef05c632178096c9f0fe19830f17e26f386c)) - Daniel Liu

---
## [0.4.2](https://github.com/tyrchen/sqlx-db-tester/compare/v0.4.1..v0.4.2) - 2024-04-29

### Miscellaneous Chores

- use unwrap_or_else - ([c037409](https://github.com/tyrchen/sqlx-db-tester/commit/c037409e10e3904fed93447ada382e1eedaa5e5c)) - Tyr Chen

### Other

- Update CHANGELOG.md - ([a12be4d](https://github.com/tyrchen/sqlx-db-tester/commit/a12be4d4279267ebf5f35d7f344a17b4dd7a627d)) - Tyr Chen

---
## [0.4.1](https://github.com/tyrchen/sqlx-db-tester/compare/v0.4.0..v0.4.1) - 2024-04-29

### Miscellaneous Chores

- trigger gh action again - ([1a6f82a](https://github.com/tyrchen/sqlx-db-tester/commit/1a6f82a7ad0866c0ab1a486bf013d5040c1a56e3)) - Tyr Chen
- panic with proper message - ([e0dedfd](https://github.com/tyrchen/sqlx-db-tester/commit/e0dedfdaed610b9ae2c83c69f612ceed5c511801)) - Tyr Chen

### Other

- Update CHANGELOG.md - ([5dc1868](https://github.com/tyrchen/sqlx-db-tester/commit/5dc18687c9b394bb35ba73316f7ed2e2db884ef9)) - Tyr Chen

---
## [0.4.0](https://github.com/tyrchen/sqlx-db-tester/compare/v0.3.6..v0.4.0) - 2023-12-18

### Bug Fixes

- gh action - ([870e9fe](https://github.com/tyrchen/sqlx-db-tester/commit/870e9fed0076bd0c21b427eb183e55cbc586eff4)) - Tyr Chen

### Miscellaneous Chores

- update dependencies - ([6969947](https://github.com/tyrchen/sqlx-db-tester/commit/69699474532baa756ba5bbd0bd359857c6cce9e3)) - Tyr Chen

### Other

- Update CHANGELOG.md - ([22733a6](https://github.com/tyrchen/sqlx-db-tester/commit/22733a61e1d154e0e17b2a9d2cd622ac3165ba38)) - Tyr Chen
- Update CHANGELOG.md - ([c67f83d](https://github.com/tyrchen/sqlx-db-tester/commit/c67f83da53cb928a3b2f9d5e87c66af2ff09a9a5)) - Tyr Chen

---
## [0.3.6](https://github.com/tyrchen/sqlx-db-tester/compare/v0.3.5..v0.3.6) - 2023-02-14

### Miscellaneous Chores

- update deps - ([57ad45c](https://github.com/tyrchen/sqlx-db-tester/commit/57ad45c51b16406fd15f34ed6330679fb6bcee74)) - Tyr Chen

### Other

- Update CHANGELOG.md - ([6e6e88b](https://github.com/tyrchen/sqlx-db-tester/commit/6e6e88bd4b3c86a58bd36dd818f5e1cc7cbf5c98)) - Tyr Chen

---
## [0.3.5](https://github.com/tyrchen/sqlx-db-tester/compare/v0.3.4..v0.3.5) - 2023-02-10

### Features

- support to load csv data - ([1290868](https://github.com/tyrchen/sqlx-db-tester/commit/1290868db99dc771500245cb1e6c3149bcc95813)) - Tyr Chen

### Other

- Update CHANGELOG.md - ([dd95d56](https://github.com/tyrchen/sqlx-db-tester/commit/dd95d56cdd5679cf27898011386a290f0a6f6f76)) - Tyr Chen

---
## [0.3.4](https://github.com/tyrchen/sqlx-db-tester/compare/v0.3.3..v0.3.4) - 2023-02-07

### Bug Fixes

- more flexible interface for load_csv - ([72a2ba9](https://github.com/tyrchen/sqlx-db-tester/commit/72a2ba9b54006d28c914d2117bfb0799ba9ba64f)) - Tyr Chen

### Other

- Update CHANGELOG.md - ([7b5eb21](https://github.com/tyrchen/sqlx-db-tester/commit/7b5eb2182c10d0e2b8c0cd2f3c056a2dba7b5467)) - Tyr Chen

---
## [0.3.3](https://github.com/tyrchen/sqlx-db-tester/compare/v0.3.2..v0.3.3) - 2023-02-06

### Features

- support copy from csv - ([ab03c5d](https://github.com/tyrchen/sqlx-db-tester/commit/ab03c5dc386afcb1d8bade5ec17a2c3f06e2c9d0)) - Tyr Chen

### Other

- Update CHANGELOG.md - ([2710f8c](https://github.com/tyrchen/sqlx-db-tester/commit/2710f8c982039059413e9f24ec2459c802c126a2)) - Tyr Chen

---
## [0.3.2](https://github.com/tyrchen/sqlx-db-tester/compare/v0.3.1..v0.3.2) - 2023-01-11

### Miscellaneous Chores

- udpate deps and add Makefile - ([2774e93](https://github.com/tyrchen/sqlx-db-tester/commit/2774e93464b971b97a9f1d1c97ea45903f4c2c9e)) - Tyr Chen

### Other

- update changelog - ([219dad1](https://github.com/tyrchen/sqlx-db-tester/commit/219dad1d9ddcc85947d73e2744d2f9ad3149e878)) - Tyr Chen

---
## [0.3.1](https://github.com/tyrchen/sqlx-db-tester/compare/v0.3.0..v0.3.1) - 2022-11-18

### Miscellaneous Chores

- simplify the interface - ([ab2f81f](https://github.com/tyrchen/sqlx-db-tester/commit/ab2f81f126e70e942801b2f3251a9593ce3b5ccd)) - Tyr Chen

### Other

- update changelog - ([d8b1032](https://github.com/tyrchen/sqlx-db-tester/commit/d8b1032393dd51b19adece89ff0f2543cbe77db8)) - Tyr Chen

---
## [0.3.0](https://github.com/tyrchen/sqlx-db-tester/compare/v0.2.1..v0.3.0) - 2022-11-18

### Miscellaneous Chores

- roll code back to only support postgres - ([7f15d4a](https://github.com/tyrchen/sqlx-db-tester/commit/7f15d4acf6ae02298d7a813dba065bc0d43b2f76)) - Tyr Chen

### Other

- update changelog - ([ca970ee](https://github.com/tyrchen/sqlx-db-tester/commit/ca970ee95332060e58f0e86dc1cad7df98b146be)) - Tyr Chen

---
## [0.2.1](https://github.com/tyrchen/sqlx-db-tester/compare/v0.2.0..v0.2.1) - 2022-11-17

### Miscellaneous Chores

- simplify sqlx features - ([22fddc9](https://github.com/tyrchen/sqlx-db-tester/commit/22fddc9fddf8267b36d82899ec029af4d28aa5ee)) - Tyr Chen

### Other

- update changelog - ([cab4444](https://github.com/tyrchen/sqlx-db-tester/commit/cab4444e0a3e7724d8e9f480d65398c8203beb0f)) - Tyr Chen

---
## [0.2.0] - 2022-11-17

### Bug Fixes

- fix github action - ([27cc627](https://github.com/tyrchen/sqlx-db-tester/commit/27cc6274cd0e183f1911ccf77c14b13a502f13f0)) - Tyr Chen
- fix github action - ([68a19a9](https://github.com/tyrchen/sqlx-db-tester/commit/68a19a9c429c6ab78e0f6831956445c20bab534c)) - Tyr Chen

### Features

- use AnyConnect so that db tester support more dbs. Also make migration source more generic - ([b30152a](https://github.com/tyrchen/sqlx-db-tester/commit/b30152a9899f1f04199d337b457dcbaa2a102505)) - Tyr Chen

### Other

- init project - moved the TestConfig from reservation repo to this new crate - ([3043123](https://github.com/tyrchen/sqlx-db-tester/commit/3043123d9d7fa28ba3e9294d3ed6d88b08d7be1b)) - Tyr Chen
- add postgres service - ([b4b08c9](https://github.com/tyrchen/sqlx-db-tester/commit/b4b08c90f7f9d7fee5585a137b04d8618a9150d9)) - Tyr Chen
- use health cmd - ([bbbcfc5](https://github.com/tyrchen/sqlx-db-tester/commit/bbbcfc580ffd18ab2f6a7991a7e4a016b01dd465)) - Tyr Chen
- provide a default TestDb implementation - ([89d6385](https://github.com/tyrchen/sqlx-db-tester/commit/89d638571e3a67616a5712bdb5d2ce852ffcba20)) - Tyr Chen

<!-- generated by git-cliff -->
