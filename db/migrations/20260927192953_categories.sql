-- migrate:up
CREATE TABLE "categories"
(
    "id"         uuid         NOT NULL DEFAULT uuidv7(),
    "name"       varchar(255) NOT NULL,
    "slug"       varchar(255) NOT NULL,
    "created_at" TIMESTAMPTZ  NOT NULL DEFAULT now(),
    "updated_at" TIMESTAMPTZ  NOT NULL DEFAULT now(),
    CONSTRAINT "categories_pkey" PRIMARY KEY ("id"),
    CONSTRAINT "category_slug_unique" UNIQUE ("slug")
);

CREATE INDEX "category_name_idx" ON "categories" ("name");
CREATE INDEX "category_name_like_idx" ON "categories" USING gin ("name" gin_trgm_ops);

CREATE TRIGGER "category_update_at_tgr"
    BEFORE UPDATE
    ON categories
    FOR EACH ROW
EXECUTE PROCEDURE moddatetime("updated_at");

-- migrate:down
DROP TABLE "categories";
