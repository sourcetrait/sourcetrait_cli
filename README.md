SourceTrait CLI
================================================================================
[![License Badge]][License]

*Command-line interface for the SourceTrait productivity suite*

Products
--------------------------------------------------------------------------------

The **SourceTrait** productivity suite is comprised of three core products:
- [SourceTrait Backup](#SourceTrait-Backup)
- [SourceTrait Filer](#SourceTrait-Filer)
- [SourceTrait Sync](#SourceTrait-Sync)

### SourceTrait Backup
--------------------------------------------------------------------------------
**SourceTrait Backup** is a rotational backup system for workstation users.

It allows easy access to previous backups on the system to quickly restore lost work.

Full backups are typically ran monthly and incremental backups (changes only) are typically ran daily. Users can customize scheduling.

Backups can be configured to be zipped, encrypted, and securely synchronized across multiple devices and cloud storage providers.

There is no vendor lock-in; The reliable rsync backup tool is used internally and the same can be used manually if necessary.


### SourceTrait Filer
**SourceTrait Filer** is a novel method of record-keeping using existing filesystem tools.

It assists users in filing consistently, according to their own rules, which in turn allows the entire directory structure to be reliably cross-referencable.

Files can be maintained entirely offline, self-hosted over a company network, or through a cloud service.

There is no vendor lock-in; No special tools are required to access anything.

### SourceTrait Sync 
**SourceTrait Sync** is a portable file synchronization system that maintains a
user's important files in a standardized way across multiple operating systems.

It is essentially a unified repository that synchronizes the configuration of other
repositories. It supports Git and SourceTrait Filer across any service provider.
All files are categorized by the [SourceTrait Who](#SourceTrait-Who)
standard.

Each of the user's repositories are stored in a standard way within their home
directory. The user can then configure symbolic links (shortcuts) for easy access,
which is automatically replicated to any other operating system that they
use.

Standards
--------------------------------------------------------------------------------

### Operating Systems 
### Cross-Platform
We provide shell scripting for:
- `nu` via [nushell](https://www.nushell.sh)

We provide development packaging for:
- `cargo` via [Rust](https://rust-lang.org/tools/install)

#### Linux
We provide packaging for:
- `apt` and `dpkg` via `.deb` packages 
- `dnf`, `yum`, and `rpm` via `.rpm` packages

#### MacOS
We provide packaging for:
- `brew` via [Homebrew](https://brew.sh) formulas

#### Windows
We provide packaging for:
- `choco` via [Chocolatey](https://chocolatey.org/install) packages

### SourceTrait Who
**SourceTrait Who** is a standard used across our suite that allow users
to operate under multiple roles on a single system.

We all wear a many hats. This is how SourceTrait classifes them ...

#### Identities and Roles
- **At** (*org*): The user's identity within an organization (typical work role)
  - `/at/(org)/`
- **As** (*alias*): The user's aliases and specialized roles (hobby accounts, admin roles, etc.)
  - `/as/(alias)/` or `/as/(org)/(role)/`
- **Me**: The user's personal identity
  - `/me/`


Crates
--------------------------------------------------------------------------------

### [SourceTrait CLI](./crates/sourcetrait_cli)
[![SourceTrait CLI Crate Badge]][SourceTrait CLI Crate] [![Docs Badge]][SourceTrait CLI Docs]

*Modular command-line interface for SourceTrait projects*


Repository
--------------------------------------------------------------------------------

Found a bug? Let us know! Upvote an existing issue on GitHub or create one if it
doesn't exist.

### Contributors
Contributors, please review [SOURCETRAIT.md](./SOURCETRAIT.md).  

#### Copyright Assignment Agreement (CAA)
By committing to this repository you agree to assign to
[Asmov LLC](https://asmov.software) all right, title, and interest worldwide in
all copyright covering your contribution.


License (AGPL3)
--------------------------------------------------------------------------------
SourceTrait CLI: Command-line interface for SourceTrait tools  
Developed by [SourceTrait](https://sourcetrait.com), a division of **Asmov LLC**  
Copyright (C) 2025 [Asmov LLC](https://asmov.software)  

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU Affero General Public License as
published by the Free Software Foundation, either version 3 of the
License, or (at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Affero General Public License for more details.

You should have received a [copy](./LICENSE-AGPL-3.txt) of the
GNU Affero General Public License along with this program.
If not, see https://www.gnu.org/licenses/.

[Docs Badge]: https://img.shields.io/badge/docs-blue
[License]: #License-AGPL3
[License Badge]: https://img.shields.io/badge/license-AGPL3-blue.svg

[SourceTrait CLI Crate]: https://crates.io/crates/sourcetrait_cli
[SourceTrait CLI Crate Badge]: https://img.shields.io/crates/v/sourcetrait_cli.svg
[SourceTrait CLI Docs]: https://docs.rs/sourcetrait_cli

