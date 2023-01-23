# audio_device_tester

![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/tanelikaivola/audio_device_tester/build.yml?style=plastic)
[![GitHub release](https://img.shields.io/github/release/tanelikaivola/audio_device_tester.svg)](https://GitHub.com/tanelikaivola/audio_device_tester/releases/latest)

Audio Device Tester attempts to test system audio devices to find anomalies and make debugging audio devices easier.

It is currently developed for Windows. It might compile on other systems for now, might migrate to using Windows APIs only if [cpal](https://github.com/RustAudio/cpal/) ends up being too restrictive.

Currently it does:
- Finds all audio devices and enumerates their configuration.
- Compiles and displays sample rates for the devices.
- Opens output devices and plays silence.

While looking for errors, it also detects long delays on opening the devices and on other operations.
