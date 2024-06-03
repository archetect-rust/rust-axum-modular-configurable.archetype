# Archetype: Rust Axum Modular Configurable Archetype

A modular Rust Axum project with a CLI and Config system

- Strong adherence to [Twelve Factor](https://12factor.net/) principles
  - Layered, hierarchical configuration using [config](https://github.com/mehcode/config-rs) 
  - Run and managed by an ergonomic CLI interface, powered by [clap](https://github.com/clap-rs/clap)
- Completely asynchronous, powered by [Tokio](https://tokio.rs/)
- Modular, with individually usable and tested layers:
  - Core: business layer
  - Server: network layer


To generate a project from this archetype using [Archetect](https://github.com/archetect/archetect):

```shell
archetect render https://github.com/archetect-rust/rust-axum-modular-configurable.archetype.git
```
