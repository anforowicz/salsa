use crate::runtime::Revision;
use crate::Database;
use std::fmt::Debug;
use std::hash::Hasher;
use std::sync::Arc;

/// Unsafe proof obligations:
///
/// - If `DB::DatabaseData: Send + Sync`, then `Self: Send + Sync`
/// - If `DB: 'static` and `DB::DatabaseData: 'static`, then `Self: 'static`
pub(crate) unsafe trait DatabaseSlot<DB: Database>: Debug {
    /// Returns true if the value of this query may have changed since
    /// the given revision.
    fn maybe_changed_since(&self, db: &DB, revision: Revision) -> bool;
}

pub(crate) struct Dependency<DB: Database> {
    slot: Arc<dyn DatabaseSlot<DB> + Send + Sync>,
    phantom: std::marker::PhantomData<Arc<DB::DatabaseData>>,
}

impl<DB: Database> Dependency<DB> {
    pub(crate) fn new(slot: Arc<dyn DatabaseSlot<DB> + '_>) -> Self {
        // Unsafety note: It is safe to 'pretend' the trait object is
        // Send+Sync+'static because the phantom-data will reflect the
        // reality.
        let slot: Arc<dyn DatabaseSlot<DB> + Send + Sync> = unsafe { std::mem::transmute(slot) };
        Self {
            slot,
            phantom: std::marker::PhantomData,
        }
    }

    fn raw_slot(&self) -> *const dyn DatabaseSlot<DB> {
        &*self.slot
    }

    pub(crate) fn maybe_changed_since(&self, db: &DB, revision: Revision) -> bool {
        self.slot.maybe_changed_since(db, revision)
    }
}

impl<DB: Database> std::hash::Hash for Dependency<DB> {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.raw_slot().hash(state)
    }
}

impl<DB: Database> std::cmp::PartialEq for Dependency<DB> {
    fn eq(&self, other: &Self) -> bool {
        self.raw_slot() == other.raw_slot()
    }
}

impl<DB: Database> std::cmp::Eq for Dependency<DB> {}

impl<DB: Database> std::fmt::Debug for Dependency<DB> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.slot.fmt(fmt)
    }
}