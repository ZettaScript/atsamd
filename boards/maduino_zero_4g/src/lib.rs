#![no_std]

pub use atsamd_hal as hal;
pub use hal::pac;

#[cfg(feature = "rt")]
pub use cortex_m_rt::entry;

use hal::clock::GenericClockController;
use hal::sercom::uart;
use hal::sercom::uart::BaudMode;
use hal::sercom::uart::Oversampling;
use hal::sercom::{i2c, spi, Sercom0, Sercom3, Sercom4};
use hal::time::Hertz;

#[cfg(feature = "usb")]
use hal::usb::{usb_device::bus::UsbBusAllocator, UsbBus};

hal::bsp_pins!(
    PA02 {
        name: a0,
    }
    PA03 {
        name: aref,
        aliases: {
            AlternateB: Aref,
        }
    }
    PA04 {
        name: a3,
    }
    PA05 {
        name: a4,
    }
    PA06 {
        name: d8,
    }
    PA07 {
        name: d9,
    }
    PA08 {
        name: d4,
        aliases: {
            PushPullOutput: SdCs,
        }
    }
    PA09 {
        name: d3,
    }
    PA10 {
        name: d1,
        aliases: {
            AlternateC: ModemUartTx,
        }
    }
    PA11 {
        name: d0,
        aliases: {
            AlternateC: ModemUartRx,
        }
    }
    PA12 {
        name: sd_miso,
        aliases: {
            AlternateD: SdMiso,
        }
    }
    PA14 {
        name: d2,
    }
    PA15 {
        name: d5,
        aliases: {
            PushPullOutput: ModemPower,
        }
    }
    PA16 {
        name: d11,
    }
    PA17 {
        name: d13,
    }
    PA18 {
        name: d10,
    }
    PA19 {
        name: d12,
    }
    PA20 {
        name: d6,
        aliases: {
            PushPullOutput: ModemReset,
        }
    }
    PA21 {
        name: d7,
        aliases: {
            PushPullOutput: ModemFlight,
        }
    }
    PA22 {
        name: sda,
        aliases: {
            AlternateC: Sda,
        }
    }
    PA23 {
        name: scl,
        aliases: {
            AlternateC: Scl,
        }
    }
    PA24 {
        name: usb_dm,
        aliases: {
            AlternateG: UsbDm,
        }
    }
    PA25 {
        name: usb_dp,
        aliases: {
            AlternateG: UsbDp,
        }
    }
    PA30 {
        name: swclk,
    }
    PA31 {
        name: swdio,
    }
    PB02 {
        name: a5,
    }
    PB08 {
        name: a1,
        aliases: {
            AlternateB: Vbat,
        }
    }
    PB09 {
        name: a2,
        aliases: {
            AlternateB: Vsys,
        }
    }
    PB10 {
        name: sd_mosi,
        aliases: {
            AlternateD: SdMosi,
        }
    }
    PB11 {
        name: sd_sck,
        aliases: {
            AlternateD: SdSck,
        }
    }
);

pub type SdSpiSercom = Sercom4;
pub type SdSpiPads = spi::Pads<SdSpiSercom, SdMiso, SdMosi, SdSck>;
/// SPI device for the MCU SD card
pub type SdSpi = spi::Spi<spi::Config<SdSpiPads>, spi::Duplex>;

pub type I2cSercom = Sercom3;
pub type I2cPads = i2c::Pads<I2cSercom, Sda, Scl>;
/// I2C device for the general purpose I2C interface
pub type I2c = i2c::I2c<i2c::Config<I2cPads>>;

pub type ModemUartSercom = Sercom0;
pub type ModemUartPads = uart::Pads<ModemUartSercom, ModemUartRx, ModemUartTx>;
/// UART device for the SIM7600 modem
pub type ModemUart = uart::Uart<uart::Config<ModemUartPads>, uart::Duplex>;

#[cfg(feature = "usb")]
pub fn usb_allocator(
    usb: pac::Usb,
    clocks: &mut GenericClockController,
    pm: &mut pac::Pm,
    dm: impl Into<UsbDm>,
    dp: impl Into<UsbDp>,
) -> UsbBusAllocator<UsbBus> {
    let gclk0 = clocks.gclk0();
    let usb_clock = &clocks.usb(&gclk0).unwrap();
    let (dm, dp) = (dm.into(), dp.into());
    UsbBusAllocator::new(UsbBus::new(usb_clock, pm, dm, dp, usb))
}

/// Set up the UART modem
pub fn setup_modem(
    clocks: &mut GenericClockController,
    baud: Hertz,
    uart_sercom: ModemUartSercom,
    pm: &pac::Pm,
    uart_rx: impl Into<ModemUartRx>,
    uart_tx: impl Into<ModemUartTx>,
) -> ModemUart {
    let gclk0 = clocks.gclk0();
    let clock = clocks.sercom0_core(&gclk0).unwrap();
    let pads = uart::Pads::default().rx(uart_rx.into()).tx(uart_tx.into());
    uart::Config::new(pm, uart_sercom, pads, clock.freq())
        .baud(baud, BaudMode::Fractional(Oversampling::Bits16))
        .enable()
}
