use std::boxed::Box;
#[derive(Debug)]
struct BinarySearchNode<T> 
    where T:std::cmp::PartialOrd
{
    data: T,
    left : Option<Box<Self>>,
    right : Option<Box<Self>>,
}

impl<T> BinarySearchNode<T>
    where T:std::cmp::PartialOrd + std::default::Default
{
    fn from(data: T) -> Self {
        Self {
            data,
            left: None,
            right: None,
        }
    }
    fn insert_node(&mut self, another_node: Box<Self>) {
        let target_node = if another_node.data < self.data {
            &mut self.left
        } else {
            &mut self.right
        };

        if let Some(target_node) = target_node {
            target_node.insert_node(another_node);
        } else {
            *target_node = Some(another_node);
        }
    }
    fn insert(&mut self, data: T) {
        self.insert_node(Box::new(Self::from(data)));
    }
}

/// DFS Iterator is the depth first search iterator for binary search node.
/// Tbh, dfs iterator for binary search tree is inefficent and unnecessary.
/// Though, we can implement for learning purpose
///               A
///           /      \
///         B         C
///       /   \      /
///      D    E     F
///       \
///        G
/// Here,
/// implement pre order traversal for DFS:
/// we go to A and check it.
/// then we check if B is empty: if yes skip
///         if B is not empty, go deeper into B
/// then we we check if C is empty: if yes skip
///         if C is not empty, go deeper into C
/// So, the stopping condition would be:
///     The node has exhausted right and left search.
///     In that case, we have to propagate back to it's root and and select next

// Actually, DFS can be implemented pretty easily using the concept of Stack, lol.
#[derive(Debug)]
struct DfsIterator<'a, T> 
    where T: std::cmp::PartialOrd
{
    items : Vec<&'a T>,
    index : usize,
}
impl<'a, T> DfsIterator<'a, T> 
    where T: std::cmp::PartialOrd
{
    fn new(node: &'a BinarySearchNode<T>) -> Self {
        Self {
            items: {
                let mut items = vec![];
                items.push(&node.data);
                if let Some(thing) = &node.left {
                    items.extend(Self::new(thing).items);
                }
                if let Some(thing) = &node.right {
                    items.extend(Self::new(thing).items);
                }
                items
            },
            index: 0,
        }
    }
}

impl<'a, T> Iterator for DfsIterator<'a, T>
    where T: std::cmp::PartialOrd
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.items.len() {
            let return_item = &self.items[self.index];
            self.index += 1;
            return Some(return_item);
        } else {
            return None;
        }
    }
}


// #[derive(Clone)]
// struct DfsIterator<'a, T>
    // where T: std::cmp::PartialOrd
// {
//     current_branch: BinBranch,
//     traversal_mode: TraversalMode,
//     inverted: bool,
//     this_node: &'a BinarySearchNode<T>,
//     root_stack: Option<&'a Self>,
// }
// enum BinBranch { MySelf, Left, Right, Exhausted }
// enum TraversalMode { InOrder, PreOrder, PostOrder }
// impl<'a, T> DfsIterator<'a, T>
//     where T: std::cmp::PartialOrd
// {
//     fn new(node: &'a BinarySearchNode<T>) -> Self {
//         Self {
//             current_branch: BinBranch::MySelf,
//             traversal_mode: TraversalMode::PreOrder,
//             inverted: false,
//             this_node: node,
//             root_stack: None,
//         }
//     }
// }

// impl Iterator for DfsIterator {
//     // type that I iterate with
//     type item = &T;
//     /// So, where do I go next?
//     /// If I am on this branch, I have to return this data
//     /// 
//     /// If I am on the left branch, I have to go inside that branch (same for right):
//     ///     To go inside:
//     ///         First, create a new iterator that starts from left iterator.
//     ///             Then return next of that iterator
//     /// If I am on the right branch, I do the same
//     /// 
//     /// If I am already exhausted, I return None.
//     /// 
//     /// Then I modify the iterator, to prepare for the next invocation of `next()`
//     fn next(&mut self) -> Option<Self::item> {
//         let return_item = match self.current_branch {
//             BinBranch::MySelf => &this_node.data;
//             BinBranch::Right => {
//                 let mut newiterator = 
//             }
//         }
//         &self.this_node.data;


//         Some(return_item)
//     }
// }

fn main() {
    let mut data_items = [45, 15, 79, 90, 10, 55, 12, 20, 50];
    let first_item = data_items[0];
    let mut my_tree = BinarySearchNode::from(first_item);
    for item in &mut data_items[1..] {
        my_tree.insert(*item);
    }
    println!("{my_tree:#?}");
    let dfs = DfsIterator::new(&my_tree);
    println!("{dfs:?}");
    println!("Iterating over the DFS list: ");
    for item in dfs {
        println!("Item = {item}");
    }
}