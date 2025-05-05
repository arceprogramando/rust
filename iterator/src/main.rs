fn main() {
    println!("Hello, world!");
    let v1 = vec![1, 2, 3];
    
    let v1_iter = v1.iter();
    
    for val in v1_iter {
        println!("Got: {val}");
    }

    let v2 = vec![1, 2, 3];

        let v2_iter = v2.iter();

        let total: i32 = v2_iter.sum();
    
    println!("La Suma es de : {:?} ",total);
    
    
}
/*
#[test]
    fn iterator_demonstration() {
        let v1 = vec![1, 2, 3];

        let mut v1_iter = v1.iter();

        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }
#[test]
    fn iterator_sum() {
        let v2 = vec![1, 2, 3];

        let v2_iter = v2.iter();

        let total: i32 = v2_iter.sum();

        assert_eq!(total, 6);
    }

 */
