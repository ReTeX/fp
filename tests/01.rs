extern crate fp;
use fp::F24P8;

#[test]
fn add() {
    let fp_2 = F24P8::from(2.0);
    let fp_3 = F24P8::from(3.0);
    assert_eq!(fp_2 + fp_2.half(), fp_3);
}#[test]
fn sub() {
    let fp_2 = F24P8::from(2.0);
    let fp_3 = F24P8::from(3.0);
    assert_eq!(fp_3 - fp_2.half(), fp_2);
}
#[test]
fn mul() {
    let fp_2 = F24P8::from(2.0);
    let fp_3 = F24P8::from(3.0);
    assert_eq!(fp_2 * fp_3, fp_3 * fp_2);
    assert_eq!(fp_2 * fp_3.half(), fp_3);
}
#[test]
fn print() {
    let fp_2 = F24P8::from(2.0);
    let fp_3 = F24P8::from(3.0);
    println!("{} + {} = {}", fp_2, fp_3, fp_2 + fp_3);
    println!("{} - {} = {}", fp_3, fp_2, fp_3 - fp_2);
    println!("{} * {} = {}", fp_2, fp_3, fp_2 * fp_3);
}
