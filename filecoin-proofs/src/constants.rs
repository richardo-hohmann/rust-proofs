use std::collections::HashMap;
use std::sync::RwLock;

use lazy_static::lazy_static;
use storage_proofs::hasher::Hasher;
use storage_proofs::util::NODE_SIZE;
use typenum::{U0, U2, U8};

use crate::param::{ParameterData, ParameterMap};
use crate::types::UnpaddedBytesAmount;

pub const SECTOR_SIZE_2_KIB: u64 = 1 << 11;
pub const SECTOR_SIZE_4_KIB: u64 = 1 << 12;
pub const SECTOR_SIZE_16_KIB: u64 = 1 << 14;
pub const SECTOR_SIZE_32_KIB: u64 = 1 << 15;
pub const SECTOR_SIZE_8_MIB: u64 = 1 << 23;
pub const SECTOR_SIZE_16_MIB: u64 = 1 << 24;
pub const SECTOR_SIZE_512_MIB: u64 = 1 << 29;
pub const SECTOR_SIZE_1_GIB: u64 = 1 << 30;
pub const SECTOR_SIZE_32_GIB: u64 = 1 << 35;
pub const SECTOR_SIZE_64_GIB: u64 = 1 << 36;

pub const WINNING_POST_CHALLENGE_COUNT: usize = 66;
pub const WINNING_POST_SECTOR_COUNT: usize = 1;

pub const WINDOW_POST_CHALLENGE_COUNT: usize = 10;

pub const DRG_DEGREE: usize = storage_proofs::drgraph::BASE_DEGREE;
pub const EXP_DEGREE: usize = storage_proofs::porep::stacked::EXP_DEGREE;

lazy_static! {
    pub static ref PARAMETERS: ParameterMap =
        serde_json::from_str(include_str!("../parameters.json")).expect("Invalid parameters.json");
    pub static ref POREP_MINIMUM_CHALLENGES: RwLock<HashMap<u64, u64>> = RwLock::new(
        [
            (SECTOR_SIZE_2_KIB, 2),
            (SECTOR_SIZE_4_KIB, 2),
            (SECTOR_SIZE_16_KIB, 2),
            (SECTOR_SIZE_32_KIB, 2),
            (SECTOR_SIZE_8_MIB, 2),
            (SECTOR_SIZE_16_MIB, 2),
            (SECTOR_SIZE_512_MIB, 2),
            (SECTOR_SIZE_1_GIB, 2),
            (SECTOR_SIZE_32_GIB, 138),
            (SECTOR_SIZE_64_GIB, 138),
        ]
        .iter()
        .copied()
        .collect()
    );
    pub static ref POREP_PARTITIONS: RwLock<HashMap<u64, u8>> = RwLock::new(
        [
            (SECTOR_SIZE_2_KIB, 1),
            (SECTOR_SIZE_4_KIB, 1),
            (SECTOR_SIZE_16_KIB, 1),
            (SECTOR_SIZE_32_KIB, 1),
            (SECTOR_SIZE_8_MIB, 1),
            (SECTOR_SIZE_16_MIB, 1),
            (SECTOR_SIZE_512_MIB, 1),
            (SECTOR_SIZE_1_GIB, 1),
            (SECTOR_SIZE_32_GIB, 8),
            (SECTOR_SIZE_64_GIB, 8),
        ]
        .iter()
        .copied()
        .collect()
    );
    pub static ref LAYERS: RwLock<HashMap<u64, usize>> = RwLock::new(
        [
            (SECTOR_SIZE_2_KIB, 2),
            (SECTOR_SIZE_4_KIB, 2),
            (SECTOR_SIZE_16_KIB, 2),
            (SECTOR_SIZE_32_KIB, 2),
            (SECTOR_SIZE_8_MIB, 2),
            (SECTOR_SIZE_16_MIB, 2),
            (SECTOR_SIZE_512_MIB, 2),
            (SECTOR_SIZE_1_GIB, 2),
            (SECTOR_SIZE_32_GIB, 11),
            (SECTOR_SIZE_64_GIB, 11),
        ]
        .iter()
        .copied()
        .collect()
    );
    // These numbers must match those used for Window PoSt scheduling in the miner actor.
    // Please coordinate changes with actor code.
    // https://github.com/filecoin-project/specs-actors/blob/master/actors/abi/sector.go
    pub static ref WINDOW_POST_SECTOR_COUNT: RwLock<HashMap<u64, usize>> = RwLock::new(
        [
            (SECTOR_SIZE_2_KIB, 2),
            (SECTOR_SIZE_4_KIB, 2),
            (SECTOR_SIZE_16_KIB, 2),
            (SECTOR_SIZE_32_KIB, 2),
            (SECTOR_SIZE_8_MIB, 2),
            (SECTOR_SIZE_16_MIB, 2),
            (SECTOR_SIZE_512_MIB, 2),
            (SECTOR_SIZE_1_GIB, 2),
            (SECTOR_SIZE_32_GIB, 2349), // this gives 133,977,564 constraints, fitting just in a single partition
            (SECTOR_SIZE_64_GIB, 2300), // this gives 131,182,800 constraints, fitting just in a single partition
        ]
        .iter()
        .copied()
        .collect()
    );
}

