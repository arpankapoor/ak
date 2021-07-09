use std::collections::HashMap;
use std::lazy::SyncLazy;
use std::sync::RwLock;

use crate::k::K;
use crate::sym::Sym;

static GLOBAL_ENV: SyncLazy<RwLock<Environment>> =
    SyncLazy::new(|| RwLock::new(Environment::new()));

pub fn define_variable(name: Sym, value: K) {
    GLOBAL_ENV
        .write()
        .expect("poisoned rwlock")
        .define(name, value);
}

pub fn get_variable(name: Sym) -> Option<K> {
    GLOBAL_ENV
        .read()
        .expect("poisoned rwlock")
        .get(name)
        .cloned()
}

#[derive(Default)]
struct Environment {
    map: HashMap<Sym, K>,
}

impl Environment {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    fn define(&mut self, name: Sym, value: K) {
        self.map.insert(name, value);
    }

    fn get(&self, name: Sym) -> Option<&K> {
        self.map.get(&name)
    }
}
