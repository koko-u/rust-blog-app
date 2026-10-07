use std::sync::LazyLock;

use uuid::uuid;

use crate::CategoryInput;
use crate::PostInput;
use crate::TagInput;

pub static ALICE_USER_ID: uuid::Uuid = uuid!("4e4092c6-3b74-4609-aaef-1d52429f3926");
pub static BOB_USER_ID: uuid::Uuid = uuid!("75f28300-5445-4c9e-bcf2-3337ab77d296");

pub static CATEGORIES: LazyLock<Vec<CategoryInput>> = LazyLock::new(|| {
    [
        ("Programming", ALICE_USER_ID),
        ("Web Development", ALICE_USER_ID),
        ("Database", ALICE_USER_ID),
        ("DevOps", ALICE_USER_ID),
        ("Cloud", ALICE_USER_ID),
        ("Security", ALICE_USER_ID),
        ("Architecture", ALICE_USER_ID),
        ("Tools", ALICE_USER_ID),
        ("Linux", ALICE_USER_ID),
        ("Other", ALICE_USER_ID),
        ("Programming", BOB_USER_ID),
        ("Database", BOB_USER_ID),
        ("Other", BOB_USER_ID),
    ]
    .into_iter()
    .map(|(name, user_id)| CategoryInput::new(name, user_id))
    .collect::<Vec<_>>()
});
pub static TAGS: LazyLock<Vec<TagInput>> = LazyLock::new(|| {
    vec![
        ("Rust", ALICE_USER_ID),
        ("Typescript", ALICE_USER_ID),
        ("Angular", ALICE_USER_ID),
        ("PostgreSQL", ALICE_USER_ID),
        ("Docker", ALICE_USER_ID),
        ("AWS", ALICE_USER_ID),
        ("Linux", ALICE_USER_ID),
        ("REST API", ALICE_USER_ID),
        ("Testing", ALICE_USER_ID),
        ("Git", ALICE_USER_ID),
        ("Rust", BOB_USER_ID),
        ("Typescrippt", BOB_USER_ID),
        ("PostgreSQL", BOB_USER_ID),
        ("MySQL", BOB_USER_ID),
    ]
    .into_iter()
    .map(|(name, user_id)| TagInput::new(name, user_id))
    .collect::<Vec<_>>()
});

pub static POSTS: LazyLock<Vec<PostInput>> = LazyLock::new(|| {
    vec![
        PostInput::new(
            "Rust で REST API を作ってみる",
            ALICE_USER_ID,
            "Axum を使った API 構築の導入",
            "Web Development",
            ["Rust", "REST API"],
        ),
        PostInput::new(
            "PostgreSQL のインデックス入門",
            BOB_USER_ID,
            "B-tree とインデックス設計",
            "Database",
            ["PostgreSQL"],
        ),
        PostInput::new(
            "Angular Signals を使った状態管理",
            ALICE_USER_ID,
            "Signals の基本と利用例",
            "Web Development",
            ["Typescript", "Angular"],
        ),
        PostInput::new(
            "Docker Compose で開発環境を構築する",
            ALICE_USER_ID,
            "PostgreSQL 等を含む開発環境",
            "Tools",
            ["Docker", "Git"],
        ),
        PostInput::new(
            "SQLx で PostgreSQL にアクセスする",
            BOB_USER_ID,
            "Pool、query!、トランザクション",
            "Database",
            ["PostgreSQL", "Rust"],
        ),
        PostInput::new(
            "Rust のエラーハンドリングを整理する",
            BOB_USER_ID,
            "Result、?、独自エラー",
            "Programming",
            ["Rust"],
        ),
        PostInput::new(
            "OpenAPI から API ドキュメントを生成する",
            ALICE_USER_ID,
            "utoipa 等によるドキュメント生成",
            "DevOps",
            ["REST API"],
        ),
        PostInput::new(
            "UUID v7 を主キーとして使う",
            ALICE_USER_ID,
            "UUID v7 の特徴と DB での利用",
            "Database",
            Vec::<&str>::new(),
        ),
        PostInput::new(
            "PostgreSQL の配列を使ってみる",
            BOB_USER_ID,
            "ARRAY、ANY、unnest の利用",
            "Database",
            ["PostgreSQL"],
        ),
        PostInput::new(
            "Web API のページネーション設計",
            ALICE_USER_ID,
            "page/size、total、レスポンス設計",
            "Web Development",
            ["REST API"],
        ),
    ]
});
