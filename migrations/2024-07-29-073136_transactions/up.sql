-- Your SQL goes here
create table if not exists transactions (
    id serial primary key,
    user_id integer references users(id) not null,
    photo_id integer references images(id) not null,
    amount bigint not null,
    mpesa_number varchar(20),
    mpesa_transaction_id varchar(255),
    status varchar(50),
    created_at timestamp not null default now()
);



