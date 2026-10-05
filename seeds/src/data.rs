use std::sync::LazyLock;

use uuid::uuid;

use crate::CategoryInput;
use crate::PostInput;

pub static USER_ID: uuid::Uuid = uuid!("01a106a7-4327-7287-9940-af4254498604");
pub static CATEGORIES: LazyLock<Vec<CategoryInput>> = LazyLock::new(|| {
    [
        "Programming",
        "Web Development",
        "Database",
        "DevOps",
        "Cloud",
        "Security",
        "Architecture",
        "Tools",
        "Linux",
        "Other",
    ]
    .into_iter()
    .map(CategoryInput::new)
    .collect::<Vec<_>>()
});
pub static TAGS: LazyLock<Vec<String>> = LazyLock::new(|| {
    vec![
        "Rust".to_string(),
        "Typescript".to_string(),
        "Angular".to_string(),
        "PostgreSQL".to_string(),
        "Docker".to_string(),
        "AWS".to_string(),
        "Linux".to_string(),
        "REST API".to_string(),
        "Testing".to_string(),
        "Git".to_string(),
    ]
});

pub static POSTS: LazyLock<Vec<PostInput>> = LazyLock::new(|| {
    vec![
        PostInput::new(
            "Rust で REST API を作ってみる",
            "Axum を使った API 構築の導入",
            "Web Development",
            ["Rust", "REST API"],
        ),
        PostInput::new(
            "PostgreSQL のインデックス入門",
            "B-tree とインデックス設計",
            "Database",
            ["PostgreSQL"],
        ),
        PostInput::new(
            "Angular Signals を使った状態管理",
            "Signals の基本と利用例",
            "Web Development",
            ["Typescript", "Angular"],
        ),
        PostInput::new(
            "Docker Compose で開発環境を構築する",
            "PostgreSQL 等を含む開発環境",
            "Tools",
            ["Docker", "Git"],
        ),
        PostInput::new(
            "SQLx で PostgreSQL にアクセスする",
            "Pool、query!、トランザクション",
            "Database",
            ["PostgreSQL", "Rust"],
        ),
        PostInput::new(
            "Rust のエラーハンドリングを整理する",
            "Result、?、独自エラー",
            "Programming",
            ["Rust"],
        ),
        PostInput::new(
            "OpenAPI から API ドキュメントを生成する",
            "utoipa 等によるドキュメント生成",
            "DevOps",
            ["REST API"],
        ),
        PostInput::new(
            "UUID v7 を主キーとして使う",
            "UUID v7 の特徴と DB での利用",
            "Database",
            Vec::<&str>::new(),
        ),
        PostInput::new(
            "PostgreSQL の配列を使ってみる",
            "ARRAY、ANY、unnest の利用",
            "Database",
            ["PostgreSQL"],
        ),
        PostInput::new(
            "Web API のページネーション設計",
            "page/size、total、レスポンス設計",
            "Web Development",
            ["REST API"],
        ),
    ]
});
