-- Your SQL goes here
create extension if not exists cube;
create extension if not exists earthdistance;


create function update_update_on()
returns trigger as $$
begin
	NEW.update_on = now();
	return NEW;
end
$$ language 'plpgsql';

create table users (
	id serial not null,
	name varchar not null,
	phone varchar not null,
	password varchar not null,
	salt varchar not null,
	create_on timestamp default current_timestamp not null,
	update_on timestamp default current_timestamp not null,
	primary key (id)
);


create trigger update_users
	before update
	on
		users
	for each row
execute procedure update_update_on();


create table playings (
	id serial not null,
	name varchar not null,
	latitude double precision not null,
	longitude double precision not null,
	discoverer int not null references users (id),
	create_on timestamp default current_timestamp not null,
	update_on timestamp default current_timestamp not null,
	primary key (id)
);

create index playings_latitude_longitude on playings using gist (ll_to_earth(latitude, longitude));

create trigger update_playings
	before update
	on
		playings
	for each row
execute procedure update_update_on();

create table eatings (
	id serial not null,
	name varchar not null,
	latitude double precision not null,
	longitude double precision not null,
	discoverer int not null references users (id),
	create_on timestamp default current_timestamp not null,
	update_on timestamp default current_timestamp not null,
	primary key (id)
);

create index eatings_latitude_longitude on eatings using gist (ll_to_earth(latitude, longitude));

create trigger update_eatings
	before update
	on
		eatings
	for each row
execute procedure update_update_on();
