use anyhow::{Context, Result};
use chrono;
use prost::Message;
use rusqlite::types::{FromSql, FromSqlResult, ValueRef};
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use std::path::Path;

pub mod binexport {
    include!(concat!(env!("OUT_DIR"), "/binexport.rs"));
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct File {
    pub id: i64,
    pub filename: String,
    pub exe_filename: String,
    pub hash: String,
    pub functions: i64,
    pub lib_functions: i64,
    pub calls: i64,
    pub basic_blocks: i64,
    pub lib_basic_blocks: i64,
    pub edges: i64,
    pub lib_edges: i64,
    pub instructions: i64,
    pub lib_instructions: i64,
}

impl std::fmt::Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "FILE:\n  \
            id:               {}\n  \
            filename:         {}\n  \
            exe_filename:     {}\n  \
            hash:             {}\n  \
            functions:        {}\n  \
            lib_functions:    {}\n  \
            calls:            {}\n  \
            basic_blocks:     {}\n  \
            lib_basic_blocks: {}\n  \
            edges:            {}\n  \
            lib_edges:        {}\n  \
            instructions:     {}\n  \
            lib_instructions: {}\n",
            self.id,
            self.filename,
            self.exe_filename,
            self.hash,
            self.functions,
            self.lib_functions,
            self.calls,
            self.basic_blocks,
            self.lib_basic_blocks,
            self.edges,
            self.lib_edges,
            self.instructions,
            self.lib_instructions
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
    pub version: String,
    pub file1: i64,
    pub file2: i64,
    pub description: String,
    pub created: chrono::DateTime<chrono::Utc>,
    pub modified: chrono::DateTime<chrono::Utc>,
    pub similarity: f64,
    pub confidence: f64,
}

impl std::fmt::Display for Metadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "METADATA:\n  \
            version:      {}\n  \
            file1:        {}\n  \
            file2:        {}\n  \
            description:  {}\n  \
            created:      {}\n  \
            modified:     {}\n  \
            similarity:   {:.2}\n  \
            confidence:   {:.2}\n",
            self.version,
            self.file1,
            self.file2,
            self.description,
            self.created.format("%Y-%m-%d %H:%M:%S"),
            self.modified.format("%Y-%m-%d %H:%M:%S"),
            self.similarity,
            self.confidence
        )
    }
}

/// Enum representing the different function matching algorithms used in BinDiff
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FunctionAlgorithm {
    None,
    NameHashMatching,
    HashMatching,
    EdgesFlowgraphMdIndex,
    EdgesCallgraphMdIndex,
    MdIndexMatchingFlowgraphTopDown,
    MdIndexMatchingFlowgraphBottomUp,
    PrimeSignatureMatching,
    MdIndexMatchingCallGraphTopDown,
    MdIndexMatchingCallGraphBottomUp,
    RelaxedMdIndexMatching,
    InstructionCount,
    AddressSequence,
    StringReferences,
    LoopCountMatching,
    CallSequenceMatchingExact,
    CallSequenceMatchingTopology,
    CallSequenceMatchingSequence,
    CallReferenceMatching,
    Manual,
    /// Unknown or custom algorithm
    Other(String),
}

impl std::fmt::Display for FunctionAlgorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FunctionAlgorithm::None => write!(f, "none"),
            FunctionAlgorithm::NameHashMatching => write!(f, "name hash matching"),
            FunctionAlgorithm::HashMatching => write!(f, "hash matching"),
            FunctionAlgorithm::EdgesFlowgraphMdIndex => write!(f, "edges flowgraph MD index"),
            FunctionAlgorithm::EdgesCallgraphMdIndex => write!(f, "edges callgraph MD index"),
            FunctionAlgorithm::MdIndexMatchingFlowgraphTopDown => {
                write!(f, "MD index matching (flowgraph MD index, top down)")
            }
            FunctionAlgorithm::MdIndexMatchingFlowgraphBottomUp => {
                write!(f, "MD index matching (flowgraph MD index, bottom up)")
            }
            FunctionAlgorithm::PrimeSignatureMatching => write!(f, "signature matching"),
            FunctionAlgorithm::MdIndexMatchingCallGraphTopDown => {
                write!(f, "MD index matching (callGraph MD index, top down)")
            }
            FunctionAlgorithm::MdIndexMatchingCallGraphBottomUp => {
                write!(f, "MD index matching (callGraph MD index, bottom up)")
            }
            FunctionAlgorithm::RelaxedMdIndexMatching => write!(f, "MD index matching"),
            FunctionAlgorithm::InstructionCount => write!(f, "instruction count"),
            FunctionAlgorithm::AddressSequence => write!(f, "address sequence"),
            FunctionAlgorithm::StringReferences => write!(f, "string references"),
            FunctionAlgorithm::LoopCountMatching => write!(f, "loop count matching"),
            FunctionAlgorithm::CallSequenceMatchingExact => {
                write!(f, "call sequence matching(exact)")
            }
            FunctionAlgorithm::CallSequenceMatchingTopology => {
                write!(f, "call sequence matching(topology)")
            }
            FunctionAlgorithm::CallSequenceMatchingSequence => {
                write!(f, "call sequence matching(sequence)")
            }
            FunctionAlgorithm::CallReferenceMatching => write!(f, "call references matching"),
            FunctionAlgorithm::Manual => write!(f, "manual"),
            FunctionAlgorithm::Other(s) => write!(f, "other({})", s),
        }
    }
}

