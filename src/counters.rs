//! 全局计数器用于跟踪函数调用次数

use std::sync::atomic::{AtomicU32, Ordering};

// 全局计数器
static RANK_COUNT: AtomicU32 = AtomicU32::new(0);
static ORDER_COUNT: AtomicU32 = AtomicU32::new(0);
static POSITION_COUNT: AtomicU32 = AtomicU32::new(0);
static CROSS_COUNT: AtomicU32 = AtomicU32::new(0);
static INIT_ORDER_COUNT: AtomicU32 = AtomicU32::new(0);
static BK_COUNT: AtomicU32 = AtomicU32::new(0);

pub fn reset_counters() {
    RANK_COUNT.store(0, Ordering::SeqCst);
    ORDER_COUNT.store(0, Ordering::SeqCst);
    POSITION_COUNT.store(0, Ordering::SeqCst);
    CROSS_COUNT.store(0, Ordering::SeqCst);
    INIT_ORDER_COUNT.store(0, Ordering::SeqCst);
    BK_COUNT.store(0, Ordering::SeqCst);
}

pub fn increment_rank() {
    RANK_COUNT.fetch_add(1, Ordering::SeqCst);
}

pub fn increment_order() {
    ORDER_COUNT.fetch_add(1, Ordering::SeqCst);
}

pub fn increment_position() {
    POSITION_COUNT.fetch_add(1, Ordering::SeqCst);
}

pub fn increment_cross_count() {
    CROSS_COUNT.fetch_add(1, Ordering::SeqCst);
}

pub fn increment_init_order() {
    INIT_ORDER_COUNT.fetch_add(1, Ordering::SeqCst);
}

pub fn increment_bk() {
    BK_COUNT.fetch_add(1, Ordering::SeqCst);
}

pub fn get_rank_count() -> u32 {
    RANK_COUNT.load(Ordering::SeqCst)
}

pub fn get_order_count() -> u32 {
    ORDER_COUNT.load(Ordering::SeqCst)
}

pub fn get_position_count() -> u32 {
    POSITION_COUNT.load(Ordering::SeqCst)
}

pub fn get_cross_count() -> u32 {
    CROSS_COUNT.load(Ordering::SeqCst)
}

pub fn get_init_order_count() -> u32 {
    INIT_ORDER_COUNT.load(Ordering::SeqCst)
}

pub fn get_bk_count() -> u32 {
    BK_COUNT.load(Ordering::SeqCst)
}

pub fn get_all_counts() -> (u32, u32, u32, u32, u32, u32) {
    (
        get_rank_count(),
        get_order_count(),
        get_position_count(),
        get_cross_count(),
        get_init_order_count(),
        get_bk_count(),
    )
}
