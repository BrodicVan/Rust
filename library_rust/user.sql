drop table if exists user1;
create table user1 (
       id serial primary key,
       name varchar(140) not null,
       password varchar(140) not null,
	   is_mana boolean not null
);