impl FromSql for FunctionAlgorithm {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        let algorithm_id: i32 = value.as_i64()? as i32;
        match algorithm_id {
            0 => Ok(FunctionAlgorithm::None),
            1 => Ok(FunctionAlgorithm::NameHashMatching),
            2 => Ok(FunctionAlgorithm::HashMatching),
            3 => Ok(FunctionAlgorithm::EdgesFlowgraphMdIndex),
            4 => Ok(FunctionAlgorithm::EdgesCallgraphMdIndex),
            5 => Ok(FunctionAlgorithm::MdIndexMatchingFlowgraphTopDown),
            6 => Ok(FunctionAlgorithm::MdIndexMatchingFlowgraphBottomUp),
            7 => Ok(FunctionAlgorithm::PrimeSignatureMatching),
            8 => Ok(FunctionAlgorithm::MdIndexMatchingCallGraphTopDown),
            9 => Ok(FunctionAlgorithm::MdIndexMatchingCallGraphBottomUp),
            10 => Ok(FunctionAlgorithm::RelaxedMdIndexMatching),
            11 => Ok(FunctionAlgorithm::InstructionCount),
            12 => Ok(FunctionAlgorithm::AddressSequence),
            13 => Ok(FunctionAlgorithm::StringReferences),
            14 => Ok(FunctionAlgorithm::LoopCountMatching),
            15 => Ok(FunctionAlgorithm::CallSequenceMatchingExact),
            16 => Ok(FunctionAlgorithm::CallSequenceMatchingTopology),
            17 => Ok(FunctionAlgorithm::CallSequenceMatchingSequence),
            18 => Ok(FunctionAlgorithm::CallReferenceMatching),
            19 => Ok(FunctionAlgorithm::Manual),
            other => Ok(FunctionAlgorithm::Other(other.to_string())),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionMatch {
    pub id: i64,
    pub address1: i64,
    pub name1: String,
    pub address2: i64,
    pub name2: String,
    pub similarity: f64,
    pub confidence: f64,
    pub flags: i64,
    pub algorithm: FunctionAlgorithm,
    pub evaluate: bool,
    pub comment_supported: bool,
    pub basic_blocks: i64,
    pub edges: i64,
    pub instructions: i64,
}

impl std::fmt::Display for FunctionMatch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.name1 != self.name2 {
            write!(
                f,
                "{} -> {}\tsimilarity: {:.2}, confidence: {:.2}, algorithm: {}",
                self.name1, self.name2, self.similarity, self.confidence, self.algorithm
            )
        } else {
            write!(
                f,
                "{}:\tsimilarity: {:.2}, confidence: {:.2}",
                self.name1, self.similarity, self.confidence
            )
        }
    }
}

/// Enum representing the different basic block matching algorithms used in BinDiff
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BasicBlockAlgorithm {
    None,
    EdgesPrimeProduct,
    HashMatchingFourInstMin,
    PrimeMatchingFourInstMin,
    CallReferenceMatching,
    StringReferencesMatching,
    EdgesMdIndexTopDown,
    MdIndexMatchingTopDown,
    EdgesMdIndexBottomUp,
    MdIndexMatchingBottomUp,
    RelaxedMdIndexMatching,
    PrimeMatchingNoInstMin,
    EdgesLengauerTarjanDominated,
    LoopEntryMatching,
    SelfLoopMatching,
    EntryPointMatching,
    ExitPointMatching,
    InstructionCountMatching,
    JumpSequenceMatching,
    PropagationSizeOne,
    Manual,
    /// Unknown or custom algorithm
    Other(String),
}

