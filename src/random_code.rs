#[cfg(test)]
mod test {

    use itertools::Itertools;
    use std::vec;

    fn sum_2_smallest_num(num_arr: &[u32]) -> u32{
        let mut in_vec  = num_arr.to_vec();
        let new_slice: &mut [u32] = &mut in_vec;
        new_slice.sort();
        new_slice[0] + new_slice[1]
    }

    fn sum_2_smallest_num_v2(num_arr: &[u32]) -> u32{
        num_arr.iter().k_smallest(2).sum()
    }

    #[test]
    fn test_rndm(){
        dbg!("Hellow");
        let sum = sum_2_smallest_num(&[5, 3, 1, 2, 4]);
        dbg!(sum);

        let a = vec::from_elem(2,3);
        dbg!(a);

        let b = vec!(2,3);
        dbg!(b);
    }
}