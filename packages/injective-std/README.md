# injective-std

 **BETA VERSION:**

The bindings are currently tracking an unofficial release of Injective.

Injective's proto-generated types and helpers built using [Buf](https://github.com/bufbuild/buf). Enables interaction with both custom and standard modules.

## Supported Version

* Injective-Core@e38d508c746a9b90d6e8a261ec2b03f99cc49351

## Build Instructions

**Pre-requisites:**

[Buf 1.17.0](https://github.com/bufbuild/buf)

In order to generate an individual proto file run:

```shell
buf generate [/path/to/injective-core] --template [/path/to/buf.gen.yaml] --output [output-path] --path [/path/to/module/proto]
```

Example `buf.gen.yaml` file:

```yaml
plugins:
  - plugin: buf.build/community/neoeinstein-prost
    out: .
    opt:
      - extern_path=.google.protobuf.Timestamp=crate::shim::Timestamp
      - extern_path=.google.protobuf.Duration=crate::shim::Duration
      - extern_path=.google.protobuf.Any=crate::shim::Any
```