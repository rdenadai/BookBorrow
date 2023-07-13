use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        manager
            .create_table(
                Table::create()
                    .table(Books::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Books::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .extra("DEFAULT uuid_generate_v4()".to_owned()),
                    )
                    .col(
                        ColumnDef::new(Books::Title)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(Books::Author).string().not_null())
                    .col(
                        ColumnDef::new(Books::YearOfPublication)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Books::Available)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .col(
                        ColumnDef::new(Books::CreatedAt)
                            .timestamp()
                            .extra("DEFAULT NOW()".to_owned()),
                    )
                    .col(ColumnDef::new(Books::UpdatedAt).timestamp())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Users::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .extra("DEFAULT uuid_generate_v4()".to_owned()),
                    )
                    .col(
                        ColumnDef::new(Users::Email)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(Users::Password).string().not_null())
                    .col(
                        ColumnDef::new(Users::Active)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .col(ColumnDef::new(Users::Name).string())
                    .col(ColumnDef::new(Users::Phone).string())
                    .col(ColumnDef::new(Users::Address).string())
                    .col(
                        ColumnDef::new(Users::CreatedAt)
                            .timestamp()
                            .extra("DEFAULT NOW()".to_owned()),
                    )
                    .col(ColumnDef::new(Users::UpdatedAt).timestamp())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Reservations::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Reservations::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .extra("DEFAULT uuid_generate_v4()".to_owned()),
                    )
                    .col(ColumnDef::new(Reservations::UserId).uuid().not_null())
                    .col(ColumnDef::new(Reservations::BookId).uuid().not_null())
                    .col(ColumnDef::new(Reservations::ReservationDate).timestamp())
                    .col(ColumnDef::new(Reservations::ReturnDate).timestamp())
                    .col(
                        ColumnDef::new(Reservations::CreatedAt)
                            .timestamp()
                            .extra("DEFAULT NOW()".to_owned()),
                    )
                    .col(ColumnDef::new(Reservations::UpdatedAt).timestamp())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-Reservations-Users_id-Users-id")
                            .from(Reservations::Table, Reservations::UserId)
                            .to(Users::Table, Users::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-Reservations-Books_id-Books-id")
                            .from(Reservations::Table, Reservations::BookId)
                            .to(Books::Table, Books::Id),
                    )
                    .to_owned(),
            )
            .await?;

        // Construct a `Statement` if the SQL contains value bindings
        // password = admin
        db.execute_unprepared(
            "
            INSERT INTO public.users 
                (id, email, password, active, created_at) 
                VALUES 
                (uuid_generate_v4(), 'admin@localhost', '21232f297a57a5a743894a0e4a801fc3', true, now());
            ",
        )
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Reservations::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Books::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await?;

        Ok(())
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Books {
    Table,
    Id,
    Title,
    Author,
    YearOfPublication,
    Available,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum Users {
    Table,
    Id,
    Email,
    Password,
    Active,
    Name,
    Phone,
    Address,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum Reservations {
    Table,
    Id,
    UserId,
    BookId,
    ReservationDate,
    ReturnDate,
    CreatedAt,
    UpdatedAt,
}