impl std::fmt::Display for BasicBlockAlgorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BasicBlockAlgorithm::None => write!(f, "none"),
            BasicBlockAlgorithm::EdgesPrimeProduct => write!(f, "edges prime product"),
            BasicBlockAlgorithm::HashMatchingFourInstMin => {
                write!(f, "hash matching (4 instructions minimum)")
            }
            BasicBlockAlgorithm::PrimeMatchingFourInstMin => {
                write!(f, "prime matching (4 instructions minimum)")
            }
            BasicBlockAlgorithm::CallReferenceMatching => write!(f, "call reference matching"),
            BasicBlockAlgorithm::StringReferencesMatching => write!(f, "string reference matching"),
            BasicBlockAlgorithm::EdgesMdIndexTopDown => write!(f, "edges MD index (top down)"),
            BasicBlockAlgorithm::MdIndexMatchingTopDown => {
                write!(f, "MD index matching (top down)")
            }
            BasicBlockAlgorithm::EdgesMdIndexBottomUp => write!(f, "edges MD index (bottom up)"),
            BasicBlockAlgorithm::MdIndexMatchingBottomUp => {
                write!(f, "MD index matching (bottom up)")
            }
            BasicBlockAlgorithm::RelaxedMdIndexMatching => write!(f, "relaxed MD index matching"),
            BasicBlockAlgorithm::PrimeMatchingNoInstMin => {
                write!(f, "prime matching (0 instructions minimum)")
            }
            BasicBlockAlgorithm::EdgesLengauerTarjanDominated => {
                write!(f, "edges Lengauer Tarjan dominated")
            }
            BasicBlockAlgorithm::LoopEntryMatching => write!(f, "loop entry matching"),
            BasicBlockAlgorithm::SelfLoopMatching => write!(f, "self loop matching"),
            BasicBlockAlgorithm::EntryPointMatching => write!(f, "entry point matching"),
            BasicBlockAlgorithm::ExitPointMatching => write!(f, "exit point matching"),
            BasicBlockAlgorithm::InstructionCountMatching => {
                write!(f, "instruction count matching")
            }
            BasicBlockAlgorithm::JumpSequenceMatching => write!(f, "jump sequence matching"),
            BasicBlockAlgorithm::PropagationSizeOne => write!(f, "propagation (size==1)"),
            BasicBlockAlgorithm::Manual => write!(f, "manual"),
            BasicBlockAlgorithm::Other(s) => write!(f, "other({})", s),
        }
    }
}

impl FromSql for BasicBlockAlgorithm {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        let algorithm_id: i32 = value.as_i64()? as i32;
        match algorithm_id {
            0 => Ok(BasicBlockAlgorithm::None),
            1 => Ok(BasicBlockAlgorithm::EdgesPrimeProduct),
            2 => Ok(BasicBlockAlgorithm::HashMatchingFourInstMin),
            3 => Ok(BasicBlockAlgorithm::PrimeMatchingFourInstMin),
            4 => Ok(BasicBlockAlgorithm::CallReferenceMatching),
            5 => Ok(BasicBlockAlgorithm::StringReferencesMatching),
            6 => Ok(BasicBlockAlgorithm::EdgesMdIndexTopDown),
            7 => Ok(BasicBlockAlgorithm::MdIndexMatchingTopDown),
            8 => Ok(BasicBlockAlgorithm::EdgesMdIndexBottomUp),
            9 => Ok(BasicBlockAlgorithm::MdIndexMatchingBottomUp),
            10 => Ok(BasicBlockAlgorithm::RelaxedMdIndexMatching),
            11 => Ok(BasicBlockAlgorithm::PrimeMatchingNoInstMin),
            12 => Ok(BasicBlockAlgorithm::EdgesLengauerTarjanDominated),
            13 => Ok(BasicBlockAlgorithm::LoopEntryMatching),
            14 => Ok(BasicBlockAlgorithm::SelfLoopMatching),
            15 => Ok(BasicBlockAlgorithm::EntryPointMatching),
            16 => Ok(BasicBlockAlgorithm::ExitPointMatching),
            17 => Ok(BasicBlockAlgorithm::InstructionCountMatching),
            18 => Ok(BasicBlockAlgorithm::JumpSequenceMatching),
            19 => Ok(BasicBlockAlgorithm::PropagationSizeOne),
            20 => Ok(BasicBlockAlgorithm::Manual),
            other => Ok(BasicBlockAlgorithm::Other(other.to_string())),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasicBlockMatch {
    pub id: i64,
    pub function_id: i64,
    pub address1: i64,
    pub address2: i64,
    pub algorithm: BasicBlockAlgorithm,
    pub evaluate: bool,
}

impl std::fmt::Display for BasicBlockMatch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:#x} -> {:#x} ({})",
            self.address1, self.address2, self.algorithm
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Instruction {
    pub id: i64,
    pub address1: i64,
    pub address2: i64,
}

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -> {}", self.address1, self.address2)
    }
}

