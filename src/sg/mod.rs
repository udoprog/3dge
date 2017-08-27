//! # Scene Graph implementation suitable for game development

#![allow(dead_code)]

use std::collections::{HashMap, LinkedList};
use std::fmt;
use std::ptr::Shared;
use std::slice;

pub type SceneId = u64;

struct SceneGraphData<T, P> {
    /// by-id references to each node
    references: HashMap<SceneId, Shared<Node<T, P>>>,
    /// id
    id_allocator: SceneId,
}

impl<T, P> SceneGraphData<T, P> {
    fn new_child(
        &mut self,
        parent: Option<Shared<Node<T, P>>>,
        data: T,
        props: P,
    ) -> (Shared<Node<T, P>>, SceneId) {
        let graph = unsafe { Shared::new(self) };

        let node = Box::into_raw(Box::new(Node {
            graph: graph,
            parent: parent,
            data: data,
            props: props,
            children: vec![],
        }));

        let node = unsafe { Shared::new(node) };

        let id = self.id_allocator;
        self.id_allocator += 1;
        self.references.insert(id, node.clone());
        (node, id)
    }
}

pub struct SceneGraph<T, P> {
    data: Shared<SceneGraphData<T, P>>,
    root: Shared<Node<T, P>>,
}

impl<T, P> Drop for SceneGraph<T, P> {
    /// Traverse the entire (uni-directional) graph and take ownership of all data.
    fn drop(&mut self) {
        let mut queue: LinkedList<Shared<Node<T, P>>> = LinkedList::new();
        queue.push_back(self.root);

        while let Some(node) = queue.pop_front() {
            // convert to box, and it will be dropped at the end of this function.
            let node = unsafe { Box::from_raw(node.as_ptr()) };
            queue.extend(node.children.iter().map(Clone::clone));

            // drop not required, only used for clarity
            drop(node);
        }

        // drop not required, only used for clarity
        // de-alloc shared data.
        drop(unsafe { Box::from_raw(self.data.as_ptr()) });
    }
}

impl<T: fmt::Debug, P: fmt::Debug> fmt::Debug for SceneGraph<T, P> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "SceneGraph {{ }}")
    }
}


impl<T, P: Default> SceneGraph<T, P> {
    pub fn new(root: T) -> SceneGraph<T, P> {
        SceneGraph::new_with_props(root, P::default())
    }
}

impl<T, P> SceneGraph<T, P> {
    /// Create a new scene graph.
    pub fn new_with_props(root: T, props: P) -> SceneGraph<T, P> {
        let data = Box::into_raw(Box::new(SceneGraphData {
            references: HashMap::new(),
            id_allocator: 0,
        }));

        let mut data = unsafe { Shared::new(data) };
        let root = unsafe { data.as_mut().new_child(None, root, props).0 };

        SceneGraph {
            data: data,
            root: root,
        }
    }

    /// # Get the root node of the scene.
    pub fn mut_root(&mut self) -> &mut Node<T, P> {
        unsafe { self.root.as_mut() }
    }

    /// # Get an immutable root from the scene.
    pub fn root(&self) -> &Node<T, P> {
        unsafe { self.root.as_ref() }
    }
}

pub struct Node<T, P> {
    /// reference to the graph this node belongs to
    graph: Shared<SceneGraphData<T, P>>,
    /// referenc to the parent node of this node
    parent: Option<Shared<Node<T, P>>>,
    /// stored data in node
    data: T,
    /// stored properties in node
    props: P,
    /// immediate childen to this node
    children: Vec<Shared<Node<T, P>>>,
}

impl<T: fmt::Debug, P: fmt::Debug> fmt::Debug for Node<T, P> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(
            fmt,
            "Node {{ data: {:?}, props: {:?} }}",
            self.data,
            self.props
        )
    }
}

impl<T, P: Default> Node<T, P> {
    pub fn push(&mut self, data: T) -> (SceneId, &mut Node<T, P>) {
        self.push_with_props(data, P::default())
    }
}

