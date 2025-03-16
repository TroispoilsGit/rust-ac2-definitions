use std::{io::SeekFrom, mem};

use crate::{
    DatReader, constantes::static_const::MAX_NUM_CHILDREN, extensions::binary_reader::BinaryReader,
};

use super::node_entry::NodeEntry;

pub struct TreeNode {
    offset: usize,
    num_entries: u32,
    num_children: u32,
    children: Vec<TreeNode>,
    entry: Option<Vec<NodeEntry>>,
}

impl TreeNode {
    pub fn new(offset: usize, dat_reader: &mut DatReader) -> Self {
        let mut tree_node = TreeNode {
            offset,
            num_entries: 0,
            num_children: 0,
            children: Vec::new(),
            entry: None,
        };

        let node_children_size: u64 = mem::size_of::<u32>() as u64 * MAX_NUM_CHILDREN;
        let pos_new_test: u64 = offset as u64 + mem::size_of::<u32>() as u64 + &node_children_size;

        dat_reader
            .data
            .seek(SeekFrom::Start(pos_new_test))
            .expect("Seek issue");
        let num_entries = dat_reader.data.read_u32().expect("read_u32 issue");
        let mut num_children = num_entries + 1;
        let node_file_size = &node_children_size
            + mem::size_of::<u32>() as u64
            + (mem::size_of::<u32>() * 4) as u64 * u64::from(num_entries);

        tree_node.num_children = num_children;
        tree_node.num_entries = num_entries;

        let mut geting_file_raw = dat_reader
            .get_file_raw(offset as u64, node_file_size)
            .expect("get_file_raw issue");
        for i in 0..num_children {
            let child_offset = geting_file_raw.read_u32().expect("read_u32 issue");
            if child_offset == 0 || (child_offset & 0x80000000) != 0 {
                num_children = i + 1;
                break;
            }
            let child_node = TreeNode::new(child_offset as usize, dat_reader);

            tree_node.add_child(child_node);
        }

        if num_children < MAX_NUM_CHILDREN as u32 {
            geting_file_raw
                .seek(SeekFrom::Current(
                    mem::size_of::<u32>() as i64 * (MAX_NUM_CHILDREN as i64 - num_children as i64),
                ))
                .expect("Seek issue");
        }

        geting_file_raw
            .seek(SeekFrom::Current(4))
            .expect("Seek issue");

        match add_entries(&mut geting_file_raw, num_entries) {
            Ok(v) => {
                tree_node.entry = Some(v);
            }
            Err(e) => {
                println!(
                    "{} : num_entry ({}) file: [{}]",
                    e,
                    num_entries,
                    geting_file_raw.file_to_vec_u3_to_string()
                );
            }
        };

        tree_node
    }

    pub fn display_tree_node(&self) {
        fn display_tree_node_recursive(node: &TreeNode, indent: usize) {
            println!(
                "{}-[{}.{}]-{:08X}",
                " ".repeat(indent),
                node.num_children,
                node.num_entries,
                node.offset
            );
            if let Some(entries) = &node.entry {
                for entry in entries {
                    println!("{}-{:?}", " ".repeat(indent + 2), entry.to_string());
                }
            }
            for child in &node.children {
                display_tree_node_recursive(child, indent + 2);
            }
        }

        display_tree_node_recursive(self, 0);
    }

    fn add_child(&mut self, child: TreeNode) {
        self.children.push(child);
    }
}

fn add_entries(
    geting_file_raw: &mut BinaryReader,
    num_entries: u32,
) -> Result<Vec<NodeEntry>, std::io::Error> {
    let mut node_entries: Vec<NodeEntry> = Vec::new();

    for _ in 0..num_entries {
        node_entries.push(NodeEntry::new(geting_file_raw)?);
    }

    Ok(node_entries)
}
