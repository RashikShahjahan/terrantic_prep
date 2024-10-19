// Write a Rust function that merges two sorted arrays into a single sorted array.


fn mergesorted(arr1:&[i32],arr2:&[i32]){
    let small:&[i32];
    let big:&[i32];
    let mut result:Vec<i32> = vec![];
    if arr1.len()<arr2.len(){
        small = arr1;
        big = arr2;
    } 
    else{
        small = arr2;
        big = arr1;
    }

    let mut i =0;
    let  mut j =0;

    while i<small.len() && j<big.len()
    {
        if small[i]<big[j]{
            result.push(small[i]);
            i+=1;
        }
        else{
            result.push(big[j]);
            j+=1;
        }
    }

    while  i<small.len(){
        result.push(small[i]);
        i+=1;

    }

    while  j<big.len(){
        result.push(big[i]);
        j+=1;

    }

    println!("{:?}",result);
}

fn main(){
    
    mergesorted(&[1,2,9],&[6,7,8,8]);
}