/// Struct to handle SQLite database operations
pub struct BinDiff {
    connection: Connection,
}

impl BinDiff {
    /// Open a connection to the SQLite database
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let connection = Connection::open(path).context("Failed to open SQLite database")?;
        Ok(Self { connection })
    }

    pub fn close(self) -> std::result::Result<(), rusqlite::Error> {
        self.connection.close().map_err(|(_, err)| err)
    }

    pub fn read_metadata(&self) -> Result<Metadata> {
        let mut stmt = self
            .connection
            .prepare("SELECT * FROM metadata")
            .context("Failed to prepare metadata statement")?;

        stmt.query_row(params![], |row| {
            Ok(Metadata {
                version: row.get(0)?,
                file1: row.get(1)?,
                file2: row.get(2)?,
                description: row.get(3)?,
                created: row.get(4)?,
                modified: row.get(5)?,
                similarity: row.get(6)?,
                confidence: row.get(7)?,
            })
        })
        .context("Failed to query metadata row")
    }

    /// Read all function matches from the database
    pub fn read_file(&self) -> Result<File> {
        let mut stmt = self
            .connection
            .prepare("SELECT * FROM file")
            .context("Failed to prepare file statement")?;

        stmt.query_row(params![], |row| {
            Ok(File {
                id: row.get(0)?,
                filename: row.get(1)?,
                exe_filename: row.get(2)?,
                hash: row.get(3)?,
                functions: row.get(4)?,
                lib_functions: row.get(5)?,
                calls: row.get(6)?,
                basic_blocks: row.get(7)?,
                lib_basic_blocks: row.get(8)?,
                edges: row.get(9)?,
                lib_edges: row.get(10)?,
                instructions: row.get(11)?,
                lib_instructions: row.get(12)?,
            })
        })
        .context("Failed to query file row")
    }

    /// Count the number of function matches
    pub fn count_function_matches(&self) -> Result<usize> {
        let count: i64 = self
            .connection
            .query_row("SELECT COUNT(*) FROM function", params![], |row| row.get(0))
            .context("Failed to count function matches")?;

        Ok(count as usize)
    }

    pub fn read_function_matches(&self) -> Result<Vec<FunctionMatch>> {
        let mut stmt = self
            .connection
            .prepare("SELECT * FROM function")
            .context("Failed to prepare function statement")?;

        let matches = stmt
            .query_map(params![], |row| {
                Ok(FunctionMatch {
                    id: row.get(0)?,
                    address1: row.get(1)?,
                    name1: row.get(2)?,
                    address2: row.get(3)?,
                    name2: row.get(4)?,
                    similarity: row.get(5)?,
                    confidence: row.get(6)?,
                    flags: row.get(7)?,
                    algorithm: row.get(8)?,
                    evaluate: row.get(9)?,
                    comment_supported: row.get(10)?,
                    basic_blocks: row.get(11)?,
                    edges: row.get(12)?,
                    instructions: row.get(13)?,
                })
            })
            .context("Failed to query function row")?
            .collect::<Result<Vec<FunctionMatch>, _>>()?;

        Ok(matches)
    }

    /// Count the number of basic block matches
    pub fn count_basic_block_matches(&self) -> Result<usize> {
        let count: i64 = self
            .connection
            .query_row("SELECT COUNT(*) FROM basicblock", params![], |row| {
                row.get(0)
            })
            .context("Failed to count basic block matches")?;

        Ok(count as usize)
    }

    pub fn read_basic_block_matches(&self) -> Result<Vec<BasicBlockMatch>> {
        let mut stmt = self
            .connection
            .prepare("SELECT * FROM basicblock")
            .context("Failed to prepare basicblock statement")?;

        let matches = stmt
            .query_map(params![], |row| {
                Ok(BasicBlockMatch {
                    id: row.get(0)?,
                    function_id: row.get(1)?,
                    address1: row.get(2)?,
                    address2: row.get(3)?,
                    algorithm: row.get(4)?,
                    evaluate: row.get(5)?,
                })
            })
            .context("Failed to query basicblock row")?
            .collect::<Result<Vec<BasicBlockMatch>, _>>()?;

        Ok(matches)
    }

    /// Count the number of instruction matches
    pub fn count_instruction_matches(&self) -> Result<usize> {
        let count: i64 = self
            .connection
            .query_row("SELECT COUNT(*) FROM instruction", params![], |row| {
                row.get(0)
            })
            .context("Failed to count instruction matches")?;

        Ok(count as usize)
    }

    pub fn read_instruction_matches(&self) -> Result<Vec<Instruction>> {
        let mut stmt = self
            .connection
            .prepare("SELECT * FROM instruction")
            .context("Failed to prepare instruction statement")?;

        let matches = stmt
            .query_map(params![], |row| {
                Ok(Instruction {
                    id: row.get(0)?,
                    address1: row.get(1)?,
                    address2: row.get(2)?,
                })
            })
            .context("Failed to query instruction row")?
            .collect::<Result<Vec<Instruction>, _>>()?;

        Ok(matches)
    }
}