/// The size of a single snark proof.
pub const SINGLE_PARTITION_PROOF_LEN: usize = 192;

pub const MINIMUM_RESERVED_LEAVES_FOR_PIECE_IN_SECTOR: u64 = 4;

// Bit padding causes bytes to only be aligned at every 127 bytes (for 31.75 bytes).
pub const MINIMUM_RESERVED_BYTES_FOR_PIECE_IN_FULLY_ALIGNED_SECTOR: u64 =
    (MINIMUM_RESERVED_LEAVES_FOR_PIECE_IN_SECTOR * NODE_SIZE as u64) - 1;

/// The minimum size a single piece must have before padding.
pub const MIN_PIECE_SIZE: UnpaddedBytesAmount = UnpaddedBytesAmount(127);

/// The hasher used for creating comm_d.
pub type DefaultPieceHasher = storage_proofs::hasher::Sha256Hasher;
pub type DefaultPieceDomain = <DefaultPieceHasher as Hasher>::Domain;

/// The default hasher for merkle trees currently in use.
pub type DefaultTreeHasher = storage_proofs::hasher::PoseidonHasher;
pub type DefaultTreeDomain = <DefaultTreeHasher as Hasher>::Domain;

pub type DefaultBinaryTree = storage_proofs::merkle::BinaryMerkleTree<DefaultTreeHasher>;
pub type DefaultOctTree = storage_proofs::merkle::OctMerkleTree<DefaultTreeHasher>;
pub type DefaultOctLCTree = storage_proofs::merkle::OctLCMerkleTree<DefaultTreeHasher>;

pub type SectorShape2KiB = LCTree<DefaultTreeHasher, U8, U0, U0>;
pub type SectorShape4KiB = LCTree<DefaultTreeHasher, U8, U2, U0>;
pub type SectorShape16KiB = LCTree<DefaultTreeHasher, U8, U8, U0>;
pub type SectorShape32KiB = LCTree<DefaultTreeHasher, U8, U8, U2>;
pub type SectorShape8MiB = LCTree<DefaultTreeHasher, U8, U0, U0>;
pub type SectorShape16MiB = LCTree<DefaultTreeHasher, U8, U2, U0>;
pub type SectorShape512MiB = LCTree<DefaultTreeHasher, U8, U0, U0>;
pub type SectorShape1GiB = LCTree<DefaultTreeHasher, U8, U2, U0>;
pub type SectorShape32GiB = LCTree<DefaultTreeHasher, U8, U8, U0>;
pub type SectorShape64GiB = LCTree<DefaultTreeHasher, U8, U8, U2>;

pub use storage_proofs::merkle::{DiskTree, LCTree};

/// Get the correct parameter data for a given cache id.
pub fn get_parameter_data(cache_id: &str) -> Option<&ParameterData> {
    PARAMETERS.get(&parameter_id(cache_id))
}

fn parameter_id(cache_id: &str) -> String {
    format!(
        "v{}-{}.params",
        storage_proofs::parameter_cache::VERSION,
        cache_id
    )
}

