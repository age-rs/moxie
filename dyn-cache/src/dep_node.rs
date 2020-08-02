use super::{Gc, Liveness};
use illicit::AsContext;
use parking_lot::Mutex;
use std::sync::{Arc, Weak};

#[derive(Clone, Debug, Default)]
pub(crate) struct DepNode {
    inner: Arc<Mutex<InnerDepNode>>,
}

impl DepNode {
    pub fn new() -> Self {
        Self { inner: Arc::new(Mutex::new(Default::default())) }
    }

    pub fn root(&self, dependent: Dependent) {
        self.inner.lock().root(dependent);
    }

    pub fn as_dependent(&self) -> Dependent {
        Dependent { inner: Arc::downgrade(&self.inner) }
    }
}

impl Gc for DepNode {
    fn mark(&mut self) -> bool {
        self.inner.lock().mark()
    }

    fn sweep(&mut self) -> Liveness {
        self.inner.lock().sweep()
    }
}

#[derive(Debug, Default)]
struct InnerDepNode {
    has_root: bool,
    dependents: Vec<Dependent>,
}

impl InnerDepNode {
    fn root(&mut self, dependent: Dependent) {
        self.dependents.push(dependent);
        self.dependents.dedup(); // TODO benchmark this?
        self.has_root = true;
    }

    fn mark(&mut self) -> bool {
        // TODO check dependents
        self.has_root
    }

    fn sweep(&mut self) -> Liveness {
        if std::mem::replace(&mut self.has_root, false) { Liveness::Live } else { Liveness::Dead }
    }
}

#[derive(Clone, Debug, Default)]
pub(crate) struct Dependent {
    inner: Weak<Mutex<InnerDepNode>>,
}

impl Dependent {
    /// Return the corresponding `DepNode` if it is still live.
    fn upgrade(&self) -> Option<DepNode> {
        self.inner.upgrade().map(|inner| DepNode { inner })
    }

    /// Returns the current incoming `Dependent`. If about to execute a
    /// top-level query this will return a null/no-op `Dependent`.
    pub fn incoming() -> Self {
        if let Ok(dep) = illicit::get::<Self>() { dep.clone() } else { Self::default() }
    }

    /// Initialize the dependency query with `self` marked as its immediate
    /// dependent.
    pub fn init_dependency<R>(self, op: impl FnOnce() -> R) -> R {
        self.offer(op)
    }

    /// Return the memory address of this `Dependent`.
    fn addr(&self) -> usize {
        self.inner.as_ptr() as *const _ as _
    }
}

impl PartialEq for Dependent {
    fn eq(&self, other: &Self) -> bool {
        self.addr().eq(&other.addr())
    }
}
impl Eq for Dependent {}
