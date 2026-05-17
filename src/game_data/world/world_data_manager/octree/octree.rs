#![allow(dead_code)]



/*
############
## Branch ##
############
Functions for minipulating and indexing data in a branch
value.

0 = Leaf node
1 = Node Pointer

*/
#[inline]
pub fn get_branch_value(node: u64, branch_index: u64) -> u64 {
    (node >> (48 + branch_index * 2)) & 0b11
}

#[inline]
pub fn set_branch_value(node: u64, branch_index: u64, value: u64) -> u64 {
    let shift = 48 + branch_index * 2;
    (node & !(0b11 << shift)) | ((value & 0b11) << shift)
}



/*
############
## Node ##
############
Functions for minipulating and indexing data in a node
The last 32 bits of a u64 node are the node value
*/

#[inline]
pub fn get_node_value(node: u64) -> u32 {
    node as u32
}

#[inline]
pub fn set_node_value(node: u64, value: u32) -> u64 {
    (node & 0xFFFFFFFF_00000000) | (value as u64)
}

pub struct Octree {
    dimensions: u32,
    depth: u32,

    node_data: Vec<u64>,
}

impl Octree {
    pub fn new(depth: u32) -> Octree {
        let dimensions = 2u32.pow(depth);
        let mut node_data: Vec<u64> = Vec::new();

        // Root node at index 0
        let root_node: u64 = 0;
        node_data.push(root_node);

        Octree {
            depth,
            dimensions,
            node_data,
        }
    }

    /// Create a node on the current node
    /// 
    /// Set branch value to 
    pub fn create_node_at_index(&mut self, root_index: usize, branch_index: u64) {
        self.node_data[root_index] = set_branch_value(self.node_data[root_index], branch_index, 1);

        
        self.node_data.push(0);
    }


}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_set_branch_value() {
        let mut node: u64 = 0;

        // Set each index to a different value and verify
        node = set_branch_value(node, 0, 1);
        node = set_branch_value(node, 3, 2);
        node = set_branch_value(node, 7, 3);

        assert_eq!(get_branch_value(node, 0), 1);
        assert_eq!(get_branch_value(node, 3), 2);
        assert_eq!(get_branch_value(node, 7), 3);
        // Unset indexes should be 0
        assert_eq!(get_branch_value(node, 1), 0);
    }

    #[test]
    fn test_branch_overwrite() {
        let mut node: u64 = 0;
        node = set_branch_value(node, 2, 3);
        assert_eq!(get_branch_value(node, 2), 3);

        // Overwrite with a new value
        node = set_branch_value(node, 2, 1);
        assert_eq!(get_branch_value(node, 2), 1);
    }

    #[test]
    fn test_get_set_node_value() {
        let node = set_node_value(0, 42);
        assert_eq!(get_node_value(node), 42);

        let node = set_node_value(0, u32::MAX);
        assert_eq!(get_node_value(node), u32::MAX);
    }

    #[test]
    fn test_node_value_preserves_upper_bits() {
        // Set some branch data in the upper 16 bits
        let mut node: u64 = 0;
        node |= 0xABCD_0000_0000_0000;

        // Setting node value should not touch upper bits
        node = set_node_value(node, 123);
        assert_eq!(get_node_value(node), 123);
        assert_eq!(node >> 48, 0xABCD);
    }


    #[test]
    fn test_octree_node_setting() {
        // Set some branch data in the upper 16 bits
        let mut node: u64 = 0;
        node |= 0xABCD_0000_0000_0000;

        // Setting node value should not touch upper bits
        node = set_node_value(node, 123);
        assert_eq!(get_node_value(node), 123);
        assert_eq!(node >> 48, 0xABCD);
    }
}