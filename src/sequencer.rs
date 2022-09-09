use workflow_allocator::prelude::*;
use std::sync::{atomic::{AtomicU64, Ordering}, Arc};
use workflow_allocator::identity::Identity;
use workflow_allocator::result::Result;

#[derive(Debug, Clone)]
pub struct Sequencer {
    sequence : Arc<AtomicU64>
}

impl Sequencer {
    pub fn new() -> Sequencer {
        Sequencer {
            sequence : Arc::new(AtomicU64::new(0))
        }
    }

    pub fn create_from_identity(reference: &Arc<AccountDataReference>) -> Result<Sequencer> {
        let identity = reference.try_load_container::<Identity>()?;
        let seq = identity.meta.borrow().get_pda_sequence();
        Ok(Sequencer {
            sequence : Arc::new(AtomicU64::new(seq))
        })
    }

    pub fn load_from_identity(&self, reference: &Arc<AccountDataReference>) -> Result<()> {
        let identity = reference.try_load_container::<Identity>()?;
        let seq = identity.meta.borrow().get_pda_sequence();
        self.sequence.store(seq, Ordering::SeqCst);
        Ok(())
    }

    pub fn next(&self) -> u64 {
        self.sequence.fetch_add(1, Ordering::SeqCst)
    }

    pub fn advance(&self, n : usize) {
        self.sequence.fetch_add(n as u64, Ordering::SeqCst);
    }

    pub fn get(&self) -> u64 {
        self.sequence.load(Ordering::SeqCst)
    }

}

impl Default for Sequencer {
    fn default() -> Sequencer {
        Sequencer::new()
    }
}