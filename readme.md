# Munin the watchful

This is a hacked system monitor built in rust and tui-rs. It is built as a submission to the missing semester acceptance test.

## Project Description

[!image](./assets/munin.png)

- The monitor should display the data in real-time with a refresh rate of `1.5 seconds`.
- munin can filter the list of processes by process name.( just start typing and the list will be filtered)
- press ctrl-q to quit

### Monitored Stats

- **CPU**: Overall usage, overall free, usage per application/process.
- **MEMORY**: Total, free, usage per application/process.
- **Disk**: Devices and partitions, read rate, write rate.

### Build Instructions

```Bash
  cargo build --release
```

and then run the binary

```Bash
   ./target/release/munin
```

### Verification

- [x] `README.md` with build instructions
- [x] Builds on Linux / Linux VM
- [x] Used one of the allowed programming languages (C, C++, Rust, Zig, Golang)
- [x] Monitor CPU
  - [x] Current total usage
  - [x] Current usage per application
  - [x] Current free
- [x] Monitor Memory
  - [x] Current total usage
  - [x] Current total free
  - [x] Current usage per application
- [x] Monitor Disk
  - [x] Available devices
  - [x] Available partitions
  - [x] Read rate per application
  - [x] Write rate per application
- [x] Real-time stats collection
- [ ] UI (check any or both)
  - [x] Terminal-based UI
  - [ ] Prometheus + Grafana

