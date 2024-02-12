fn main() {
    let mut a: [i8; 10] = [42; 10];
    a[5] = 0;
    println!("a: {a:#?}");
    let t: (i8, bool) = (7, true);
    println!("t.0: {}", t.0);
    println!("t.1: {}", t.1);

    // destructuring
    let point = (3, 4);
    match point {
        (0, _) => println!("on Y axis"),
        (_, 0) => println!("on X axis"),
        (x, _) if x < 0 => println!("left of Y axis"),
        (_, y) if y < 0 => println!("below X axis"),
        _ => println!("first quadrant"),
    }
    let triple = [0, -2, 3];
    println!("Tell me about {triple:?}");
    match triple {
        [0, y, z] => println!("First is 0, y = {y}, and z = {z}"),
        [1, ..]   => println!("First is 1 and the rest were ignored"),
        _         => println!("All elements were ignored"),
    }
    // transpose
    let matrix = [
        [101, 102, 103], // <-- the comment makes rustfmt add a newline
        [201, 202, 203],
        [301, 302, 303],
    ];

    println!("matrix: {:#?}", matrix);
    let transposed = transpose(matrix);
    println!("transposed: {:#?}", transposed);

    // iter
    // enumerateがなぜ参照を返すのか https://stackoverflow.com/questions/34733811/what-is-the-difference-between-iter-and-into-iter
    // https://hermanradtke.com/2015/06/22/effectively-using-iterators-in-rust.html/
    let names = vec!["Jane", "Jill", "Jack", "John"];

    // iter_into()だとuse_names_for_something_elseが動かない
    // use of moved value: `names`. value used here after move
    let total_bytes = names
        .iter()
        .map(|name: &&str| name.len())
        .fold(0, |acc, len| acc + len );

    assert_eq!(total_bytes, 16);
    use_names_for_something_else(names);

    let x = vec!["Jill", "Jack", "Jane", "John"];

    // cloneで作った値の所有権がiterだと、関数内でfreeされるので、iter_intoで所有権をこっちに戻さないといけない
    let r = x
        .clone()
        .into_iter()
        .collect::<Vec<_>>();
    println!("{:?}", r);

    // reference
    // Calculate the magnitude of a vector by summing the squares of its coordinates
    // and taking the square root. Use the `sqrt()` method to calculate the square
    // root, like `v.sqrt()`.

    // Use the following `main` to test your work.
    println!("Magnitude of a unit vector: {}", magnitude(&[0.0, 1.0, 0.0]));

    let mut v = [1.0, 2.0, 9.0];
    println!("Magnitude of {v:?}: {}", magnitude(&v));
    normalize(&mut v);
    println!("Magnitude of {v:?} after normalization: {}", magnitude(&v))
}


fn magnitude(v: &[f64; 3]) -> f64 {
    v.iter().fold(0.0, |acc, vl| acc + vl * vl).sqrt()
}

// Normalize a vector by calculating its magnitude and dividing all of its
// coordinates by that magnitude.
fn normalize(v: &mut[f64; 3]) {
    let mag = magnitude(v);
    for vl in v {
        *vl = *vl / mag;
    }
}

fn use_names_for_something_else(names: Vec<&str>) {
    println!("{:?}", names)
}

fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut transposed = [[0; 3]; 3];

    for (i, row) in matrix.iter().enumerate() {
        for (j, &element) in row.iter().enumerate() {
            transposed[j][i] = element;
        }
    }

    transposed
}
#[test]
fn test_transpose() {
    let matrix = [
        [101, 102, 103], //
        [201, 202, 203],
        [301, 302, 303],
    ];
    let transposed = transpose(matrix);
    assert_eq!(
        transposed,
        [
            [101, 201, 301], //
            [102, 202, 302],
            [103, 203, 303],
        ]
    );
}
