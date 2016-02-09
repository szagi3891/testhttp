fn main() {
    
    
    println!("przedefiniowania");
    
    //pub trait TransportOut<R> {
    trait Converter<A,B> {
        fn converter(A) -> B;
    }
    
    struct clous<T,R> {
        funk : Box<Fn(T) -> R + 'static + Send + Sync>,
    }
    
    struct a {
        val : u32
    }
    
    struct b {
        val : a,
    }
    
    struct c {
        val : b,
    }
    
    struct d {
        val : c,
    }
    
    let tran1 = clous{
        funk : Box::new(|val: a|{
            b { val : val}
        })
    };
    
    
}
