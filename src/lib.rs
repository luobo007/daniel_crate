//! # utils
//! A module for mathematic calculations!  
pub mod utils {
    /// Add two numbers
    /// # Examples
    /// ```rust
    /// use {template_lib_crate::utils::add};
    /// assert_eq!(add(1,1),2);
    /// ```
    /// # Panic
    /// parameters or result large then 255 will panic
    ///
    pub fn add(left: u8, right: u8) -> u8 {
        left + right
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::add;

    /// 测试正常的加法运算
    ///
    /// 此测试用例验证`add`函数在正常输入时是否返回正确的加法结果。
    #[test]
    fn it_works() {
        let result = add(1, 1);
        assert_eq!(result, 2); // 检查1 + 1的结果是否为2
    }

    /// 测试当加法结果超过u8类型的最大值时的panic行为
    ///
    /// 由于u8的最大值为255，当输入参数之和超过255时，程序应触发panic。
    #[test]
    #[should_panic]
    fn it_should_panic() {
        crate::utils::add(255, 1); // 由于结果超过255，预期会触发panic
    }
}

