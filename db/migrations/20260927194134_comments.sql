-- migrate:up
CREATE TABLE "comments"
(
    "id"         uuid        NOT NULL DEFAULT uuidv7(),
    "post_id"    uuid        NOT NULL,
    "user_id"    uuid        NOT NULL,
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT now(),
    "updated_at" TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT "comments_pkey" PRIMARY KEY ("id"),
    CONSTRAINT "comment_post_fkey" FOREIGN KEY ("post_id") REFERENCES "posts" ("id") ON DELETE CASCADE
);

CREATE INDEX "comment_post_idx" ON "comments" ("post_id");
CREATE INDEX "comment_user_idx" ON "comments" ("user_id");

CREATE TRIGGER "comment_update_at_tgr"
    BEFORE UPDATE
    ON comments
    FOR EACH ROW
EXECUTE PROCEDURE moddatetime("updated_at");

-- migrate:down
DROP TABLE "comments";
