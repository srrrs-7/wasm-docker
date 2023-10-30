pub fn starlin() {
    let arr = vec![1, 2, 0, 8, 3, 7, 5, 6, 1, 2, 8, 9, 0, 7, 3, 4];
    let mut starlin_arr: Vec<i32> = vec![];

    starlin_arr.push(arr[0]);
    let mut idx = 1;
    let mut starlin_idx = starlin_arr.len();

    while idx != arr.len() {
        if starlin_arr[starlin_idx - 1] < arr[idx] {
            starlin_arr.push(arr[idx]);
            starlin_idx += 1;
        }
        idx += 1;
        continue;
    }
    println!("{:?}", starlin_arr);
}
