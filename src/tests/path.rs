use crate::structs::path::astar;
use crate::structs::path::PathPoint;

#[test]
fn test_astar_diagonal_path() {
    let grid = vec![vec![2, 0, 0], vec![0, 0, 0], vec![0, 0, 3]];

    let path = astar(&grid).unwrap();
    assert_eq!(
        path,
        vec![
            PathPoint::from_tuple((0, 0)),
            PathPoint::from_tuple((1, 1)),
            PathPoint::from_tuple((2, 2))
        ]
    );
}

#[test]
fn test_astar_example() {
    let grid = vec![
        vec![0, 0, 0, 1, 0],
        vec![1, 1, 0, 1, 0],
        vec![2, 0, 0, 0, 3],
        vec![1, 0, 1, 1, 0],
        vec![0, 0, 0, 0, 0],
    ];

    let path = astar(&grid).unwrap();
    assert_eq!(
        path,
        vec![
            PathPoint::from_tuple((0, 2)),
            PathPoint::from_tuple((1, 2)),
            PathPoint::from_tuple((2, 2)),
            PathPoint::from_tuple((3, 2)),
            PathPoint::from_tuple((4, 2))
        ]
    );

    let grid = vec![
        vec![0, 0, 0, 1, 0],
        vec![1, 1, 0, 1, 0],
        vec![0, 0, 0, 0, 3],
        vec![1, 0, 1, 1, 0],
        vec![2, 0, 0, 0, 0],
    ];
    let path = astar(&grid).unwrap();
    assert_eq!(
        path,
        vec![
            PathPoint::from_tuple((0, 4)),
            PathPoint::from_tuple((1, 3)),
            PathPoint::from_tuple((2, 2)),
            PathPoint::from_tuple((3, 2)),
            PathPoint::from_tuple((4, 2)),
        ]
    );
}
