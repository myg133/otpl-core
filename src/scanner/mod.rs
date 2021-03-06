mod bytes_scanner;

pub use self::bytes_scanner::BytesScanner;
use {Result};
use token::Token;
use std::fmt::Debug;
use std::path::Path;
use std::str::from_utf8_unchecked;

/// 定义的要解析的输入源。
pub trait Source: Debug {
    /// 获取给定 `Token` 的用于定位源的行号.
    fn line(&self, offset: usize) -> usize;
    /// 获取给定 `Token` 的用于定位源的行的开始位置.
    fn column(&self, offset: usize) -> usize;
    /// 获取给定 `Token` 的输入源文件名.
    /// 注意：该文件名只是用于错误定位的提示。
    fn filename(&self) -> &Path;
    /// 获取源
    fn body(&self) -> &[u8];
}

pub trait Tokenizer: Debug {
    fn back_token(&mut self, tok: Token);
    fn scan(&mut self) -> Result<Token>;
    fn source(&self) -> &Source;
    /// 标记一个还原点
    fn mark(&mut self);
    /// 取消一个还原点
    fn unmark(&mut self);
    /// 重新设置最新的还原点
    fn reset(&mut self);
    /// 创建子扫描
    fn new_tokenizer(&self,source: &[u8], filename: &Path) -> Box<Tokenizer>;
}