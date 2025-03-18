use std::{
    cmp::min,
    collections::HashMap,
    fs::File,
    io::{self, Read, SeekFrom},
    mem,
};

use crate::{
    definitions::header::dat_header::DatHeader,
    reader::binary_reader::BinaryReader,
    tree::{node_entry::NodeEntry, node_tree::TreeNode},
    types::data_id::DataId,
};

pub struct DatReader {
    pub path_file: String,
    pub header: DatHeader,
    pub data: BinaryReader,
    pub tree: Option<TreeNode>,
    pub file_list: HashMap<DataId, NodeEntry>,
}

impl DatReader {
    pub fn new(path: &str) -> io::Result<DatReader> {
        // Ouvre le fichier à l'emplacement spécifié
        let mut file = File::open(path).expect("[Issue DatReader::new] open file.");

        // Lit le contenu du fichier dans un vecteur
        let mut cursor_data = Vec::new();
        file.read_to_end(&mut cursor_data)
            .expect("[Issue DatReader::new] read to end file.");

        // Crée un curseur à partir des données lues
        let mut reader = BinaryReader::new(cursor_data);

        // Generer le header
        let header = DatHeader::new(&mut reader).expect("[Issue DatReader::new] read Header");

        // Retourne un DatReader initialisé
        Ok(DatReader {
            path_file: path.to_string(),
            header,
            data: reader,
            tree: None,
            file_list: HashMap::new(),
        })
    }

    pub fn init_tree(&mut self) {
        let tree_node = TreeNode::new(self.header.disk_file_info.tree_root_offset as usize, self);
        self.tree = Some(tree_node);
    }

    pub fn get_file_raw(&mut self, position: u64, node_file_size: u64) -> io::Result<BinaryReader> {
        let mut remaining_size = node_file_size;
        let mut offset = position;
        let mut buffer = vec![0; node_file_size as usize];

        while remaining_size > 0 {
            self.data.seek(SeekFrom::Start(offset)).expect("Seek issue");
            let mut remaining_block_size = self.header.disk_file_info.block_size;

            let next_block_offset = self.data.read_u32()?;
            remaining_block_size -= mem::size_of::<u32>() as u32;

            let size_to_read = min(remaining_size, remaining_block_size as u64);

            //howto in rust: C#:  data.Read(fileData, size - remainingSize, sizeToRead);
            self.data.read_exact(
                &mut buffer[(node_file_size - remaining_size) as usize
                    ..(node_file_size - remaining_size + size_to_read) as usize],
            )?;

            remaining_size -= size_to_read;
            offset = next_block_offset as u64;
        }

        Ok(BinaryReader::new(buffer))
    }

    pub fn get_binary_file_u32(&mut self, data_id: u32) -> io::Result<BinaryReader> {
        let id = DataId::new(data_id);
        Ok(self.get_binary_file(&id)?)
    }

    pub fn get_binary_file(&mut self, data_id: &DataId) -> io::Result<BinaryReader> {
        if let Some(file) = self.file_list.get(data_id) {
            let binary_file = self.get_file_raw(file.offset as u64, file.size as u64)?;
            Ok(binary_file)
        } else {
            Err(io::Error::new(io::ErrorKind::NotFound, "DataId not found"))
        }
    }
}
