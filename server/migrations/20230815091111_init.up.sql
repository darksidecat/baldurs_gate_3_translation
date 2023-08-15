CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE translation_location (
  contentuid TEXT NOT NULL,
  PRIMARY KEY (contentuid)
);

CREATE TABLE translation_variant (
  id uuid NOT NULL DEFAULT uuid_generate_v4(),
  contentuid TEXT NOT NULL,
  file_path TEXT NOT NULL,
  localization_date TIMESTAMP NOT NULL,
  lang TEXT NOT NULL,
  version INT NOT NULL,
  text TEXT NOT NULL,
  PRIMARY KEY (id),
  CONSTRAINT fk_translation_location FOREIGN KEY(contentuid) REFERENCES translation_location(contentuid)
);

CREATE TABLE vocabulary (
  id uuid NOT NULL DEFAULT uuid_generate_v4(),
  word TEXT NOT NULL,
  PRIMARY KEY (id)
);

CREATE TABLE vocabulary_usages (
  id uuid NOT NULL DEFAULT uuid_generate_v4(),
  word_id uuid NOT NULL,
  translation_variant_id UUID NOT NULL,
  CONSTRAINT fk_translation_translation_variant FOREIGN KEY(translation_variant_id) REFERENCES translation_variant(id),
  CONSTRAINT fk_vocabulary FOREIGN KEY(word_id) REFERENCES vocabulary(id),
  PRIMARY KEY (id)
);