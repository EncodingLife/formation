use apis::{HexCoord, HexCoordinate, HexWorldShape, MapIndex};

pub fn get_spawn_coords(world_shape: HexWorldShape, pawn_count: usize) -> Vec<HexCoord> {
    let indexer = MapIndex::new(world_shape);
    let width = get_width(world_shape);
    let max_pawns_per_rank = max_pawns_in_rank(width, pawn_count);

    let mut remaining_pawns = pawn_count;
    let mut rank = pawn_count.div_ceil(max_pawns_per_rank);
    let mut pos = Vec::with_capacity(pawn_count);
    while remaining_pawns > 0 {
        let pawns_in_rank = usize::min(remaining_pawns, max_pawns_per_rank);
        pos.append(&mut fill_rank(indexer, rank, width, pawns_in_rank, false));
        remaining_pawns = remaining_pawns - pawns_in_rank;
        rank = rank - 1;
    }

    pos
}

fn fill_rank(
    indexer: MapIndex,
    rank: usize,
    width: usize,
    pawn_count: usize,
    stagger: bool,
) -> Vec<HexCoord> {
    let required_space = pawn_count * 2 - 1;
    let offset = (width - required_space) / 2 + if stagger { 1 } else { 0 };
    let mut v = Vec::with_capacity(pawn_count);
    for i in 0..pawn_count {
        let col = offset + i * 2;
        v.push(indexer.offset_coord(col as i32, rank as i32))
    }
    v
}

fn row_start(index: usize, world_shape: HexWorldShape) -> HexCoord {
    let indexer = MapIndex::new(world_shape);
    indexer.coord(index)
}

fn get_width(world_shape: HexWorldShape) -> usize {
    match world_shape {
        HexWorldShape::Hexagon(radius, _) => radius + 1,
        HexWorldShape::Rectangle(width, _, _) => width,
        HexWorldShape::Square(width, _) => width,
    }
}

fn get_height(world_shape: HexWorldShape) -> usize {
    match world_shape {
        HexWorldShape::Hexagon(radius, _) => radius + 1,
        HexWorldShape::Rectangle(_, height, _) => height,
        HexWorldShape::Square(height, _) => height,
    }
}

fn max_pawns_in_rank(width: usize, pawn_count: usize) -> usize {
    let max_pawns_in_rank = width.div_ceil(2);
    let last_rank_size = pawn_count % max_pawns_in_rank;

    if max_pawns_in_rank > pawn_count {
        return pawn_count;
    }

    // Will reducing the rank size result in more regularly shaped formation?
    // if last rank == 0 then no
    // if last rank is less than half of the others then yes
    if last_rank_size != 0 && (max_pawns_in_rank / 2) >= last_rank_size {
        max_pawns_in_rank - 1
    } else {
        max_pawns_in_rank
    }
}

pub fn spawn_offsets(width: usize, pawn_count: usize) -> Vec<usize> {
    let mut offsets = Vec::new();
    let max_pawns_in_rank = max_pawns_in_rank(width, pawn_count);

    let mut remaining_pawns = pawn_count;
    while remaining_pawns > 0 {
        let pawns_in_rank = usize::min(max_pawns_in_rank, remaining_pawns);
        let required_space = 1 + ((pawns_in_rank - 1) * 2); // 5

        let offset = (width - required_space) / 2;
        offsets.push(offset);
        remaining_pawns = remaining_pawns - pawns_in_rank;
    }

    offsets
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;

    #[test_case(5, 1, 1)]
    #[test_case(5, 5, 3)]
    #[test_case(5, 6, 3)]
    #[test_case(5, 25, 2)]
    #[test_case(7, 25, 2)]
    #[test_case(7, 14, 3)]
    #[test_case(12, 14, 5)]
    #[test_case(12, 15, 5)]
    #[test_case(12, 16, 6)]
    #[test_case(12, 17, 6)]
    #[test_case(12, 22, 6)]
    fn max_pawns_in_rank_tests(width: usize, pawn_count: usize, expected: usize) {
        assert_eq!(max_pawns_in_rank(width, pawn_count), expected)
    }

    #[test_case(5,1,vec!(2))]
    #[test_case(5,2,vec!(1))]
    #[test_case(5,3,vec!(0))]
    #[test_case(5,4,vec!(1,1))]
    #[test_case(5,5,vec!(0,1))]
    #[test_case(5,6,vec!(0,0))]
    #[test_case(12,1,vec!(5))]
    #[test_case(12,12,vec!(0,0))]
    #[test_case(12,15,vec!(1,1,1))]
    #[test_case(12,16,vec!(0,0,2))]
    pub fn spawn_offsets_tests(width: usize, pawn_count: usize, expected: Vec<usize>) {
        assert_eq!(spawn_offsets(width, pawn_count), expected)
    }

    #[test]
    pub fn get_spawn_coords_tests() {
        let r = get_spawn_coords(HexWorldShape::Square(12, apis::HexOrientation::Flat), 15);

        // Currently only generates the start of each row
        assert_eq!(
            r,
            vec![
                HexCoord::from_axial(0, 2),
                HexCoord::from_axial(0, 1),
                HexCoord::from_axial(0, 0)
            ]
        )
    }

    #[test]
    pub fn fill_rank_tests() {
        let shape = HexWorldShape::Square(5, apis::HexOrientation::Flat);
        let indexer = MapIndex::new(shape);
        let width = get_width(shape);
        let rank_1: Vec<HexCoord> = fill_rank(indexer, 0, width, 3, false);
        assert_eq!(
            rank_1,
            vec![
                HexCoord::from_axial(0, 0),
                HexCoord::from_axial(2, -1),
                HexCoord::from_axial(4, -2)
            ]
        );

        let rank_1: Vec<HexCoord> = fill_rank(indexer, 1, width, 3, false);
        assert_eq!(
            rank_1,
            vec![
                HexCoord::from_axial(0, 1),
                HexCoord::from_axial(2, 0),
                HexCoord::from_axial(4, -1)
            ]
        );
    }
}
