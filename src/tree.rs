use std::rc::Rc;
use std::cell::{RefCell, Ref};
use std::collections::BTreeMap;
use std::cmp::Reverse;
use std::fmt::Display;

use crate::*;
use crate::format::format_count;

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
    fn new(top_nodes: Vec<Rc<RefCell<TreeNode<T>>>>, node_map: BTreeMap<T, Rc<RefCell<TreeNode<T>>>>, do_calculations: bool) -> Self {
        let mut tree = Self {
            top_nodes,
            node_map,
            calc_done: false,
            height: 0,
            node_count: 0,
            leaf_count: 0,
        };
        if do_calculations {
            tree.do_calculations();
        }
        tree
    }

    /*
    fn new_from_nodes(mut nodes: Vec<TreeNode<T>>, do_calculations: bool) -> Self {
        let mut top_nodes = vec![];
        let mut node_map= BTreeMap::new();
        for node in nodes.drain(..) {
            let item = node.item.clone();
            let is_top_node = node.parent.is_none();
            let node_rc = r!(node);
            if is_top_node {
                top_nodes.push(node_rc.clone());
            }
            node_map.insert(item, node_rc);
        }
        Self::new(top_nodes, node_map, do_calculations)
    }
     */

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

        Self::new(top_nodes, node_map, do_calculations)
    }

    pub fn get_node(&self, key: &T) -> Option<Rc<RefCell<TreeNode<T>>>> {
        self.node_map.get(key).map(|node_rc| node_rc.clone())
    }

    pub fn sort_recursive<F>(&mut self, f: &F)
        where F: Fn(&Rc<RefCell<TreeNode<T>>>,) -> String,
    {
        self.top_nodes.sort_by_cached_key(f);
        for top_node_rc in self.top_nodes.iter_mut() {
            m!(top_node_rc).sort_child_nodes_recursive(f);
        }
    }

    /*
    pub fn filter<F>(&self, filter_func: &F) -> Tree<T>
        where F: Fn(&Ref<TreeNode<T>>) -> bool
    {
        // let mut top_nodes = vec![];
        // let mut node_map = BTreeMap::new();
        let mut new_nodes = vec![];
        for top_node_rc in self.top_nodes.iter() {
            TreeNode::add_filtered( &mut new_nodes, None,b!(top_node_rc), filter_func);
        }
        let tree = Self::new_from_nodes(new_nodes, true);
        tree
    }
    */

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
        assert!(!self.node_map.is_empty());
        assert!(!self.top_nodes.is_empty());
        for node_rc in self.node_map.values() {
            m!(node_rc).do_calculations();
        }
        self.height = self.top_nodes.iter().map(|node_rc| b!(node_rc).height).max().unwrap();
        self.node_count = self.top_nodes.iter().map(|node_rc| b!(node_rc).subtree_node_count).sum();
        self.leaf_count = self.top_nodes.iter().map(|node_rc| b!(node_rc).subtree_leaf_count).sum();
        self.calc_done = true;
    }

    pub fn max_depth_for_max_count(&self, max_count: usize) -> usize {
        assert_calc_done(self.calc_done);
        // Start with the largest depth of the tree.
        let mut depth = self.height - 1;
        // The worst case is to return a depth of zero, meaning only the top-level nodes. Do this
        // even if there are more top-level nodes than max_count.
        while depth > 0 && self.count_to_depth(depth) > max_count {
            depth -= 1;
        }
        depth
    }

    fn count_to_depth(&self, max_depth: usize) -> usize {
        let mut count = 0;
        for top_node_rc in self.top_nodes.iter() {
            count += b!(top_node_rc).count_to_depth(max_depth);
        }
        count
    }

    pub fn unroll_to_depth(&self, max_depth: Option<usize>) -> Vec<Rc<RefCell<TreeNode<T>>>> {
        assert_calc_done(self.calc_done);
        let mut list = vec![];
        for top_node_rc in self.top_nodes.iter() {
            list.push(top_node_rc.clone());
            b!(top_node_rc).add_child_nodes_to_unroll_to_depth(&mut list, max_depth);
        }
        list
    }

    pub fn description_line(&self) -> String {
        format!("util::tree::Tree: top_nodes size = {}, node_map size = {}, height = {}, node_count = {}, leaf_count = {}",
                format_count(self.top_nodes.len()), format_count(self.node_map.len()),
                format_count(self.height), format_count(self.node_count), format_count(self.leaf_count))
    }

    pub fn print_counts_to_depth(&self) {
        assert_calc_done(self.calc_done);
        println!("\n{}", self.description_line());
        for depth in 0..self.height {
            println!("Depth {}: node count = {}", depth, format_count(self.count_to_depth(depth)));
        }
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

    pub fn print_with_items(&self, max_depth: Option<usize>) {
        println!("\n{}", self.description_line());
        for top_node_rc in self.top_nodes.iter() {
            b!(top_node_rc).print_with_items(max_depth);
        }
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

    pub fn get_subtree_filtered<F>(&self, filter_func: &F) -> Tree<T>
        where F: Fn(Ref<Self>) -> bool
    {
        let mut pairs = vec![];
        // The tree is made from a collection of pairs of parent-child relationships. It's not
        // really designed for trees consisting of isolated nodes. So if there are no child nodes,
        // leave the tree empty.
        if !self.child_nodes.is_empty() {
            // let self_ref = Ref::try_from(self).unwrap();
            // This is a convoluted way of getting a Ref<> of the current node.
            let first_child_node = b!(&self.child_nodes[0]);
            let self_ref = b!(&first_child_node.parent.as_ref().unwrap());
            // If the current node doesn't pass the filter, the subtree will be empty.
            if filter_func(self_ref) {
                self.add_child_nodes_to_subtree_filtered(&mut pairs, filter_func);
            }
        }
        let subtree = Tree::create(pairs, true);
        subtree
    }

    fn add_child_nodes_to_subtree_filtered<F>(&self, pairs: &mut Vec<(T, T)>, filter_func: &F)
        where F: Fn(Ref<Self>) -> bool
    {
        // We've already established that the current node passes the filter.
        for child_node_rc in self.child_nodes.iter() {
            if filter_func(b!(child_node_rc)) {
                // The child node passes the filter, so add the parent/child pair.
                let child_item = b!(child_node_rc).item.clone();
                pairs.push((self.item.clone(), child_item));
                b!(child_node_rc).add_child_nodes_to_subtree_filtered(pairs, filter_func);
            }
        }
    }

    /*
    fn add_filtered<F>(new_nodes: &mut Vec<Self>, new_parent_rc: Option<Rc<RefCell<TreeNode<T>>>>, source_node_ref: Ref<Self>, filter_func: &F)
        where F: Fn(&Ref<Self>) -> bool
    {
        if filter_func(&source_node_ref) {
            let new_node = Self::new(new_parent_rc,source_node_ref.item.clone());
            let new_node_rc = r!(new_node);
            for child_node_rc in source_node_ref.child_nodes.iter() {
                Self::add_filtered(new_nodes,Some(new_node_rc.clone()), b!(child_node_rc), filter_func);
            }
            new_nodes.push(new_node);
        }
    }
    */

    pub fn max_depth_for_max_count(&self, max_count: usize) -> usize {
        assert_calc_done(self.calc_done);
        // Start with the largest depth of the subtree.
        let mut depth = (self.depth + self.height) - 1;
        // The worst case is to return the current depth, meaning only the current node.
        while depth > self.depth && self.count_to_depth(depth) > max_count {
            depth -= 1;
        }
        depth
    }

    fn count_to_depth(&self, max_depth: usize) -> usize {
        // We shouldn't have gotten here if we're already past the max depth.
        debug_assert!(self.depth <= max_depth);
        let mut count = 1;
        if self.depth < max_depth {
            for child_node_rc in self.child_nodes.iter() {
                count += b!(child_node_rc).count_to_depth(max_depth);
            }
        }
        count
    }

    pub fn max_depth_for_max_count_filtered<F>(&self, max_count: usize, filter_func: &F) -> usize
        where F: Fn(Ref<Self>) -> bool
    {
        assert_calc_done(self.calc_done);
        // Start with the largest depth of the subtree.
        let mut depth = (self.depth + self.height) - 1;
        // The worst case is to return the current depth, meaning only the current node.
        while depth > self.depth && self.count_to_depth_filtered(depth, filter_func) > max_count {
            depth -= 1;
        }
        depth
    }

    fn count_to_depth_filtered<F>(&self, max_depth: usize, filter_func: &F) -> usize
        where F: Fn(Ref<Self>) -> bool
    {
        // We shouldn't have gotten here if we're already past the max depth.
        debug_assert!(self.depth <= max_depth);
        let mut count = 1;
        if self.depth < max_depth {
            for child_node_rc in self.child_nodes.iter() {
                if filter_func(b!(child_node_rc)) {
                    count += b!(child_node_rc).count_to_depth_filtered(max_depth, filter_func);
                }
            }
        }
        count
    }

    pub fn unroll_to_depth(&self, max_depth: Option<usize>, self_rc: Option<Rc<RefCell<Self>>>) -> Vec<Rc<RefCell<Self>>> {
        assert_calc_done(self.calc_done);
        let mut list = vec![];
        if max_depth.map_or(true, |max_depth| max_depth >= self.depth) {
            if let Some(self_rc) = self_rc {
                list.push(self_rc);
            }
            if max_depth.map_or(true, |max_depth| max_depth > self.depth) {
                self.add_child_nodes_to_unroll_to_depth(&mut list, max_depth);
            }
        }
        list
    }

    fn add_child_nodes_to_unroll_to_depth(&self, list: &mut Vec<Rc<RefCell<Self>>>, max_depth: Option<usize>) {
        // We're asssuming that the current node is within the max depth.
        for child_rc in self.child_nodes.iter() {
            list.push(child_rc.clone());
            if max_depth.map_or(true, |max_depth| max_depth > self.depth + 1) {
                b!(child_rc).add_child_nodes_to_unroll_to_depth(list, max_depth);
            }
        }
    }

    pub fn sort_child_nodes_recursive<F>(&mut self, f: &F)
        where F: Fn(&Rc<RefCell<TreeNode<T>>>,) -> String,
    {
        self.child_nodes.sort_by_cached_key(f);
        for child_node_rc in self.child_nodes.iter_mut() {
            m!(child_node_rc).sort_child_nodes_recursive(f);
        }
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

    pub fn description_line(&self) -> String {
        format!("depth = {}, height = {}, child nodes = {}, subtree nodes = {}, subtree leaves = {}",
                self.depth, self.height, self.child_count(), self.subtree_node_count(), self.subtree_leaf_count)
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
        let line = self.description_line_with_item();
        crate::format::println_indent_tab(depth, &line);
        let list = self.child_nodes.clone();
        Tree::report_by_node_count_list(list);
    }

    pub fn description_line_with_item(&self) -> String {
        format!("{}: {}", self.item, self.description_line())
    }

    fn print_with_items(&self, max_depth: Option<usize>) {
        let depth = self.depth;
        let line = self.description_line_with_item();
        crate::format::println_indent_tab(depth, &line);
        if max_depth.map_or(true, |max_depth| self.depth < max_depth) {
            for child_node_rc in self.child_nodes.iter() {
                b!(child_node_rc).print_with_items(max_depth);
            }
        }
    }

}

fn assert_calc_done(calc_done: bool) {
    assert!(calc_done, "The tree was not created with the parameter do_calculations = true, so it's not valid to ask for things like depth and subtree counts.");
}
