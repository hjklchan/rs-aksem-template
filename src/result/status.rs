use serde::Serialize;

#[derive(Serialize)]
pub struct ReplyTemplate<T>
where
    T: Serialize,
{
    /// 六位的错误码
    pub code: u32,
    /// 错误消息
    pub message: String,
    /// 数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

/// ### Status 包含两个变体
///
/// Good(Option<T>) 好消息好消息!  
/// Bad(u32, String) 坏消息坏消息!
///
/// - 第一个变体存放响应数据
/// - 第二个变体存放构建错误的错误码和错误描述消息
///
/// Note: T 必须实现 Serialize 特征
pub enum Status<T>
where
    T: Serialize,
{
    Good(Option<T>),
    Bad(u32, String),
}

impl<T> Status<T>
where
    T: Serialize,
{
    pub fn to_reply(self) -> ReplyTemplate<T> {
        // 匹配状态并直接返回
        match self {
            // 返回好消息模板实例
            Self::Good(data) => ReplyTemplate {
                code: 0,
                message: "ok".into(),
                data: data,
            },
            // 返回坏消息模板实例
            Self::Bad(code, message) => ReplyTemplate {
                code,
                message,
                data: None,
            },
        }
    }
}
