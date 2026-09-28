-- migrate:up
CREATE TABLE "posts"
(
    "id"          uuid         NOT NULL DEFAULT uuidv7(),
    "user_id"     uuid         NOT NULL,
    "category_id" uuid         NOT NULL,
    "title"       varchar(255) NOT NULL,
    "slug"        varchar(255) NOT NULL,
    "content"     text         NULL,
    "created_at"  TIMESTAMPTZ  NOT NULL DEFAULT now(),
    "updated_at"  TIMESTAMPTZ  NOT NULL DEFAULT now(),
    CONSTRAINT "posts_pkey" PRIMARY KEY ("id"),
    CONSTRAINT "post_category_fkey" FOREIGN KEY ("category_id") REFERENCES "categories" ("id") ON DELETE CASCADE,
    CONSTRAINT "post_slug_unique" UNIQUE ("slug")
);

CREATE INDEX "post_user_idx" ON "posts" ("user_id");
CREATE INDEX "post_category_id" ON "posts" ("category_id");

CREATE TRIGGER "post_update_at_tgr"
    BEFORE UPDATE
    ON posts
    FOR EACH ROW
EXECUTE PROCEDURE moddatetime("updated_at");

-- migrate:down
DROP TABLE "posts";
