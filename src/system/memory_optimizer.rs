use crate::system::memory::{MemoryManager, MemoryStats};
use crate::system::swap_manager::{SwapInfo, SwapManager};

#[derive(Debug, Clone)]
pub struct MemoryOptimizer {
    pub memory_stats: MemoryStats,
    pub swap_info: SwapInfo,
    pub memory_optimization: bool,
    pub zram_optimization: bool,
}

impl MemoryOptimizer {
    pub fn current(memory_optimization: bool, zram_optimization: bool) -> Self {
        Self {
            memory_stats: MemoryManager::fetch_stats(),
            swap_info: SwapManager::inspect_swap(),
            memory_optimization,
            zram_optimization,
        }
    }

    pub fn refresh(&mut self) {
        self.memory_stats = MemoryManager::fetch_stats();
        self.swap_info = SwapManager::inspect_swap();
    }

    pub fn optimize(&mut self) -> Result<Vec<String>, String> {
        let mut steps = Vec::new();

        if self.memory_optimization {
            steps.extend(MemoryManager::optimize()?);
        } else {
            tracing::info!("Memory optimization disabled; skipping cache flush.");
        }

        if self.zram_optimization && self.swap_info.has_zram {
            steps.extend(SwapManager::optimize_zram()?);
            // Apply the user-configured ZRAM memory cap (0 = unlimited).
            let limit = crate::core::config::CavarluxConfig::cached().zram_mem_limit_mb;
            if limit > 0 {
                if let Ok(msg) = SwapManager::set_zram_mem_limit(limit) {
                    steps.push(msg);
                }
            }
        }

        self.refresh();
        Ok(steps)
    }

    pub fn ram_ratio(&self) -> f32 {
        self.memory_stats.ram_ratio()
    }
    pub fn swap_ratio(&self) -> f32 {
        self.memory_stats.swap_ratio()
    }
    pub fn ram_text(&self) -> String {
        self.memory_stats.ram_text()
    }
    pub fn swap_text(&self) -> String {
        self.memory_stats.swap_text()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_optimizer_current() {
        let opt = MemoryOptimizer::current(false, false);
        assert!(opt.memory_stats.total_mb > 0);
        assert!((0.0..=1.0).contains(&opt.ram_ratio()));
    }

    #[test]
    fn test_optimizer_refresh() {
        let mut opt = MemoryOptimizer::current(true, false);
        let before = opt.ram_text();
        opt.refresh();
        assert_eq!(before, opt.ram_text());
    }
}
