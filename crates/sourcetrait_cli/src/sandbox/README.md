SourceTrait CLI: Box 
================================================================================
*Sandboxed command-line environments for data analytics scripting*

**SourceTrait Box** uses `podman` containers to provide specialized
`nushell` scripting environments. 

Each environment includes plugins that extend nushell with libraries and tools
for use in data analytics and infrastructure management:
- OpenAPI
- JSONL to SQLite
- (Roadmap):
  - Cloudflare
  - AWS


Design
--------------------------------------------------------------------------------

Box is designed for a complex downstream:
- SourceTrait Box
  - Sandbox Developer: Designs and maintains a specialized toolkit & environment
    - Sandbox Community Developers: Extend a specialized sandbox
      - User: Uses a specialized toolkit along with various extensions

## On containers

Benefits of using a containerized shell include security and portability.

Containers help mitigate security concerns for the end-user; By default, the
host system isn't widely available to the container and its add-ons. The primary
attack vector is then, by default, limited to configuration and data that exists
in the sandbox itself. Container developers must go out of their way to create
security holes on the host system.

Data analytics environments often involve a decent amount of configuration and
one-off customization. Each user has their own little toolbox that works for
them. They are not production environments; Source-code and configuration
management, data replication, and system setup automation are not reliably
involved. Containerization is a benefit here in that, at the very least,
an end-user can simply copy-paste and entire container's data volume to share
it or use it on a different host system.

## On nushell

Nushell has advantages for data-centric use cases:
- Scripting language: Well designed and maintainable
- Data: Treats data output as records, tables, and streams
- API: Includes JSON manipulation, http queries, sqlite queries


Usage
--------------------------------------------------------------------------------

For help, see `srctrait box --help`. Aliases are available for some subcommands.

`srctrait box <command>`
- `fetch <url | path> [name]` Downloads a sandbox image, with a custom *name* if desired
- `shell <name>` Shells into a sandbox container, starting it if necessary 
- `stop <name>` Stops a sandbox container
- `start <name>` Starts a sandbox container
- `update [name]` Updates a sandbox image or all of them if *name* is omitted
- `restart <name>` Restarts a sandbox container
- `refresh <name>` Updates a sandbox image and restarts its container if running


Installation
--------------------------------------------------------------------------------

### Dependencies 
- [podman](https://podman.io/docs/installation)
  - Install
    - Linux package name (Arch, Debian, RedHat): `podman`
    - MacOS & Windows: [Download & Install](https://github.com/containers/podman/releases) 
  - Setup
    - run: `podman machine init`
