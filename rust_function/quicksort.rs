
fn quicksort(array:&[i32])->Vec<i32>{
    if array.len()<=1{
        return array.to_vec();
    }

    let pivot_idx = array.len()/2;
    let pivot = array[pivot_idx];
    let left: Vec<i32>  = array.iter().filter(|&&x| x < pivot).copied().collect();
    let right: Vec<i32>  = array.iter().filter(|&&x| x > pivot).copied().collect();
    let mut middle: Vec<i32>  = array.iter().filter(|&&x| x == pivot).copied().collect();

    let mut sorted_left = quicksort(&left);
    let mut sorted_right = quicksort(&right);

    sorted_left.append(&mut middle);
    sorted_left.append(&mut sorted_right);

    return sorted_left;
}

fn main(){
    let arr = [4,53,44,35,64];
    quicksort(&arr);
}