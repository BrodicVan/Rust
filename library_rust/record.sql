drop table if exists record;
create table record (
       id serial primary key,
       user_id int not null,
       book_id int not null,
       borrow_time TIMESTAMP default now(),
       return_time TIMESTAMP,
	is_return boolean not null
);
