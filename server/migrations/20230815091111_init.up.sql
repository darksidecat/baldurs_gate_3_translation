CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

create or replace function set_updated_at()
    returns trigger as
$$
begin
    NEW.updated_at = now();
    return NEW;
end;
$$ language plpgsql;

create or replace function trigger_updated_at(tablename regclass)
    returns void as
$$
begin
    execute format('CREATE TRIGGER set_updated_at
        BEFORE UPDATE
        ON %s
        FOR EACH ROW
        WHEN (OLD is distinct from NEW)
    EXECUTE FUNCTION set_updated_at();', tablename);
end;
$$ language plpgsql;


CREATE TABLE translation_location (
  contentuid TEXT NOT NULL UNIQUE,
  created_at TIMESTAMP NOT NULL default now(),
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
  created_at TIMESTAMP NOT NULL DEFAULT now(),
  updated_at TIMESTAMP,
  PRIMARY KEY (id),
  CONSTRAINT fk_translation_location FOREIGN KEY(contentuid) REFERENCES translation_location(contentuid)
);

SELECT trigger_updated_at('"translation_variant"');

CREATE TABLE vocabulary (
  id uuid NOT NULL DEFAULT uuid_generate_v4(),
  word TEXT NOT NULL UNIQUE,
  created_at TIMESTAMP NOT NULL DEFAULT now(),
  PRIMARY KEY (id)
);

CREATE TABLE vocabulary_usages (
  id uuid NOT NULL DEFAULT uuid_generate_v4(),
  word_id uuid NOT NULL,
  translation_variant_id UUID NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT now(),
  CONSTRAINT fk_translation_translation_variant FOREIGN KEY(translation_variant_id) REFERENCES translation_variant(id),
  CONSTRAINT fk_vocabulary FOREIGN KEY(word_id) REFERENCES vocabulary(id),
  PRIMARY KEY (id)
);