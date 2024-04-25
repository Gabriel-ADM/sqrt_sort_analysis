pub fn sqrt_sort(n: &mut Vec<i32>, sort_type: &str) {
    let len = n.len() as f64;
    let sqrt = len.sqrt();
    let mut aux_vecs = vec![];
    for chunk in n.chunks(sqrt as usize) {
        let mut chunk_vec = chunk.to_vec();
        
        match sort_type {
            "bubble" => bubble_sort(&mut chunk_vec),
            "heap" => build_max_heap(&mut chunk_vec),
            _ => panic!("Valid sort types are 'heap' and 'bubble'"),
        }

        aux_vecs.push(chunk_vec);
    }
    n.clear();
    
    while !aux_vecs.is_empty() {
        let mut max_index = 0;
        for i in 1..aux_vecs.len() {
            if aux_vecs[i].last() > aux_vecs[max_index].last() {
                max_index = i;
            }
        }
        match sort_type {
            "bubble" => n.push(aux_vecs[max_index].pop().unwrap()),
            "heap" => n.push(heap_pop(&mut aux_vecs[max_index])),
            _ => panic!("Valid sort types are 'heap' and 'bubble'"),
        }
        ;
        if aux_vecs[max_index].is_empty() {
            aux_vecs.swap_remove(max_index);
        }
    }
    n.reverse();
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

pub fn heapify(n: &mut Vec<i32>, point: usize, i: usize) {
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

pub fn build_max_heap(n: &mut Vec<i32>) {
    let heap_size = n.len();
    
    for i in (0..heap_size / 2).rev() {
        heapify(n, heap_size, i);
    }
}


pub fn heap_pop(arr: &mut Vec<i32>) -> i32 {    
    let heap_size = arr.len();
    
    // Swap root (maximum element) with the last element
    arr.swap(0, heap_size - 1);
    
    // Extract the maximum element (previously root)
    let max_value = arr.pop().unwrap(); // Safe to unwrap because the vector is not empty
    
    // Restore heap property by heapifying from the root
    heapify(arr, heap_size - 1, 0);
    
    max_value
}


pub fn _heap_sort(n: &mut Vec<i32>) {
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
