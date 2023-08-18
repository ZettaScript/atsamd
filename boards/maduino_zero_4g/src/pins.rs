//! Maduino Zero 4G LTE pins

use super::hal;

hal::bsp_pins!(
    PA02 {
        name: a0,
    }
    PA03 {
        name: aref,
        aliases: {
            AlternateB: Aref,
        },
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
    }
    PA09 {
        name: d3,
    }
    PA10 {
        name: d1,
        aliases: {
            AlternateC: ModemUartTx,
        },
    }
    PA11 {
        name: d0,
        aliases: {
            AlternateC: ModemUartRx,
        },
    }
    PA12 {
        name: sd_miso,
        aliases: {
            AlternateD: SdMiso,
        },
    }
    PA14 {
        name: d2,
    }
    PA15 {
        name: d5,
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
    }
    PA21 {
        name: d7,
    }
    PA22 {
        name: sda,
        aliases: {
            AlternateC: Sda,
        },
    }
    PA23 {
        name: scl,
        aliases: {
            AlternateC: Scl,
        },
    }
    PA24 {
        name: usb_dm,
        aliases: {
            AlternateG: UsbDm,
        },
    }
    PA25 {
        name: usb_dp,
        aliases: {
            AlternateG: UsbDp,
        },
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
        },
    }
    PB09 {
        name: a2,
        aliases: {
            AlternateB: Vsys,
        },
    }
    PB10 {
        name: sd_mosi,
        aliases: {
            AlternateD: SdMosi,
        },
    }
    PB11 {
        name: sd_sck,
        aliases: {
            AlternateD: SdSck,
        },
    }
);

impl Pins {
    /// Split the device pins into subsets
    pub fn split(self) -> Sets {
        let modem = Modem {
            flight: self.d7,
            reset: self.d6,
            pwrkey: self.d5,
        };

        let modem_uart = ModemUart {
            tx: self.d1,
            rx: self.d0,
        };

        let sd_spi = SdSpi {
            mosi: self.sd_mosi,
            sck: self.sd_sck,
            miso: self.sd_miso,
            cs: self.d4,
        };

        let i2c = I2c {
            sda: self.sda,
            scl: self.scl,
        };

        let usb = Usb {
            usb_dm: self.usb_dm,
            usb_dp: self.usb_dp,
        };

        let voltage = Voltage {
            vbat: self.a1,
            vsys: self.a2,
        };

        Sets {
            i2c,
            sd_spi,
            modem,
            modem_uart,
            usb,
            voltage,
        }
    }
}

pub struct Sets {
    pub i2c: I2c,
    pub sd_spi: SdSpi,
    pub modem: Modem,
    pub modem_uart: ModemUart,
    pub usb: Usb,
    pub voltage: Voltage,
}

pub struct I2c {
    pub sda: SdaReset,
    pub scl: SclReset,
}

pub struct SdSpi {
    pub mosi: SdMosiReset,
    pub sck: SdSckReset,
    pub miso: SdMisoReset,
    pub cs: SdCsReset,
}

pub struct Modem {
    pub flight: ModemFlightReset,
    pub pwrkey: ModemPwrkeyReset,
    pub reset: ModemResetReset,
}

pub struct ModemUart {
    pub tx: ModemUartTxReset,
    pub rx: ModemUartRxReset,
}

pub struct Usb {
    pub usb_dm: UsbDmReset,
    pub usb_dp: UsbDpReset,
}

pub struct Voltage {
    pub vbat: VoltageVbatReset,
    pub vsys: VoltageVsysReset,
}
