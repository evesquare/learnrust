use rand::Rng;

fn main(){
    //乱数生成機を初期化
    let mut rng = rand::thread_rng();
    //True or Falseをそれぞれ0.5の確率で生成
    if rng.gen() { // random bool
        //rng.gen<T>のTの部分で型を指定しそれにあった乱数を生成
        println!("i32: {}, u32: {}", rng.gen::<i32>(), rng.gen::<u32>());   
    }
    println!("True or False:: {}", rng.gen::<bool>());

}