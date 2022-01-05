#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TrieNode<K, V> {
    children: Vec<TrieNode<K, V>>,
    key: Option<K>,
    value: Option<V>,
}

impl<K, V> TrieNode<K, V>
where
    K: Ord + Eq + PartialEq,
{
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            key: None,
            value: None,
        }
    }

    pub fn from(key: K) -> Self {
        Self {
            children: Vec::new(),
            key: Some(key),
            value: None,
        }
    }

    /*
    pub fn compare_key(&self, k: &K) -> bool {
        match self.key {
            Some(ref key) if key == k => true,
            _ => false,
        }
    }
    */

    pub fn value(&self) -> Option<&V> {
        self.value.as_ref()
    }

    pub fn get(&self, k: &K) -> Option<&TrieNode<K, V>> {
        for node in &self.children {
            if let Some(ref key) = node.key {
                if key == k {
                    return Some(&node);
                }
            }
        }
        None
    }

    pub fn insert<T>(&mut self, k: T, value: V)
    where
        T: AsRef<[K]>,
        K: Clone,
    {
        let mut root = &mut *self;
        let mut i = 0;
        let key = k.as_ref();

        loop {
            if key.len() == i {
                root.value = Some(value);
                break;
            }

            match root.children.binary_search_by_key(&Some(&key[i]), |e| e.key.as_ref()) {
                Ok(idx) => {
                    root = &mut root.children[idx];
                },
                Err(idx) => {
                    let node = TrieNode::from(key[i].clone());
                    root.children.insert(idx, node);
                    root = &mut root.children[idx];
                }
            }
            i += 1;
        }

        /*
        if key.is_empty() {
            self.value = Some(value);
            return;
        }

        for node in self.children.iter_mut() {
            if node.compare_key(&key[0]) {
                node.insert(&key[1..], value);
                return;
            }
        }

        let mut node = Self::from(key[0].clone());
        node.insert(&key[1..], value);
        self.children.push(node);
        */
    }
}
