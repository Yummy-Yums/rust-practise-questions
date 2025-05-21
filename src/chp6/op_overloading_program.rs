pub mod op_overloading_program {

    use std::iter;
    use std::vec::IntoIter;

    pub fn combine_vecs_explicit_return_types(
        v: Vec<i32>,
        u: Vec<i32>
    ) -> iter::Cycle<iter::Chain<IntoIter<i32>, IntoIter<i32>>> {
        v.into_iter().chain(u.into_iter()).cycle()
    }

    pub fn combine_vecs(
        v: Vec<i32>,
        u: Vec<i32>,
    ) -> impl Iterator<Item=i32> {
        v.into_iter().chain(u.into_iter()).cycle()
    }

    pub fn double_positives(numbers: &Vec<i32>) -> impl Iterator<Item=i32> + '_ {
        numbers
            .iter()
            .filter(|x| x > &&0)
            .map(|x| x * 2)
    }

}

mod tests{
    use crate::chp6::op_overloading_program::op_overloading_program::*;

    #[test]
    pub fn test_combine_vecs(){

        let v1 = vec![1, 2, 3];
        let v2 = vec![4, 5, 6];

        let mut v3  = combine_vecs(v1, v2);

        assert_eq!(Some(1), v3.next());
        assert_eq!(Some(2), v3.next());
        assert_eq!(Some(3), v3.next());
        assert_eq!(Some(4), v3.next());
        assert_eq!(Some(5), v3.next());

    }

    #[test]
    pub fn test_double_positives(){

        let singles = vec![-3, -2, 2, 3];
        let doubles = double_positives(&singles);
        assert_eq!(doubles.collect::<Vec<i32>>(), vec![4, 6]);

    }

}