#[derive(Debug, Clone)]
pub struct BinExport {
    pub binexport: binexport::BinExport2,
}

impl BinExport {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let file = std::fs::read(&path).with_context(|| {
            format!("Failed to read BinExport file: {}", path.as_ref().display())
        })?;
        let binexport = binexport::BinExport2::decode(&file[..])
            .context("Failed to decode BinExport protobuf")?;
        Ok(Self { binexport })
    }

    pub fn executable_name(&self) -> Result<String> {
        let executable_name = self
            .binexport
            .meta_information
            .as_ref()
            .context("No meta information available")?
            .executable_name
            .clone()
            .unwrap_or_else(|| "unknown executable".to_string());
        Ok(executable_name)
    }

    // TODO: Add more methods to handle the BinExport protobuf
}

// Example usage demonstration
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_database_operations() -> Result<()> {
        let test_file_path = "tests/kernel.release_vs_kernel.release.BinDiff";

        // Check if the test file exists before running the test
        if !std::path::Path::new(test_file_path).exists() {
            println!("Test file {} not found. Skipping test.", test_file_path);
            return Ok(());
        }

        let db = BinDiff::open(test_file_path)?;

        let file = db.read_file()?;
        println!("{}", file);

        let metadata = db.read_metadata()?;
        println!("{}", metadata);

        let count = db.count_function_matches()?;
        println!("Total Matches: {}", count);

        let matches = db.read_function_matches()?;
        assert_eq!(matches.len(), count);
        for func_match in matches {
            println!("{}", func_match);
        }

        let count = db.count_basic_block_matches()?;
        println!("Total Basic Block Matches: {}", count);

        let basic_block_matches = db.read_basic_block_matches()?;
        assert_eq!(basic_block_matches.len(), count);
        // for basic_block_match in basic_block_matches {
        //     println!("{}", basic_block_match);
        // }

        let count = db.count_instruction_matches()?;
        println!("Total Instruction Matches: {}", count);

        let instruction_matches = db.read_instruction_matches()?;
        assert_eq!(instruction_matches.len(), count);
        // for instruction_match in instruction_matches {
        //     println!("{}", instruction_match);
        // }

        db.close()?;

        Ok(())
    }

    #[test]
    fn test_read_binexport() -> Result<()> {
        let test_file_path = "tests/kernel.release.t6020.BinExport";

        // Check if the test file exists before running the test
        if !std::path::Path::new(test_file_path).exists() {
            println!("Test file {} not found. Skipping test.", test_file_path);
            return Ok(());
        }
        let binexport = BinExport::open(test_file_path)?;
        println!("executable_name: {}", binexport.executable_name()?);

        Ok(())
    }
}
