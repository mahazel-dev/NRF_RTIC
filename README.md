# NRF_RTIC

Hello everyone,

It is my project of self-learning Rust on Embedded Devices. Purpose of this project to get familiar with embedded programming in modern environment.
I work on NRF52840DK (Cortex M4 with NFC and BLE).
What I want to achieve:
  - UART communication PC-uC with interrupt trigger - basic is done, add features with collision resolution, test with some external VCOM
  - usage of OLED via I2C - display SSD1306 ordered;
  - usage of fingerprint sensor via SPI - BM-Lite ordered;
  - NFC communication - have to make plan, but sense interrupt is working fine;
  - and more by the time..


I use RTIC as main event handler.
By the time code is growing and getting mroe complicated, but I try to keep it clean.
I'm NOT professional, but I'd like to become. If you have suggestions, please don't hesitate to tell me. Or if you can offer help with learning 
by giving assigments or intern I'm open for every opportunity.


Best regards and welcome to Rust and RTIC


Materials:
- Embedded Rust Book: https://docs.rust-embedded.org/book/intro/index.html
- Ferrous Systems training: https://embedded-trainings.ferrous-systems.com/preparations.html
- Github to training /\ /\ /\ /\: https://github.com/ferrous-systems/embedded-trainings-2020
- Blog from what I started: https://nitschinger.at/Getting-Started-with-the-nRF52840-in-Rust/
- Bare MEtal Rust: https://bacelarhenrique.me/2021/02/24/how-to-access-peripherals-rust.html
- Knurling project: https://github.com/knurling-rs
- RTIC: https://rtic.rs/1/book/en/preface.html
- and more to find in links from above
