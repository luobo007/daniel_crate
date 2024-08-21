use daniel_crate::utils::{self, add};
/// 测试`utils`模块的集成效果
///
/// 此测试用例验证`add`函数在一个完整应用中的表现，确保它在实际使用场景中能够正常工作。
#[test]
fn test_add_integration() {
    let result = add(100, 100);
    assert_eq!(result, 200); // 验证100 + 100的结果是否为200
}

fn main() {
    let sum = add(1, 1);
    assert_eq!(sum, 2);
    println!("sum is: {}", sum);
}
