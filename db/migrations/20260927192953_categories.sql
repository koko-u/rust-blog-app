-- migrate:up
CREATE TABLE "categories"
(
    "id"         uuid         NOT NULL DEFAULT uuidv7(),
    "user_id"    uuid         NOT NULL,
    "name"       varchar(255) NOT NULL,
    "slug"       varchar(255) NOT NULL,
    "created_at" TIMESTAMPTZ  NOT NULL DEFAULT now(),
    "updated_at" TIMESTAMPTZ  NOT NULL DEFAULT now(),
    CONSTRAINT "categories_pkey" PRIMARY KEY ("id"),
    CONSTRAINT "category_slug_unique" UNIQUE ("slug"),
    CONSTRAINT "category_id_user_id_unique" UNIQUE ("user_id", "id")
);

CREATE INDEX "category_name_idx" ON "categories" ("name");
CREATE INDEX "category_name_like_idx" ON "categories" USING gin ("name" gin_trgm_ops);
CREATE INDEX "category_user_idx" ON "categories" ("user_id");

CREATE TRIGGER "category_update_at_tgr"
    BEFORE UPDATE
    ON categories
    FOR EACH ROW
EXECUTE PROCEDURE moddatetime("updated_at");

-- migrate:down
DROP TABLE "categories";
