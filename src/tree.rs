use std::rc::Rc;
use std::cell::{RefCell, Ref};
use std::collections::BTreeMap;
use std::cmp::Reverse;
use std::fmt::Display;

use crate::*;

pub struct Tree<T>
    where T: Clone + Ord
{
    pub top_nodes: Vec<Rc<RefCell<TreeNode<T>>>>,
    pub node_map: BTreeMap<T, Rc<RefCell<TreeNode<T>>>>,
    pub calc_done: bool,
    height: usize,
    node_count: usize,
    leaf_count: usize,
}

#[derive(Clone)]
pub struct TreeNode<T>
    where T: Clone + Ord
{
    pub parent: Option<Rc<RefCell<TreeNode<T>>>>,
    pub item: T,
    pub child_nodes: Vec<Rc<RefCell<TreeNode<T>>>>,
    pub calc_done: bool,
    depth: usize,
    height: usize,
    subtree_node_count: usize,
    subtree_leaf_count: usize,
}

impl <T> Tree<T>
    where T: Clone + Ord
{
    fn new(top_nodes: Vec<Rc<RefCell<TreeNode<T>>>>, node_map: BTreeMap<T, Rc<RefCell<TreeNode<T>>>>) -> Self {
        Self {
            top_nodes,
            node_map,
            calc_done: false,
            height: 0,
            node_count: 0,
            leaf_count: 0,
        }
    }

    pub fn create(pairs: Vec<(T, T)>, do_calculations: bool) -> Self {
        let mut node_map: BTreeMap<T, Rc<RefCell<TreeNode<T>>>> = BTreeMap::new();
        for (parent, child) in pairs.iter() {
            let parent_rc = if node_map.contains_key(parent) {
                node_map.get(parent).unwrap().clone()
            } else {
                let parent_rc: Rc<RefCell<TreeNode<T>>> = r!(TreeNode::new(None, parent.clone()));
                node_map.insert(parent.clone(), parent_rc.clone());
                parent_rc
            };
            let child_rc = if node_map.contains_key(child) {
                node_map.get(child).unwrap().clone()
            } else {
                let child_rc = r!(TreeNode::new(None, child.clone()));
                node_map.insert(child.clone(), child_rc.clone());
                child_rc
            };
            RefCell::borrow_mut(&parent_rc).child_nodes.push(child_rc.clone());
            RefCell::borrow_mut(&child_rc).parent = Some(parent_rc);
            node_map.insert(child.clone(), child_rc);
        }

        let mut top_nodes = vec![];
        node_map.values()
            .filter(|node_rc| b!(node_rc).parent.is_none())
            .for_each(|node_rc| {
                top_nodes.push(node_rc.clone());
            });

        let mut tree = Self::new(top_nodes, node_map);

        if do_calculations {
            tree.do_calculations();
        }

        tree
    }

    pub fn get_node(&self, key: &T) -> Option<Rc<RefCell<TreeNode<T>>>> {
        self.node_map.get(key).map(|node_rc| node_rc.clone())
    }

    #[inline]
    pub fn height(&self) -> usize {
        assert_calc_done(self.calc_done);
        self.height
    }

    #[inline]
    pub fn node_count(&self) -> usize {
        assert_calc_done(self.calc_done);
        self.node_count
    }

    #[inline]
    pub fn leaf_count(&self) -> usize {
        assert_calc_done(self.calc_done);
        self.leaf_count
    }

    fn do_calculations(&mut self) {
        assert!(!self.calc_done, "do_calculations() called twice.");
        for node_rc in self.node_map.values() {
            m!(node_rc).do_calculations();
        }
        self.height = self.top_nodes.iter().map(|node_rc| b!(node_rc).height).max().unwrap();
        self.node_count = self.top_nodes.iter().map(|node_rc| b!(node_rc).subtree_node_count).sum();
        self.leaf_count = self.top_nodes.iter().map(|node_rc| b!(node_rc).subtree_leaf_count).sum();
        self.calc_done = true;
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
        assert_calc_done(self.calc_done);
        let top_list = self.top_nodes.clone();
        Self::report_by_node_count_list(top_list);
    }

    fn report_by_node_count_list(list: Vec<Rc<RefCell<TreeNode<T>>>>) {
        let mut list = list.iter()
            .filter(|node_rc| b!(node_rc).child_count() > 0)
            .map(|node_rc| node_rc.clone())
            .collect::<Vec<_>>();
        list.sort_by_cached_key(|node_rc| {
            let node = b!(node_rc);
            (Reverse(node.subtree_node_count), node.item.to_string())
        });
        list.iter()
            .for_each(|node_rc| {
                RefCell::borrow(node_rc).report_by_node_count_one();
            });
    }

}

