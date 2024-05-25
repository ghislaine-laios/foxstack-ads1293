#![allow(non_camel_case_types)]

pub mod access;
pub mod addressable;
pub mod data;

use enum_variant_type::EnumVariantType;
use ux::u24;

use self::{
    addressable::{Address, Addressable},
    data::DataStatus,
};

#[derive(EnumVariantType)]
pub enum Register {
    CONFIG,
    CH_CNFG,
    DATA_STATUS,
    DATA_LOOP,
}

#[derive(Clone, Copy, Debug)]
pub enum DataRegister {
    DATA_STATUS(DataStatus),
    DATA_CH1_PACE(u16),
    DATA_CH2_PACE(u16),
    DATA_CH3_PACE(u16),
    DATA_CH1_ECG(u24),
    DATA_CH2_ECG(u24),
    DATA_CH3_ECG(u24),
}

macro_rules! implement_addressable {
    ($struct: ty, $value: expr) => {
        impl Addressable for $struct {
            fn get_address(&self) -> Address {
                return $value;
            }
        }
    };
}

implement_addressable!(CONFIG, 0x01);
implement_addressable!(CH_CNFG, 0x2F);
implement_addressable!(DATA_STATUS, 0x30);
implement_addressable!(DATA_LOOP, 0x50);
