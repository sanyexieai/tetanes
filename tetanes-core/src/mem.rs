//! Memory and Bankswitching implementations.

use crate::common::{Reset, ResetKind};
use rand::RngCore;
use serde::{
    Deserialize, Deserializer, Serialize, Serializer,
    de::{SeqAccess, Visitor},
    ser::SerializeTuple,
};
use std::{
    fmt,
    marker::PhantomData,
    num::NonZeroUsize,
    ops::{Deref, DerefMut, Index, IndexMut},
    str::FromStr,
};

/// Represents ROM or RAM memory in bytes, with a custom Debug implementation that avoids
/// printing the entire contents.
#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Memory<D> {
    ram_state: RamState,
    is_ram: bool,
    data: D,
}

impl<D> Memory<D> {
    /// Create a new `Memory` instance.
    pub fn new() -> Self
    where
        D: Default,
    {
        Self::default()
    }

    /// Whether this `Memory` is RAM or ROM.
    pub const fn is_ram(&self) -> bool {
        self.is_ram
    }
}

impl Memory<Vec<u8>> {
    /// Create a default ROM `Memory` instance.
    pub fn rom() -> Self {
        Self::default()
    }

    /// Create a default RAM `Memory` instance.
    pub const fn ram(ram_state: RamState) -> Self {
        Self {
            ram_state,
            is_ram: true,
            data: Vec::new(),
        }
    }

    /// Set `Memory` as ram.
    pub const fn set_ram(&mut self, ram_state: RamState) {
        self.ram_state = ram_state;
        self.is_ram = true;
    }

    /// Fill ram based on [`RamState`].
    pub fn with_ram_state(mut self, state: RamState) -> Self {
        self.ram_state = state;
        self.ram_state.fill(&mut self.data);
        self
    }

    /// Set `Memory` to have the given size, filled by `ram_state`.
    pub fn with_size(mut self, size: usize) -> Self {
        self.resize(size);
        self
    }

    /// Resize `Memory` to the given size, filled by `ram_state`.
    pub fn resize(&mut self, size: usize) {
        self.data.resize(size, 0);
        self.ram_state.fill(&mut self.data);
    }
}

impl<T, const N: usize> Memory<ConstSlice<T, N>> {
    /// Create a default ROM `Memory` instance.
    pub fn rom_const() -> Self
    where
        T: Default + Copy,
    {
        Self::default()
    }

    /// Create a default RAM `Memory` instance.
    pub fn ram_const(ram_state: RamState) -> Self
    where
        T: Default + Copy,
    {
        Self {
            ram_state,
            is_ram: true,
            data: ConstSlice::new(),
        }
    }
}

impl Reset for Memory<Vec<u8>> {
    fn reset(&mut self, kind: ResetKind) {
        if self.is_ram && kind == ResetKind::Hard {
            self.ram_state.fill(&mut self.data);
        }
    }
}

impl<const N: usize> Reset for Memory<ConstSlice<u8, N>> {
    fn reset(&mut self, kind: ResetKind) {
        if self.is_ram && kind == ResetKind::Hard {
            self.ram_state.fill(&mut *self.data);
        }
    }
}

impl fmt::Debug for Memory<Vec<u8>> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Memory")
            .field("ram_state", &self.ram_state)
            .field("is_ram", &self.is_ram)
            .field("len", &self.data.len())
            .field("capacity", &self.data.capacity())
            .finish()
    }
}

impl<T, const N: usize> fmt::Debug for Memory<ConstSlice<T, N>> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Memory")
            .field("ram_state", &self.ram_state)
            .field("is_ram", &self.is_ram)
            .field("len", &self.data.len())
            .finish()
    }
}

impl<T> From<T> for Memory<T> {
    fn from(data: T) -> Self {
        Self {
            ram_state: RamState::default(),
            is_ram: false,
            data,
        }
    }
}

