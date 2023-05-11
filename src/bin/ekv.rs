#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]
#![feature(async_fn_in_trait)]
#![allow(incomplete_features)]

use defmt::*;
use ekv::flash::{self, PageID};
use ekv::{config, Database};
use embassy_executor::Spawner;
use embassy_rp::flash::{Flash, ERASE_SIZE};
use embassy_rp::peripherals::FLASH;
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use {defmt_rtt as _, panic_probe as _};

pub const FLASH_SIZE: usize = 2 * 1024 * 1024;
extern "C" {
    // Flash storage used for configuration
    static __config_start: u32;
}

pub struct DbFlash<'a>(pub Flash<'a, FLASH, FLASH_SIZE>);

// Workaround for alignment requirements.
#[repr(C, align(4))]
pub struct AlignedBuf([u8; ERASE_SIZE]);

impl<'a> flash::Flash for DbFlash<'a> {
    type Error = embassy_rp::flash::Error;

    fn page_count(&self) -> usize {
        config::MAX_PAGE_COUNT
    }

    async fn erase(&mut self, page_id: PageID) -> Result<(), <DbFlash<'a> as flash::Flash>::Error> {
        let start = unsafe { &__config_start as *const u32 as usize };
        self.0.erase(
            (start + page_id.index() * config::PAGE_SIZE) as u32,
            (start + page_id.index() * config::PAGE_SIZE + config::PAGE_SIZE) as u32,
        )
    }

    async fn read(
        &mut self,
        page_id: PageID,
        offset: usize,
        data: &mut [u8],
    ) -> Result<(), <DbFlash<'a> as flash::Flash>::Error> {
        let start: usize = unsafe { &__config_start as *const u32 as usize };
        let address = start + page_id.index() * config::PAGE_SIZE + offset;
        let mut buf = AlignedBuf([0; ERASE_SIZE]);
        self.0.read(address as u32, &mut buf.0[..data.len()])?;
        data.copy_from_slice(&buf.0[..data.len()]);
        Ok(())
    }

    async fn write(
        &mut self,
        page_id: PageID,
        offset: usize,
        data: &[u8],
    ) -> Result<(), <DbFlash<'a> as flash::Flash>::Error> {
        let start = unsafe { &__config_start as *const u32 as usize };
        let address = start + page_id.index() * config::PAGE_SIZE + offset;
        let mut buf = AlignedBuf([0; ERASE_SIZE]);
        buf.0[..data.len()].copy_from_slice(data);
        self.0.write(address as u32, &buf.0[..data.len()])?;
        Ok(())
    }
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    let flash = DbFlash(Flash::<_, FLASH_SIZE>::new(p.FLASH));
    let db = Database::<_, NoopRawMutex>::new(flash, ekv::Config::default());

    if db.mount().await.is_err() {
        info!("Formatting...");
        db.format().await.unwrap();
    }

    let mut wtx = db.write_transaction().await;
    wtx.write(b"HELLO", b"WORLD").await.unwrap();
    wtx.commit().await.unwrap();

    let mut rtx = db.read_transaction().await;
    let mut buf = [0u8; 32];
    let hello = rtx.read(b"HELLO", &mut buf).await.map(|n| &buf[..n]).ok();
    if let Some(s) = hello {
        info!("HELLO: {:a}", s);
    }

    cortex_m::asm::bkpt();
}