impl <T> TreeNode<T>
    where T: Clone + Ord
{
    pub fn new(parent: Option<Rc<RefCell<TreeNode<T>>>>, item: T) -> Self {
        Self {
            parent,
            item,
            child_nodes: vec![],
            calc_done: false,
            depth: 0,
            height: 0,
            subtree_node_count: 0,
            subtree_leaf_count: 0
        }
    }

    #[inline]
    pub fn is_leaf(&self) -> bool {
        self.child_count() == 0
    }

    #[inline]
    pub fn child_count(&self) -> usize {
        self.child_nodes.len()
    }

    #[inline]
    pub fn depth(&self) -> usize {
        assert_calc_done(self.calc_done);
        self.depth
    }

    #[inline]
    pub fn height(&self) -> usize {
        assert_calc_done(self.calc_done);
        self.height
    }

    #[inline]
    pub fn subtree_node_count(&self) -> usize {
        assert_calc_done(self.calc_done);
        self.subtree_node_count
    }

    #[inline]
    pub fn subtree_leaf_count(&self) -> usize {
        assert_calc_done(self.calc_done);
        self.subtree_leaf_count
    }

    pub fn get_direct_child_nodes<F>(&self, filter_func: &F) -> Vec<Rc<RefCell<Self>>>
        where F: Fn(Ref<Self>) -> bool
    {
        self.child_nodes.iter()
            .filter(|child_node_rc| filter_func(b!(child_node_rc)))
            .map(|child_node_rc| child_node_rc.clone())
            .collect::<Vec<_>>()
        /*
        let mut items = vec![];
        for child_node_rc in self.child_nodes.iter() {
            let child_node = b!(child_node_rc);
            if filter_func(&child_node) {
                items.push(child_node.clone());
            }
        }
        *
         */
    }

    pub fn get_direct_child_items<F>(&self, filter_func: &F) -> Vec<T>
        where F: Fn(Ref<Self>) -> bool
    {
        self.child_nodes.iter()
            .filter(|child_node_rc| filter_func(b!(child_node_rc)))
            .map(|child_node_rc| b!(child_node_rc).item.clone())
            .collect::<Vec<_>>()
    }

    pub fn get_indirect_child_items<F>(&self, filter_func: &F) -> Vec<T>
        where F: Fn(Ref<Self>) -> bool
    {
        let mut items = self.get_direct_child_items(filter_func);
        for child_node_rc in self.child_nodes.iter() {
            let mut child_items= b!(child_node_rc).get_indirect_child_items(filter_func);
            items.append(&mut child_items);
        }
        items
    }

    fn do_calculations(&mut self) {
        assert!(!self.calc_done, "do_calculations() called twice.");
        self.depth = self.calc_depth();
        self.height = self.calc_height();
        self.subtree_node_count = self.count_subtree_nodes();
        self.subtree_leaf_count = self.count_subtree_leaves();
        self.calc_done = true;
    }

    fn calc_depth(&self) -> usize {
        match &self.parent {
            Some(parent_rc) => b!(parent_rc).calc_depth() + 1,
            None => 0,
        }
    }

    fn calc_height(&self) -> usize {
        if self.child_nodes.is_empty() {
            1
        } else {
            self.child_nodes.iter()
                .map(|child_node_rc| b!(child_node_rc).calc_height())
                .max().unwrap() + 1
        }
    }

    fn count_subtree_nodes(&self) -> usize {
        if self.child_nodes.is_empty() {
            1
        } else {
            self.child_nodes.iter()
                .map(|child_node_rc| b!(child_node_rc).count_subtree_nodes())
                .sum::<usize>() + 1
        }
    }

    fn count_subtree_leaves(&self) -> usize {
        if self.child_nodes.is_empty() {
            1
        } else {
            self.child_nodes.iter()
                .map(|child_node_rc| b!(child_node_rc).count_subtree_leaves())
                .sum()
        }
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

    fn report_by_node_count_one(&self) {
        assert_calc_done(self.calc_done);
        let depth = self.depth;
        let line = format!("{}: depth = {}, height = {}, child nodes = {}, subtree nodes = {}, subtree leaves = {}", self.item, self.depth, self.height, self.child_count(), self.subtree_node_count(), self.subtree_leaf_count);
        crate::format::println_indent_tab(depth, &line);
        let list = self.child_nodes.clone();
        Tree::report_by_node_count_list(list);
    }

}

fn assert_calc_done(calc_done: bool) {
    assert!(calc_done, "The tree was not created with the parameter do_calculations = true, so it's not valid to ask for things like depth and subtree counts.");
}