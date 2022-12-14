use std::collections::BTreeMap;

pub fn get_type_efficacy() -> BTreeMap<u64, BTreeMap<u64, u64>> {
    let mut te = BTreeMap::new();
    te.insert(
        1,
        BTreeMap::from([
            (1, 100),
            (2, 100),
            (3, 100),
            (4, 100),
            (5, 100),
            (6, 50),
            (7, 100),
            (8, 0),
            (9, 50),
            (10, 100),
            (11, 100),
            (12, 100),
            (13, 100),
            (14, 100),
            (15, 100),
            (16, 100),
            (17, 100),
            (18, 100),
        ]),
    );
    te.insert(
        2,
        BTreeMap::from([
            (1, 200),
            (2, 100),
            (3, 50),
            (4, 50),
            (5, 100),
            (6, 200),
            (7, 50),
            (8, 0),
            (9, 200),
            (10, 100),
            (11, 100),
            (12, 100),
            (13, 100),
            (14, 50),
            (15, 200),
            (16, 100),
            (17, 200),
            (18, 50),
        ]),
    );
    te.insert(
        3,
        BTreeMap::from([
            (1, 100),
            (2, 200),
            (3, 100),
            (4, 100),
            (5, 100),
            (6, 50),
            (7, 200),
            (8, 100),
            (9, 50),
            (10, 100),
            (11, 100),
            (12, 200),
            (13, 50),
            (14, 100),
            (15, 100),
            (16, 100),
            (17, 100),
            (18, 100),
        ]),
    );
    te.insert(
        4,
        BTreeMap::from([
            (1, 100),
            (2, 100),
            (3, 100),
            (4, 50),
            (5, 50),
            (6, 50),
            (7, 100),
            (8, 50),
            (9, 0),
            (10, 100),
            (11, 100),
            (12, 200),
            (13, 100),
            (14, 100),
            (15, 100),
            (16, 100),
            (17, 100),
            (18, 200),
        ]),
    );
    te.insert(
        5,
        BTreeMap::from([
            (1, 100),
            (2, 100),
            (3, 0),
            (4, 200),
            (5, 100),
            (6, 200),
            (7, 50),
            (8, 100),
            (9, 200),
            (10, 200),
            (11, 100),
            (12, 50),
            (13, 200),
            (14, 100),
            (15, 100),
            (16, 100),
            (17, 100),
            (18, 100),
        ]),
    );
    te.insert(
        6,
        BTreeMap::from([
            (1, 100),
            (2, 50),
            (3, 200),
            (4, 100),
            (5, 50),
            (6, 100),
            (7, 200),
            (8, 100),
            (9, 50),
            (10, 200),
            (11, 100),
            (12, 100),
            (13, 100),
            (14, 100),
            (15, 200),
            (16, 100),
            (17, 100),
            (18, 100),
        ]),
    );
    te.insert(
        7,
        BTreeMap::from([
            (1, 100),
            (2, 50),
            (3, 50),
            (4, 50),
            (5, 100),
            (6, 100),
            (7, 100),
            (8, 50),
            (9, 50),
            (10, 50),
            (11, 100),
            (12, 200),
            (13, 100),
            (14, 200),
            (15, 100),
            (16, 100),
            (17, 200),
            (18, 50),
        ]),
    );
    te.insert(
        8,
        BTreeMap::from([
            (1, 0),
            (2, 100),
            (3, 100),
            (4, 100),
            (5, 100),
            (6, 100),
            (7, 100),
            (8, 200),
            (9, 100),
            (10, 100),
            (11, 100),
            (12, 100),
            (13, 100),
            (14, 200),
            (15, 100),
            (16, 100),
            (17, 50),
            (18, 100),
        ]),
    );
    te.insert(
        9,
        BTreeMap::from([
            (1, 100),
            (2, 100),
            (3, 100),
            (4, 100),
            (5, 100),
            (6, 200),
            (7, 100),
            (8, 100),
            (9, 50),
            (10, 50),
            (11, 50),
            (12, 100),
            (13, 50),
            (14, 100),
            (15, 200),
            (16, 100),
            (17, 100),
            (18, 200),
        ]),
    );
    te.insert(
        10,
        BTreeMap::from([
            (1, 100),
            (2, 100),
            (3, 100),
            (4, 100),
            (5, 100),
            (6, 50),
            (7, 200),
            (8, 100),
            (9, 200),
            (10, 50),
            (11, 50),
            (12, 200),
            (13, 100),
            (14, 100),
            (15, 200),
            (16, 50),
            (17, 100),
            (18, 100),
        ]),
    );
    te.insert(
        11,
        BTreeMap::from([
            (1, 100),
            (2, 100),
            (3, 100),
            (4, 100),
            (5, 200),
            (6, 200),
            (7, 100),
            (8, 100),
            (9, 100),
            (10, 200),
            (11, 50),
            (12, 50),
            (13, 100),
            (14, 100),
            (15, 100),
            (16, 50),
            (17, 100),
            (18, 100),
        ]),
    );
    te.insert(
        12,
        BTreeMap::from([
            (1, 100),
            (2, 100),
            (3, 50),
            (4, 50),
            (5, 200),
            (6, 200),
            (7, 50),
            (8, 100),
            (9, 50),
            (10, 50),
            (11, 200),
            (12, 50),
            (13, 100),
            (14, 100),
            (15, 100),
            (16, 50),
            (17, 100),
            (18, 100),
        ]),
    );
    te.insert(
        13,
        BTreeMap::from([
            (1, 100),
            (2, 100),
            (3, 200),
            (4, 100),
            (5, 0),
            (6, 100),
            (7, 100),
            (8, 100),
            (9, 100),
            (10, 100),
            (11, 200),
            (12, 50),
            (13, 50),
            (14, 100),
            (15, 100),
            (16, 50),
            (17, 100),
            (18, 100),
        ]),
    );
    te.insert(
        14,
        BTreeMap::from([
            (1, 100),
            (2, 200),
            (3, 100),
            (4, 200),
            (5, 100),
            (6, 100),
            (7, 100),
            (8, 100),
            (9, 50),
            (10, 100),
            (11, 100),
            (12, 100),
            (13, 100),
            (14, 50),
            (15, 100),
            (16, 100),
            (17, 0),
            (18, 100),
        ]),
    );
    te.insert(
        15,
        BTreeMap::from([
            (1, 100),
            (2, 100),
            (3, 200),
            (4, 100),
            (5, 200),
            (6, 100),
            (7, 100),
            (8, 100),
            (9, 50),
            (10, 50),
            (11, 50),
            (12, 200),
            (13, 100),
            (14, 100),
            (15, 50),
            (16, 200),
            (17, 100),
            (18, 100),
        ]),
    );
    te.insert(
        16,
        BTreeMap::from([
            (1, 100),
            (2, 100),
            (3, 100),
            (4, 100),
            (5, 100),
            (6, 100),
            (7, 100),
            (8, 100),
            (9, 50),
            (10, 100),
            (11, 100),
            (12, 100),
            (13, 100),
            (14, 100),
            (15, 100),
            (16, 200),
            (17, 100),
            (18, 0),
        ]),
    );
    te.insert(
        17,
        BTreeMap::from([
            (1, 100),
            (2, 50),
            (3, 100),
            (4, 100),
            (5, 100),
            (6, 100),
            (7, 100),
            (8, 200),
            (9, 100),
            (10, 100),
            (11, 100),
            (12, 100),
            (13, 100),
            (14, 200),
            (15, 100),
            (16, 100),
            (17, 50),
            (18, 50),
        ]),
    );
    te.insert(
        18,
        BTreeMap::from([
            (1, 100),
            (2, 200),
            (3, 100),
            (4, 50),
            (5, 100),
            (6, 100),
            (7, 100),
            (8, 100),
            (9, 50),
            (10, 50),
            (11, 100),
            (12, 100),
            (13, 100),
            (14, 100),
            (15, 100),
            (16, 200),
            (17, 200),
            (18, 100),
        ]),
    );
    te
}
