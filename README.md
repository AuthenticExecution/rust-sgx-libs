# rust-sgx-libs

Utility rust libraries for my thesis work

## reactive_crypto

Rust library that contains functions for authenticated encryption. Supports: AES, SPONGENT.

See library documentation for more details.

## reactive_net

Rust library for network communications.

It handles the communication between:

- an Event Manager and its Software Modules
- the Deployer and an Event Manager
- two Event Managers

### Messages types

**message**

Simplest message that can be exchanged.

See functions `read_message` and `write_message`

The format is: `<payload_size><payload>`

  - Payload size is 16 bits

**result**

Result of a operation (like a entrypoint call).

See functions `read_result` and `write_result`

The format is: `<code><payload_size><payload>`

  - Code size is 8 bits (it can be intended as the return value)
  - Payload size is 16 bits

This data is mapped into a `ResultMessage` struct

**command**

Command sent to an Event Manager.

See functions `read_command` and `write_command`

The format is: `<cmd_id><payload_size><payload>`

  - Command id is 16 bits
  - Payload size is 16 bits
