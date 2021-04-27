use std::rc::Rc;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::cmp::Reverse;
use std::fmt::Display;

pub struct Tree<T>
    where T: Clone + Ord
{
    pub top_nodes: Vec<Rc<RefCell<TreeNode<T>>>>,
}

#[derive(Clone)]
pub struct TreeNode<T>
    where T: Clone + Ord
{
    pub parent: Option<Rc<RefCell<TreeNode<T>>>>,
    pub item: T,
    pub child_nodes: Vec<Rc<RefCell<TreeNode<T>>>>,
}

impl <T> Tree<T>
    where T: Clone + Ord
{
    fn new() -> Self {
        Self {
            top_nodes: vec![],
        }
    }

    pub fn create(pairs: Vec<(T, T)>) -> Self {
        let mut nodes: BTreeMap<T, Rc<RefCell<TreeNode<T>>>> = BTreeMap::new();
        for (parent, child) in pairs.iter() {
            let parent_rc = if nodes.contains_key(parent) {
                nodes.get(parent).unwrap().clone()
            } else {
                let parent_rc: Rc<RefCell<TreeNode<T>>> = Rc::new(RefCell::new(TreeNode {
                    parent: None,
                    item: parent.clone(),
                    child_nodes: vec![],
                }));
                nodes.insert(parent.clone(), parent_rc.clone());
                parent_rc
            };
            let child_rc = if nodes.contains_key(child) {
                nodes.get(child).unwrap().clone()
            } else {
                let child_rc = Rc::new(RefCell::new(TreeNode {
                    parent: None,
                    item: child.clone(),
                    child_nodes: vec![]
                }));
                nodes.insert(child.clone(), child_rc.clone());
                child_rc
            };
            RefCell::borrow_mut(&parent_rc).child_nodes.push(child_rc.clone());
            RefCell::borrow_mut(&child_rc).parent = Some(parent_rc);
            nodes.insert(child.clone(), child_rc);
        }

        let mut tree = Self::new();
        nodes.values()
            .filter(|node| RefCell::borrow(node).parent.is_none())
            .for_each(|node| {
                tree.top_nodes.push(node.clone());
            });
        tree
    }
}

impl <T> Tree<T>
    where T: Clone + Display + Ord
{
    /*
    pub fn report_non_leaf(&self) {
        self.top_nodes.iter()
            .for_each(|node_rc| {
                RefCell::borrow(node_rc).report_non_leaf_one(0);
            })
    }
     */

    pub fn report_by_node_count(&self) {
        let top_list = self.top_nodes.clone();
        Self::report_by_node_count_list(0, top_list);
    }

    fn report_by_node_count_list(depth: usize, list: Vec<Rc<RefCell<TreeNode<T>>>>) {
        let mut list = list.iter()
            .filter(|node_rc| RefCell::borrow(node_rc).child_count() > 0)
            .map(|node_rc| node_rc.clone())
            .collect::<Vec<_>>();
        list.sort_by_cached_key(|node_rc| {
            let node = RefCell::borrow(node_rc);
            (Reverse(node.child_count_all()), node.item.to_string())
        });
        list.iter()
            .for_each(|node_rc| {
                RefCell::borrow(node_rc).report_by_node_count_one(depth);
            });
    }

}

impl <T> TreeNode<T>
    where T: Clone + Ord
{
    pub fn is_leaf(&self) -> bool {
        self.child_count() == 0
    }

    pub fn child_count(&self) -> usize {
        self.child_nodes.len()
    }

    pub fn child_count_all(&self) -> usize {
        self.child_nodes.iter()
            .map(|child_node_rc| RefCell::borrow(child_node_rc).child_count_all() + 1)
            .sum()
    }
}

impl <T> TreeNode<T>
    where T: Clone + Display + Ord
{
    /*
    pub fn report_non_leaf_one(&self, depth: usize) {
        self.top_nodes.iter()
            .for_each(|node_rc| {
                RefCell::borrow(node_rc).report_line(0);
            })
    }

     */

    fn report_by_node_count_one(&self, depth: usize) {
        let line = format!("{}: direct nodes = {}, indirect nodes = {}", self.item, self.child_count(), self.child_count_all());
        crate::format::println_indent_tab(depth, &line);
        let list = self.child_nodes.clone();
        Tree::report_by_node_count_list(depth + 1, list);
    }

}

