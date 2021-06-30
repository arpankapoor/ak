use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt;
use std::lazy::SyncLazy;
use std::mem;
use std::sync::RwLock;

static INTERNER: SyncLazy<RwLock<Interner>> = SyncLazy::new(|| RwLock::new(Interner::new()));

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
pub struct Sym(u32);

impl Sym {
    pub fn new(string: &[u8]) -> Self {
        INTERNER.write().expect("poisoned rwlock").intern(string)
    }
}

impl Ord for Sym {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.eq(other) {
            return Ordering::Equal;
        }
        let lock = INTERNER.read().expect("poisoned rwlock");
        lock.lookup(*self).cmp(lock.lookup(*other))
    }
}

impl PartialOrd for Sym {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Debug for Sym {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "`{:?}({})",
            String::from_utf8_lossy(INTERNER.read().expect("poisoned rwlock").lookup(*self)),
            self.0
        )
    }
}

impl fmt::Display for Sym {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "`{}",
            String::from_utf8_lossy(INTERNER.read().expect("poisoned rwlock").lookup(*self))
        )
    }
}

struct Interner {
    map: HashMap<&'static [u8], Sym>,
    vec: Vec<&'static [u8]>,
    head: Vec<u8>,
    rest: Vec<Vec<u8>>,
}

impl Interner {
    const INIT_SIZE: usize = 4096;
    const MAX_SIZE: usize = 64 << 20;

    fn new() -> Self {
        Interner {
            map: HashMap::default(),
            vec: Vec::new(),
            head: Vec::with_capacity(Self::INIT_SIZE),
            rest: Vec::new(),
        }
    }

    fn intern(&mut self, name: &[u8]) -> Sym {
        if let Some(&sym) = self.map.get(name) {
            return sym;
        }
        let name: &'static [u8] = unsafe { &*(self.alloc(name) as *const [u8]) };
        let sym = Sym(self.vec.len() as u32);
        self.map.insert(name, sym);
        self.vec.push(name);
        sym
    }

    fn lookup(&self, id: Sym) -> &'static [u8] {
        self.vec[id.0 as usize]
    }

    fn alloc(&mut self, name: &[u8]) -> &[u8] {
        let cap = self.head.capacity();
        if name.len() > cap - self.head.len() {
            let new_cap = (cap.min(Self::MAX_SIZE / 2).max(name.len()) + 1)
                .checked_next_power_of_two()
                .expect("capacity overflow");
            let old_head = mem::replace(&mut self.head, Vec::with_capacity(new_cap));
            self.rest.push(old_head);
        }

        let start = self.head.len();
        self.head.extend_from_slice(name);
        &self.head[start..]
    }
}
