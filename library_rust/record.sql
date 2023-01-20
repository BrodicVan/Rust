drop table if exists record;
create table record (
       id serial primary key,
       user_id int not null,
       book_id int not null,
       borrow_time TIMESTAMP default now(),
       return_time TIMESTAMP,
       writer varchar(140) not null,
       press varchar(140) not null,
	   is_return boolean not null
);
