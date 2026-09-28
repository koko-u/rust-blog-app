-- migrate:up
CREATE TABLE "post_tags"
(
    "post_id"    uuid        NOT NULL,
    "tag_id"     uuid        NOT NULL,
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT now(),
    "updated_at" TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT "post_tags_pkey" PRIMARY KEY ("post_id", "tag_id"),
    CONSTRAINT "post_tags_post_pkey" FOREIGN KEY ("post_id") REFERENCES "posts" ("id") ON DELETE CASCADE,
    CONSTRAINT "post_tags_tag_pkey" FOREIGN KEY ("tag_id") REFERENCES "tags" ("id") ON DELETE CASCADE
);

CREATE INDEX "post_tags_post_idx" ON "post_tags" ("post_id");
CREATE INDEX "post_tags_tag_idx" ON "post_tags" ("tag_id");

CREATE TRIGGER "post_tag_update_at_tgr"
    BEFORE UPDATE
    ON post_tags
    FOR EACH ROW
EXECUTE PROCEDURE moddatetime("updated_at");

-- migrate:down
DROP TABLE "post_tags";