impl<D: Deref> Deref for Memory<D> {
    type Target = <D as Deref>::Target;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<D: DerefMut> DerefMut for Memory<D> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl<T, D: AsRef<[T]>> AsRef<[T]> for Memory<D> {
    fn as_ref(&self) -> &[T] {
        self.data.as_ref()
    }
}

impl<T, D: AsMut<[T]>> AsMut<[T]> for Memory<D> {
    fn as_mut(&mut self) -> &mut [T] {
        self.data.as_mut()
    }
}

#[derive(Clone)]
pub struct ConstSlice<T, const N: usize>([T; N]);

impl<T, const N: usize> ConstSlice<T, N> {
    /// Create a new `ConstSlice` instance.
    pub fn new() -> Self
    where
        T: Default + Copy,
    {
        Self::default()
    }

    /// Create a new `ConstSlice` instance filled with `val`.
    pub const fn filled(val: T) -> Self
    where
        T: Copy,
    {
        Self([val; N])
    }
}

impl<T: Default + Copy, const N: usize> Default for ConstSlice<T, N> {
    fn default() -> Self {
        Self([T::default(); N])
    }
}

impl<T, const N: usize> From<[T; N]> for ConstSlice<T, N> {
    fn from(data: [T; N]) -> Self {
        Self(data)
    }
}

impl<T, const N: usize> Deref for ConstSlice<T, N> {
    type Target = [T; N];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T, const N: usize> DerefMut for ConstSlice<T, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T, const N: usize> AsRef<[T]> for ConstSlice<T, N> {
    fn as_ref(&self) -> &[T] {
        self.0.as_ref()
    }
}

impl<T, const N: usize> AsMut<[T]> for ConstSlice<T, N> {
    fn as_mut(&mut self) -> &mut [T] {
        self.0.as_mut()
    }
}

impl<T, const N: usize> Index<usize> for ConstSlice<T, N> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        debug_assert!(self.0.len().is_power_of_two());
        self.0.index(index & (self.0.len() - 1))
    }
}

impl<T, const N: usize> IndexMut<usize> for ConstSlice<T, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        debug_assert!(self.0.len().is_power_of_two());
        self.0.index_mut(index & (self.0.len() - 1))
    }
}

impl<T: Serialize, const N: usize> Serialize for ConstSlice<T, N> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_tuple(N)?;
        for item in &self.0 {
            s.serialize_element(item)?;
        }
        s.end()
    }
}

impl<'de, T, const N: usize> Deserialize<'de> for ConstSlice<T, N>
where
    T: Deserialize<'de> + Default + Copy,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ArrayVisitor<T, const N: usize>(PhantomData<T>);

        impl<'de, T, const N: usize> Visitor<'de> for ArrayVisitor<T, N>
        where
            T: Deserialize<'de> + Default + Copy,
        {
            type Value = [T; N];

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str(&format!("an array of length {N}"))
            }

            #[inline]
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut data = [T::default(); N];
                for data in &mut data {
                    match (seq.next_element())? {
                        Some(val) => *data = val,
                        None => return Err(serde::de::Error::invalid_length(N, &self)),
                    }
                }
                Ok(data)
            }
        }

        deserializer
            .deserialize_tuple(N, ArrayVisitor(PhantomData))
            .map(Self)
    }
}

/// A trait that represents memory read operations. Reads typically have side-effects.
pub trait Read {
    /// Read from the given address.
    fn read(&mut self, addr: u16) -> u8 {
        self.peek(addr)
    }

    /// Read two bytes from the given address.
    fn read_u16(&mut self, addr: u16) -> u16 {
        let lo = self.read(addr);
        let hi = self.read(addr.wrapping_add(1));
        u16::from_le_bytes([lo, hi])
    }

    /// Peek from the given address.
    fn peek(&self, addr: u16) -> u8;

    /// Peek two bytes from the given address.
    fn peek_u16(&self, addr: u16) -> u16 {
        let lo = self.peek(addr);
        let hi = self.peek(addr.wrapping_add(1));
        u16::from_le_bytes([lo, hi])
    }
}

/// A trait that represents memory write operations.
pub trait Write {
    /// Write value to the given address.
    fn write(&mut self, addr: u16, val: u8);

