fn main() {
    let mut vet = Vec::<i32>::new();
    vet.push(1);
    vet.push(2);

    println!("before clearing vet: {:?}", vet);
    vet.clear();
    println!("after clearing vet: {:?}", vet);
}