/// Calls a function with the type hint of the sector shape matching the provided sector.
/// Panics if provided with an unknown sector size.
#[macro_export]
macro_rules! with_shape {
    ($size:expr, $f:ident) => {
        with_shape!($size, $f,)
    };
    ($size:expr, $f:ident, $($args:expr,)*) => {
        match $size {
            _x if $size == $crate::constants::SECTOR_SIZE_2_KIB => {
              $f::<$crate::constants::SectorShape2KiB>($($args),*)
            },
            _x if $size == $crate::constants::SECTOR_SIZE_4_KIB => {
              $f::<$crate::constants::SectorShape4KiB>($($args),*)
            },
            _x if $size == $crate::constants::SECTOR_SIZE_16_KIB => {
              $f::<$crate::constants::SectorShape16KiB>($($args),*)
            },
            _x if $size == $crate::constants::SECTOR_SIZE_32_KIB => {
              $f::<$crate::constants::SectorShape32KiB>($($args),*)
            },
            _xx if $size == $crate::constants::SECTOR_SIZE_8_MIB => {
              $f::<$crate::constants::SectorShape8MiB>($($args),*)
            },
            _xx if $size == $crate::constants::SECTOR_SIZE_16_MIB => {
              $f::<$crate::constants::SectorShape16MiB>($($args),*)
            },
            _x if $size == $crate::constants::SECTOR_SIZE_512_MIB => {
              $f::<$crate::constants::SectorShape512MiB>($($args),*)
            },
            _x if $size == $crate::constants::SECTOR_SIZE_1_GIB => {
              $f::<$crate::constants::SectorShape1GiB>($($args),*)
            },
            _x if $size == $crate::constants::SECTOR_SIZE_32_GIB => {
              $f::<$crate::constants::SectorShape32GiB>($($args),*)
            },
            _x if $size == $crate::constants::SECTOR_SIZE_64_GIB => {
              $f::<$crate::constants::SectorShape64GiB>($($args),*)
            },
            _ => panic!("unsupported sector size: {}", $size),
        }
    };
    ($size:expr, $f:ident, $($args:expr),*) => {
        with_shape!($size, $f, $($args,)*)
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    use generic_array::typenum::Unsigned;
    use storage_proofs::merkle::MerkleTreeTrait;

    fn canonical_shape(sector_size: u64) -> (usize, usize, usize) {
        // This could perhaps be cleaned up, but I think it expresses the intended constraints
        // and is consistent with our current hard-coded size->shape mappings.
        assert_eq!(sector_size.count_ones(), 1);
        let log_byte_size = sector_size.trailing_zeros();
        let log_nodes = log_byte_size - 5; // 2^5 = 32-byte nodes

        let max_tree_log = 3; // Largest allowable arity. The optimal shape.

        let log_max_base = 27; // 4 GiB worth of nodes
        let log_base = max_tree_log; // Base must be oct trees.x
        let log_in_base = u32::min(log_max_base, (log_nodes / log_base) * log_base); // How many nodes in base?

        let log_upper = log_nodes - log_in_base; // Nodes in sub and upper combined.
        let log_rem = log_upper % max_tree_log; // Remainder after filling optimal trees.

        let (log_sub, log_top) = {
            // Are the upper trees empty?
            if log_upper > 0 {
                // Do we need a remainder tree?
                if log_rem == 0 {
                    (Some(max_tree_log), None) // No remainder tree, fill the sub tree optimall.y
                } else {
                    // Need a remainder tree.

                    // Do we have room for another max tree?
                    if log_upper > max_tree_log {
                        // There is room. Use the sub tree for as much overflow as we can fit optimally.
                        // And put the rest in the top tree.
                        (Some(max_tree_log), Some(log_rem))
                    } else {
                        // Can't fit another max tree.
                        // Just put the remainder in the sub tree.
                        (Some(log_rem), None)
                    }
                }
            } else {
                // Upper trees are empty.
                (None, None)
            }
        };

        let base = 1 << log_base;
        let sub = if let Some(l) = log_sub { 1 << l } else { 0 };
        let top = if let Some(l) = log_top { 1 << l } else { 0 };

        (base, sub, top)
    }

    fn arities_to_usize<Tree: MerkleTreeTrait>() -> (usize, usize, usize) {
        (
            Tree::Arity::to_usize(),
            Tree::SubTreeArity::to_usize(),
            Tree::TopTreeArity::to_usize(),
        )
    }

    #[test]
    fn test_with_shape_macro() {
        test_with_shape_macro_aux(SECTOR_SIZE_2_KIB);
        test_with_shape_macro_aux(SECTOR_SIZE_4_KIB);
        test_with_shape_macro_aux(SECTOR_SIZE_8_MIB);
        test_with_shape_macro_aux(SECTOR_SIZE_16_MIB);
        test_with_shape_macro_aux(SECTOR_SIZE_512_MIB);
        test_with_shape_macro_aux(SECTOR_SIZE_1_GIB);
        test_with_shape_macro_aux(SECTOR_SIZE_32_GIB);
        test_with_shape_macro_aux(SECTOR_SIZE_64_GIB);
    }

    fn test_with_shape_macro_aux(sector_size: u64) {
        let expected = canonical_shape(sector_size);
        let arities = with_shape!(sector_size, arities_to_usize);
        assert_eq!(
            arities, expected,
            "Wrong shape for sector size {}: have {:?} but need {:?}.",
            sector_size, arities, expected
        );
    }
}
