use tga::test_entropies;

fn main() {
    test_entropies("testy/example0.tga");
    println!();
    test_entropies("testy/example1.tga");
    println!();
    test_entropies("testy/example2.tga");
    println!();
    test_entropies("testy/example3.tga");
}
