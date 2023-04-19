# GameController

This is the GameController software for robot soccer games in the RoboCup Standard Platform League.

## Compilation

### Prerequisites

- Rust and other platform-specific tauri dependencies as [listed here](https://tauri.app/v1/guides/getting-started/prerequisites)
- nodejs and npm (or a compatible package manager)
- libclang (for bindgen)
    - On Windows:
    ```powershell
    winget install LLVM
    set LIBCLANG_PATH 'C:\Program Files\LLVM\bin\'
    ```

### Commands

First, the frontend must be compiled:

```bash
cd game_controller_app/frontend
npm install
npm run build
```

The Rust code is compiled with cargo:

```bash
cargo build [-r]
```

## Configuration

Configuration files that are read at runtime are located in the directory `config`.
The global `teams.yaml` is a list of all teams in the SPL, their names, and their available jersey colors both for field players and goalkeepers.
Each (sub)competition has a subdirectory with two files:
`params.yaml` corresponds to the Rust struct `game_controller::types::CompetitionParams` and sets (mostly numeric) constants for the competition.
`teams.yaml` is a list of team numbers of the teams that participate in the competition.
Only those teams can be selected when playing in this competition.
Therefore, for a new team to appear in the UI, an entry must be added both to the global `teams.yaml` (with an unused team number) and in the competition's `teams.yaml` (referencing the team number).

## Network Communication

Currently, all network communication with the GameController uses IPv4, although most parts of the code can also handle IPv6.

The GameController communicates with robot players via three channels:
- It sends control messages at a rate of 2 hertz (UDP broadcast on port 3838, format specified in the struct `RoboCupGameControlData` in `game_controller_msgs/headers/RoboCupGameControlData.h`).
    These control messages do not always represent the true game state, specifically after a goal or a transition to the `playing` state.
    After these events, they continue to maintain the state before the event for up to 15 seconds, or until another event happens that could not have happened in this "fake" state.
    Note that this behavior differs from the old GameController, which would always keep the state attribute (and some others) at the old value for 15 seconds, even when other attributes already clearly indicated that it was the new state (e.g. players are unpenalized although their timers aren't at zero yet, or set plays starting during the "fake" `set` state when it is actually already `playing`).
- It receives status messages from the robot players which must send them at a rate between 0.5 hertz and 2 hertz (UDP unicast on port 3939, format specified in the struct `RoboCupGameControlReturnData` in `game_controller_msgs/headers/RoboCupGameControlData.h`).
- It receives team messages from the robot players (UDP broadcast on port 10000 + team number, up to 128 bytes of payload with arbitrary format).

In addition, the GameController offers an interface for monitor applications (such as the TeamCommunicationMonitor or the EventRecorder):
- It receives monitor requests (UDP unicast on port 3636, 4 bytes header magic `RGTr` + 1 byte version number `0`).
    It refuses to accept monitor requests from hosts that have previously sent status messages, as those are presumed to be robot players which should not get true data.
    Similarly, if a host that had previously sent a monitor request sends a status message, it will not receive monitor data anymore.
- Each registered monitor host will get:
    - control messages with the true game state at a rate of 2 hertz (UDP unicast on port 3838, with the same format as regular control messages, but with the header magic `RGTD`).
    - forwarded status messages (UDP unicast on port 3940, prefixed by the IPv4 address of the original sender).
        The forwarded payload has not been validated.

The user must ensure that all of the aforementioned network communication channels are allowed to be used by the firewall.
The GameController runs on a specific network interface, which generally specifies where packets are sent and from where they are received.
The exceptions are that control messages can be configured to be sent to the limited broadcast address (`255.255.255.255`) instead of the interface's broadcast address, and that team messages are received from any address.

## Usage

### Start

Given the absence of binary packages at this point, the user will have compiled the GameController at this point (or do it now).
Consequently, the most convenient way to run it is by executing

```bash
cargo run [-r]
```

from a command line within any directory of this workspace.
The program accepts command line arguments which can be passed to `cargo` after `--`.
They override the defaults of the launcher.
A list of arguments (that is always up to date, in contrast to what would be written here) can by obtained by running with the `-h` option:

```bash
cargo run -- -h
```

Note that release builds on Windows do not output any text.

## Logs

The GameController writes log files to the directory `logs`.
They can get quite large because they are YAML.
The main reason for YAML is that it is human-readable and can be appended (in contrast to JSON which requires a closing bracket in the end to be well-formed).

At the moment, there are no tools to process these log files, but eventually, the tools from the old GameController should be ported.