    /// Write  valuetwo bytes to the given address.
    fn write_u16(&mut self, addr: u16, val: u16) {
        let [lo, hi] = val.to_le_bytes();
        self.write(addr, lo);
        self.write(addr, hi);
    }
}

/// RAM in a given state on startup.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[must_use]
pub enum RamState {
    #[default]
    AllZeros,
    AllOnes,
    Random,
}

impl RamState {
    /// Return `RamState` options as a slice.
    pub const fn as_slice() -> &'static [Self] {
        &[Self::AllZeros, Self::AllOnes, Self::Random]
    }

    /// Return `RamState` as a `str`.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::AllZeros => "all-zeros",
            Self::AllOnes => "all-ones",
            Self::Random => "random",
        }
    }

    /// Fills data slice based on `RamState`.
    pub fn fill(&self, data: &mut [u8]) {
        match self {
            RamState::AllZeros => data.fill(0x00),
            RamState::AllOnes => data.fill(0xFF),
            RamState::Random => {
                rand::rng().fill_bytes(data);
            }
        }
    }
}

impl From<usize> for RamState {
    fn from(value: usize) -> Self {
        match value {
            0 => Self::AllZeros,
            1 => Self::AllOnes,
            _ => Self::Random,
        }
    }
}

impl AsRef<str> for RamState {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for RamState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::AllZeros => "All $00",
            Self::AllOnes => "All $FF",
            Self::Random => "Random",
        };
        write!(f, "{s}")
    }
}

impl FromStr for RamState {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "all-zeros" => Ok(Self::AllZeros),
            "all-ones" => Ok(Self::AllOnes),
            "random" => Ok(Self::Random),
            _ => Err("invalid RamState value. valid options: `all-zeros`, `all-ones`, or `random`"),
        }
    }
}

/// Represents allowed memory bank access.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[must_use]
pub enum BankAccess {
    None,
    Read,
    ReadWrite,
}

/// Represents a set of memory banks.
#[derive(Clone, Serialize, Deserialize)]
#[must_use]
pub struct Banks {
    start: usize,
    end: usize,
    size: NonZeroUsize,
    window: NonZeroUsize,
    shift: usize,
    mask: usize,
    banks: Vec<usize>,
    access: Vec<BankAccess>,
    page_count: usize,
}

#[derive(thiserror::Error, Debug)]
#[must_use]
pub enum Error {
    #[error("bank `window` must a non-zero power of two")]
    InvalidWindow,
    #[error("bank `size` must be non-zero")]
    InvalidSize,
}

impl Banks {
    pub fn new(
        start: usize,
        end: usize,
        capacity: usize,
        window: impl TryInto<NonZeroUsize>,
    ) -> Result<Self, Error> {
        let window = window.try_into().map_err(|_| Error::InvalidWindow)?;
        if !window.is_power_of_two() {
            return Err(Error::InvalidWindow);
        }

        let size = NonZeroUsize::try_from(end - start).map_err(|_| Error::InvalidSize)?;
        let bank_count = (size.get() + 1) / window;

        let mut banks = vec![0; bank_count];
        let access = vec![BankAccess::ReadWrite; bank_count];
        for (i, bank) in banks.iter_mut().enumerate() {
            *bank = (i * window.get()) % capacity;
        }
        let page_count = capacity / window.get();

        Ok(Self {
            start,
            end,
            size,
            window,
            shift: window.trailing_zeros() as usize,
            mask: page_count.saturating_sub(1),
            banks,
            access,
            page_count,
        })
    }

    pub fn set(&mut self, mut bank: usize, page: usize) {
        if bank >= self.banks.len() {
            bank %= self.banks.len();
        }
        assert!(bank < self.banks.len());
        self.banks[bank] = (page & self.mask) << self.shift;
        debug_assert!(self.banks[bank] < self.page_count * self.window.get());
    }

