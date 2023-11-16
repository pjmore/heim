#### Misc

* [ ] Investigate os dependeant sizes/os dependent metric results. For example [issue #207](https://github.com/heim-rs/heim/issues/207)

#### Memory

* [ ] May not have MemAvailable for linux kernal version < 3.14. Do I want to support this? Detect and error?

#### Disk

* [ ] `disk-quota`: Investigate if possible in cross platform way


#### Sensors

* [ ] `temp`: Windows, macos, bsd

* [ ] `fan`: Windows, macos, bsd

#### `process::Process`
There is a lot to add to `process::Process` struct, this issue should help to track the missing things. Method names are based on [`psutil` naming](https://psutil.readthedocs.io/en/latest/#functions), so there might be some renaming made during the implementation process later.

* [ ]  `cmdline`: [process::Process::cmdline for Windows #99](https://github.com/heim-rs/heim/issues/99)
 
* [ ]  `environ`: [process::Process::environment for Windows #210](https://github.com/heim-rs/heim/issues/210)
 
* [x]  `create_time`: [process::Process::create_time for Linux #100](https://github.com/heim-rs/heim/issues/100), [process::Process::create_time for macOS #101](https://github.com/heim-rs/heim/issues/101), [process::Process::create_time for Windows #102](https://github.com/heim-rs/heim/issues/102)
 
* [x]  `cwd`
 
* [ ]  `username`: [Process::user method #194](https://github.com/heim-rs/heim/issues/194)
 
* [ ]  `uids`
 
* [ ]  `gids`
 
* [ ]  `terminal`
 
* [x]  `nice`: [Process::niceness for *nix systems #216](https://github.com/heim-rs/heim/issues/216), [Process::priority for Windows #217](https://github.com/heim-rs/heim/issues/217)
 
* [ ]  `ionice`
 
* [ ]  `rlimit`
 
* [x]  `disk_io_counters`: [Process disk IO counters for Linux #127](https://github.com/heim-rs/heim/issues/127)
 
* [x]  `net_io_counters`: [process::Process::net_io_counters for Linux #124](https://github.com/heim-rs/heim/issues/124)
 
* [ ]  `num_ctx_switches`
 
* [ ]  `num_fds`
 
* [ ]  `num_handles`
 
* [ ]  `num_threads`
 
* [ ]  `threads`
 
* [x]  `cpu_times`: [process::Process::cpu_times method for Linux #107](https://github.com/heim-rs/heim/issues/107), [process::Process::cpu_times method for macOS #108](https://github.com/heim-rs/heim/issues/108), [process::Process::cpu_times method for Windows #109](https://github.com/heim-rs/heim/issues/109)
 
* [x]  `cpu_percent`: [process::Process::cpu_percent for Linux #134](https://github.com/heim-rs/heim/issues/134), [process::Process::cpu_percent for macOS #135](https://github.com/heim-rs/heim/issues/135), [process::Process::cpu_percent for Windows #136](https://github.com/heim-rs/heim/issues/136)
 
* [ ]  `cpu_affinity`
 
* [ ]  `cpu_num`
 
* [x]  `memory_info`: [process::Process::memory for Linux #121](https://github.com/heim-rs/heim/issues/121), [process::Process::memory for macOS #122](https://github.com/heim-rs/heim/issues/122), [process::Process::memory for Windows #123](https://github.com/heim-rs/heim/issues/123)
 
* [ ]  `memory_full_info`
 
* [ ]  `memory_maps`
 
* [ ]  `children`
 
* [ ]  `open_files`
 
* [ ]  `connections`
 
* [x]  `is_running`: [process::Process::is_running for Linux #151](https://github.com/heim-rs/heim/issues/151), [process::Process::is_running for macOS #152](https://github.com/heim-rs/heim/issues/152), [process::Process::is_running for Windows #153](https://github.com/heim-rs/heim/issues/153)
 
* [x]  `send_signal`: [process::Process::signal for nix systems #156](https://github.com/heim-rs/heim/issues/156)
 
* [x]  `suspend`: [process::Process::suspend and ::resume for nix systems #164](https://github.com/heim-rs/heim/issues/164), [process::Process::suspend for Windows #165](https://github.com/heim-rs/heim/issues/165)
 
* [x]  `resume`: [process::Process::suspend and ::resume for nix systems #164](https://github.com/heim-rs/heim/issues/164), [process::Process::resume for Windows #166](https://github.com/heim-rs/heim/issues/166)
 
* [x]  `terminate`: [process::Process::terminate for nix systems #162](https://github.com/heim-rs/heim/issues/162), [process::Process::terminate for Windows #163](https://github.com/heim-rs/heim/issues/163)
 
* [x]  `kill`: [process::Process::kill for nix systems #158](https://github.com/heim-rs/heim/issues/158), [process::Process::kill for Windows #159](https://github.com/heim-rs/heim/issues/159)
 
* [ ] `wait`: [Process::wait for Windows #215](https://github.com/heim-rs/heim/issues/215)