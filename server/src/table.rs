use sea_query::Iden;

#[derive(Iden)]
pub enum TranslationLocation {
    Table,
    Contentuid,
}

#[derive(Iden)]
pub enum TranslationVariant {
    Table,
    Id,
    Contentuid,
    FilePath,
    LocalizationDate,
    Lang,
    Version,
    Text
}