drop table if exists book;
create table book (
       id serial primary key,
       name varchar(140) not null,
       writer varchar(140) not null,
       press varchar(140) not null,
	   is_borrowed boolean not null
);
