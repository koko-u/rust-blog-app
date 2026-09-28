-- migrate:up
CREATE TABLE "tags"
(
    "id"         uuid         NOT NULL DEFAULT uuidv7(),
    "name"       varchar(255) NOT NULL,
    "created_at" TIMESTAMPTZ  NOT NULL DEFAULT now(),
    "updated_at" TIMESTAMPTZ  NOT NULL DEFAULT now(),
    CONSTRAINT "tags_pkey" PRIMARY KEY ("id"),
    CONSTRAINT "tag_name_unique" UNIQUE ("name")
);

CREATE TRIGGER "tag_update_at_tgr"
    BEFORE UPDATE
    ON tags
    FOR EACH ROW
EXECUTE PROCEDURE moddatetime("updated_at");

-- migrate:down
DROP TABLE "tags";
