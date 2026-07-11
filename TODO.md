# TODO

## High Priority

* [ ] Add `disk info <device>`
  * Show device model
  * Show serial number
  * Show PARTUUID
  * Show UUID
  * Show filesystem
  * Show label
  * Show mount point
  * Show read-only status
  * Show removable flag

* [ ] Improve `repair`
  * Detect more filesystems
  * Better error messages
  * Show repair progress when possible

* [ ] Improve `mount`
  * Support custom mount directory
  * Allow custom mount options
  * Detect busy devices before mounting

---

## Medium Priority

* [ ] Add `disk eject <device>`
  * Safely unmount
  * Power off removable USB drives (`udisksctl power-off`)

* [ ] Add `disk benchmark`
  * Sequential read
  * Sequential write
  * Random read
  * Random write

* [ ] Add `disk smart`
  * Read SMART information (`smartctl`)
  * Show disk health
  * Show temperature
  * Show power-on hours
  * Show reallocated sectors

* [ ] Add `disk watch`
  * Continuously monitor mounted devices
  * Refresh every few seconds

* [ ] Colored output for filesystem types

* [ ] Detect encrypted (LUKS) containers more intelligently

---

## Low Priority

* [ ] Add `disk format`
  * ext4
  * exFAT
  * NTFS
  * FAT32
  * Confirmation prompt before formatting

* [ ] Add `disk label`
  * Rename filesystem labels

* [ ] Add `disk uuid --random`
  * Regenerate filesystem UUID (where supported)

* [ ] Add `disk tree`
  * Tree view only
  * Hide filesystem statistics

* [ ] Export device list as JSON

* [ ] Export device list as YAML

* [ ] Export device list as TOML

---

## Nice to Have

* [ ] Show filesystem icons

* [ ] Detect external SSD/HDD automatically

* [ ] Human-friendly disk health summary

* [ ] Optional compact output mode

* [ ] Optional detailed output mode

* [ ] Shell completions
  * Bash
  * Zsh
  * Fish

* [ ] Man page generation

* [ ] Package for AUR