impl<T, P> Node<T, P> {
    pub fn parent(&self) -> Option<&Node<T, P>> {
        self.parent.as_ref().map(
            |parent| unsafe { parent.as_ref() },
        )
    }

    pub fn parent_mut(&mut self) -> Option<&mut Node<T, P>> {
        self.parent.as_mut().map(
            |parent| unsafe { parent.as_mut() },
        )
    }

    pub fn push_with_props(&mut self, data: T, props: P) -> (SceneId, &mut Node<T, P>) {
        let parent = unsafe { Shared::new(self) };
        let graph = unsafe { self.graph.as_mut() };
        let (node, id) = graph.new_child(Some(parent), data, props);
        self.children.push(node.clone());
        (id, unsafe { &mut *node.as_ptr() })
    }

    pub fn children(&self) -> ChildIter<T, P> {
        ChildIter { iter: self.children.iter() }
    }

    pub fn children_mut(&mut self) -> ChildIterMut<T, P> {
        ChildIterMut { iter: self.children.iter_mut() }
    }

    fn traverse<F>(&self, mut traverser: F)
    where
        F: FnMut(&Node<T, P>) -> (),
    {
        let mut queue: LinkedList<Shared<Node<T, P>>> = LinkedList::new();
        queue.extend(self.children.iter().map(Clone::clone));

        while let Some(next) = queue.pop_front() {
            traverser(unsafe { next.as_ref() });
        }
    }

    fn traverse_mut<F>(&mut self, mut traverser: F)
    where
        F: FnMut(&mut Node<T, P>) -> (),
    {
        let mut queue: LinkedList<Shared<Node<T, P>>> = LinkedList::new();
        queue.extend(self.children.iter().map(Clone::clone));

        while let Some(mut next) = queue.pop_front() {
            traverser(unsafe { next.as_mut() });
        }
    }
}

pub struct ChildIter<'a, T: 'a, P: 'a> {
    iter: slice::Iter<'a, Shared<Node<T, P>>>,
}

impl<'a, T, P> Iterator for ChildIter<'a, T, P> {
    type Item = &'a Node<T, P>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|next| unsafe { next.as_ref() })
    }
}

pub struct ChildIterMut<'a, T: 'a, P: 'a> {
    iter: slice::IterMut<'a, Shared<Node<T, P>>>,
}

impl<'a, T, P> Iterator for ChildIterMut<'a, T, P> {
    type Item = &'a mut Node<T, P>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|next| unsafe { next.as_mut() })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_root() {
        let graph: SceneGraph<_, u32> = SceneGraph::new(42u32);
        assert_eq!(42u32, graph.root().data);
        assert_eq!(0, graph.root().props);
    }

    #[test]
    fn test_children_iter() {
        let mut graph: SceneGraph<_, u32> = SceneGraph::new(1u32);

        graph.mut_root().push_with_props(2u32, 1234);
        graph.mut_root().push(3u32);

        let values: Vec<_> = graph.root().children().map(|n| n.props).collect();
        assert_eq!(vec![1234, 0], values);
    }

    #[test]
    fn test_children_iter_mut() {
        let mut graph: SceneGraph<_, u32> = SceneGraph::new(1u32);

        graph.mut_root().push_with_props(2u32, 1234);
        graph.mut_root().push(3u32);

        for child in graph.mut_root().children_mut() {
            child.props += 1
        }

        let values: Vec<_> = graph.root().children().map(|n| n.props).collect();
        assert_eq!(vec![1235, 1], values);
    }

    #[test]
    fn test_traverse() {
        let mut graph: SceneGraph<_, u32> = SceneGraph::new(1u32);

        {
            let root = graph.mut_root();
            let (id, child1) = root.push_with_props(2u32, 1234);
            root.push(3u32);
            child1.push(4u32);
        }

        for child in graph.mut_root().children_mut() {
            child.props += 1
        }

        let values: Vec<_> = graph.root().children().map(|n| n.props).collect();
        assert_eq!(vec![1235, 1], values);
    }
}
