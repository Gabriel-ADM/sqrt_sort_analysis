pub fn sqrt_sort(n: &Vec<i32>, sort_type: String) -> Vec<i32> {
    let len = n.len() as f64;
    let sqrt = len.sqrt();
    let mut aux_vecs = vec![];
    let mut result = vec![];
    // while partition_size < sqrt {
    //     println!("{}", partition_size* sqrt);
    //     let mut aux_vec = vec![];
    //     for i in (sqrt * partition_size) as usize..(sqrt * (partition_size + 1.0)) as usize {
    //         aux_vec.push(n[i]);
    //     }
    //     if len % sqrt != 0.0 && partition_size == sqrt - 1.0 {
    //         partition_size += len % sqrt;
    //     } else {
    //         partition_size += 1.0;
    //     }

    //     if sort_type == "bubble" {
    //         bubble_sort(&mut aux_vec);
    //     } else if sort_type == "heap" {
    //         heap_sort(&mut aux_vec);
    //     } else {
    //         panic!("Valid sort types are 'heap' and 'bubble'");
    //     }
    //     aux_vecs.push(aux_vec);
    // }
    for i in 0..sqrt as usize{
        let start = i * sqrt as usize;
        let end = (i + 1) * sqrt as usize;
        let mut aux_vec = n[start..end].to_vec();
        
        if sort_type == "bubble" {
            bubble_sort(&mut aux_vec);
        } else if sort_type == "heap" {
            heap_sort(&mut aux_vec);
        } else {
            panic!("Valid sort types are 'heap' and 'bubble'");
        }
        aux_vecs.push(aux_vec);
    }
    while !aux_vecs.is_empty() {
        let (mut biggest, mut biggest_pos): (i32, usize) = (-1, 0);
        for (i, smaller_vec) in aux_vecs.iter().enumerate() {
            let bigger_in_vec = smaller_vec[smaller_vec.len() - 1];
            if bigger_in_vec > biggest {
                biggest = bigger_in_vec;
                biggest_pos = i;
            }
        }
        result.insert(0, aux_vecs[biggest_pos].pop().unwrap());

        aux_vecs.retain(|x| !x.is_empty());
    }
    // println!("{:?}", result);
    return result;
}

pub fn bubble_sort(n: &mut Vec<i32>) {
    let mut swaped = true;
    while swaped {
        swaped = false;
        for j in 0..n.len() - 1 {
            if n[j] > n[j + 1] {
                let aux = n[j];
                n[j] = n[j + 1];
                n[j + 1] = aux;
                swaped = true;
            }
        }
    }
}

fn heapify(n: &mut Vec<i32>, point: usize, i: usize) {
    let (mut largest, left, right) = (i, 2 * i + 1, 2 * i + 2);
    if left < point && n[left] > n[largest] {
        largest = left;
    }
    if right < point && n[right] > n[largest] {
        largest = right;
    }
    if largest != i {
        let aux = n[i];
        n[i] = n[largest];
        n[largest] = aux;
        heapify(n, point, largest);
    }
}

pub fn heap_sort(n: &mut Vec<i32>) {
    for i in (0..n.len() / 2 + 1).rev() {
        heapify(n, n.len(), i);
    }
    for i in (1..n.len()).rev() {
        let aux = n[0];
        n[0] = n[i];
        n[i] = aux;

        heapify(n, i, 0);
    }
}