    pub fn set_range(&mut self, start: usize, end: usize, page: usize) {
        let mut new_addr = (page & self.mask) << self.shift;
        for mut bank in start..=end {
            if bank >= self.banks.len() {
                bank %= self.banks.len();
            }
            assert!(bank < self.banks.len());
            self.banks[bank] = new_addr;
            debug_assert!(self.banks[bank] < self.page_count * self.window.get());
            new_addr += self.window.get();
        }
    }

    pub fn set_access(&mut self, mut bank: usize, access: BankAccess) {
        if bank >= self.banks.len() {
            bank %= self.banks.len();
        }
        assert!(bank < self.banks.len());
        self.access[bank] = access;
    }

    pub fn set_access_range(&mut self, start: usize, end: usize, access: BankAccess) {
        for slot in start..=end {
            self.set_access(slot, access);
        }
    }

    pub fn readable(&self, addr: u16) -> bool {
        let slot = self.get(addr);
        assert!(slot < self.banks.len());
        matches!(self.access[slot], BankAccess::Read | BankAccess::ReadWrite)
    }

    pub fn writable(&self, addr: u16) -> bool {
        let slot = self.get(addr);
        assert!(slot < self.banks.len());
        self.access[slot] == BankAccess::ReadWrite
    }

    #[must_use]
    pub const fn last(&self) -> usize {
        self.page_count.saturating_sub(1)
    }

    #[must_use]
    pub fn banks_len(&self) -> usize {
        self.banks.len()
    }

    #[must_use]
    pub const fn get(&self, addr: u16) -> usize {
        (addr as usize & self.size.get()) >> self.shift
    }

    #[must_use]
    pub fn translate(&self, addr: u16) -> usize {
        let slot = self.get(addr);
        assert!(slot < self.banks.len());
        let page_offset = self.banks[slot];
        page_offset | (addr as usize) & (self.window.get() - 1)
    }

    #[must_use]
    pub fn page(&self, bank: usize) -> usize {
        self.banks[bank] >> self.shift
    }

    #[must_use]
    pub fn page_offset(&self, bank: usize) -> usize {
        self.banks[bank]
    }

    #[must_use]
    pub const fn page_count(&self) -> usize {
        self.page_count
    }
}

impl std::fmt::Debug for Banks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        f.debug_struct("Bank")
            .field("start", &format_args!("${:04X}", self.start))
            .field("end", &format_args!("${:04X}", self.end))
            .field("size", &format_args!("${:04X}", self.size))
            .field("window", &format_args!("${:04X}", self.window))
            .field("shift", &self.shift)
            .field("mask", &self.shift)
            .field("banks", &self.banks)
            .field("page_count", &self.page_count)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_bank() {
        let banks = Banks::new(
            0x8000,
            0xFFFF,
            128 * 1024,
            NonZeroUsize::new(0x4000).unwrap(),
        )
        .unwrap();
        assert_eq!(banks.get(0x8000), 0);
        assert_eq!(banks.get(0x9FFF), 0);
        assert_eq!(banks.get(0xA000), 0);
        assert_eq!(banks.get(0xBFFF), 0);
        assert_eq!(banks.get(0xC000), 1);
        assert_eq!(banks.get(0xDFFF), 1);
        assert_eq!(banks.get(0xE000), 1);
        assert_eq!(banks.get(0xFFFF), 1);
    }

    #[test]
    fn bank_translate() {
        let mut banks = Banks::new(
            0x8000,
            0xFFFF,
            128 * 1024,
            NonZeroUsize::new(0x2000).unwrap(),
        )
        .unwrap();

        let last_bank = banks.last();
        assert_eq!(last_bank, 15, "bank count");

        assert_eq!(banks.translate(0x8000), 0x0000);
        banks.set(0, 1);
        assert_eq!(banks.translate(0x8000), 0x2000);
        banks.set(0, 2);
        assert_eq!(banks.translate(0x8000), 0x4000);
        banks.set(0, 0);
        assert_eq!(banks.translate(0x8000), 0x0000);
        banks.set(0, banks.last());
        assert_eq!(banks.translate(0x8000), 0x1E000);
    }
}
